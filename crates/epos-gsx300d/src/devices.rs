use anyhow::Result;
use epos_shared::device::DeviceInfo;
use std::time::Duration;
use tracing::{debug, warn};

/// Detect EPOS GSX 300 on USB bus — always a fresh scan including `pw-dump`.
pub async fn detect() -> Option<DeviceInfo> {
    detect_with_nodes(None, true).await
}

/// Like [`detect`], but reuses cached PipeWire node names (sink, source) when
/// available so the 5s hotplug loop does NOT spawn `pw-dump` on every poll
/// (~17k subprocesses/day).
///
/// - `cached`: known (sink, source) names → returned as-is, zero `pw-dump`.
/// - `cached == None` + `needs_fresh == true`: device is present, we lack
///   names → single fresh `pw-dump` lookup (once per (re)connect).
/// - `cached == None` + `needs_fresh == false`: device is absent, names are
///   meaningless → nothing spawns.
pub async fn detect_with_nodes(
    cached: Option<(String, String)>,
    needs_fresh: bool,
) -> Option<DeviceInfo> {
    // The scan touches sysfs and may spawn a blocking `pw-dump` subprocess
    // (when node names aren't cached). Run it on a blocking thread instead of
    // stalling the async worker — the 5s hotplug loop calls this every tick and
    // GetStatus's startup-race fallback calls it too (audit F4).
    match tokio::task::spawn_blocking(move || scan_usb_devices(cached, needs_fresh)).await {
        Ok(Ok(devices)) => {
            if let Some(dev) = devices.first() {
                debug!(
                    "EPOS GSX 300 present at bus {}:{}",
                    dev.usb_bus, dev.usb_addr
                );
                Some(dev.clone())
            } else {
                None
            }
        }
        Ok(Err(e)) => {
            warn!("USB scan failed: {}", e);
            None
        }
        Err(e) => {
            warn!("USB scan task panicked: {}", e);
            None
        }
    }
}

/// Cheap sysfs-only check: is an EPOS GSX 300 on the USB bus right now?
/// No subprocess, no `pw-dump` — safe to call on every 5s hotplug tick.
pub fn usb_present() -> bool {
    let usb_dir = std::path::PathBuf::from("/sys/bus/usb/devices");
    let Ok(entries) = std::fs::read_dir(&usb_dir) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let vendor = path.join("idVendor");
        let product = path.join("idProduct");
        if !vendor.exists() || !product.exists() {
            continue;
        }
        if std::fs::read_to_string(&vendor).unwrap_or_default().trim() == "1395"
            && std::fs::read_to_string(&product).unwrap_or_default().trim() == "0098"
        {
            return true;
        }
    }
    false
}

fn scan_usb_devices(
    cached_nodes: Option<(String, String)>,
    needs_fresh: bool,
) -> Result<Vec<DeviceInfo>> {
    let mut devices = Vec::new();

    // Scan /sys/bus/usb/devices/ for matching VID:PID
    let usb_dir = std::path::PathBuf::from("/sys/bus/usb/devices");
    if !usb_dir.exists() {
        return Ok(devices);
    }

    for entry in std::fs::read_dir(&usb_dir)? {
        let entry = entry?;
        let path = entry.path();
        let vendor_path = path.join("idVendor");
        let product_path = path.join("idProduct");

        if !vendor_path.exists() || !product_path.exists() {
            continue;
        }

        let vid_str = std::fs::read_to_string(&vendor_path)?.trim().to_string();
        let pid_str = std::fs::read_to_string(&product_path)?.trim().to_string();

        let vid = u16::from_str_radix(&vid_str, 16).unwrap_or(0);
        let pid = u16::from_str_radix(&pid_str, 16).unwrap_or(0);

        if DeviceInfo::is_epos_gs300(vid, pid) {
            // Extract bus:addr from directory name (e.g., "1-7")
            let dir_name = entry.file_name().to_string_lossy().to_string();
            let parts: Vec<&str> = dir_name.split('-').collect();
            let usb_bus = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
            let usb_addr = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);

            // Find ALSA card number
            // Stay unknown until ALSA enumerates; 0 is a real card, not a sentinel.
            let alsa_card = find_alsa_card(vid, pid);

            // Find PipeWire node names — from cache when available (no pw-dump
            // spawn), otherwise a fresh lookup (startup / reconnect).
            let (sink, source) = match (&cached_nodes, needs_fresh) {
                (Some((s, m)), _) => (s.clone(), m.clone()),
                (None, true) => find_pipewire_nodes(),
                (None, false) => (String::new(), String::new()),
            };

            // Find hidraw
            let hidraw = find_hidraw(vid, pid);

            // Find input event
            let input_event = find_input_event(vid, pid);

            devices.push(DeviceInfo {
                usb_bus,
                usb_addr,
                alsa_card,
                pipewire_sink: sink,
                pipewire_source: source,
                hidraw,
                input_event,
                firmware_version: None,
                chip_id: None,
                hw_snapshot: None,
            });
        }
    }

    Ok(devices)
}

