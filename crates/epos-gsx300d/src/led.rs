//! EPOS GSX 300 LED Ring Control via USB HID
//!
//! The GSX 300 has an LED ring around the volume dial:
//!   - Blue  = Stereo (2.0)
//!   - Red   = Surround (7.1)
//!   - Pink  = both bits set (0x03) — not used by the daemon
//!
//! HID Report Descriptor — fully decoded (120 bytes, verified on hardware):
//!   Report ID 0x01 (Consumer): volume dial — input bits 0x09E9 (up) / 0x09EA
//!     (down) / 0x09CF (mute). Incremental detents only; NO absolute readback.
//!   Report ID 0x02 (Vendor 0xFF13, 1-byte):
//!     Output 2 bits → descriptor usages 0x05 (bit0) / 0x06 (bit1). The
//!     descriptor *names* them blue/red, but the firmware renders them the
//!     other way round. Hardware-measured 2026-09-24 by writing one payload at
//!     a time and observing the ring:
//!       0x00 = off, 0x01 = red, 0x02 = blue, 0x03 = both
//!     The colour bytes live in `LedProbeConfig` (config.json `led_probe`) for
//!     precisely this reason — trust the measurement over the descriptor
//!     labels, which are wrong on this firmware.
//!     Input  3 bits → wire 0x01 (stereo) / 0x02 (7.1) / 0x04 (long-press),
//!     hardware-confirmed (AGENT-FINDINGS §3.2). Remaining bits constant
//!     padding (must be zero).
//!   Report ID 0x04 (Output 38B) / 0x05 (Input 34B): **memory bus** — read
//!     pages with `[flags, len, addr_hi, addr_lo]` payload (flags: 0x00=RAM,
//!     0x20=EEPROM, 0x10=high page ≥0x10000). Read-only; NEVER set bit6
//!     (EEPROM write = firmware flash = brick). See docs/reverse-engineering/.
//!   Report ID 0x06 (Output 36B) / 0x07 (Input 32B) / 0x1A (Input 16B):
//!     firmware update / profile-write protocol — **NEVER WRITE these reports**
//!     (flash/brick risk). The write paths in this module hard-block them.
//!
//! Protocol is not publicly documented. This module uses the vendor Report ID 2
//! as the simplest LED control path (2-bit output). Values are configurable in
//! config.json under `led_probe` for easy adjustment once the real protocol is
//! decoded. NOTE: because the output field is only 2 bits, values >0x03 are
//! ignored by the firmware — writes are clamped in `write_vendor_report`.

use crate::hid_io::{drain_nonblocking, O_NONBLOCK};
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tracing::{debug, info, warn};

use epos_shared::config::AudioMode;
use epos_shared::led::{LedProbeConfig, LedReportPath};

use crate::audio::sink_is_managed;

/// What the ring should be showing right now.
///
/// The ring answers one question — is playback on the EPOS, and if so which mode
/// — and the two answers are visually distinct. Showing a mode colour while
/// playback is somewhere else would be a lie the user cannot see through: the
/// device would look engaged while nothing is going through it, which is the same
/// "on but not working" failure this project keeps removing elsewhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedIndicator {
    /// Playback is not on the EPOS. The ring goes dark.
    Unused,
    /// Playback is on the EPOS, showing the active profile's mode.
    InUse(AudioMode),
}

impl LedIndicator {
    /// A word for the log and the IPC status, never a guess about intent.
    pub fn describe(self) -> &'static str {
        match self {
            LedIndicator::Unused => "dark (playback is not on the EPOS)",
            LedIndicator::InUse(AudioMode::Stereo) => "blue (2.0)",
            LedIndicator::InUse(AudioMode::Surround71) => "red (7.1)",
        }
    }
}

/// The vendor byte for an indicator, or `None` on the unconfigured primary path.
///
/// Pure, so the mapping is pinned by tests rather than by whatever the ring
/// happened to show when a change was made.
pub fn vendor_byte(indicator: LedIndicator, cfg: &LedProbeConfig) -> u8 {
    match indicator {
        LedIndicator::Unused => cfg.vendor_off,
        LedIndicator::InUse(AudioMode::Stereo) => cfg.vendor_blue,
        LedIndicator::InUse(AudioMode::Surround71) => cfg.vendor_red,
    }
}

