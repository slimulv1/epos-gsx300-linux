use crate::audio::AudioPipeline;
use crate::config;
use crate::devices;
use crate::hwinfo;
use crate::hwinfo::HwInfo;
use crate::led::LedController;
use epos_shared::config::AudioConfig;
use epos_shared::config::AudioMode;
use epos_shared::config::FLAT_PROFILE_NAME;
use epos_shared::ipc::{Request, Response};
use epos_shared::Config;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};
use tokio::sync::Notify;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use anyhow::Result;

/// True when a request mutates daemon state (RAM + config file). Read-only
/// requests (GetStatus/GetEq/GetMode/GetProfiles/GetDevice) return here false,
/// so the GUI's 3s status polling never rewrites the config file to disk.
pub fn request_is_mutation(req: &Request) -> bool {
    matches!(
        req,
        Request::SetEq { .. }
            | Request::SetSidetone { .. }
            | Request::SetNoiseGate { .. }
            | Request::SetVoiceEnhancer { .. }
            | Request::SetMicGain { .. }
            | Request::SetMode { .. }
            | Request::ToggleMode
            | Request::SetActiveProfile { .. }
            | Request::CreateProfile { .. }
            | Request::DeleteProfile { .. }
            | Request::SetSmartButton { .. }
            | Request::Reload
            | Request::Quit
    )
}

/// Mirror the live audio config back into the currently-active profile.
///
/// Previously the live setters (SetEq / SetVoiceEnhancer / SetNoiseGate /
/// SetSidetone / SetMicGain) only updated the top-level `config.audio`, leaving
/// the selected profile's own `audio` stale. Switching to another profile and
/// back then silently reverted every change the user had made. Treat the
/// active profile as "the profile you are editing", so live edits stick.
///
/// Takes the two pieces separately (rather than `&mut IpcState`) so the rule is
/// directly testable without constructing a live audio pipeline.
fn sync_profile_audio(profiles: &mut [epos_shared::Profile], active: &str, live: &AudioConfig) {
    if active.is_empty() {
        return;
    }
    // Case-insensitive, as the shared config contract requires: the shipped
    // `active_profile` is "FLAT" while configs in the wild also use "Flat",
    // and an exact compare silently left the active profile stale, so an EQ or
    // mic change reverted on the next profile switch. `DeleteProfile` already
    // compares this way.
    if let Some(p) = profiles
        .iter_mut()
        .find(|p| p.name.eq_ignore_ascii_case(active))
    {
        p.audio = live.clone();
    }
}

fn sync_active_profile(state: &mut IpcState) {
    let active = state.config.active_profile.clone();
    let live = state.config.audio.clone();
    sync_profile_audio(&mut state.config.profiles, &active, &live);
}

/// Apply a new playback EQ to an audio config.
///
/// Voice mode `custom` is a *view* of the playback EQ, never an independent
/// copy. Previously the custom bands were a one-off snapshot taken when the
/// Custom button was clicked, so every later EQ edit was ignored by the voice
/// chain and the two silently diverged. Re-deriving here keeps a single source
/// of truth; non-custom modes are left alone.
fn apply_eq_to_audio(audio: &mut AudioConfig, eq: epos_shared::config::EqConfig) {
    audio.eq = eq;
    if audio.voice_enhancer.mode == epos_shared::config::VoiceMode::Custom {
        audio.voice_enhancer.custom_bands = Some(audio.eq.bands.clone());
    }
}