fn find_alsa_card(vid: u16, pid: u16) -> Option<u8> {
    let proc_sound = std::path::PathBuf::from("/proc/asound");
    if !proc_sound.exists() {
        return None;
    }

    // Primary: match by USB vendor:product via /proc/asound/card*/usbid.
    // This is robust against card-number churn at boot (USB sysfs settles
    // before ALSA enumerates, so a name scan can race) and against other
    // USB audio devices present on the system.
    if let Ok(read_dir) = std::fs::read_dir(&proc_sound) {
        for entry in read_dir.flatten() {
            let dir_name = entry.file_name().to_string_lossy().to_string();
            if !dir_name.starts_with("card") {
                continue;
            }
            let card = match dir_name.trim_start_matches("card").parse::<u8>() {
                Ok(c) => c,
                // Non-numeric entries like the "cards" file must not
                // abort the whole scan — skip and keep iterating.
                Err(_) => continue,
            };
            if let Ok(usbid) = std::fs::read_to_string(entry.path().join("usbid")) {
                let usbid = usbid.trim().to_lowercase();
                let target = format!("{:04x}:{:04x}", vid, pid);
                if usbid == target {
                    return Some(card);
                }
            }
        }
    }

    // Fallback: match by device NAME — /proc/asound/cards lists
    // "EPOS GSX 300" / "Sennheiser EPOS GSX 300". Never match plain
    // "USB Audio" (wrong device → mic gain hits the wrong card).
    if let Ok(cards) = std::fs::read_to_string(proc_sound.join("cards")) {
        for line in cards.lines() {
            if line.contains("EPOS") || line.contains("GSX 300") {
                // Extract card number from beginning of line
                if let Some(card_str) = line.split_whitespace().next() {
                    if let Ok(card) = card_str.parse::<u8>() {
                        return Some(card);
                    }
                }
            }
        }
    }

    None
}

/// PipeWire node names for the EPOS, used when the live graph cannot answer.
///
/// The serial is embedded in the node name and is stable across reboots, so
/// these are real names rather than a guess. They are the last line of defence:
/// if PipeWire ever renames them, the test below is where that shows up.
pub const EPOS_SINK_FALLBACK: &str =
    "alsa_output.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.analog-stereo";
pub const EPOS_SOURCE_FALLBACK: &str =
    "alsa_input.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.mono-fallback";

