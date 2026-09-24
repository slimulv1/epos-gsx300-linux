use serde::{Deserialize, Serialize};

/// LED color bytes — configurable via config.json for hardware testing.
/// Protocol is undocumented; adjust these values after running led-probe.py.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedProbeConfig {
    /// Vendor Report ID 0x02 output byte for Blue (Stereo).
    ///
    /// Measured on hardware 2026-09-24: `0x02` renders blue, `0x01` renders
    /// red. This is the **reverse** of the descriptor's own usage names
    /// (`ff13.0005` = bit0, `ff13.0006` = bit1), so the vendor descriptor
    /// mislabels its LED bits. Do not "correct" these back to `0x01`/`0x02`
    /// on the strength of the descriptor alone — that was tried and it
    /// displays Stereo as red.
    pub vendor_blue: u8,
    /// Vendor Report ID 0x02 output byte for Red (7.1) — `0x01`, see
    /// [`LedProbeConfig::vendor_blue`].
    pub vendor_red: u8,
    /// Alternative: Primary Report ID 0x04 payload for Blue (38 bytes).
    ///
    /// Never exercised: the shipped default config sets this to `null`, and
    /// the memory-bus path is guarded against EEPROM writes. Unverified.
    pub primary_blue: Option<Vec<u8>>,
    /// Alternative: Primary Report ID 0x04 payload for Red (38 bytes).
    /// Never exercised and unverified, see [`LedProbeConfig::primary_blue`].
    pub primary_red: Option<Vec<u8>>,
    /// Which report path to use: "vendor" (Report ID 2) or "primary" (Report ID 4)
    pub use_report: LedReportPath,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LedReportPath {
    /// Use vendor Report ID 0x02 (1-byte output) — simplest
    Vendor,
    /// Use consumer Report ID 0x04 (38-byte output) — more control
    Primary,
}

impl Default for LedProbeConfig {
    fn default() -> Self {
        Self {
            vendor_blue: 0x02,
            vendor_red: 0x01,
            primary_blue: Some({
                let mut buf = vec![0u8; 38];
                buf[0] = 0x01;
                buf
            }),
            primary_red: Some({
                let mut buf = vec![0u8; 38];
                buf[0] = 0x02;
                buf
            }),
            use_report: LedReportPath::Vendor,
        }
    }
}
