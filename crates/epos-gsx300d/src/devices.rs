use anyhow::Result;
use epos_shared::device::DeviceInfo;
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
                (None, true) => find_pipewire_nodes(alsa_card),
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

fn find_pipewire_nodes(_card: Option<u8>) -> (String, String) {
    // Resolve the real PipeWire node names for this device by querying pw-dump.
    // Falls back to wildcard patterns if pw-dump is unavailable.
    let dump = std::process::Command::new("pw-dump")
        .output()
        .ok()
        .and_then(|o| {
            (o.status.success()).then(|| String::from_utf8_lossy(&o.stdout).into_owned())
        });

    let mut sink = "alsa_output.usb-*:*.analog-stereo".to_string();
    let mut source = "alsa_input.usb-*:*.mono-fallback".to_string();

    if let Some(dump) = dump {
        // Find nodes whose description mentions EPOS GSX 300
        let mut in_node = false;
        let mut props: Vec<(String, String)> = Vec::new();
        for line in dump.lines() {
            let t = line.trim();
            if t.contains("PipeWire:Interface:Node") {
                in_node = true;
                props.clear();
                continue;
            }
            if in_node {
                if t == "}" || t.starts_with(']') {
                    // node object boundary — evaluate collected props
                    let desc = props.iter().find(|(k, _)| k == "node.description");
                    let name = props.iter().find(|(k, _)| k == "node.name");
                    if let (Some((_, d)), Some((_, n))) = (desc, name) {
                        if d.contains("EPOS GSX 300") {
                            if n.contains("output") && n.contains("analog-stereo") {
                                sink = n.clone();
                            } else if n.contains("input") && n.contains("mono-fallback") {
                                source = n.clone();
                            }
                        }
                    }
                    in_node = false;
                } else if let Some(eq) = t.find(':') {
                    // JSON format: "key": "value"  or  "key": value
                    let k = t[..eq].trim().trim_matches('"').to_string();
                    let v = t[eq + 1..]
                        .trim()
                        .trim_matches(',')
                        .trim_matches('"')
                        .to_string();
                    props.push((k, v));
                }
            }
        }
    }

    (sink, source)
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
