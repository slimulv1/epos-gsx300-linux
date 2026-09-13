use serde::{Deserialize, Serialize};

use crate::led::LedProbeConfig;

/// Audio output mode — controls LED ring color
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum AudioMode {
    /// Stereo 2.0 — LED ring blue
    #[default]
    Stereo,
    /// Virtual surround 7.1 — LED ring red
    Surround71,
}

impl AudioMode {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Stereo => "Stereo (2.0)",
            Self::Surround71 => "Surround (7.1)",
        }
    }
}

/// Root configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: u32,
    pub device: DeviceConfig,
    pub audio: AudioConfig,
    /// Audio output mode (stereo vs 7.1) — controls LED ring color
    #[serde(default)]
    pub mode: AudioMode,
    /// LED probe config (HID report bytes for LED control — adjust after hardware testing)
    #[serde(default)]
    pub led_probe: Option<LedProbeConfig>,
    pub profiles: Vec<Profile>,
    pub active_profile: String,
    pub smart_button: SmartButtonConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub auto_detect: bool,
    pub usb_vid: String,
    pub usb_pid: String,
    /// Last-known volume dial position (0-100), persisted host-side.
    ///
    /// The GSX 300 has NO absolute HID volume readback and NO NVM persistence
    /// (verified in firmware RE / NVM-PERSISTENCE-REPORT) — the host config is
    /// the only record of the dial position across daemon restarts. `None`
    /// means "never persisted yet" → daemon falls back to the device power-on
    /// default (100).
    #[serde(default)]
    pub volume: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub eq: EqConfig,
    pub sidetone: SidetoneConfig,
    pub noise_gate: NoiseGateConfig,
    pub voice_enhancer: VoiceEnhancerConfig,
    pub mic_gain: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqConfig {
    pub enabled: bool,
    pub bands: Vec<EqBand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqBand {
    pub freq: u32,
    pub gain_db: f32,
    #[serde(default = "default_q")]
    pub q: f32,
}

fn default_q() -> f32 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidetoneConfig {
    pub enabled: bool,
    pub level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseGateConfig {
    pub enabled: bool,
    pub threshold_db: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceEnhancerConfig {
    pub mode: VoiceMode,
    pub custom_bands: Option<Vec<EqBand>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VoiceMode {
    Off,
    Warm,
    Clear,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub audio: AudioConfig,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartButtonConfig {
    pub action: SmartButtonAction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SmartButtonAction {
    /// Toggle stereo ⇄ 7.1 (matches EPOS Gaming Suite default: LED blue ⇄ red)
    ToggleMode,
    ToggleEq,
    CyclePreset,
    ToggleSidetone,
    ToggleNoiseGate,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            device: DeviceConfig {
                auto_detect: true,
                usb_vid: "1395".into(),
                usb_pid: "0098".into(),
                volume: None,
            },
            audio: AudioConfig::default(),
            mode: AudioMode::Stereo,
            led_probe: None,
            profiles: vec![
                Profile::flat(),
                Profile::music(),
                Profile::movie(),
                Profile::esport(),
            ],
            active_profile: "FLAT".into(),
            smart_button: SmartButtonConfig {
                action: SmartButtonAction::ToggleMode,
            },
        }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            eq: EqConfig {
                enabled: false,
                bands: Self::default_bands(),
            },
            sidetone: SidetoneConfig {
                enabled: false,
                level: 0.0,
            },
            noise_gate: NoiseGateConfig {
                enabled: false,
                threshold_db: -30.0,
            },
            voice_enhancer: VoiceEnhancerConfig {
                mode: VoiceMode::Off,
                custom_bands: None,
            },
            mic_gain: 80,
        }
    }
}

impl AudioConfig {
    pub fn default_bands() -> Vec<EqBand> {
        vec![
            EqBand { freq: 64, gain_db: 0.0, q: 1.0 },
            EqBand { freq: 125, gain_db: 0.0, q: 1.0 },
            EqBand { freq: 250, gain_db: 0.0, q: 1.0 },
            EqBand { freq: 500, gain_db: 0.0, q: 1.0 },
            EqBand { freq: 1000, gain_db: 0.0, q: 1.0 },
            EqBand { freq: 2000, gain_db: 0.0, q: 1.0 },
            EqBand { freq: 4000, gain_db: 0.0, q: 1.0 },
            EqBand { freq: 8000, gain_db: 0.0, q: 1.0 },
            EqBand { freq: 16000, gain_db: 0.0, q: 1.0 },
        ]
    }
}

impl Profile {
    pub fn flat() -> Self {
        Self {
            name: "FLAT".into(),
            audio: AudioConfig::default(),
            created_at: "2026-09-09".into(),
        }
    }

    /// MUSIC — smile curve, values measured from EPOS Gaming Suite screenshots
    /// (64:+0, 125:+1, 250:+3.5, 500:0, 1k:-2.5, 2k:-2, 4k:-0.5, 8k:+3, 16k:+5.5)
    pub fn music() -> Self {
        let mut audio = AudioConfig::default();
        audio.eq.enabled = true;
        audio.eq.bands = vec![
            EqBand { freq: 64, gain_db: 0.0, q: 1.0 },
            EqBand { freq: 125, gain_db: 1.0, q: 1.0 },
            EqBand { freq: 250, gain_db: 4.5, q: 1.0 },
            EqBand { freq: 500, gain_db: 0.5, q: 1.0 },
            EqBand { freq: 1000, gain_db: -1.0, q: 1.0 },
            EqBand { freq: 2000, gain_db: -0.5, q: 1.0 },
            EqBand { freq: 4000, gain_db: 0.0, q: 1.0 },
            EqBand { freq: 8000, gain_db: 2.5, q: 1.0 },
            EqBand { freq: 16000, gain_db: 5.5, q: 1.0 },
        ];
        Self {
            name: "MUSIC".into(),
            audio,
            created_at: "2026-09-09".into(),
        }
    }

    /// MOVIE — double-peak (250Hz body + 16kHz air);
    /// values measured from EPOS Gaming Suite screenshots
    pub fn movie() -> Self {
        let mut audio = AudioConfig::default();
        audio.eq.enabled = true;
        audio.eq.bands = vec![
            EqBand { freq: 64, gain_db: 1.0, q: 1.0 },
            EqBand { freq: 125, gain_db: 1.5, q: 1.0 },
            EqBand { freq: 250, gain_db: 4.5, q: 1.0 },
            EqBand { freq: 500, gain_db: 0.5, q: 1.0 },
            EqBand { freq: 1000, gain_db: 0.0, q: 1.0 },
            EqBand { freq: 2000, gain_db: 0.0, q: 1.0 },
            EqBand { freq: 4000, gain_db: 0.5, q: 1.0 },
            EqBand { freq: 8000, gain_db: 3.0, q: 1.0 },
            EqBand { freq: 16000, gain_db: 6.0, q: 1.0 },
        ];
        Self {
            name: "MOVIE".into(),
            audio,
            created_at: "2026-09-09".into(),
        }
    }

    /// ESPORT (TREBLE) — cut mid-bass (250Hz) + boosted highs (4k-16k);
    /// values measured from EPOS Gaming Suite screenshots
    pub fn esport() -> Self {
        let mut audio = AudioConfig::default();
        audio.eq.enabled = true;
        audio.eq.bands = vec![
            EqBand { freq: 64, gain_db: -1.0, q: 1.0 },
            EqBand { freq: 125, gain_db: -1.0, q: 1.0 },
            EqBand { freq: 250, gain_db: -4.5, q: 1.0 },
            EqBand { freq: 500, gain_db: -2.5, q: 1.0 },
            EqBand { freq: 1000, gain_db: 2.0, q: 1.0 },
            EqBand { freq: 2000, gain_db: 3.5, q: 1.0 },
            EqBand { freq: 4000, gain_db: 5.5, q: 1.0 },
            EqBand { freq: 8000, gain_db: 6.0, q: 1.0 },
            EqBand { freq: 16000, gain_db: 6.0, q: 1.0 },
        ];
        Self {
            name: "ESPORT (TREBLE)".into(),
            audio,
            created_at: "2026-09-09".into(),
        }
    }
}