/// The mode indicator is the "in use" indicator with a mode attached: the ring is
/// only ever asked to show a mode when playback is on the EPOS.
pub fn indicator_for(mode: AudioMode, in_use: bool) -> LedIndicator {
    if in_use {
        LedIndicator::InUse(mode)
    } else {
        LedIndicator::Unused
    }
}

/// The daemon's own playback nodes, which sit on the EPOS permanently.
///
/// Measured node names, not guesses. They are `Corked: no` at all times because
/// the EQ drains its monitor and the sidetone loopback runs, so counting them as
/// "something is playing on the EPOS" would make the EPOS look like the busiest
/// device on the machine — which is how the first version of this ended up
/// refusing to leave.
pub const OWN_STREAM_NODES: [&str; 2] = ["epos-eq-output", "epos-sidetone-output"];

/// A playback stream, reduced to what choosing a destination needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayingStream {
    pub sink_name: String,
    /// `Corked: yes` means the application is not sending data.
    pub corked: bool,
    pub node_name: String,
}

/// Where to send playback when leaving the EPOS, in the order the user means it.
///
/// The place they were *listening* comes first, not the place the default sink
/// was pointing. Those are different things and the difference is not academic:
/// measured, audio was playing on the speakers while the default still pointed at
/// a virtual sink with nothing on it, so a long press that went by the default
/// sent the user somewhere silent.
pub fn exit_destination(
    streams: &[PlayingStream],
    remembered: Option<&str>,
    raw_sink: &str,
    anchor: &str,
    published: &[String],
) -> Option<String> {
    let is_ours = |s: &str| sink_is_managed(s, raw_sink) || s == anchor;
    // A stream counts when it is not corked, not one of ours, and on a sink that
    // is really there. First match wins, which is the order the server reported
    // them in and is therefore stable between runs.
    let playing = streams.iter().find(|s| {
        !s.corked
            && !OWN_STREAM_NODES.contains(&s.node_name.as_str())
            && !is_ours(s.sink_name.as_str())
            && published.iter().any(|p| p == &s.sink_name)
    });
    if let Some(chosen) = playing {
        return Some(chosen.sink_name.clone());
    }
    // Nothing is playing, so the best available answer is where the default was.
    exit_target(remembered, raw_sink, anchor, published)
}

/// Which sink to return to when the user asks to leave the EPOS.
///
/// Pure, because the whole decision is a preference question and preferences are
/// exactly what should not be discovered by trying things on live audio.
///
/// The remembered device wins if it is still published and still not ours — that
/// is "the device you were on a moment ago". Otherwise the first published sink
/// that is not ours, so leaving the EPOS always lands somewhere real. `None`
/// means there is nowhere to go, and the caller must leave the default alone
/// rather than point it at a sink that does not exist.
pub fn exit_target(
    remembered: Option<&str>,
    raw_sink: &str,
    anchor: &str,
    published: &[String],
) -> Option<String> {
    let is_ours = |s: &str| sink_is_managed(s, raw_sink) || s == anchor;
    if let Some(previous) = remembered.filter(|p| !is_ours(p)) {
        if published.iter().any(|s| s == previous) {
            return Some(previous.to_string());
        }
    }
    published
        .iter()
        .find(|s| !is_ours(s))
        .map(|s| (*s).to_string())
}

/// HID Output payload size for Report ID 0x02 (the LED ring).
///
/// Derived from the 120-byte report descriptor shipped by this device:
/// `75 01` (Report Size = 1 bit) + `95 02` (Report Count = 2) for the two LED
/// usage bits, followed by `95 06` (Report Count = 6) padding, i.e. an 8-bit
/// OUTPUT field = 1 byte. A descriptor-exact hidraw write is therefore the
/// report id plus one payload byte.
///
/// The long-standing 64-byte buffer also happens to be accepted, because the
/// kernel pads/truncates an oversized report instead of rejecting it (verified:
/// both sizes succeed on the same unit). The exact size is used because it is
/// what the descriptor specifies, not because the old one was failing.
const VENDOR_LED_PAYLOAD_SIZE: usize = 1;

/// Report ID for the LED ring (vendor page 0xFF13).
const REPORT_ID_VENDOR_LED: u8 = 0x02;

/// HID Output payload size for Report ID 0x04 (memory-bus / primary command).
/// Descriptor: `75 08` + `95 26` (38) = 304 bits = 38 bytes.
const PRIMARY_CMD_PAYLOAD_SIZE: usize = 38;