pub struct IpcState {
    pub config: Config,
    pub audio: AudioPipeline,
    pub led: Option<LedController>,
    /// Most recent volume-dial position (0-100, tracked from detents: device
    /// reports incremental up/down only — no absolute readback exists).
    /// Initialized to 100 = device power-on default (full volume).
    pub volume: std::sync::atomic::AtomicI32,
    /// Last volume value the daemon *commanded* onto the sink (mirrors
    /// `volume`). The volume watcher compares the real sink level against this
    /// so it can tell its own writes apart from external changes (keyboard, DE
    /// controls, wpctl, pavucontrol, apps) and avoid clobbering them.
    pub last_volume_target: std::sync::atomic::AtomicI32,
    /// Firmware/board identity probed once from the read-only memory bus
    /// at startup (firmware version string + chip ID).
    pub hw_info: HwInfo,
    /// Last detected device, refreshed by the 5s hotplug loop. Cached so
    /// GetStatus/GetDevice don't re-spawn pw-dump on every GUI poll (3s).
    /// Use `devices::detect()` yourself if you need a genuinely fresh scan.
    pub device: Option<epos_shared::DeviceInfo>,
    /// Cached PipeWire node names (sink, source) for the EPOS card. Refreshed
    /// by the hotplug loop only on (re)connect; lets the loop's `detect()`
    /// skip the blocking `pw-dump` subprocess on every 5s poll.
    pub pipewire_nodes: Option<(String, String)>,
    /// Which sink the tracked `volume` currently refers to.
    ///
    /// The volume target follows whichever sink is actually carrying audio, so
    /// it changes when playback moves — the user picking the speakers, or the EQ
    /// anchor being swapped for raw output. Remembering the previous target lets
    /// a target change be reported as exactly that, instead of being mistaken for
    /// somebody adjusting the volume externally.
    pub last_volume_sink: std::sync::Mutex<String>,
    /// Raw bytes of the config file as last written by the daemon itself.
    /// `config_watch_loop` compares the on-disk bytes against this so it can
    /// tell the daemon's own atomic `save()` apart from a genuine external
    /// edit — without this, a hot-reload would revert in-memory state that
    /// has advanced past the on-disk snapshot (e.g. two rapid profile
    /// switches inside the 2s poll window, or a knob turn queued behind a
    /// save), silently discarding the newest change.
    pub last_written: std::sync::Mutex<Option<Vec<u8>>>,
    /// Shared wake-up for the debounced PipeWire-reload worker. Mutation arms
    /// that change on-disk audio config (EQ / noise gate / voice / profile)
    /// notify it so the actual `systemctl restart pipewire` runs *outside* the
    /// IPC lock (see `pipewire_reload_worker` in main.rs).
    /// Shared wake-up for the debounced volume-save worker. The knob handler and
    /// the external-volume watcher both notify it instead of each spawning their
    /// own 2s-debounced save task — a fast knob drag or a held media key would
    /// otherwise queue dozens of concurrent `save_config` writes (see
    /// `volume_save_worker` in main.rs, audit F5).
    pub volume_save_notify: Arc<Notify>,
    /// Monotonic press counter for the physical smart button, bumped by every
    /// smart-button dispatch arm in main.rs (5 arms: ToggleMode/ToggleEq/
    /// CyclePreset/ToggleSidetone/ToggleNoiseGate). Exposed via
    /// `Status::smart_button_seq` so the GUI can tell a *smart-button* change
    /// (paper trail: seq bump) apart from a GUI-initiated EQ/profile apply
    /// (same config fields change but seq stays put) — the GUI's poll-diff
    /// notifies only on seq movement, so desktop notifications never spam on
    /// the user's own EQ slider drags.
    pub smart_button_seq: std::sync::atomic::AtomicU64,
}

pub async fn run_server(state: Arc<RwLock<IpcState>>) -> Result<()> {
    let runtime_dir = dirs::runtime_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    let socket_path = runtime_dir.join("epos-gsx300d.sock");

    // Remove stale socket
    if socket_path.exists() {
        std::fs::remove_file(&socket_path)?;
    }

    // Ensure parent directory exists
    if let Some(parent) = socket_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let listener = UnixListener::bind(&socket_path)?;
    info!("IPC server listening on {}", socket_path.display());

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                let state = state.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream, state).await {
                        debug!("Client handler ended: {}", e);
                    }
                });
            }
            Err(e) => {
                warn!("Accept error: {}", e);
            }
        }
    }
}

async fn handle_client(stream: UnixStream, state: Arc<RwLock<IpcState>>) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next_line().await? {
        let request: Request = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let resp = Response::Error {
                    message: format!("Invalid request: {}", e),
                };
                send_response(&mut writer, &resp).await?;
                continue;
            }
        };

        let is_mutation = request_is_mutation(&request);

        let response = handle_request(request, state.clone()).await;

        send_response(&mut writer, &response).await?;

        // Persist config only after mutating requests. Read-only requests
        // (the GUI polls GetStatus every 3s) must not churn the file.
        if matches!(response, Response::Ok) && is_mutation {
            let st = state.read().await;
            if let Err(e) = crate::save_config(&st) {
                warn!("Failed to save config: {}", e);
            }
        }
    }

    Ok(())
}

