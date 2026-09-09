use serde::{Deserialize, Serialize};

/// Root configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: u32,
    pub device: DeviceConfig,
    pub audio: AudioConfig,
    pub profiles: Vec<Profile>,
    pub active_profile: String,
    pub smart_button: SmartButtonConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub auto_detect: bool,
    pub usb_vid: String,
    pub usb_pid: String,
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
            },
            audio: AudioConfig::default(),
            profiles: vec![
                Profile::flat(),
                Profile::music(),
                Profile::movie(),
                Profile::esport(),
            ],
            active_profile: "Flat".into(),
            smart_button: SmartButtonConfig {
                action: SmartButtonAction::CyclePreset,
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
            name: "Flat".into(),
            audio: AudioConfig::default(),
            created_at: "2026-09-09".into(),
        }
    }

    pub fn music() -> Self {
        let mut audio = AudioConfig::default();
        audio.eq.enabled = true;
        audio.eq.bands = vec![
            EqBand { freq: 64, gain_db: 4.0, q: 1.0 },
            EqBand { freq: 125, gain_db: 2.0, q: 1.0 },
            EqBand { freq: 250, gain_db: -1.0, q: 1.0 },
            EqBand { freq: 500, gain_db: -2.0, q: 1.0 },
            EqBand { freq: 1000, gain_db: 0.0, q: 1.0 },
            EqBand { freq: 2000, gain_db: 2.0, q: 1.0 },
            EqBand { freq: 4000, gain_db: 3.0, q: 1.0 },
            EqBand { freq: 8000, gain_db: 2.0, q: 1.0 },
            EqBand { freq: 16000, gain_db: 1.0, q: 1.0 },
        ];
        Self {
            name: "Music".into(),
            audio,
            created_at: "2026-09-09".into(),
        }
    }

    pub fn movie() -> Self {
        let mut audio = AudioConfig::default();
        audio.eq.enabled = true;
        audio.eq.bands = vec![
            EqBand { freq: 64, gain_db: 5.0, q: 1.0 },
            EqBand { freq: 125, gain_db: 3.0, q: 1.0 },
            EqBand { freq: 250, gain_db: 0.0, q: 1.0 },
            EqBand { freq: 500, gain_db: -1.0, q: 1.0 },
            EqBand { freq: 1000, gain_db: 1.0, q: 1.0 },
            EqBand { freq: 2000, gain_db: 2.0, q: 1.0 },
            EqBand { freq: 4000, gain_db: 3.0, q: 1.0 },
            EqBand { freq: 8000, gain_db: 1.0, q: 1.0 },
            EqBand { freq: 16000, gain_db: 0.0, q: 1.0 },
        ];
        Self {
            name: "Movie".into(),
            audio,
            created_at: "2026-09-09".into(),
        }
    }

    pub fn esport() -> Self {
        let mut audio = AudioConfig::default();
        audio.eq.enabled = true;
        audio.eq.bands = vec![
            EqBand { freq: 64, gain_db: -3.0, q: 1.0 },
            EqBand { freq: 125, gain_db: -2.0, q: 1.0 },
            EqBand { freq: 250, gain_db: 0.0, q: 1.0 },
            EqBand { freq: 500, gain_db: 2.0, q: 1.0 },
            EqBand { freq: 1000, gain_db: 3.0, q: 1.0 },
            EqBand { freq: 2000, gain_db: 4.0, q: 1.0 },
            EqBand { freq: 4000, gain_db: 3.0, q: 1.0 },
            EqBand { freq: 8000, gain_db: 1.0, q: 1.0 },
            EqBand { freq: 16000, gain_db: 0.0, q: 1.0 },
        ];
        Self {
            name: "eSport".into(),
            audio,
            created_at: "2026-09-09".into(),
        }
    }
}
