use serde::{Deserialize, Serialize};

use crate::config::AudioConfig;
use crate::device::DeviceInfo;

/// IPC request from GUI → Daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Request {
    // Status
    GetStatus,
    GetDevice,

    // Audio
    SetEq { eq: AudioConfig },
    GetEq,
    SetSidetone { enabled: bool, level: f32 },
    SetNoiseGate { enabled: bool, threshold_db: f32 },
    SetVoiceEnhancer { mode: String, custom_bands: Option<Vec<crate::EqBand>> },
    SetMicGain { gain: u32 },

    // Profiles
    GetProfiles,
    SetActiveProfile { name: String },
    CreateProfile { name: String, audio: AudioConfig },
    DeleteProfile { name: String },

    // Smart button
    SetSmartButton { action: String },

    // Lifecycle
    Reload,
    Quit,
}

/// IPC response from Daemon → GUI
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Response {
    Ok,
    Status {
        daemon_version: String,
        device_connected: bool,
        eq_active: bool,
        active_profile: String,
    },
    Device(Option<DeviceInfo>),
    Eq(AudioConfig),
    Profiles(Vec<crate::Profile>),
    Error { message: String },
}

/// IPC event pushed from Daemon → GUI
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Event {
    DeviceConnected { alsa_card: u8, name: String },
    DeviceDisconnected,
    ProfileChanged { name: String },
}
