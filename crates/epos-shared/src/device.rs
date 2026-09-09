use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// EPOS GSX 300 USB identifiers
pub const VENDOR_ID: u16 = 0x1395;
pub const PRODUCT_ID: u16 = 0x0098;

/// Detected device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub usb_bus: u8,
    pub usb_addr: u8,
    pub alsa_card: u8,
    pub pipewire_sink: String,
    pub pipewire_source: String,
    pub hidraw: Option<PathBuf>,
    pub input_event: Option<PathBuf>,
    pub firmware_version: Option<String>,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            usb_bus: 0,
            usb_addr: 0,
            alsa_card: 0,
            pipewire_sink: String::new(),
            pipewire_source: String::new(),
            hidraw: None,
            input_event: None,
            firmware_version: None,
        }
    }
}

impl DeviceInfo {
    pub fn is_epos_gs300(vid: u16, pid: u16) -> bool {
        vid == VENDOR_ID && pid == PRODUCT_ID
    }
}
