use serde::{Deserialize, Serialize};

/// The ring's off byte. Measured on hardware, and the only value that means
/// "nothing lit" on a two-bit register.
pub fn default_vendor_off() -> u8 {
    0x00
}

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
    /// Vendor Report ID 0x02 output byte for "the device is not in use".
    ///
    /// The ring is a 2-bit shift register (`$1388`/`$1389`, firmware
    /// `FREEMAN_V03.01.00.00`), so the whole palette is four states: off, red,
    /// blue, both. There is no green to ask for — the hardware has no third
    /// channel. `0x00` is the measured off value and is what "not listening on
    /// the EPOS" shows.
    ///
    /// Optional on disk: a config written before this field existed loads with
    /// the default, which is the same `0x00` the daemon was already writing when
    /// it had nothing to show.
    #[serde(default = "default_vendor_off")]
    pub vendor_off: u8,
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
            vendor_off: default_vendor_off(),
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
