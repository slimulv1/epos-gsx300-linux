use serde::{Deserialize, Serialize};

use crate::config::{AudioConfig, AudioMode};
use crate::device::DeviceInfo;

/// What the daemon knows about the microphone's actual output.
///
/// `Unknown` is a real state, not a placeholder: it covers both "no probe has
/// run yet" and "the last probe was inconclusive, or was only a suspicion", and
/// collapsing it into `Signal` would be exactly the false confidence this type
/// exists to remove.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MicInputState {
    /// The capture is carrying audio — at minimum the preamp noise floor, which
    /// is present whether or not anyone is speaking.
    Signal,
    /// The capture is running but digitally silent: connected, and converting
    /// nothing.
    Silent,
    /// Not established.
    Unknown,
}

/// IPC request from GUI → Daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Request {
    // Status
    GetStatus,
    GetDevice,

    // Audio
    SetEq {
        eq: AudioConfig,
    },
    GetEq,
    SetSidetone {
        enabled: bool,
        level: f32,
    },
    SetNoiseGate {
        enabled: bool,
        threshold_db: f32,
    },
    SetVoiceEnhancer {
        mode: String,
        custom_bands: Option<Vec<crate::EqBand>>,
    },
    SetMicGain {
        gain: u32,
    },

    // Audio mode / LED
    /// Get current audio mode (stereo/7.1)
    GetMode,
    /// Set audio mode — automatically changes LED ring color
    SetMode {
        mode: AudioMode,
    },
    /// Toggle between stereo and 7.1 (for smart button)
    ToggleMode,

    // Profiles
    GetProfiles,
    SetActiveProfile {
        name: String,
    },
    CreateProfile {
        name: String,
        audio: AudioConfig,
    },
    DeleteProfile {
        name: String,
    },

    // Smart button
    SetSmartButton {
        action: String,
    },

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
        /// How many bands are actually in the EQ filter graph right now.
        ///
        /// Separate from `eq_active` because the two are independent: bands
        /// flatter than 0.1 dB are dropped when the conf is generated, so an
        /// enabled EQ with a flat curve puts no filter in the graph at all. The
        /// audio is correct — a flat curve is transparent — but a lone
        /// `eq_active: true` tells the user the EQ is working while nothing is
        /// filtering, and there is no way to tell that apart from a real curve
        /// without this number.
        eq_active_bands: usize,
        /// Whether the microphone is actually delivering audio.
        ///
        /// Separate from `device_connected`, which only means the USB device is
        /// present. A muted capture element, a stale source after ALSA
        /// re-enumerated, or a converter that has stopped working all leave the
        /// device "connected" and every node published while applications record
        /// silence — so without this the status cannot tell a working microphone
        /// from a dead one.
        ///
        /// Starts as `Unknown` and only becomes `Signal` or `Silent` once a
        /// capture has actually been judged, because "not checked" is not the
        /// same claim as "checked and fine".
        mic_input: MicInputState,
        /// Is the EQ actually in the audio path right now?
        ///
        /// Distinct from `eq_active`, which is only the toggle. Once a user can
        /// select their own output device, playback routed elsewhere means the EQ
        /// is genuinely bypassed — and `eq_active: true` on its own would report a
        /// working equaliser that is not touching the sound, which is the exact
        /// class of "enabled but not actually working" this status has been
        /// corrected for twice already.
        eq_in_path: bool,
        active_profile: String,
        mode: AudioMode,
        smart_button_action: String,
        /// Whether the EPOS is engaged at all.
        ///
        /// False from daemon start until a long press on the smart button, and
        /// never persisted, so a reboot returns to off. Distinct from
        /// `eq_in_path`, which asks whether audio is going through the chain
        /// *right now*: with the EPOS off the instances are stopped, so the
        /// chain does not exist and `eq_in_path` is trivially false. A GUI that
        /// showed the equaliser as available and adjustable while the headset was
        /// switched off would be describing a control that does nothing.
        epos_powered: bool,
        sidetone_enabled: bool,
        noise_gate_enabled: bool,
        voice_enhancer_enabled: bool,
        /// Monotonic press counter for the physical smart button. The daemon's
        /// five DSP-apply arms `fetch_add(1)` on every smart-button dispatch
        /// (main.rs). The GUI diffs this across its 3s Status poll so it can
        /// fire a desktop notification ONLY when the *smart button* changed a
        /// DSP/mode — GUI-initiated SetMode/SetEq (which route through
        /// `Request::Set*` and never touch this counter) never trigger one.
        smart_button_seq: u64,
        volume: i32,
    },
    Device(Option<DeviceInfo>),
    Eq(AudioConfig),
    Mode(AudioMode),
    Profiles(Vec<crate::Profile>),
    Error {
        message: String,
    },
}

/// IPC event pushed from Daemon → GUI
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Event {
    /// `alsa_card` is `None` while ALSA has not enumerated the device yet, so
    /// a consumer can tell "unknown" apart from card 0 (a real index).
    DeviceConnected { alsa_card: Option<u8>, name: String },
    DeviceDisconnected,
    ProfileChanged { name: String },
    ModeChanged { mode: AudioMode },
}
