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
//!     Output 2 bits → usages 0x05 (LED blue) / 0x06 (LED red). The WIRE byte
//!     written is the logical value, offset by +4 from the usage id:
//!       0x00 = off, 0x01 = blue, 0x02 = red, 0x03 = pink
//!     (hardware-confirmed, AGENT-FINDINGS §3.1). Usage-id and wire-value are
//!     two different numbering systems — do not mix them.
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

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tracing::{debug, info};

use epos_shared::config::AudioMode;
use epos_shared::led::{LedProbeConfig, LedReportPath};

/// HID Output report buffer size (must match device max)
const HID_OUTPUT_SIZE: usize = 64;

/// Report IDs for EPOS GSX 300
const REPORT_ID_VENDOR_LED: u8 = 0x02;
const REPORT_ID_PRIMARY_CMD: u8 = 0x04;

/// LED controller for EPOS GSX 300
pub struct LedController {
    _hidraw_path: PathBuf,
    file: Option<File>,
    probe_config: LedProbeConfig,
    current_mode: Option<AudioMode>,
}

impl LedController {
    /// Create a new LED controller. Searches for GSX 300 hidraw device.
    pub fn new(probe_config: LedProbeConfig) -> Result<Self> {
        let hidraw_path = Self::find_hidraw()?;
        info!("Found GSX 300 at {}", hidraw_path.display());

        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(&hidraw_path)
            .context("Failed to open hidraw device")?;

        Ok(Self {
            _hidraw_path: hidraw_path,
            file: Some(file),
            probe_config,
            current_mode: None,
        })
    }

    /// Reopen the device (e.g. after USB reconnect)
    #[allow(dead_code)]
    pub fn reopen(&mut self) -> Result<()> {
        self.file = None;
        self._hidraw_path = Self::find_hidraw()?;
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(&self._hidraw_path)
            .context("Failed to reopen hidraw device")?;
        self.file = Some(file);
        self.current_mode = None;
        Ok(())
    }

    /// Set the LED color based on audio mode
    ///
    /// Note: always writes, even when the cached mode matches the target.
    /// The device LED state can drift from what we last set (e.g. the smart
    /// button toggles the LED on-device while a HID readback event is lost),
    /// so skipping identical writes can leave the physical LED out of sync
    /// with the daemon/web UI.
    pub fn set_mode(&mut self, mode: AudioMode) -> Result<()> {

        let probe_config = self.probe_config.clone();
        let file = self
            .file
            .as_mut()
            .context("LED device not open")?;

        match probe_config.use_report {
            LedReportPath::Vendor => {
                let byte = match mode {
                    AudioMode::Stereo => probe_config.vendor_blue,
                    AudioMode::Surround71 => probe_config.vendor_red,
                };
                write_vendor_report(file, byte)?;
                debug!(
                    "LED: Report ID 0x{:02X} → 0x{:02X} ({})",
                    REPORT_ID_VENDOR_LED,
                    byte,
                    mode.display_name()
                );
            }
            LedReportPath::Primary => {
                let payload = match mode {
                    AudioMode::Stereo => probe_config
                        .primary_blue
                        .as_deref()
                        .context("No primary_blue payload configured")?,
                    AudioMode::Surround71 => probe_config
                        .primary_red
                        .as_deref()
                        .context("No primary_red payload configured")?,
                };
                write_primary_report(file, payload)?;
                debug!(
                    "LED: Report ID 0x{:02X} → {} bytes ({})",
                    REPORT_ID_PRIMARY_CMD,
                    payload.len(),
                    mode.display_name()
                );
            }
        }

        self.current_mode = Some(mode);
        info!("LED ring set to {} for {}", match mode {
            AudioMode::Stereo => "blue",
            AudioMode::Surround71 => "red",
        }, mode.display_name());
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

    #[allow(dead_code)]
    pub fn current_mode(&self) -> Option<AudioMode> {
        self.current_mode
    }
}

/// Write vendor Report ID 0x02 (1-byte output)
fn write_vendor_report(file: &mut File, byte: u8) -> Result<()> {
    // The descriptor's output field is only 2 bits (wire values 0x00..=0x03:
    // off / blue / red / pink — see module doc). The firmware silently ignores
    // any higher bits → clamp to 0x00..=0x03 so a bad config value can never
    // produce a no-op write.
    let byte = byte & 0x03;
    let mut packet = vec![0u8; HID_OUTPUT_SIZE];
    packet[0] = REPORT_ID_VENDOR_LED;
    packet[1] = byte;
    file.write_all(&packet)
        .context("Failed to write vendor LED report")?;
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
    // Memory-bus read-request guard: the first payload byte is the flags byte.
    // bit6 (0x40) = EEPROM write (firmware flash) — refuse it unconditionally.
    if payload.first().is_some_and(|f| f & 0x40 != 0) {
        anyhow::bail!(
            "Refusing report 0x04 payload with EEPROM write bit6 set \
             (firmware flash = brick risk); dropping write"
        );
    }
    let mut packet = vec![0u8; HID_OUTPUT_SIZE];
    packet[0] = REPORT_ID_PRIMARY_CMD;
    let len = payload.len().min(HID_OUTPUT_SIZE - 1);
    packet[1..=len].copy_from_slice(&payload[..len]);
    file.write_all(&packet)
        .context("Failed to write primary LED report")?;
    file.flush().ok();
    Ok(())
}

impl Drop for LedController {
    fn drop(&mut self) {
        // Try to reset LED to default (blue/stereo) on shutdown
        if let Some(ref mut file) = self.file {
            let byte = self.probe_config.vendor_blue;
            let mut packet = vec![0u8; HID_OUTPUT_SIZE];
            packet[0] = REPORT_ID_VENDOR_LED;
            packet[1] = byte;
            let _ = file.write_all(&packet);
            let _ = file.flush();
        }
    }
}
