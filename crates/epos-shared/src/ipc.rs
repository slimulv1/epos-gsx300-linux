use serde::{Deserialize, Serialize};

use crate::config::{AudioConfig, AudioMode};
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

    // Audio mode / LED
    /// Get current audio mode (stereo/7.1)
    GetMode,
    /// Set audio mode — automatically changes LED ring color
    SetMode { mode: AudioMode },
    /// Toggle between stereo and 7.1 (for smart button)
    ToggleMode,

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
        mode: AudioMode,
    },
    Device(Option<DeviceInfo>),
    Eq(AudioConfig),
    Mode(AudioMode),
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
    ModeChanged { mode: AudioMode },
}