async fn send_response(writer: &mut (impl AsyncWriteExt + Unpin), resp: &Response) -> Result<()> {
    let json = serde_json::to_string(resp)?;
    writer.write_all(json.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    Ok(())
}

async fn handle_request(request: Request, state: Arc<RwLock<IpcState>>) -> Response {
    // Clone the shared reload signal up-front; mutation arms notify it so the
    // actual PipeWire restart runs off the IPC lock.
    match request {
        // --- Status ---
        Request::GetStatus => {
            // Read the cached device without holding the lock across the (blocking)
            // fallback scan. `devices::detect()` spawns a pw-dump subprocess, so
            // running it under the read lock would stall other IPC during the
            // startup race (audit F8).
            let device = {
                let state = state.read().await;
                if state.device.is_some() {
                    state.device.clone()
                } else {
                    drop(state);
                    devices::detect().await
                }
            };
            let state = state.read().await;
            Response::Status {
                daemon_version: env!("CARGO_PKG_VERSION").into(),
                device_connected: device.is_some(),
                eq_active: state.config.audio.eq.enabled,
                eq_active_bands: crate::audio::effective_eq_band_count(&state.config.audio.eq),
                mic_input: state.audio.mic_input_state(),
                eq_in_path: state.audio.eq_in_path_now().await,
                active_profile: state.config.active_profile.clone(),
                mode: state.config.mode,
                smart_button_action: serde_json::to_string(&state.config.smart_button.action)
                    .unwrap_or_else(|_| "\"toggle_mode\"".into())
                    .trim_matches('"')
                    .into(),
                volume: state.volume.load(std::sync::atomic::Ordering::Relaxed),
                sidetone_enabled: state.config.audio.sidetone.enabled,
                noise_gate_enabled: state.config.audio.noise_gate.enabled,
                voice_enhancer_enabled: state.config.audio.voice_enhancer.mode
                    != epos_shared::config::VoiceMode::Off,
                smart_button_seq: state
                    .smart_button_seq
                    .load(std::sync::atomic::Ordering::Relaxed),
            }
        }
        Request::GetDevice => {
            // Device identity comes from the 5s hotplug cache (fresh enough
            // for a GUI poll); the live register snapshot below is re-read
            // on every request so runtime state is never stale.
            // Clone the cached device + firmware identity under a short read
            // lock, then DROP the lock before the blocking ~80ms HID read so a
            // GetDevice can't stall the rest of the IPC surface (the GUI's 3s
            // GetStatus poll shares the same lock).
            let (mut device, hw_info) = {
                let state = state.read().await;
                (state.device.clone(), state.hw_info.clone())
            };
            // Merge firmware identity probed read-only from the memory bus at
            // startup (firmware version string + chip ID). Keep raw USB info
            // from the fresh detect.
            if let Some(ref mut d) = device {
                d.firmware_version = hw_info.firmware_version.clone();
                d.chip_id = hw_info.chip_id;
                // Live read-only snapshot of the runtime state registers
                // (mode state, LED shift pair, EQ indices, encoder positions).
                // Best-effort: a timeout leaves fields None, never fails the
                // whole response. Pure read — bit6 (EEPROM write) never set.
                // Runs WITHOUT the IPC lock held.
                if let Some(path) = d.hidraw.clone() {
                    let snap = hwinfo::snapshot(&path);
                    d.hw_snapshot = if snap.is_empty() { None } else { Some(snap) };
                }
            }
            Response::Device(device)
        }

        // --- EQ ---
        Request::GetEq => {
            let state = state.read().await;
            Response::Eq(state.config.audio.clone())
        }
        Request::SetEq { eq } => {
            let mut state = state.write().await;
            // Voice "custom" is re-derived from the new EQ by apply_eq_to_audio.
            apply_eq_to_audio(&mut state.config.audio, eq.eq);
            sync_active_profile(&mut state);
            let audio_cfg = state.config.audio.clone();
            state.audio.update_config(&audio_cfg);
            // Toggling EQ must also move the default sink, otherwise new
            // streams keep attaching to the raw hardware sink and the change
            // is invisible. Failures here are reported but do not fail the
            // request: the filter graph itself was still written.
            if let Err(e) = state.audio.route_output().await {
                warn!("Failed to route output after EQ change: {}", e);
            }
            match state.audio.apply_eq().await {
                Ok(changed) => {
                    // DSP confs are seeded per-role (pipewire-epos@eq) by
                    // write_instance_conf, which restarts only that instance.
                    // MAIN is never touched → Discord keeps both sink (epos-eq-input
                    // fail-closed anchor) and mic links while EQ is applied.
                    if changed {
                        debug!("EQ conf changed — instance-only restart (main untouched)");
                    }
                }
                Err(e) => warn!("Failed to apply EQ: {}", e),
            }
            Response::Ok
        }

        // --- Sidetone ---
        Request::SetSidetone { enabled, level } => {
            let mut state = state.write().await;
            state.config.audio.sidetone.enabled = enabled;
            state.config.audio.sidetone.level = level;
            sync_active_profile(&mut state);
            let audio_cfg = state.config.audio.clone();
            state.audio.update_config(&audio_cfg);
            if let Err(e) = state.audio.apply_sidetone().await {
                warn!("Failed to apply sidetone: {}", e);
            }
            Response::Ok
        }

        // --- Noise Gate ---
        Request::SetNoiseGate {
            enabled,
            threshold_db,
        } => {
            let mut state = state.write().await;
            state.config.audio.noise_gate.enabled = enabled;
            state.config.audio.noise_gate.threshold_db = threshold_db;
            sync_active_profile(&mut state);
            let audio_cfg = state.config.audio.clone();
            state.audio.update_config(&audio_cfg);
            match state.audio.apply_noise_gate().await {
                Ok(_changed) => {
                    // apply_noise_gate → write_voice_conf seeds the per-role
                    // voice instance conf + restarts that INSTANCE (fail-closed).
                    // Do NOT restart main pipewire → Discord sink/source links
                    // stay pinned.
                }
                Err(e) => warn!("Failed to apply noise gate: {}", e),
            }
            Response::Ok
        }

        // --- Voice Enhancer ---
        Request::SetVoiceEnhancer { mode, custom_bands: _ } => {
            let mut state = state.write().await;
            use epos_shared::config::VoiceMode;
            // Reject an unknown mode instead of silently falling back to Off:
            // a casing typo ("Warm") or a future name would otherwise disable
            // the enhancer while still answering Ok, which reads as "the
            // button does nothing".
            let Some(voice_mode) = VoiceMode::from_wire(&mode) else {
                return Response::Error {
                    message: format!(
                        "Unknown voice enhancer mode '{}' (expected one of: {})",
                        mode,
                        VoiceMode::WIRE_NAMES.join(", ")
                    ),
                };
            };
            state.config.audio.voice_enhancer.mode = voice_mode;
            // "Custom" is a VIEW of the playback EQ, not an independent copy.
            // Ignore whatever the client sent and always derive it from the
            // current EQ bands, so there is exactly one source of truth and the
            // value can never drift. SetEq keeps it in sync while Custom is
            // selected.
            state.config.audio.voice_enhancer.custom_bands = if voice_mode
                == epos_shared::config::VoiceMode::Custom
            {
                Some(state.config.audio.eq.bands.clone())
            } else {
                // Non-custom modes carry no bands; storing stale ones only
                // invited confusion when switching back.
                None
            };
            sync_active_profile(&mut state);
            let audio_cfg = state.config.audio.clone();
            state.audio.update_config(&audio_cfg);
            match state.audio.apply_voice_enhancer().await {
                Ok(_changed) => {
                    // DSP confs are seeded per-role (pipewire-epos@voice) by
                    // write_instance_conf, which restarts only that instance.
                    // MAIN is never touched, so application mic links stay
                    // pinned across the instance restart.
                }
                Err(e) => {
                    return Response::Error {
                        message: format!("Failed to apply voice enhancer: {}", e),
                    }
                }
            }
            Response::Ok
        }

        // --- Mic ---
        Request::SetMicGain { gain } => {
            let mut state = state.write().await;
            // mic_gain is a UI percentage. The IPC boundary is public (HTTP
            // bridge on 127.0.0.1 + the Unix socket), so clamp here instead of
            // trusting the caller: an out-of-range value was previously passed
            // straight to `amixer` as e.g. "5000%".
            let gain = gain.min(100);
            state.config.audio.mic_gain = gain;
            sync_active_profile(&mut state);
            // Sync into the pipeline's config copy — apply_mic_gain reads
            // self.config.mic_gain, and without this the handler applied the
            // previous value instead of the requested one.
            let audio_cfg = state.config.audio.clone();
            state.audio.update_config(&audio_cfg);
            if let Err(e) = state.audio.apply_mic_gain().await {
                return Response::Error {
                    message: format!("Failed to apply mic gain: {}", e),
                };
            }
            Response::Ok
        }

        // --- Audio Mode / LED ---
        Request::GetMode => {
            let state = state.read().await;
            Response::Mode(state.config.mode)
        }
        Request::SetMode { mode } => {
            let mut state = state.write().await;
            state.config.mode = mode;
            // Update LED color
            if let Some(ref mut led) = state.led {
                if let Err(e) = led.set_mode(mode) {
                    warn!("Failed to set LED mode: {}", e);
                }
            }
            info!(
                "Audio mode changed to {} (LED: {})",
                mode.display_name(),
                match mode {
                    AudioMode::Stereo => "blue",
                    AudioMode::Surround71 => "red",
                }
            );
            Response::Ok
        }
        Request::ToggleMode => {
            let mut state = state.write().await;
            let new_mode = match state.config.mode {
                AudioMode::Stereo => AudioMode::Surround71,
                AudioMode::Surround71 => AudioMode::Stereo,
            };
            state.config.mode = new_mode;
            // Update LED color
            if let Some(ref mut led) = state.led {
                if let Err(e) = led.set_mode(new_mode) {
                    warn!("Failed to toggle LED mode: {}", e);
                }
            }
            info!(
                "Audio mode toggled to {} (LED: {})",
                new_mode.display_name(),
                match new_mode {
                    AudioMode::Stereo => "blue",
                    AudioMode::Surround71 => "red",
                }
            );
            Response::Mode(new_mode)
        }

        // --- Profiles ---
        Request::GetProfiles => {
            let state = state.read().await;
            Response::Profiles(state.config.profiles.clone())
        }
        Request::SetActiveProfile { name } => {
            let mut state = state.write().await;
            if let Some(profile) = state.config.profiles.iter().find(|p| p.name == name) {
                let profile_audio = profile.audio.clone();
                let profile_mode = profile.mode;
                state.config.audio = profile_audio;
                state.config.active_profile = name;
                // A profile also carries the audio mode (7.1 for MOVIE/MUSIC,
                // stereo for FLAT/ESPORT). The smart-button path already
                // applies it; the GUI path did not, so switching profile from
                // the GUI left the previous mode (and its LED) in place.
                state.config.mode = profile_mode;
                if let Some(ref mut led) = state.led {
                    if let Err(e) = led.set_mode(profile_mode) {
                        warn!("Failed to set LED mode on profile switch: {}", e);
                    }
                }
                // Sync the selected profile into the pipeline's own config copy
                // BEFORE applying — apply_full() reads self.config, so without
                // this the OLD pipeline config would be applied and the switch
                // would be silently ignored.
                let audio_cfg = state.config.audio.clone();
                state.audio.update_config(&audio_cfg);
                match state.audio.apply_full().await {
                    // apply_full() already enqueues any required instance restart on the
                    // RestartBus; `changed` only reports whether a conf actually differed.
                    Ok(true) => debug!("audio conf changed - instance restart enqueued"),
                    Ok(false) => debug!("audio conf unchanged - no instance restart"),
                    Err(e) => warn!("Failed to apply profile: {}", e),
                }
                Response::Ok
            } else {
                Response::Error {
                    message: format!("Profile '{}' not found", name),
                }
            }
        }
        Request::CreateProfile { name, audio } => {
            let mut state = state.write().await;
            let now = chrono_now();
            state.config.profiles.push(epos_shared::Profile {
                name: name.clone(),
                mode: epos_shared::AudioMode::Stereo,
                audio,
                created_at: now,
            });
            info!("Created profile '{}'", name);
            Response::Ok
        }
        Request::DeleteProfile { name } => {
            let mut state = state.write().await;
            let before = state.config.profiles.len();
            state.config.profiles.retain(|p| p.name != name);
            if state.config.profiles.len() < before {
                // If we deleted the active profile, fall back to Flat (or first remaining).
                if state.config.active_profile == name {
                    // Match the flat profile case-insensitively: Config::default()
                    // and Profile::flat() use "FLAT", the checked-in
                    // config/default.json uses "Flat", and a user-created profile
                    // can use anything. An exact "Flat" match silently missed the
                    // real default and fell through to an arbitrary profile.
                    let fallback = state
                        .config
                        .profiles
                        .iter()
                        .find(|p| p.name.eq_ignore_ascii_case(FLAT_PROFILE_NAME))
                        .or_else(|| state.config.profiles.first())
                        .cloned();
                    if let Some(profile) = fallback {
                        state.config.audio = profile.audio.clone();
                        state.config.active_profile = profile.name.clone();
                        let audio_cfg = state.config.audio.clone();
                        state.audio.update_config(&audio_cfg);
                        match state.audio.apply_full().await {
                            // apply_full() already enqueues any required instance restart on the
                            // RestartBus; `changed` only reports whether a conf actually differed.
                            Ok(true) => debug!("audio conf changed - instance restart enqueued"),
                            Ok(false) => debug!("audio conf unchanged - no instance restart"),
                            Err(e) => warn!("Failed to apply fallback profile: {}", e),
                        }
                        info!(
                            "Deleted active profile '{}' → fallback to '{}'",
                            name, profile.name
                        );
                    } else {
                        // No profiles left: reset to defaults.
                        state.config.audio = AudioConfig::default();
                        state.config.active_profile =
                            epos_shared::config::FLAT_PROFILE_NAME.to_string();
                        let audio_cfg = state.config.audio.clone();
                        state.audio.update_config(&audio_cfg);
                        match state.audio.apply_full().await {
                            // apply_full() already enqueues any required instance restart on the
                            // RestartBus; `changed` only reports whether a conf actually differed.
                            Ok(true) => debug!("audio conf changed - instance restart enqueued"),
                            Ok(false) => debug!("audio conf unchanged - no instance restart"),
                            Err(e) => warn!("Failed to apply default audio: {}", e),
                        }
                        info!("Deleted last profile '{}' → reset to defaults", name);
                    }
                } else {
                    info!("Deleted profile '{}'", name);
                }
                Response::Ok
            } else {
                Response::Error {
                    message: format!("Profile '{}' not found", name),
                }
            }
        }

        // --- Smart Button ---
        Request::SetSmartButton { action } => {
            let mut state = state.write().await;
            use epos_shared::config::SmartButtonAction;
            let btn_action = match action.as_str() {
                "toggle_mode" => SmartButtonAction::ToggleMode,
                "toggle_eq" => SmartButtonAction::ToggleEq,
                "cycle_preset" => SmartButtonAction::CyclePreset,
                "toggle_sidetone" => SmartButtonAction::ToggleSidetone,
                "toggle_noise_gate" => SmartButtonAction::ToggleNoiseGate,
                _ => SmartButtonAction::ToggleMode,
            };
            info!("Smart button action set to {:?}", btn_action);
            state.config.smart_button.action = btn_action;
            Response::Ok
        }

        // --- Lifecycle ---
        Request::Reload => {
            let mut state = state.write().await;
            match config::load() {
                Ok(new_config) => {
                    state.config = new_config;
                    // Sync the reloaded config into the pipeline's own copy
                    // BEFORE applying. apply_full() reads self.config, so
                    // without this the OLD pipeline config would be applied
                    // and the reload would be silently ignored while GetEq
                    // already reports the new values. Same fix as
                    // SetActiveProfile above.
                    let audio_cfg = state.config.audio.clone();
                    state.audio.update_config(&audio_cfg);
                    match state.audio.apply_full().await {
                        // apply_full() already enqueues any required instance restart on the
                        // RestartBus; `changed` only reports whether a conf actually differed.
                        Ok(true) => debug!("audio conf changed - instance restart enqueued"),
                        Ok(false) => debug!("audio conf unchanged - no instance restart"),
                        Err(e) => warn!("Failed to apply reloaded config: {}", e),
                    }
                    Response::Ok
                }
                Err(e) => Response::Error {
                    message: format!("Reload failed: {}", e),
                },
            }
        }
        Request::Quit => {
            info!("Quit requested via IPC");
            // Kill the sidetone loopback child before exiting — std::process::exit
            // bypasses Drop, so the orphaned pw-loopback would keep running.
            let mut state = state.write().await;
            state.audio.kill_sidetone().await;
            std::process::exit(0);
        }
    }
}

fn chrono_now() -> String {
    // Simple timestamp without pulling in chrono crate
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}", secs)
}