/// How long one LED report may keep retrying before it is given up.
///
/// Short on purpose: the ring is cosmetic and the heartbeat re-asserts it every
/// couple of seconds, so a slow report is abandoned quickly and retried rather
/// than holding whatever caller is waiting — and the signal handler, which holds
/// the global state lock while it resets the ring.
const LED_WRITE_BUDGET: std::time::Duration = std::time::Duration::from_millis(500);

/// Report ID for the memory-bus / primary command interface.
const REPORT_ID_PRIMARY_CMD: u8 = 0x04;

/// How many times the daemon reopens the hidraw node on its own after a write
/// failure, before it stops retrying and leaves the diagnosis to the log.
///
/// Reopening is a plain close/open of the hidraw node. It deliberately does
/// **not** touch USB power, `authorized` or port reset: those escalated a
/// still-enumerated device into a fully unenumerable one on 2026-09-24, and
/// only a physical replug recovered it. See
/// docs/reverse-engineering/LED-HID-WEDGED-ENDPOINT.md.
const MAX_LED_REOPEN_ATTEMPTS: u8 = 3;

/// LED controller for EPOS GSX 300
pub struct LedController {
    _hidraw_path: PathBuf,
    file: Option<File>,
    probe_config: LedProbeConfig,
    current_indicator: Option<LedIndicator>,
    /// Set once the first write fails, so the failure is reported exactly once
    /// instead of on every 2s heartbeat. Reset on a successful write or reopen.
    last_write_failed: bool,
    /// Automatic close/open retries spent on the current failure. Reset only by
    /// [`LedController::reopen`], i.e. by a real USB re-enumeration.
    reopen_attempts: u8,
}

impl LedController {
    /// Create a new LED controller. Searches for GSX 300 hidraw device.
    pub fn new(probe_config: LedProbeConfig) -> Result<Self> {
        let hidraw_path = Self::find_hidraw()?;
        info!("Found GSX 300 at {}", hidraw_path.display());

        // Retry with backoff: at session boot the udev ACL (audio group) may
        // not be granted yet when USB sysfs settles faster than udevd — same
        // race seen with Lian Li/OpenRGB. EPERM/EACCES here is transient;
        // give udev up to ~4s to catch up before failing for real.
        let mut file = None;
        for attempt in 0..10u32 {
            match OpenOptions::new()
                .write(true)
                .read(true)
                .custom_flags(O_NONBLOCK)
                .open(&hidraw_path)
            {
                Ok(f) => {
                    file = Some(f);
                    break;
                }
                Err(e) if (e.kind() == std::io::ErrorKind::PermissionDenied) && attempt < 9 => {
                    warn!(
                        "hidraw open denied (udev ACL race?), attempt {}/10: {}",
                        attempt + 1,
                        e
                    );
                    std::thread::sleep(std::time::Duration::from_millis(400));
                }
                Err(e) => return Err(e).context("Failed to open hidraw device"),
            }
        }

        Ok(Self {
            _hidraw_path: hidraw_path,
            file,
            probe_config,
            current_indicator: None,
            last_write_failed: false,
            reopen_attempts: 0,
        })
    }

    /// Reopen the device after a real USB re-enumeration.
    ///
    /// A genuine replug is a fresh start, so it refills the automatic-recovery
    /// budget (unlike [`LedController::recover_if_needed`]).
    pub fn reopen(&mut self) -> Result<()> {
        self.reopen_attempts = 0;
        self.open_device()
    }

    /// Reopen the hidraw node after a failed write, on a bounded budget.
    ///
    /// A device that is still enumerated but refuses every output write has
    /// either a stale file descriptor or a wedged interrupt-OUT endpoint. The
    /// fd case is fixable in-process; the wedged-endpoint case is not, and is
    /// why this is capped and silent at `debug` — the actionable warning comes
    /// from the single failure-transition message in [`LedController::set_mode`].
    pub fn recover_if_needed(&mut self) {
        if !self.last_write_failed || self.reopen_attempts >= MAX_LED_REOPEN_ATTEMPTS {
            return;
        }
        match self.open_device() {
            Ok(()) => {
                self.reopen_attempts += 1;
                debug!(
                    "LED: reopened hidraw after write failure (attempt {}/{})",
                    self.reopen_attempts, MAX_LED_REOPEN_ATTEMPTS
                );
            }
            Err(e) => debug!(
                "LED: reopen attempt {} failed: {}",
                self.reopen_attempts + 1,
                e
            ),
        }
    }

