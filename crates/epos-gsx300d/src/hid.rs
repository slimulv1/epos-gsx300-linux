use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use epos_shared::config::AudioMode;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, info, warn};

/// Events parsed from the GSX 300 HID input stream.
#[derive(Debug, Clone, Copy)]
pub enum HidEvent {
    /// Smart button pressed — device reports its current mode state after toggling.
    /// Value 0x01 = stereo, 0x02 = 7.1 (matches the LED output bits exactly).
    ModeChanged(AudioMode),
    /// Smart button long-press (value 0x04). Confirmed on hardware: holding the
    /// dial for ~2s sends this report. Treated as a mode toggle.
    LongPress,
    /// Volume dial detent (+1 = up, -1 = down). The device applies gain locally;
    /// the daemon only logs this for now.
    VolumeChanged(i32),
}

/// HID input listener for the smart button and volume dial.
///
/// Reads the GSX 300's hidraw device in a dedicated blocking thread and forwards
/// parsed events through a tokio channel. The device sends:
///   - Report ID 0x01 (Consumer): volume dial — 0x01 = up, 0x02 = down
///   - Report ID 0x02 (Vendor):   mode state readback — 0x01 = stereo, 0x02 = 7.1
pub struct HidHandler {
    tx: UnboundedSender<HidEvent>,
}

impl HidHandler {
    pub fn new(tx: UnboundedSender<HidEvent>) -> Self {
        Self { tx }
    }

    /// Start the blocking HID reader thread. Never returns until daemon exit.
    ///
    /// Retries finding the hidraw node forever: the daemon may start before the
    /// device is plugged in (or the device may be unplugged/replugged at any
    /// time), and hidraw node numbers change on re-enumeration — so after any
    /// read failure or EOF we close the file and re-scan from scratch.
    pub fn spawn_reader(&self) -> thread::JoinHandle<()> {
        let tx = self.tx.clone();
        thread::spawn(move || loop {
            // Outer retry loop: locate the device (may be absent at boot).
            let path = match Self::find_hidraw() {
                Some(p) => p,
                None => {
                    thread::sleep(Duration::from_secs(2));
                    continue;
                }
            };

            let mut file = match OpenOptions::new().read(true).open(&path) {
                Ok(f) => f,
                Err(e) => {
                    warn!(
                        "Failed to open {} for reading — retrying: {}",
                        path.display(),
                        e
                    );
                    thread::sleep(Duration::from_secs(2));
                    continue;
                }
            };
            info!("HID listener active on {}", path.display());

            let mut buf = [0u8; 64];
            let mut stuck_errors = 0;
            loop {
                match file.read(&mut buf) {
                    Ok(0) => {
                        // EOF — device closed/unplugged. Re-scan from scratch.
                        info!(
                            "HID stream closed on {} — re-scanning for device",
                            path.display()
                        );
                        break;
                    }
                    Ok(n) => {
                        stuck_errors = 0;
                        let rid = buf[0];
                        let val = buf.get(1).copied().unwrap_or(0);
                        match rid {
                            0x01 => match val {
                                0x01 => {
                                    let _ = tx.send(HidEvent::VolumeChanged(1));
                                }
                                0x02 => {
                                    let _ = tx.send(HidEvent::VolumeChanged(-1));
                                }
                                0x00 => {} // release
                                _ => debug!("HID 0x01 value {:#04x}", val),
                            },
                            0x02 => {
                                let mode = match val {
                                    0x01 => Some(AudioMode::Stereo),
                                    0x02 => Some(AudioMode::Surround71),
                                    0x04 => None, // long press — handled below
                                    _ => None,
                                };
                                if let Some(m) = mode {
                                    let _ = tx.send(HidEvent::ModeChanged(m));
                                } else if val == 0x04 {
                                    debug!("HID 0x02 value 0x04 — smart button long press");
                                    let _ = tx.send(HidEvent::LongPress);
                                } else {
                                    debug!("HID 0x02 unknown value {:#04x}", val);
                                }
                            }
                            _ => {
                                debug!("HID report {:#04x}: {:02x?}", rid, &buf[..n.min(buf.len())])
                            }
                        }
                    }
                    Err(e) => {
                        // Stale fd after unplug returns EIO/ENODEV; a few of
                        // these may race the device teardown, but if it keeps
                        // failing the file is dead — drop it and re-scan.
                        stuck_errors += 1;
                        if stuck_errors > 3 {
                            warn!("HID read persistently failing: {} — re-scanning", e);
                            break;
                        }
                        thread::sleep(Duration::from_secs(1));
                    }
                }
            }
        })
    }

    /// Find the GSX 300 hidraw node via sysfs uevent (VID 1395 / PID 0098).
    fn find_hidraw() -> Option<PathBuf> {
        let base = "/sys/class/hidraw";
        let mut entries: Vec<_> = std::fs::read_dir(base).ok()?.collect();
        entries.sort_by_key(|e| e.as_ref().map(|e| e.file_name()).ok());
        for entry in entries {
            let entry = entry.ok()?;
            let uevent = entry.path().join("device").join("uevent");
            if let Ok(content) = std::fs::read_to_string(&uevent) {
                if content.contains("00001395") && content.contains("00000098") {
                    return Some(PathBuf::from("/dev").join(entry.file_name()));
                }
            }
        }
        None
    }
}