// ─── HTTP bridge (development GUI: vite dev server → daemon) ──
//
// Serves POST /ipc on 127.0.0.1:9898, translating JSON requests to the
// same handler the Unix socket uses. Includes CORS headers so the Vue
// dev server (localhost:5173) can reach it from a browser.

pub async fn run_http_bridge(state: Arc<RwLock<IpcState>>) -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9898").await?;
    info!("HTTP bridge listening on http://127.0.0.1:9898/ipc");

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                let state = state.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_http_client(stream, state).await {
                        debug!("HTTP handler ended: {}", e);
                    }
                });
            }
            Err(e) => {
                warn!("HTTP accept error: {}", e);
            }
        }
    }
}

async fn handle_http_client(mut stream: TcpStream, state: Arc<RwLock<IpcState>>) -> Result<()> {
    // Read request head (until \r\n\r\n) — cap at 8 KiB to avoid abuse.
    let mut head = Vec::new();
    let mut buf = [0u8; 1024];
    let mut content_length: Option<usize> = None;

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        head.extend_from_slice(&buf[..n]);
        if let Some(pos) = find_subslice(&head, b"\r\n\r\n") {
            let header_block = &head[..pos];
            // Parse Content-Length
            for line in header_block.split(|&b| b == b'\n') {
                let line_str = String::from_utf8_lossy(line).trim().to_string();
                if let Some(v) = line_str
                    .to_ascii_lowercase()
                    .strip_prefix("content-length:")
                {
                    content_length = v.trim().parse::<usize>().ok();
                }
            }
            // Now read the body (already partially in head)
            let body = Vec::from(&head[pos + 4..]);
            let mut body = body;
            if let Some(cl) = content_length {
                while body.len() < cl {
                    let n = stream.read(&mut buf).await?;
                    if n == 0 {
                        break;
                    }
                    body.extend_from_slice(&buf[..n]);
                }
                body.truncate(cl);
            }
            return process_http_body(stream, body, state).await;
        }
        if head.len() > 8192 {
            // Malformed / oversized request
            let _ = stream
                .write_all(
                    b"HTTP/1.1 400 Bad Request\r\nConnection: close\r\nContent-Length: 0\r\n\r\n",
                )
                .await;
            return Ok(());
        }
    }

    // Connection closed without full request — nothing to do.
    Ok(())
}