    /// Close and reopen the hidraw node. Leaves the recovery budget untouched.
    fn open_device(&mut self) -> Result<()> {
        self.file = None;
        self._hidraw_path = Self::find_hidraw()?;
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .custom_flags(O_NONBLOCK)
            .open(&self._hidraw_path)
            .context("Failed to reopen hidraw device")?;
        self.file = Some(file);
        self.current_indicator = None;
        self.last_write_failed = false;
        Ok(())
    }

    /// Set the LED color based on audio mode
    ///
    /// Note: always writes, even when the cached mode matches the target.
    /// The device LED state can drift from what we last set (e.g. the smart
    /// button toggles the LED on-device while a HID readback event is lost),
    /// so skipping identical writes can leave the physical LED out of sync
    /// with the daemon/web UI.
    pub fn set_indicator(&mut self, indicator: LedIndicator) -> Result<()> {
        let probe_config = self.probe_config.clone();
        let file = self.file.as_mut().context("LED device not open")?;

        let result = match probe_config.use_report {
            LedReportPath::Vendor => {
                let byte = vendor_byte(indicator, &probe_config);
                write_vendor_report(file, byte).map(|()| {
                    debug!(
                        "LED: Report ID 0x{:02X} → 0x{:02X} ({})",
                        REPORT_ID_VENDOR_LED,
                        byte & 0x03,
                        indicator.describe()
                    );
                })
            }
            LedReportPath::Primary => {
                let payload = match indicator {
                    LedIndicator::Unused => None,
                    LedIndicator::InUse(AudioMode::Stereo) => probe_config.primary_blue.as_deref(),
                    LedIndicator::InUse(AudioMode::Surround71) => {
                        probe_config.primary_red.as_deref()
                    }
                };
                let payload = payload.context(
                    "the primary LED path has no payload for this indicator; \
                     it has never been decoded, use the vendor path",
                )?;
                write_primary_report(file, payload).map(|()| {
                    debug!(
                        "LED: Report ID 0x{:02X} → {} bytes ({})",
                        REPORT_ID_PRIMARY_CMD,
                        payload.len(),
                        indicator.describe()
                    );
                })
            }
        };

        // The 2s heartbeat calls this unconditionally, so an unsupported
        // output path used to emit a fresh warning every two seconds
        // (~1400 lines/hour) and drown out real diagnostics. Report the
        // transition into failure once, then stay quiet until it recovers.
        //
        // EPROTO here is a *transport* symptom, not evidence that the device
        // lacks an output path: the same report succeeds on the same unit after
        // a physical replug. The device stays enumerated but its interrupt-OUT
        // endpoint goes dead, and only a replug clears it.
        if result.is_err() && !self.last_write_failed {
            let e = result
                .as_ref()
                .err()
                .map(|e| e.to_string())
                .unwrap_or_default();
            warn!(
                "LED write failed: {e}. The report is descriptor-correct and the \
                 same write succeeds after a replug, so this is a wedged USB \
                 endpoint rather than an unsupported device. Audio is unaffected. \
                 Unplug the GSX 300 for ~10s and plug it back in; the daemon \
                 re-syncs the ring on reconnect. See \
                 docs/reverse-engineering/LED-HID-WEDGED-ENDPOINT.md"
            );
        }
        if result.is_ok() && self.last_write_failed {
            info!("LED writes recovered");
        }
        self.last_write_failed = result.is_err();

        result?;

        self.current_indicator = Some(indicator);
        // Heartbeat re-asserts this every 2s — keep at debug level to avoid
        // ~1400 journald lines/hour of identical noise.
        debug!("LED ring set: {}", indicator.describe());
        Ok(())
    }

    /// Find the hidraw device for GSX 300
    fn find_hidraw() -> Result<PathBuf> {
        let hidraw_dir = Path::new("/sys/class/hidraw");
        if !hidraw_dir.exists() {
            anyhow::bail!("/sys/class/hidraw not found");
        }

        for entry in std::fs::read_dir(hidraw_dir).context("Failed to list /sys/class/hidraw")? {
            let entry = entry?;
            let uevent_path = entry.path().join("device/uevent");
            if let Ok(content) = std::fs::read_to_string(&uevent_path) {
                if content.contains("00001395") && content.contains("00000098") {
                    let name = entry.file_name();
                    return Ok(PathBuf::from(format!("/dev/{}", name.to_string_lossy())));
                }
            }
        }
        anyhow::bail!("GSX 300 not found in /sys/class/hidraw")
    }

