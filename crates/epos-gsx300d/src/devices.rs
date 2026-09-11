use anyhow::Result;
use tracing::{info, warn};
use epos_shared::device::DeviceInfo;

/// Detect EPOS GSX 300 on USB bus
pub async fn detect() -> Option<DeviceInfo> {
    // Scan /sys/bus/usb/devices for matching VID:PID
    match scan_usb_devices() {
        Ok(devices) => {
            if let Some(dev) = devices.first() {
                info!("Found EPOS GSX 300 at bus {}:{}", dev.usb_bus, dev.usb_addr);
                Some(dev.clone())
            } else {
                None
            }
        }
        Err(e) => {
            warn!("USB scan failed: {}", e);
            None
        }
    }
}

fn scan_usb_devices() -> Result<Vec<DeviceInfo>> {
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
            let alsa_card = find_alsa_card(vid, pid).unwrap_or(0);

            // Find PipeWire node names
            let (sink, source) = find_pipewire_nodes(alsa_card);

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
            });
        }
    }

    Ok(devices)
}

fn find_alsa_card(_vid: u16, _pid: u16) -> Option<u8> {
    let proc_sound = std::path::PathBuf::from("/proc/asound");
    if !proc_sound.exists() {
        return None;
    }

    // Read /proc/asound/cards to find matching card.
    //
    // IMPORTANT: match by device NAME, not by "USB Audio" — the system may
    // host several USB audio devices (e.g. a Generic USB Audio card with its
    // own 'Mic Capture Volume' control). Matching "USB Audio" blindly returns
    // whichever card is enumerated first, so mic gain was applied to the WRONG
    // device while amixer happily reported success. EPOS shows up as
    // "EPOS GSX 300" / "Sennheiser EPOS GSX 300" in the cards file.
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

fn find_pipewire_nodes(_card: u8) -> (String, String) {
    // Resolve the real PipeWire node names for this device by querying pw-dump.
    // Falls back to wildcard patterns if pw-dump is unavailable.
    let dump = std::process::Command::new("pw-dump")
        .output()
        .ok()
        .and_then(|o| (o.status.success()).then(|| String::from_utf8_lossy(&o.stdout).into_owned()));

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
                if t == "}" || t == "}" || t.starts_with(']') {
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
                    let v = t[eq + 1..].trim().trim_matches(',').trim_matches('"').to_string();
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
            let sysfs = std::path::PathBuf::from(format!(
                "/sys/class/input/event{}/device/uevent",
                i
            ));
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