async fn process_http_body(
    mut stream: TcpStream,
    body: Vec<u8>,
    state: Arc<RwLock<IpcState>>,
) -> Result<()> {
    let body_str = String::from_utf8_lossy(&body);

    // CORS preflight (OPTIONS)
    // We can't easily read the method here after body parsing, so handle
    // POST bodies only; preflight sent without body is answered below.
    if body_str.trim().is_empty() {
        let resp = "HTTP/1.1 200 OK\r\n\
                    Access-Control-Allow-Origin: *\r\n\
                    Access-Control-Allow-Methods: POST, OPTIONS\r\n\
                    Access-Control-Allow-Headers: Content-Type\r\n\
                    Access-Control-Max-Age: 86400\r\n\
                    Connection: close\r\n\
                    Content-Length: 0\r\n\r\n";
        stream.write_all(resp.as_bytes()).await?;
        return Ok(());
    }

    let request: Request = match serde_json::from_str(&body_str) {
        Ok(r) => r,
        Err(e) => {
            let resp_body = format!(
                "{{\"type\":\"Error\",\"payload\":{{\"message\":\"Invalid request: {}\"}}}}",
                e
            );
            let resp = format!(
                "HTTP/1.1 400 Bad Request\r\n\
                 Access-Control-Allow-Origin: *\r\n\
                 Content-Type: application/json\r\n\
                 Connection: close\r\n\
                 Content-Length: {}\r\n\r\n{}",
                resp_body.len(),
                resp_body
            );
            stream.write_all(resp.as_bytes()).await?;
            return Ok(());
        }
    };

    let is_mutation = request_is_mutation(&request);

    let response = handle_request(request, state.clone()).await;

    // Persist config after mutations (same rule as Unix socket)
    if matches!(response, Response::Ok) && is_mutation {
        let st = state.read().await;
        if let Err(e) = crate::save_config(&st) {
            warn!("Failed to save config (http): {}", e);
        }
    }

    let json = serde_json::to_string(&response)?;
    let resp = format!(
        "HTTP/1.1 200 OK\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Content-Type: application/json\r\n\
         Connection: close\r\n\
         Content-Length: {}\r\n\r\n{}",
        json.len(),
        json
    );
    stream.write_all(resp.as_bytes()).await?;
    Ok(())
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use epos_shared::config::{EqBand, VoiceEnhancerConfig, VoiceMode};

    fn profile(name: &str, mic_gain: u32) -> epos_shared::Profile {
        let mut audio = AudioConfig::default();
        audio.mic_gain = mic_gain;
        epos_shared::Profile {
            name: name.into(),
            mode: epos_shared::AudioMode::Stereo,
            audio,
            created_at: "2026-01-01".into(),
        }
    }

    /// A live edit must land in the profile the user is currently on, so
    /// switching away and back does not silently undo their work.
    #[test]
    fn live_edit_is_written_into_the_active_profile() {
        let mut profiles = vec![profile("FLAT", 80), profile("MUSIC", 50)];
        let mut live = AudioConfig::default();
        live.mic_gain = 37;
        live.voice_enhancer.mode = VoiceMode::Warm;

        sync_profile_audio(&mut profiles, "MUSIC", &live);

        assert_eq!(profiles[1].audio.mic_gain, 37);
        assert_eq!(profiles[1].audio.voice_enhancer.mode, VoiceMode::Warm);
    }

    /// Only the active profile may be touched; the others are presets the user
    /// did not edit.
    #[test]
    fn other_profiles_are_left_untouched() {
        let mut profiles = vec![profile("FLAT", 80), profile("MUSIC", 50)];
        let mut live = AudioConfig::default();
        live.mic_gain = 37;

        sync_profile_audio(&mut profiles, "MUSIC", &live);

        assert_eq!(profiles[0].audio.mic_gain, 80, "FLAT must be unchanged");
    }

    /// An empty/unknown active-profile name must not panic and must not rewrite
    /// an arbitrary profile.
    #[test]
    fn unknown_active_profile_is_a_no_op() {
        let mut profiles = vec![profile("FLAT", 80)];
        let mut live = AudioConfig::default();
        live.mic_gain = 37;

        sync_profile_audio(&mut profiles, "", &live);
        sync_profile_audio(&mut profiles, "DOES-NOT-EXIST", &live);

        assert_eq!(profiles[0].audio.mic_gain, 80);
    }

    /// While Custom is selected, editing the playback EQ must immediately
    /// change the voice bands. This is the behaviour that was missing: the
    /// custom bands used to be a frozen snapshot from the moment the button was
    /// clicked.
    #[test]
    fn editing_eq_while_custom_updates_the_voice_bands() {
        let mut audio = AudioConfig::default();
        audio.voice_enhancer = VoiceEnhancerConfig {
            mode: VoiceMode::Custom,
            custom_bands: None,
        };

        let new_eq = epos_shared::config::EqConfig {
            enabled: true,
            bands: vec![
                EqBand { freq: 1000, gain_db: 6.0, q: 1.0 },
                EqBand { freq: 4000, gain_db: -3.0, q: 1.4 },
            ],
        };
        apply_eq_to_audio(&mut audio, new_eq);

        assert_eq!(
            audio.voice_enhancer.custom_bands,
            Some(audio.eq.bands.clone()),
            "custom bands must track the EQ instead of going stale"
        );
        assert_eq!(audio.voice_enhancer.custom_bands.unwrap()[0].gain_db, 6.0);
    }

    /// A second EQ edit must replace the bands, not accumulate or be ignored.
    #[test]
    fn repeated_eq_edits_keep_custom_in_step() {
        let mut audio = AudioConfig::default();
        audio.voice_enhancer.mode = VoiceMode::Custom;

        for gain in [-1.0f32, 2.0, 4.5] {
            let eq = epos_shared::config::EqConfig {
                enabled: true,
                bands: vec![EqBand { freq: 8000, gain_db: gain, q: 1.0 }],
            };
            apply_eq_to_audio(&mut audio, eq);
            assert_eq!(
                audio.voice_enhancer.custom_bands.as_ref().unwrap()[0].gain_db,
                gain,
                "custom must follow every successive EQ edit"
            );
        }
    }

    /// When the voice mode is NOT custom, editing the EQ must not invent voice
    /// bands — Warm/Clear are fixed presets and Off has none.
    #[test]
    fn non_custom_modes_do_not_get_eq_derived_bands() {
        for mode in [VoiceMode::Off, VoiceMode::Warm, VoiceMode::Clear] {
            let mut audio = AudioConfig::default();
            audio.voice_enhancer = VoiceEnhancerConfig {
                mode,
                custom_bands: None,
            };
            let eq = epos_shared::config::EqConfig {
                enabled: true,
                bands: vec![EqBand { freq: 1000, gain_db: 5.0, q: 1.0 }],
            };
            apply_eq_to_audio(&mut audio, eq);
            assert!(
                audio.voice_enhancer.custom_bands.is_none(),
                "{mode:?} must not accumulate EQ-derived bands"
            );
        }
    }
}