    /// Check if the hidraw device is accessible
    #[allow(dead_code)]
    pub fn is_accessible(&self) -> bool {
        self.file.is_some()
    }

    /// True when the last write failed, i.e. LED control is not working.
    /// Callers can surface this instead of pretending the ring is in sync.
    pub fn write_failing(&self) -> bool {
        self.last_write_failed
    }

    #[allow(dead_code)]
    pub fn current_indicator(&self) -> Option<LedIndicator> {
        self.current_indicator
    }
}

/// Write vendor Report ID 0x02 (1-byte LED output).
fn write_vendor_report(file: &mut File, byte: u8) -> Result<()> {
    // The descriptor's output field is only 2 bits (wire values 0x00..=0x03:
    // off / red / blue / both — see module doc). The firmware silently ignores
    // any higher bits → clamp to 0x00..=0x03 so a bad config value can never
    // produce a no-op write.
    let byte = byte & 0x03;
    let mut packet = vec![0u8; VENDOR_LED_PAYLOAD_SIZE + 1];
    packet[0] = REPORT_ID_VENDOR_LED;
    packet[1] = byte;
    if !drain_nonblocking(&packet, LED_WRITE_BUDGET, |chunk| file.write(chunk)) {
        anyhow::bail!("Vendor LED report was not accepted by the device");
    }
    file.flush().ok();
    Ok(())
}

/// Write consumer Report ID 0x04 (38-byte output, padded to 64)
///
/// SAFETY: Report 0x04 is the memory-bus interface. The payload layout is
/// `[flags, len, addr_hi, addr_lo]` — flags 0x20/0x10 select EEPROM/high-page
/// and MUST NOT contain bit6 (0x40 = EEPROM write enable). Only a pure-read
/// request (bit6 clear) is ever allowed through this path. Report IDs 0x06 /
/// 0x07 / 0x1A (firmware flash protocol) are hard-blocked by the daemon.
fn write_primary_report(file: &mut File, payload: &[u8]) -> Result<()> {
    let packet = primary_packet(payload)?;
    if !drain_nonblocking(&packet, LED_WRITE_BUDGET, |chunk| file.write(chunk)) {
        anyhow::bail!("Primary LED report was not accepted by the device");
    }
    file.flush().ok();
    Ok(())
}

/// Build the Report 0x04 packet for `payload`, or refuse it.
///
/// Pure, so the refusal rules are testable without the headset. The payload
/// is the user's own `primary_blue` / `primary_red` from `config.json`, which
/// is a documented hand-editable probe surface — so "malformed" is a shape a
/// real configuration can have, not a hypothetical.
///
/// An empty payload is refused rather than copied. The previous code computed
/// `len = 0` and then evaluated `packet[1..=0]`, an inclusive range that starts
/// after it ends, which panics. `Some([])` reached it: `.as_deref()` yields
/// `Some(&[])` so the "not configured" error never fired, and `first()` is
/// `None` so the bit6 guard never fired either. A panic there ran inside the
/// LED heartbeat and the IPC handlers, silently killing whichever worker hit
/// it.
fn primary_packet(payload: &[u8]) -> Result<Vec<u8>> {
    if payload.is_empty() {
        anyhow::bail!("Refusing empty Report 0x04 payload: there is nothing to send");
    }
    // Memory-bus read-request guard: the first payload byte is the flags byte.
    // bit6 (0x40) = EEPROM write (firmware flash) — refuse it unconditionally.
    if payload.first().is_some_and(|f| f & 0x40 != 0) {
        anyhow::bail!(
            "Refusing report 0x04 payload with EEPROM write bit6 set \
             (firmware flash = brick risk); dropping write"
        );
    }
    let mut packet = vec![0u8; PRIMARY_CMD_PAYLOAD_SIZE + 1];
    packet[0] = REPORT_ID_PRIMARY_CMD;
    // The report is a fixed size, so copy into the tail by its true length and
    // zero-pad the rest rather than indexing with an inclusive range.
    let len = payload.len().min(PRIMARY_CMD_PAYLOAD_SIZE);
    packet[1..1 + len].copy_from_slice(&payload[..len]);
    Ok(packet)
}