/// Pull the EPOS sink and source node names out of a `pw-dump` document.
///
/// Returns `None` — never a wildcard, never a partial answer — when the dump
/// cannot be parsed, contains no EPOS node, or contains only one half of the
/// pair.
///
/// The wildcard this used to fall back to is what broke the audio path on
/// 2026-09-24. `pactl set-default-sink` rejects it outright, so the EQ's
/// fail-closed fallback could not move playback off the anchor, and a wildcard
/// written into a generated conf is not a node name — the EQ and voice
/// filter-chains stopped publishing entirely and the watchdog then restarted
/// them onto the same broken conf indefinitely. "Not found" is the only safe
/// answer, because the caller has a real fallback and uses it.
///
/// Parsed as JSON rather than line by line: the previous hand-rolled parser only
/// worked because this particular `pw-dump` happens to put one property per
/// line, so compact JSON would have silently produced no names at all.
fn select_epos_nodes(dump: &str) -> Option<(String, String)> {
    let parsed: serde_json::Value = serde_json::from_str(dump).ok()?;
    let array = parsed.as_array()?;

    let mut sink: Option<String> = None;
    let mut source: Option<String> = None;
    for object in array {
        if object["type"].as_str() != Some("PipeWire:Interface:Node") {
            continue;
        }
        let props = &object["info"]["props"];
        let name = props["node.name"].as_str().unwrap_or_default();
        let description = props["node.description"].as_str().unwrap_or_default();
        if !description.contains("EPOS GSX 300") {
            continue;
        }
        if name.contains("output") && name.contains("analog-stereo") {
            sink = Some(name.to_string());
        } else if name.contains("input") && name.contains("mono-fallback") {
            source = Some(name.to_string());
        }
    }

    // Both halves or nothing: a sink with no source (or the reverse) would leave
    // one of the two DSP chains pointing at a name that is not there.
    match (sink, source) {
        (Some(sink), Some(source)) => Some((sink, source)),
        _ => None,
    }
}

/// Decide the node names to use from whatever `pw-dump` managed to say.
///
/// Split from the subprocess so the invariant that matters — *never a wildcard* —
/// is testable on the decision itself. Every unresolved outcome lands on the real
/// fallback names, which is the whole point: a usable answer is always available,
/// so there is never a reason to invent one.
fn epos_node_names_from_dump(dump: Option<&str>) -> (String, String) {
    let resolved = dump.and_then(select_epos_nodes);
    if resolved.is_none() {
        // The common cause is a startup race: the main graph has not published
        // the device yet. Loud, because the alternative this replaced was an
        // unusable wildcard written into the generated confs.
        warn!(
            "pw-dump did not yield an EPOS sink/source pair (main graph not up \
             yet?) - using the known EPOS node names"
        );
    }
    resolved.unwrap_or_else(|| {
        (
            EPOS_SINK_FALLBACK.to_string(),
            EPOS_SOURCE_FALLBACK.to_string(),
        )
    })
}

/// Resolve the EPOS's PipeWire node names from the live graph.
fn find_pipewire_nodes() -> (String, String) {
    let dump = capped_dump("pw-dump", Duration::from_secs(5));
    epos_node_names_from_dump(dump.as_deref())
}

/// Run a blocking dump command with a wall-clock cap.
///
/// This runs inside `spawn_blocking` and is awaited by both daemon startup and
/// the 5s hotplug loop, so a command that never answers would stop the daemon
/// from finishing startup and stop every watchdog from ever being reached. The
/// worker thread is abandoned rather than joined on timeout; the caller treats
/// that exactly like a failed dump, which already falls back to the compiled-in
/// node names and is re-resolved once the names look stale again.
fn capped_dump(program: &str, budget: Duration) -> Option<String> {
    // Owned for the worker thread, which requires 'static.
    let program = program.to_string();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        // ETXTBSY, the same race `run_status` handles, on the blocking path. It
        // is here too because this function does not go through `run_status`: it
        // spawns its own worker thread, so a retry added only to the async spawner
        // left this one failing - measured, not assumed. The two are the same
        // fault and the same argument applies, so the wording is shared.
        const ATTEMPTS: u32 = 8;
        let mut last_reason = String::new();
        let mut captured = None;
        for attempt in 0..ATTEMPTS {
            // The reason is carried out rather than dropped. This used `.ok()`
            // and returned None, which made a failure indistinguishable from a
            // timeout, from a non-zero exit and from a program that was never
            // there - so a failing test gave nobody a reason, and the race
            // behind it took a hundred runs to name.
            let outcome = std::process::Command::new(&program)
                .stdin(std::process::Stdio::null())
                .output();
            match outcome {
                Ok(o) if o.status.success() => {
                    captured = Some(String::from_utf8_lossy(&o.stdout).into_owned());
                    break;
                }
                Ok(o) => {
                    last_reason = format!(
                        "exited {:?}: {}",
                        o.status.code(),
                        String::from_utf8_lossy(&o.stderr).trim()
                    );
                    // A program that ran and failed is not going to start working
                    // on a second attempt.
                    break;
                }
                Err(e) => {
                    last_reason = e.to_string();
                    let busy = last_reason.contains("Text file busy")
                        || last_reason.contains("os error 26");
                    if !busy || attempt + 1 == ATTEMPTS {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(
                        20 * (attempt as u64 + 1),
                    ));
                }
            }
        }
        if captured.is_none() {
            warn!("{program} did not produce a dump: {last_reason}");
        }
        // The receiver is gone when the budget expired; sending then fails,
        // which is the intended outcome, so the error is deliberately dropped.
        let _ = tx.send(captured);
    });
    rx.recv_timeout(budget).ok().flatten()
}

