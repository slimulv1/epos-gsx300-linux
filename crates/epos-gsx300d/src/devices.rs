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

    // Read /proc/asound/cards to find matching card
    if let Ok(cards) = std::fs::read_to_string(proc_sound.join("cards")) {
        for line in cards.lines() {
            // Look for USB Audio with matching vendor/product
            if line.contains("USB Audio") {
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
    // Construct expected PipeWire node names
    let sink = format!("alsa_output.usb-*:*.analog-stereo");
    let source = format!("alsa_input.usb-*:*.mono-fallback");
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