impl Drop for LedController {
    fn drop(&mut self) {
        // Try to reset LED to default (blue/stereo) on shutdown
        if let Some(ref mut file) = self.file {
            let byte = self.probe_config.vendor_blue & 0x03;
            let mut packet = vec![0u8; VENDOR_LED_PAYLOAD_SIZE + 1];
            packet[0] = REPORT_ID_VENDOR_LED;
            packet[1] = byte;
            let _ = drain_nonblocking(&packet, LED_WRITE_BUDGET, |chunk| file.write(chunk));
            let _ = file.flush();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::EQ_SINK_NAME;

    // ─── What the ring shows ───────────────────────────────────────
    //
    // The ring is a two-bit register, so the palette is four states and there is
    // no green to ask for. "Not in use" therefore has to be the dark state,
    // because every lit state means a mode and the device is not on a mode while
    // playback is somewhere else.

    /// Playback away from the EPOS must be dark, not a mode colour.
    #[test]
    fn the_ring_is_dark_when_the_device_is_not_in_use() {
        assert_eq!(
            vendor_byte(LedIndicator::Unused, &LedProbeConfig::default()),
            0x00
        );
        assert_eq!(indicator_for(AudioMode::Stereo, false), LedIndicator::Unused);
        assert_eq!(
            indicator_for(AudioMode::Surround71, false),
            LedIndicator::Unused
        );
    }

    /// On the EPOS the ring shows the profile's mode, using the measured bytes.
    #[test]
    fn the_ring_shows_the_mode_while_the_device_is_in_use() {
        let cfg = LedProbeConfig::default();
        assert_eq!(
            vendor_byte(indicator_for(AudioMode::Stereo, true), &cfg),
            cfg.vendor_blue
        );
        assert_eq!(
            vendor_byte(indicator_for(AudioMode::Surround71, true), &cfg),
            cfg.vendor_red
        );
    }

    /// The three states must be three different bytes, or "not in use" is
    /// indistinguishable from a mode.
    #[test]
    fn in_use_and_not_in_use_are_visibly_different() {
        let cfg = LedProbeConfig::default();
        let dark = vendor_byte(LedIndicator::Unused, &cfg);
        assert_ne!(dark, cfg.vendor_blue);
        assert_ne!(dark, cfg.vendor_red);
    }

    /// An old config file with no `vendor_off` must still load, and with the
    /// byte that means "dark".
    #[test]
    fn a_config_written_before_vendor_off_still_loads() {
        let old = r#"{"vendor_blue":2,"vendor_red":1,"primary_blue":null,
                      "primary_red":null,"use_report":"vendor"}"#;
        let parsed: LedProbeConfig = serde_json::from_str(old).expect("an old config loads");
        assert_eq!(parsed.vendor_off, 0x00);
    }

    // ─── Leaving the EPOS ──────────────────────────────────────────

    /// The measured case, verbatim from this machine: audio on the speakers while
    /// the default still pointed at a virtual sink with nothing on it. Going by
    /// the default would have sent the user somewhere silent.
    #[test]
    fn leaving_goes_where_audio_is_playing_not_where_the_default_was() {
        let speakers = "alsa_output.usb-Generic_USB_Audio-00.HiFi_7_1__Speaker__sink";
        let games_sink = "games_sink";
        let raw = "alsa_output.usb-EPOS-00.analog-stereo";
        let published = vec![
            games_sink.to_string(),
            raw.to_string(),
            speakers.to_string(),
        ];
        let streams = [
            PlayingStream { sink_name: speakers.to_string(), corked: false, node_name: "miniaudio:0".to_string() },
            PlayingStream { sink_name: speakers.to_string(), corked: true, node_name: "AudioStream".to_string() },
        ];
        assert_eq!(
            exit_destination(&streams, Some(games_sink), raw, EQ_SINK_NAME, &published),
            Some(speakers.to_string()),
            "where the audio is beats where the default was"
        );
    }

    /// The daemon's own streams are on the EPOS and never stop. Counting them
    /// would make the EPOS the busiest device on the machine.
    #[test]
    fn the_daemons_own_streams_do_not_count_as_playing() {
        let raw = "alsa_output.usb-EPOS-00.analog-stereo";
        let bluetooth = "bluez_sink.00_00_00_00_00_00.a2dp_sink";
        let published = vec![raw.to_string(), bluetooth.to_string()];
        let streams = [
            PlayingStream { sink_name: raw.to_string(), corked: false, node_name: OWN_STREAM_NODES[0].to_string() },
            PlayingStream { sink_name: raw.to_string(), corked: false, node_name: OWN_STREAM_NODES[1].to_string() },
        ];
        assert_eq!(
            exit_destination(&streams, Some(bluetooth), raw, EQ_SINK_NAME, &published),
            Some(bluetooth.to_string()),
            "the EQ monitor and the sidetone are not somebody listening"
        );
    }

    /// A corked stream is an application with nothing to say.
    #[test]
    fn a_corked_stream_is_not_where_the_user_is_listening() {
        let speakers = "alsa_output.usb-Generic_USB_Audio-00.HiFi_7_1__Speaker__sink";
        let bluetooth = "bluez_sink.00_00_00_00_00_00.a2dp_sink";
        let raw = "alsa_output.usb-EPOS-00.analog-stereo";
        let published = vec![speakers.to_string(), bluetooth.to_string(), raw.to_string()];
        let streams = [PlayingStream {
            sink_name: speakers.to_string(),
            corked: true,
            node_name: "Firefox".to_string(),
        }];
        assert_eq!(
            exit_destination(&streams, Some(bluetooth), raw, EQ_SINK_NAME, &published),
            Some(bluetooth.to_string())
        );
    }

    /// Nothing playing: the remembered default, as before.
    #[test]
    fn leaving_with_nothing_playing_falls_back_to_the_remembered_default() {
        let speakers = "alsa_output.usb-Generic_USB_Audio-00.HiFi_7_1__Speaker__sink";
        let raw = "alsa_output.usb-EPOS-00.analog-stereo";
        let published = vec![raw.to_string(), speakers.to_string()];
        assert_eq!(
            exit_destination(&[], Some(speakers), raw, EQ_SINK_NAME, &published),
            Some(speakers.to_string())
        );
    }

    /// A stream on a sink that is not in the listing is a leftover, not a place.
    #[test]
    fn a_stream_on_a_sink_that_is_gone_is_not_a_destination() {
        let bluetooth = "bluez_sink.00_00_00_00_00_00.a2dp_sink";
        let raw = "alsa_output.usb-EPOS-00.analog-stereo";
        let published = vec![raw.to_string(), bluetooth.to_string()];
        let streams = [PlayingStream {
            sink_name: "a_sink_that_unplugged".to_string(),
            corked: false,
            node_name: "Firefox".to_string(),
        }];
        assert_eq!(
            exit_destination(&streams, Some(bluetooth), raw, EQ_SINK_NAME, &published),
            Some(bluetooth.to_string())
        );
    }

    /// The normal case: the device you were on a moment ago is still there.
    #[test]
    fn leaving_the_device_goes_back_to_where_it_was() {
        let speakers = "alsa_output.usb-Generic_USB_Audio-00.HiFi_7_1__Speaker__sink";
        let raw = "alsa_output.usb-EPOS-00.analog-stereo";
        let published = vec![
            raw.to_string(),
            EQ_SINK_NAME.to_string(),
            speakers.to_string(),
        ];
        assert_eq!(
            exit_target(Some(speakers), raw, EQ_SINK_NAME, &published),
            Some(speakers.to_string())
        );
    }

    /// The device you were on has gone. Leaving must still land somewhere real.
    #[test]
    fn leaving_falls_back_to_another_published_device() {
        let raw = "alsa_output.usb-EPOS-00.analog-stereo";
        let bluetooth = "bluez_sink.00_00_00_00_00_00.a2dp_sink";
        let published = vec![
            raw.to_string(),
            EQ_SINK_NAME.to_string(),
            bluetooth.to_string(),
        ];
        assert_eq!(
            exit_target(
                Some("a_device_that_unplugged"),
                raw,
                EQ_SINK_NAME,
                &published
            ),
            Some(bluetooth.to_string()),
            "the remembered device is gone, so some other real device must be used"
        );
    }

    /// Nothing to go to. The caller has to be told, not handed a sink that does
    /// not exist — pointing the default at a missing device is silence.
    #[test]
    fn leaving_with_nowhere_to_go_returns_nothing() {
        let raw = "alsa_output.usb-EPOS-00.analog-stereo";
        let published = vec![raw.to_string(), EQ_SINK_NAME.to_string()];
        assert_eq!(
            exit_target(Some("gone"), raw, EQ_SINK_NAME, &published),
            None
        );
        assert_eq!(exit_target(None, raw, EQ_SINK_NAME, &published), None);
    }

    /// A remembered device that happens to be one of ours is not a destination.
    /// That is the case where the daemon already took the route, and sending the
    /// user straight back would be leaving without leaving.
    #[test]
    fn a_remembered_device_that_is_ours_is_not_a_destination() {
        let raw = "alsa_output.usb-EPOS-00.analog-stereo";
        let bluetooth = "bluez_sink.00_00_00_00_00_00.a2dp_sink";
        let published = vec![raw.to_string(), bluetooth.to_string()];
        assert_eq!(
            exit_target(Some(raw), raw, EQ_SINK_NAME, &published),
            Some(bluetooth.to_string())
        );
        assert_eq!(
            exit_target(Some(EQ_SINK_NAME), raw, EQ_SINK_NAME, &published),
            Some(bluetooth.to_string())
        );
    }

    /// The EPOS is never a destination even when it is the only thing published,
    /// because then there is genuinely nowhere to go.
    #[test]
    fn the_epos_is_never_the_destination() {
        let raw = "alsa_output.usb-EPOS-00.analog-stereo";
        let published = vec![raw.to_string(), EQ_SINK_NAME.to_string()];
        for remembered in [None, Some(raw), Some(EQ_SINK_NAME)] {
            assert_eq!(
                exit_target(remembered, raw, EQ_SINK_NAME, &published),
                None
            );
        }
    }

    /// The panic this exists to prevent. `primary_blue: []` in config.json is
    /// reachable: `Some([])` satisfies `as_deref()` so the "not configured"
    /// error never fires, and `first()` is `None` so the bit6 guard never
    /// fires. The old code then evaluated `packet[1..=0]`, which panics.
    #[test]
    fn an_empty_payload_is_refused_rather_than_panicking() {
        let outcome = std::panic::catch_unwind(|| primary_packet(&[]));
        assert!(outcome.is_ok(), "building a packet must never panic");
        let error = primary_packet(&[]).expect_err("an empty payload has nothing to send");
        assert!(
            error.to_string().contains("empty"),
            "the reason must say what was wrong: {error}"
        );
    }

    /// The brick guard is untouched by the empty check, and still fires on the
    /// very first byte it always did.
    #[test]
    fn an_eeprom_write_payload_is_still_refused() {
        let error = primary_packet(&[0x40, 0x00]).expect_err("bit6 must be refused");
        assert!(
            error.to_string().contains("brick risk"),
            "the brick-risk reason must survive: {error}"
        );
    }

    /// A real payload still produces a full-size report: report ID first, the
    /// payload copied into the body, and the remainder zero-padded so the HID
    /// report keeps its fixed length.
    #[test]
    fn a_real_payload_still_builds_a_full_report() {
        let payload = [0x00u8, 0x04, 0xAB, 0xCD];
        let packet = primary_packet(&payload).expect("a valid payload must build");

        assert_eq!(packet.len(), PRIMARY_CMD_PAYLOAD_SIZE + 1);
        assert_eq!(packet[0], REPORT_ID_PRIMARY_CMD);
        assert_eq!(&packet[1..1 + payload.len()], &payload[..]);
        assert!(
            packet[1 + payload.len()..].iter().all(|b| *b == 0),
            "the rest of the report must be zero-padded"
        );
    }

    /// A payload longer than the report body is truncated, not a panic and not
    /// an oversized write.
    #[test]
    fn an_oversized_payload_is_truncated_to_the_report() {
        let payload = vec![0x00u8; PRIMARY_CMD_PAYLOAD_SIZE + 20];
        let packet = primary_packet(&payload).expect("oversized is truncated, not refused");
        assert_eq!(packet.len(), PRIMARY_CMD_PAYLOAD_SIZE + 1);
    }

    /// A single byte is the smallest legal payload; it must not take the
    /// inclusive-range path that made the empty case panic.
    #[test]
    fn a_one_byte_payload_is_accepted() {
        let packet = primary_packet(&[0x00]).expect("one byte is a legal payload");
        assert_eq!(packet.len(), PRIMARY_CMD_PAYLOAD_SIZE + 1);
        assert_eq!(packet[1], 0x00);
    }
}

