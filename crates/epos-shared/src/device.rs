use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// EPOS GSX 300 USB identifiers
pub const VENDOR_ID: u16 = 0x1395;
pub const PRODUCT_ID: u16 = 0x0098;

/// Live runtime state snapshot, read read-only from the device memory bus
/// (firmware RE decode — see docs/reverse-engineering/FIRMWARE-REPORT.md).
///
/// All values are `Option`: a missing value means that memory region could
/// not be read (device gone, permission denied, timeout) — never a failure
/// of the whole snapshot. Reading never sets bit6; it is pure observation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HwSnapshot {
    /// Audio mode state machine register $137D (0-5; LED processing gated to {3,4}).
    pub mode_state: Option<u8>,
    /// Secondary state register $137F.
    pub secondary_state: Option<u8>,
    /// LED shift-register pair $1388/$1389 (low byte = $1388, high = $1389).
    pub led_shift: Option<u16>,
    /// EQ-enable flags $1386/$1387 (bit0 of each byte = respective EQ path).
    pub eq_enable: Option<u8>,
    /// EQ1 band index $12D1 (0-based; written only at $BCF9).
    pub eq1_index: Option<u8>,
    /// EQ2 band index $12D0.
    pub eq2_index: Option<u8>,
    /// DSP state byte $0D08 (0x01/0x02/0x04 = mode classes).
    pub dsp_state: Option<u8>,
    /// Encoder position A: `( $0FC6 & 7 ) << 8 | $0FC7` (11-bit effective).
    pub encoder_a: Option<u16>,
    /// Encoder position B: `( $0FC4 & 7 ) << 8 | $0FC5`.
    pub encoder_b: Option<u16>,
    /// Control register $0F13: low nibble = EQ2 active band index.
    pub ctrl_0f13: Option<u8>,
    /// Control register $0F18: bits 0-1 mode, bit 5 disable.
    pub ctrl_0f18: Option<u8>,
    /// Control register $0F1D: 3-bit state.
    pub ctrl_0f1d: Option<u8>,
}

impl HwSnapshot {
    /// True when no register could be read (device absent / timing out).
    pub fn is_empty(&self) -> bool {
        self.mode_state.is_none()
            && self.secondary_state.is_none()
            && self.led_shift.is_none()
            && self.eq_enable.is_none()
            && self.eq1_index.is_none()
            && self.eq2_index.is_none()
            && self.dsp_state.is_none()
            && self.encoder_a.is_none()
            && self.encoder_b.is_none()
            && self.ctrl_0f13.is_none()
            && self.ctrl_0f18.is_none()
            && self.ctrl_0f1d.is_none()
    }
}

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
    /// Live runtime snapshot (read-only memory-bus read at request time).
    pub hw_snapshot: Option<HwSnapshot>,
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
            hw_snapshot: None,
        }
    }
}

impl DeviceInfo {
    pub fn is_epos_gs300(vid: u16, pid: u16) -> bool {
        vid == VENDOR_ID && pid == PRODUCT_ID
    }
}