fn find_hidraw(vid: u16, pid: u16) -> Option<std::path::PathBuf> {
    let hidraw_dir = std::path::PathBuf::from("/dev");
    for i in 0..16 {
        let path = hidraw_dir.join(format!("hidraw{}", i));
        if path.exists() {
            // Check if it matches our device by reading sysfs
            let sysfs = std::path::PathBuf::from(format!("/sys/class/hidraw/hidraw{}/device", i));
            if let Ok(uevent) = std::fs::read_to_string(sysfs.join("uevent")) {
                if uevent.contains(&format!("{:04X}", vid))
                    && uevent.contains(&format!("{:04X}", pid))
                {
                    return Some(path);
                }
            }
        }
    }
    None
}

fn find_input_event(vid: u16, pid: u16) -> Option<std::path::PathBuf> {
    let input_dir = std::path::PathBuf::from("/dev/input");
    for i in 0..16 {
        let path = input_dir.join(format!("event{}", i));
        if path.exists() {
            // Check sysfs for matching device
            let sysfs =
                std::path::PathBuf::from(format!("/sys/class/input/event{}/device/uevent", i));
            if let Ok(uevent) = std::fs::read_to_string(&sysfs) {
                if uevent.contains(&format!("{:04X}", vid))
                    && uevent.contains(&format!("{:04X}", pid))
                {
                    return Some(path);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A program that never exits, written to disk so it can stand in for
    /// `pw-dump`. On disk rather than on PATH because the tests in this binary
    /// share the environment.
    fn script_program(name: &str, body: &str) -> String {
        use std::os::unix::fs::PermissionsExt;
        let path = std::env::temp_dir().join(format!("epos-dev-{}-{name}", std::process::id()));
        std::fs::write(&path, body).expect("write script");
        let mut perms = std::fs::metadata(&path).expect("stat script").permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&path, perms).expect("chmod script");
        path.to_str().expect("utf-8 temp path").to_string()
    }

    /// `detect()` is awaited during startup and on every hotplug tick, and the
    /// watchdogs only run from that loop. A `pw-dump` that never answers would
    /// therefore stop the daemon from finishing startup and stop every watchdog
    /// from ever running — and a hung PipeWire client is not hypothetical here:
    /// `pw-cli -r` wedged this machine before.
    #[test]
    fn a_hung_pipewire_dump_is_abandoned() {
        let program = script_program("pwdump-hang", "#!/bin/sh\nsleep 30\n");
        let started = std::time::Instant::now();

        let out = capped_dump(&program, std::time::Duration::from_millis(150));

        assert!(out.is_none(), "a hung pw-dump must not be waited on");
        assert!(
            started.elapsed() < std::time::Duration::from_secs(5),
            "must return near its budget, took {:?}",
            started.elapsed()
        );
    }

    /// A dump that answers is still read, and a dump that fails yields nothing
    /// so the caller takes the same fallback it already takes today.
    #[test]
    fn a_working_pipewire_dump_is_still_read() {
        let program = script_program("pwdump-ok", "#!/bin/sh\necho '[{\"info\":{}}]'\n");
        let out = capped_dump(&program, std::time::Duration::from_secs(5))
            .expect("a working dump must be read");
        assert!(out.contains("info"), "stdout must survive: {out}");
    }

    #[test]
    fn a_failing_pipewire_dump_yields_nothing() {
        let program = script_program("pwdump-fail", "#!/bin/sh\nexit 1\n");
        assert!(capped_dump(&program, std::time::Duration::from_secs(5)).is_none());
    }

    /// Trimmed from a real `pw-dump` on this machine, the shape the selector has
    /// to read: JSON, one property per line.
    const REAL_DUMP_WITH_EPOS: &str = r#"[
  { "type": "PipeWire:Interface:Core" },
  { "info": { "props": {
      "node.name": "alsa_output.usb-Generic_USB_Audio-00.HiFi_7_1__Speaker__sink",
      "node.description": "Generic USB Audio Speaker" } } },
  { "type": "PipeWire:Interface:Node",
    "info": { "props": {
      "node.name": "alsa_output.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.analog-stereo",
      "node.description": "EPOS GSX 300 Analog Stereo" } } },
  { "type": "PipeWire:Interface:Node",
    "info": { "props": {
      "node.name": "alsa_input.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.mono-fallback",
      "node.description": "EPOS GSX 300 Mono" } } }
]"#;

    /// The exact situation that broke the audio path on 2026-09-24: the daemon
    /// started while the main graph was still coming up, so the EPOS nodes were
    /// not in the dump yet. The old code answered with wildcard strings, which
    /// were then written into the generated confs and left there — the EQ and
    /// voice chains never published again, and the watchdog dutifully restarted
    /// them onto the same broken conf.
    ///
    /// The answer here has to be "not found", so the caller falls back to the
    /// known-good names instead of inventing unusable ones.
    #[test]
    fn a_dump_without_the_epos_nodes_yields_nothing() {
        let dump = r#"[
  { "type": "PipeWire:Interface:Core" },
  { "type": "PipeWire:Interface:Node",
    "info": { "props": {
      "node.name": "alsa_output.usb-Generic_USB_Audio-00.HiFi_7_1__Speaker__sink",
      "node.description": "Generic USB Audio Speaker" } } }
]"#;
        assert_eq!(
            select_epos_nodes(dump),
            None,
            "a device that has not appeared yet must not produce a name"
        );
    }

    /// A wildcard is never an acceptable answer. It is not a node name, `pactl`
    /// rejects it outright, and writing one into a generated conf produces a
    /// silent, permanent failure instead of an obvious one.
    #[test]
    fn the_selector_never_returns_a_wildcard() {
        for dump in [REAL_DUMP_WITH_EPOS, "[]", "garbage", ""] {
            if let Some((sink, source)) = select_epos_nodes(dump) {
                assert!(!sink.contains('*'), "sink was a wildcard: {sink}");
                assert!(!source.contains('*'), "source was a wildcard: {source}");
            }
        }
    }

    /// The real thing has to be found, or the fallback is all that is ever used
    /// and the serial-specific names stop being verified against reality.
    #[test]
    fn the_real_epos_nodes_are_selected() {
        assert_eq!(
            select_epos_nodes(REAL_DUMP_WITH_EPOS),
            Some((
                "alsa_output.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.analog-stereo"
                    .to_string(),
                "alsa_input.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.mono-fallback"
                    .to_string(),
            ))
        );
    }

    /// Another device's nodes must never be mistaken for the EPOS. Grabbing the
    /// speaker sink here would silently route audio to the wrong hardware.
    #[test]
    fn another_devices_nodes_are_never_claimed_as_epos() {
        let dump = r#"[
  { "type": "PipeWire:Interface:Node",
    "info": { "props": {
      "node.name": "alsa_output.usb-Sennheiser_SOMETHING_ELSE-00.analog-stereo",
      "node.description": "Sennheiser Something Else" } } }
]"#;
        assert_eq!(select_epos_nodes(dump), None);
    }

    /// Only one half being present is not enough: a sink without its source (or
    /// the reverse) would leave the voice chain targeting a name that is not
    /// there, which is how the incident looked from the audio side.
    #[test]
    fn a_half_present_device_is_not_accepted() {
        let sink_only = r#"[
  { "type": "PipeWire:Interface:Node",
    "info": { "props": {
      "node.name": "alsa_output.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.analog-stereo",
      "node.description": "EPOS GSX 300 Analog Stereo" } } }
]"#;
        assert_eq!(select_epos_nodes(sink_only), None);
        assert_eq!(select_epos_nodes("[]"), None);
    }

    /// Unparseable input must be "not found", never a panic and never a guess.
    #[test]
    fn unparseable_or_empty_input_is_not_found() {
        for dump in ["", "   ", "not json at all", "[{\"type\":", "{}"] {
            assert_eq!(select_epos_nodes(dump), None, "input: {dump:?}");
        }
    }

    /// The hardcoded fallback names are the last line of defence, so they have to
    /// be the real ones. If PipeWire ever renames these, a test failing here is
    /// the warning that the fallback has gone stale.
    #[test]
    fn the_fallback_names_match_what_pipewire_actually_publishes() {
        assert_eq!(
            EPOS_SINK_FALLBACK,
            "alsa_output.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.analog-stereo"
        );
        assert_eq!(
            EPOS_SOURCE_FALLBACK,
            "alsa_input.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.mono-fallback"
        );
    }

    /// The description check has to be load-bearing on its own, not merely
    /// alongside the "both halves" rule.
    ///
    /// A different USB interface from the same vendor can present a sink and a
    /// source with exactly the right name shape. Without the description check
    /// this would happily adopt them, and audio would be routed to hardware that
    /// is not the headset.
    #[test]
    fn right_shaped_names_on_the_wrong_description_are_rejected() {
        let dump = r#"[
  { "type": "PipeWire:Interface:Node",
    "info": { "props": {
      "node.name": "alsa_output.usb-Sennheiser_OTHER-00.analog-stereo",
      "node.description": "Sennheiser Other Interface" } } },
  { "type": "PipeWire:Interface:Node",
    "info": { "props": {
      "node.name": "alsa_input.usb-Sennheiser_OTHER-00.mono-fallback",
      "node.description": "Sennheiser Other Interface" } } }
]"#;
        assert_eq!(
            select_epos_nodes(dump),
            None,
            "both halves present, so only the description check can save this"
        );
    }

    /// Only `PipeWire:Interface:Node` objects are nodes. A metadata or port
    /// object that happens to carry node-like properties must not be read as one.
    #[test]
    fn only_node_objects_are_considered() {
        let dump = r#"[
  { "type": "PipeWire:Interface:Port",
    "info": { "props": {
      "node.name": "alsa_output.usb-Sennheiser_EPOS_GSX_300_X-00.analog-stereo",
      "node.description": "EPOS GSX 300 Analog Stereo" } } },
  { "type": "PipeWire:Interface:Port",
    "info": { "props": {
      "node.name": "alsa_input.usb-Sennheiser_EPOS_GSX_300_X-00.mono-fallback",
      "node.description": "EPOS GSX 300 Mono" } } }
]"#;
        assert_eq!(
            select_epos_nodes(dump),
            None,
            "a non-Node object must never be read as a node"
        );
    }

    /// The regression guard for the 2026-09-24 outage, on the path that actually
    /// produced it. Whatever `pw-dump` does — answer with nothing, answer with a
    /// document that has no EPOS in it, answer with garbage, fail to start — the
    /// names handed back must be usable node names, never wildcards.
    ///
    /// This is the function that shells out, so the decision is split out to
    /// make it testable; the invariant lives with the decision rather than with
    /// the subprocess.
    #[test]
    fn no_pw_dump_outcome_can_produce_a_wildcard() {
        let outcomes: [Option<&str>; 6] = [
            None,                                     // pw-dump unavailable
            Some(""),                                 // empty stdout
            Some("[]"),                               // no nodes yet
            Some("not json at all"),                  // unparseable
            Some(REAL_DUMP_WITH_EPOS),                // the good case
            Some(r#"[{"type":"PipeWire:Interface:Node"}]"#),
        ];
        for dump in outcomes {
            let (sink, source) = epos_node_names_from_dump(dump);
            assert!(!sink.contains('*'), "wildcard sink from {dump:?}");
            assert!(!source.contains('*'), "wildcard source from {dump:?}");
        }
    }

    /// And the names handed back must be ones the pipeline can actually use: the
    /// resolved pair, or the real fallback pair. Never a blank, never a mix of
    /// one real name and one wildcard.
    #[test]
    fn the_fallback_pair_is_all_or_nothing() {
        for dump in [None, Some(""), Some("not json")] {
            let (sink, source) = epos_node_names_from_dump(dump);
            assert_eq!(sink, EPOS_SINK_FALLBACK, "dump: {dump:?}");
            assert_eq!(source, EPOS_SOURCE_FALLBACK, "dump: {dump:?}");
        }
        let (sink, source) = epos_node_names_from_dump(Some(REAL_DUMP_WITH_EPOS));
        assert_eq!(sink, EPOS_SINK_FALLBACK, "the live names match the fallback here");
        assert_eq!(source, EPOS_SOURCE_FALLBACK);
    }

    /// A dump that lists only one half must not produce a half-configured pair.
    /// The voice chain targets the source; a blank there means a chain pointing at
    /// nothing, which is how the incident presented from the audio side.
    #[test]
    fn an_unresolved_dump_never_yields_a_blank_name() {
        let half = r#"[
  { "type": "PipeWire:Interface:Node",
    "info": { "props": {
      "node.name": "alsa_output.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.analog-stereo",
      "node.description": "EPOS GSX 300 Analog Stereo" } } }
]"#;
        let (sink, source) = epos_node_names_from_dump(Some(half));
        assert!(!sink.is_empty(), "a blank sink would break the EQ target");
        assert!(!source.is_empty(), "a blank source would break the voice target");
    }

    /// The live names must actually be used when they resolve. The real dump on
    /// this machine happens to carry the same serial as the hardcoded fallback,
    /// so asserting equality with the fallback cannot tell the two paths apart —
    /// a fixture with a different serial is needed to prove the lookup is real
    /// and not a fallback in disguise.
    #[test]
    fn resolved_live_names_win_over_the_fallback() {
        let dump = r#"[
  { "type": "PipeWire:Interface:Node",
    "info": { "props": {
      "node.name": "alsa_output.usb-Sennheiser_EPOS_GSX_300_DIFFERENTSERIAL-00.analog-stereo",
      "node.description": "EPOS GSX 300 Analog Stereo" } } },
  { "type": "PipeWire:Interface:Node",
    "info": { "props": {
      "node.name": "alsa_input.usb-Sennheiser_EPOS_GSX_300_DIFFERENTSERIAL-00.mono-fallback",
      "node.description": "EPOS GSX 300 Mono" } } }
]"#;
        let (sink, source) = epos_node_names_from_dump(Some(dump));
        assert_eq!(
            sink, "alsa_output.usb-Sennheiser_EPOS_GSX_300_DIFFERENTSERIAL-00.analog-stereo",
            "the name PipeWire published must be the one used, not the hardcoded one"
        );
        assert_eq!(
            source, "alsa_input.usb-Sennheiser_EPOS_GSX_300_DIFFERENTSERIAL-00.mono-fallback"
        );
    }
}
