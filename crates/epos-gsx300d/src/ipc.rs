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
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};
use tokio::sync::Notify;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use anyhow::Result;

/// True when a request mutates daemon state that must be persisted to the
/// config file. Read-only requests (GetStatus/GetEq/GetMode/GetProfiles/
/// GetDevice) return here false, so the GUI's 3s status polling never rewrites
/// the config file to disk.
///
/// `Reload` is deliberately absent. It adopts the file as the source of truth
/// and then hands it to the pipeline, so writing it straight back accomplishes
/// nothing except re-serialising it — which silently drops any field this build
/// does not understand, and can clobber an edit made in the meantime.
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
            | Request::Quit
    )
}

/// Whether this request should be followed by a config save.
///
/// Every mutating request is persisted — including one whose application
/// failed. The handler has already changed `state.config` by then, and the
/// alternative was a daemon whose RAM, its disk file and its own reply all
/// disagreed: the GUI showed a setting that a restart reverted, and a later
/// successful mutation silently resurrected a value the user had just been
/// told had failed. Persisting the intent also means a restart retries it,
/// which is the right outcome for a failure caused by a busy device rather
/// than a rejected value — the values themselves are already clamped.
///
/// `Reload` is the exception. It adopts the file as the source of truth, so
/// writing anything back would overwrite the very file it was asked to read,
/// including a file the user was in the middle of editing.
pub fn should_persist(request: &Request) -> bool {
    !matches!(request, Request::Reload) && request_is_mutation(request)
}

/// Compare two profile names the way the shared config contract defines them.
///
/// `Config::default()` and `Profile::flat()` produce "FLAT", the checked-in
/// `config/default.json` says "Flat", and a user can type anything. Callers that
/// compared exactly would miss the profile they meant — `SetActiveProfile` would
/// refuse a case-only spelling, and `DeleteProfile` would remove the entry while
/// leaving `active_profile` pointing at the name it just deleted.
pub(crate) fn profile_name_matches(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

/// What deleting a profile should do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeleteOutcome {
    /// No profile carries that name.
    NotFound,
    /// Removed a profile that was not active; the active one is untouched.
    KeptActive,
    /// Removed the active one; a fallback must be applied, including its mode.
    SwitchedActive,
    /// This was the only profile, so the delete is refused. See the note on
    /// `DeleteOutcome` below.
    RefuseLast,
}

/// Decide the effect of `DeleteProfile { name }` without touching any state.
///
/// Refusing the last profile is the deliberate choice. The alternative — removing
/// it and pointing `active_profile` at "FLAT" — left a name that matched no
/// entry: the list came back empty, the status still reported "FLAT", and the
/// mismatch survived a restart. An empty list also has no valid active name, and
/// an empty active name makes `sync_profile_audio` return early, so later live
/// edits were never written to any profile. The default config always ships one
/// profile and the GUI has no empty-state row, so "at least one" is the contract
/// that keeps every other invariant true.
fn delete_plan(profiles: &[epos_shared::Profile], active: &str, name: &str) -> DeleteOutcome {
    if !profiles.iter().any(|p| profile_name_matches(&p.name, name)) {
        return DeleteOutcome::NotFound;
    }
    let remaining = profiles
        .iter()
        .filter(|p| !profile_name_matches(&p.name, name))
        .count();
    if remaining == 0 {
        return DeleteOutcome::RefuseLast;
    }
    if profile_name_matches(active, name) {
        DeleteOutcome::SwitchedActive
    } else {
        DeleteOutcome::KeptActive
    }
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

/// Decide what to answer a client once persistence has been attempted.
///
/// Pure, so the two rules that matter can be pinned without a writable config
/// directory: a change that made it to disk is answered with whatever the
/// handler produced, and a change that did not is reported as an error rather
/// than accepted. Before this, a save failure was only logged — the GUI showed
/// the setting as applied and it silently vanished on the next restart.
pub(crate) fn after_persist(response: &Response, saved: Result<(), String>) -> Response {
    match saved {
        Ok(()) => response.clone(),
        Err(detail) => Response::Error {
            message: format!(
                "Applied in memory but could not be saved, so it will be lost \
                 on restart: {detail}"
            ),
        },
    }
}

/// What a pre-existing socket at the IPC path actually means.
///
/// The path alone cannot tell these apart: a live daemon and a daemon that was
/// killed without cleaning up both leave a file there, and the only difference
/// is whether anything accepts a connection on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExistingSocket {
    /// Nothing at the path.
    Absent,
    /// The path exists and nothing accepted a connection: a leftover.
    Stale,
    /// The path exists and something answered: a live daemon owns it.
    Live,
}

/// What to do with the path before binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SocketAction {
    /// Bind directly.
    Bind,
    /// Remove the leftover, then bind.
    Replace,
}

/// Decide whether the IPC socket may be taken over.
///
/// Unlinking whatever is at the path — which is what this did before — is what
/// let a second daemon pull the socket out from under a running one. The first
/// daemon keeps its workers and its claim on the headset, the second takes the
/// socket, and neither reports anything. Refusing to start is the honest answer
/// when the socket is live; removing it is only right when nothing answered.
fn socket_precondition(existing: ExistingSocket, path: &Path) -> Result<SocketAction> {
    match existing {
        ExistingSocket::Absent => Ok(SocketAction::Bind),
        ExistingSocket::Stale => Ok(SocketAction::Replace),
        ExistingSocket::Live => anyhow::bail!(
            "another epos-gsx300d is already listening on {}; \
             refusing to start a second daemon over it",
            path.display()
        ),
    }
}

/// Classify what is at `path` right now.
async fn classify_socket(path: &Path) -> ExistingSocket {
    if !path.exists() {
        return ExistingSocket::Absent;
    }
    // A short timeout, so an unresponsive peer cannot stall startup: a socket
    // that will not answer within a moment is treated as stale, which is the
    // recoverable case, rather than as proof of a live daemon.
    match tokio::time::timeout(
        std::time::Duration::from_millis(500),
        UnixStream::connect(path),
    )
    .await
    {
        Ok(Ok(_stream)) => ExistingSocket::Live,
        // Refused, or the path vanished under us: a leftover, or nothing.
        Ok(Err(_)) | Err(_) => ExistingSocket::Stale,
    }
}

pub async fn run_server(state: Arc<RwLock<IpcState>>) -> Result<()> {
    let runtime_dir = dirs::runtime_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    let socket_path = runtime_dir.join("epos-gsx300d.sock");

    // Decide before removing anything: see `socket_precondition`.
    match socket_precondition(classify_socket(&socket_path).await, &socket_path)? {
        SocketAction::Bind => {}
        SocketAction::Replace => std::fs::remove_file(&socket_path)?,
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

        let should_save = should_persist(&request);

        let response = handle_request(request, state.clone()).await;

        // Persist BEFORE answering. The reply used to go out first and the
        // `?` on the write returned early when a client disconnected mid-reply,
        // so a change the daemon had already applied in RAM and in the pipeline
        // was never written and vanished on restart. Saving first also means a
        // failed save can still be reported to the client that asked.
        //
        // Read-only requests never write: the GUI polls GetStatus every 3s.
        let response = if should_save {
            let st = state.read().await;
            after_persist(&response, crate::save_config(&st).map_err(|e| e.to_string()))
        } else {
            response
        };

        send_response(&mut writer, &response).await?;
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
                Err(e) => {
                    return Response::Error {
                        message: format!("Failed to apply EQ: {}", e),
                    };
                }
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
                return Response::Error {
                    message: format!("Failed to apply sidetone: {}", e),
                };
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
                Err(e) => {
                    return Response::Error {
                        message: format!("Failed to apply noise gate: {}", e),
                    };
                }
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
                    Err(e) => {
                        return Response::Error {
                            message: format!("Failed to apply profile: {}", e),
                        };
                    }
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
            match delete_plan(
                &state.config.profiles,
                &state.config.active_profile,
                &name,
            ) {
                DeleteOutcome::NotFound => Response::Error {
                    message: format!("Profile '{}' not found", name),
                },
                // Refused rather than leaving `active_profile` naming a profile
                // that no longer exists. See `DeleteOutcome`.
                DeleteOutcome::RefuseLast => Response::Error {
                    message: format!(
                        "'{}' is the only profile; create another before deleting it",
                        name
                    ),
                },
                DeleteOutcome::KeptActive => {
                    state.config.profiles.retain(|p| !profile_name_matches(&p.name, &name));
                    info!("Deleted profile '{}'", name);
                    Response::Ok
                }
                DeleteOutcome::SwitchedActive => {
                    state.config.profiles.retain(|p| !profile_name_matches(&p.name, &name));
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
                    // Unreachable while the last profile is refused, but a missing
                    // fallback must be reported rather than stored.
                    let Some(profile) = fallback else {
                        return Response::Error {
                            message: format!("Deleted '{}' but found no fallback profile", name),
                        };
                    };
                    state.config.audio = profile.audio.clone();
                    // A profile carries a mode as well as audio. Only the audio was
                    // applied before, so deleting an active 7.1 profile left `mode`
                    // at Surround71 and the ring red while the daemon reported the
                    // stereo fallback -- and then saved that mismatch. The normal
                    // switch path has always done both.
                    state.config.mode = profile.mode;
                    state.config.active_profile = profile.name.clone();
                    let audio_cfg = state.config.audio.clone();
                    state.audio.update_config(&audio_cfg);
                    if let Some(ref mut led) = state.led {
                        if let Err(e) = led.set_mode(profile.mode) {
                            warn!("Failed to set LED on profile delete: {}", e);
                        }
                    }
                    match state.audio.apply_full().await {
                        // apply_full() already enqueues any required instance restart on the
                        // RestartBus; `changed` only reports whether a conf actually differed.
                        Ok(true) => debug!("audio conf changed - instance restart enqueued"),
                        Ok(false) => debug!("audio conf unchanged - no instance restart"),
                        Err(e) => {
                            return Response::Error {
                                message: format!("Failed to apply fallback profile: {}", e),
                            };
                        }
                    }
                    info!(
                        "Deleted active profile '{}' -> fallback to '{}'",
                        name, profile.name
                    );
                    Response::Ok
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
            // Read-only by design: a missing or malformed file is reported and
            // left exactly as found. The bootstrap loader answers an absent
            // file by writing `Config::default()` over the real path, so a
            // config that was momentarily gone turned a reload into a factory
            // reset that discarded the user's profiles.
            match config::load_existing() {
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
                        Err(e) => {
                            return Response::Error {
                                message: format!("Failed to apply reloaded config: {}", e),
                            };
                        }
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
//
// The bridge speaks the FULL IPC surface — SetEq, SetMicGain,
// DeleteProfile, Reload, Quit — so how permissive it is decides whether
// any page the user visits can drive the headset. Two rules keep the
// dev-server workflow without handing that surface to the open web:
//
//   * only the vite dev origin may call it, checked before the body is
//     even read, and
//   * only `Content-Type: application/json` is dispatched, so every
//     browser call is a preflighted one and JSON smuggled through a
//     `text/plain` "simple request" cannot sidestep the origin check.
//
// A request with no Origin at all is a local tool (curl, a script), not
// a web page. It is allowed: the Unix socket already trusts local
// callers, so this widens nothing.

/// Largest request head we buffer; two lines need far less.
const MAX_HEAD: usize = 8 * 1024;

/// Largest request body we buffer. IPC calls are small config updates,
/// so this sits far above any legitimate one while stopping a hostile
/// Content-Length from growing the buffer without bound.
const MAX_BODY: usize = 64 * 1024;

/// How long a client may take to send its head or body before we drop it.
/// Without this, a connection that opens and says nothing pins a task and
/// a file descriptor for the remaining life of the daemon.
const CLIENT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

/// Origins allowed to call the bridge. Vite is pinned to port 5173 with
/// `strictPort`, and `localhost` and `127.0.0.1` are distinct origins
/// even though both name this machine, so both are listed.
const ALLOWED_ORIGINS: &[&str] = &["http://localhost:5173", "http://127.0.0.1:5173"];

/// The parts of a request head that the bridge's policy depends on.
#[derive(Debug, Clone, PartialEq, Eq)]
struct RequestHead {
    method: String,
    path: String,
    origin: Option<String>,
    content_type: Option<String>,
    content_length: Option<usize>,
}

/// What the bridge should do with a request, before any body is read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeadDecision {
    /// Answer the CORS preflight; do not read a body.
    Preflight,
    /// Read the body and hand the request to `handle_request`.
    Dispatch,
    /// Refuse with this HTTP status. The body is always a JSON `Error`, so
    /// the dev GUI still gets something it can display.
    Reject(u16),
}

/// A web page must name an origin we recognise. `None` means the client sent
/// no `Origin` at all, which a browser never does for a cross-origin POST —
/// that is a local tool, inside the same trust boundary as the Unix socket.
fn origin_allowed(origin: Option<&str>) -> bool {
    match origin {
        None => true,
        Some(origin) => ALLOWED_ORIGINS.contains(&origin),
    }
}

/// Deliberately strict: this is what forces every browser call through a
/// CORS preflight, and a preflight is answered from `ALLOWED_ORIGINS`. If a
/// non-JSON type were accepted, a page could POST JSON as a "simple request"
/// with no preflight at all and simply discard the unreadable response.
fn content_type_is_json(value: Option<&str>) -> bool {
    value
        .and_then(|value| value.split(';').next())
        .map(|media_type| media_type.trim().eq_ignore_ascii_case("application/json"))
        .unwrap_or(false)
}

fn reject_message(status: u16) -> &'static str {
    match status {
        400 => "Malformed request",
        403 => "Origin not allowed",
        404 => "Not found",
        405 => "Method not allowed",
        413 => "Request body too large",
        415 => "Content-Type must be application/json",
        _ => "Bad request",
    }
}

/// The policy, in one pure function: given a parsed head, either answer a
/// preflight, dispatch, or refuse. Order matters — origin first, so a
/// foreign page is turned away before its body is read or its Content-Length
/// trusted.
fn decide_head(head: &RequestHead) -> HeadDecision {
    if !origin_allowed(head.origin.as_deref()) {
        return HeadDecision::Reject(403);
    }
    if head.path != "/ipc" {
        return HeadDecision::Reject(404);
    }
    if head.method == "OPTIONS" {
        return HeadDecision::Preflight;
    }
    if head.method != "POST" {
        return HeadDecision::Reject(405);
    }
    if !content_type_is_json(head.content_type.as_deref()) {
        return HeadDecision::Reject(415);
    }
    match head.content_length {
        Some(length) if length > MAX_BODY => HeadDecision::Reject(413),
        _ => HeadDecision::Dispatch,
    }
}

/// Parse a request head (everything before the blank line). Returns `None`
/// for anything that is not a plausible HTTP request, so junk can never be
/// mistaken for a dispatchable call.
fn parse_request_head(head: &[u8]) -> Option<RequestHead> {
    let text = String::from_utf8_lossy(head);
    let mut lines = text.split("\r\n");
    let mut request_line = lines.next()?.split_whitespace();
    let method = request_line.next()?.to_string();
    let path = request_line.next()?.to_string();

    let mut parsed = RequestHead {
        method,
        path,
        origin: None,
        content_type: None,
        content_length: None,
    };

    for line in lines {
        if line.is_empty() {
            break;
        }
        // `split_once` keeps colons inside the value, unlike `splitn(2, ':')`
        // on a trimmed line would suggest; the name is compared case-folded
        // because header names are case-insensitive.
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        let value = value.trim().to_string();
        match name.trim().to_ascii_lowercase().as_str() {
            "origin" => parsed.origin = Some(value),
            "content-type" => parsed.content_type = Some(value),
            // A length we cannot parse is treated as absent, which for a POST
            // means an empty body and a 400 rather than a silent dispatch.
            "content-length" => parsed.content_length = value.parse().ok(),
            _ => {}
        }
    }

    Some(parsed)
}

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

fn status_line(status: u16) -> &'static str {
    match status {
        200 => "200 OK",
        403 => "403 Forbidden",
        404 => "404 Not Found",
        405 => "405 Method Not Allowed",
        413 => "413 Payload Too Large",
        415 => "415 Unsupported Media Type",
        _ => "400 Bad Request",
    }
}

/// Every response is JSON, including refusals, because the dev GUI already
/// knows how to read `{"type":"Error",...}` and would otherwise show nothing.
/// Building it from the enum rather than by `format!` also means a parse-error
/// message containing a quote can no longer break the document.
fn error_body(message: &str) -> String {
    serde_json::to_string(&Response::Error {
        message: message.to_string(),
    })
    .unwrap_or_else(|_| r#"{"type":"Error","payload":{"message":"Error"}}"#.to_string())
}

/// `Access-Control-Allow-Origin` is echoed back only for an origin the
/// allowlist accepted. The previous `*` let any page on the web read the
/// reply, and since the bridge speaks the full IPC surface, that included
/// DeleteProfile and Quit.
async fn write_http(
    stream: &mut TcpStream,
    status: u16,
    cors_origin: Option<&str>,
    extra_headers: &str,
    body: &str,
) -> Result<()> {
    let mut response = format!("HTTP/1.1 {}\r\n", status_line(status));
    if let Some(origin) = cors_origin {
        response.push_str(&format!("Access-Control-Allow-Origin: {origin}\r\n"));
    }
    response.push_str(extra_headers);
    response.push_str("Content-Type: application/json\r\n");
    response.push_str("Connection: close\r\n");
    response.push_str(&format!("Content-Length: {}\r\n\r\n", body.len()));
    response.push_str(body);
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

async fn handle_http_client(mut stream: TcpStream, state: Arc<RwLock<IpcState>>) -> Result<()> {
    // Read the head, bounded in size and in time. The old loop had neither a
    // body cap nor a deadline, so a client that opened a connection and said
    // nothing pinned this task and its file descriptor for good.
    let mut raw = Vec::new();
    let mut buf = [0u8; 1024];
    let head_end = loop {
        if raw.len() > MAX_HEAD {
            let body = error_body(reject_message(413));
            return write_http(&mut stream, 413, None, "", &body).await;
        }
        let read = tokio::time::timeout(CLIENT_TIMEOUT, stream.read(&mut buf)).await;
        let n = match read {
            Ok(Ok(n)) => n,
            Ok(Err(e)) => return Err(e.into()),
            // Client stalled before finishing its head: nothing to answer.
            Err(_) => return Ok(()),
        };
        if n == 0 {
            // Closed before a complete head.
            return Ok(());
        }
        raw.extend_from_slice(&buf[..n]);
        if let Some(pos) = find_subslice(&raw, b"\r\n\r\n") {
            break pos;
        }
    };

    let Some(head) = parse_request_head(&raw[..head_end]) else {
        let body = error_body(reject_message(400));
        return write_http(&mut stream, 400, None, "", &body).await;
    };

    // A refused request never reaches the daemon, and never gets a CORS
    // header back.
    let cors = head
        .origin
        .as_deref()
        .filter(|origin| origin_allowed(Some(origin)));

    match decide_head(&head) {
        HeadDecision::Preflight => {
            let headers = "Access-Control-Allow-Methods: POST, OPTIONS\r\n\
                            Access-Control-Allow-Headers: Content-Type\r\n\
                            Access-Control-Max-Age: 86400\r\n";
            write_http(&mut stream, 200, cors, headers, "").await
        }
        HeadDecision::Reject(status) => {
            let body = error_body(reject_message(status));
            write_http(&mut stream, status, cors, "", &body).await
        }
        HeadDecision::Dispatch => {
            // The policy has already rejected an over-long declared length,
            // so this clamp is belt and braces rather than the guard.
            let wanted = head.content_length.unwrap_or(0).min(MAX_BODY);
            let mut body = raw[head_end + 4..].to_vec();
            while body.len() < wanted {
                let read = tokio::time::timeout(CLIENT_TIMEOUT, stream.read(&mut buf)).await;
                let n = match read {
                    Ok(Ok(n)) => n,
                    Ok(Err(e)) => return Err(e.into()),
                    Err(_) => break,
                };
                if n == 0 {
                    break;
                }
                body.extend_from_slice(&buf[..n]);
            }
            body.truncate(wanted);
            process_http_body(stream, body, cors.map(str::to_string), state).await
        }
    }
}

async fn process_http_body(
    mut stream: TcpStream,
    body: Vec<u8>,
    cors_origin: Option<String>,
    state: Arc<RwLock<IpcState>>,
) -> Result<()> {
    let body_str = String::from_utf8_lossy(&body);

    // An empty body is no longer mistaken for a preflight. OPTIONS is
    // answered from the parsed head, before we ever reach this function.
    let request: Request = match serde_json::from_str(&body_str) {
        Ok(r) => r,
        Err(e) => {
            let body = error_body(&format!("Invalid request: {e}"));
            return write_http(&mut stream, 400, cors_origin.as_deref(), "", &body).await;
        }
    };

    let should_save = should_persist(&request);

    let response = handle_request(request, state.clone()).await;

    // Persist before answering, through the same rule and the same helper as the
    // Unix socket, so the two paths cannot drift apart on what a client is told.
    let response = if should_save {
        let st = state.read().await;
        after_persist(&response, crate::save_config(&st).map_err(|e| e.to_string()))
    } else {
        response
    };

    let json = serde_json::to_string(&response)?;
    write_http(&mut stream, 200, cors_origin.as_deref(), "", &json).await
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

    // ─── HTTP bridge request policy ───────────────────────────
    //
    // The bridge is a development affordance for the vite dev server, but it
    // speaks the full IPC surface — SetEq, DeleteProfile, Quit. These tests
    // pin the policy that decides which requests may reach `handle_request`.

    fn head(
        method: &str,
        origin: Option<&str>,
        content_type: Option<&str>,
        content_length: Option<usize>,
    ) -> RequestHead {
        RequestHead {
            method: method.into(),
            path: "/ipc".into(),
            origin: origin.map(str::to_string),
            content_type: content_type.map(str::to_string),
            content_length,
        }
    }

    /// The bug this whole policy exists for: a page on any other origin must
    /// never reach the daemon, so it can neither read the status nor change
    /// a setting nor stop the daemon.
    #[test]
    fn a_foreign_website_is_refused() {
        for origin in [
            "https://evil.example",
            "http://localhost:5174",
            "http://127.0.0.1",
            "http://localhost:5173.evil.example",
            "null",
        ] {
            let h = head("POST", Some(origin), Some("application/json"), Some(2));
            assert_eq!(
                decide_head(&h),
                HeadDecision::Reject(403),
                "{origin} must be refused"
            );
        }
    }

    /// The workflow that must keep working: the vite dev server, in either
    /// loopback spelling.
    #[test]
    fn the_dev_server_is_allowed_from_either_loopback_spelling() {
        for origin in ALLOWED_ORIGINS {
            let h = head("POST", Some(origin), Some("application/json"), Some(2));
            assert_eq!(
                decide_head(&h),
                HeadDecision::Dispatch,
                "{origin} must keep working"
            );
        }
    }

    /// No Origin means a local tool, not a web page. The Unix socket already
    /// trusts local callers, so refusing these would only break curl and
    /// scripts without closing a real hole.
    #[test]
    fn a_local_tool_without_an_origin_header_is_allowed() {
        let h = head("POST", None, Some("application/json"), Some(2));
        assert_eq!(decide_head(&h), HeadDecision::Dispatch);
    }

    /// A browser "simple request" carries no preflight, so accepting a
    /// non-JSON content type would let any page POST JSON to the daemon with
    /// the response thrown away. Refusing the content type closes the whole
    /// class, whatever the origin says.
    #[test]
    fn a_non_json_post_is_refused_so_simple_requests_cannot_reach_the_daemon() {
        for content_type in [
            "text/plain",
            "application/x-www-form-urlencoded",
            "multipart/form-data",
            "",
        ] {
            let h = head("POST", None, Some(content_type), Some(2));
            assert_eq!(
                decide_head(&h),
                HeadDecision::Reject(415),
                "{content_type} must not be dispatched"
            );
        }
    }

    /// A missing content type is as good as a wrong one: without this the
    /// refusal above could be walked around by simply omitting the header.
    #[test]
    fn a_post_without_a_content_type_is_refused() {
        assert_eq!(
            decide_head(&head("POST", None, None, Some(2))),
            HeadDecision::Reject(415)
        );
    }

    /// Browsers append a charset, and the dev GUI's fetch may set one.
    #[test]
    fn content_type_parameters_do_not_change_the_verdict() {
        for content_type in [
            "application/json",
            "application/json; charset=utf-8",
            "APPLICATION/JSON",
            " application/json ; charset=utf-8 ",
        ] {
            let h = head("POST", None, Some(content_type), Some(2));
            assert_eq!(
                decide_head(&h),
                HeadDecision::Dispatch,
                "{content_type} is JSON and must be accepted"
            );
        }
    }

    /// Content-Length was previously honoured with no cap, so one header
    /// could make the buffer grow until the daemon ran out of memory.
    #[test]
    fn an_oversized_body_is_refused_before_it_is_read() {
        let h = head("POST", None, Some("application/json"), Some(MAX_BODY + 1));
        assert_eq!(decide_head(&h), HeadDecision::Reject(413));

        let absurd = head("POST", None, Some("application/json"), Some(usize::MAX / 2));
        assert_eq!(decide_head(&absurd), HeadDecision::Reject(413));
    }

    /// The cap must sit above every real request, not on top of it.
    #[test]
    fn a_body_at_the_cap_is_still_accepted() {
        let h = head("POST", None, Some("application/json"), Some(MAX_BODY));
        assert_eq!(decide_head(&h), HeadDecision::Dispatch);

        let none = head("POST", None, Some("application/json"), None);
        assert_eq!(decide_head(&none), HeadDecision::Dispatch);
    }

    /// The path was never checked, so any URL on the port reached the daemon.
    #[test]
    fn only_the_ipc_path_is_served() {
        let mut h = head("POST", None, Some("application/json"), Some(2));
        h.path = "/".into();
        assert_eq!(decide_head(&h), HeadDecision::Reject(404));

        h.path = "/ipc/../quit".into();
        assert_eq!(decide_head(&h), HeadDecision::Reject(404));
    }

    /// The method was never read either, so the "preflight" branch answered
    /// any empty POST. Now only OPTIONS and POST mean anything.
    #[test]
    fn only_post_and_options_reach_the_daemon() {
        for method in ["GET", "PUT", "DELETE", "HEAD", "get"] {
            let h = head(method, None, Some("application/json"), Some(2));
            assert_eq!(
                decide_head(&h),
                HeadDecision::Reject(405),
                "{method} must not be dispatched"
            );
        }
    }

    /// A preflight must be answered as a preflight, not dispatched.
    #[test]
    fn an_options_preflight_is_answered_not_dispatched() {
        let mut origins: Vec<Option<&str>> = vec![None];
        origins.extend(ALLOWED_ORIGINS.iter().copied().map(Some));
        for origin in origins {
            let h = head("OPTIONS", origin, None, None);
            assert_eq!(decide_head(&h), HeadDecision::Preflight);
        }
    }

    /// The origin is checked before the path and before the body, so a
    /// foreign page is turned away without its bytes being read.
    #[test]
    fn a_foreign_origin_is_refused_before_anything_else() {
        let h = head("POST", Some("https://evil.example"), Some("text/plain"), None);
        assert_eq!(decide_head(&h), HeadDecision::Reject(403));

        let mut wrong_path = h.clone();
        wrong_path.path = "/nope".into();
        assert_eq!(decide_head(&wrong_path), HeadDecision::Reject(403));
    }

    /// Every refusal needs a message the dev GUI can show.
    #[test]
    fn every_rejection_has_a_message() {
        for status in [400, 403, 404, 405, 413, 415] {
            assert!(
                !reject_message(status).is_empty(),
                "status {status} needs a message"
            );
        }
    }

    /// Parsing has to survive the shapes a real browser sends: a request
    /// line, mixed-case header names, and a body that arrived in the same
    /// packet as the head.
    #[test]
    fn parse_request_head_reads_the_request_line_and_headers() {
        let raw = b"POST /ipc HTTP/1.1\r\n\
                    Host: localhost:9898\r\n\
                    Origin: http://localhost:5173\r\n\
                    content-type: application/json; charset=utf-8\r\n\
                    Content-Length: 17\r\n\
                    \r\n\
                    {\"type\":\"GetStatus\"}";

        let parsed = parse_request_head(raw).expect("head must parse");
        assert_eq!(parsed.method, "POST");
        assert_eq!(parsed.path, "/ipc");
        assert_eq!(parsed.origin.as_deref(), Some("http://localhost:5173"));
        assert_eq!(
            parsed.content_type.as_deref(),
            Some("application/json; charset=utf-8")
        );
        assert_eq!(parsed.content_length, Some(17));
    }

    /// A value containing a colon must not be truncated at the first one.
    #[test]
    fn parse_request_head_keeps_colons_inside_header_values() {
        let raw = b"OPTIONS /ipc HTTP/1.1\r\nOrigin: http://localhost:5173\r\n\r\n";
        let parsed = parse_request_head(raw).expect("head must parse");
        assert_eq!(parsed.origin.as_deref(), Some("http://localhost:5173"));
    }

    /// Garbage in must not become a dispatch.
    #[test]
    fn an_unparseable_head_yields_nothing() {
        assert!(parse_request_head(b"").is_none());
        assert!(parse_request_head(b"not-a-request-line\r\n\r\n").is_none());
        assert!(parse_request_head(b"GET\r\n\r\n").is_none());
    }

    /// A save that failed must not be reported as a completed change. The value
    /// is live in RAM and in the pipeline, but it is not on disk, so a restart
    /// silently reverts it while the GUI shows it as accepted.
    #[test]
    fn a_failed_save_is_reported_rather_than_swallowed() {
        let answer = after_persist(&Response::Ok, Err("No space left on device".to_string()));
        match answer {
            Response::Error { message } => {
                assert!(
                    message.contains("not be saved") || message.contains("not saved"),
                    "the reason must say it was not saved: {message}"
                );
                assert!(
                    message.contains("No space left"),
                    "the underlying cause must survive: {message}"
                );
            }
            other => panic!("a failed save must not answer Ok, got {other:?}"),
        }
    }

    /// The success path is unchanged, including the payload the client asked
    /// for — a GUI that asked to toggle mode still needs the new mode back.
    #[test]
    fn a_successful_save_answers_unchanged() {
        assert!(matches!(
            after_persist(&Response::Ok, Ok(())),
            Response::Ok
        ));
        assert!(matches!(
            after_persist(&Response::Mode(AudioMode::Surround71), Ok(())),
            Response::Mode(AudioMode::Surround71)
        ));
    }

    // ─── Deleting a profile ──────────────────────────────────
    //
    // The old handler removed the entry, then, if it had been the active one,
    // picked a fallback and applied only its audio. Two things went missing.
    //
    // It never applied the fallback's *mode* or the LED. A profile carries a
    // mode — MOVIE and MUSIC are 7.1, FLAT and ESPORT are stereo — so deleting
    // an active 7.1 profile left the daemon reporting the stereo profile while
    // `mode` stayed `Surround71` and the ring stayed red, and that mismatch was
    // then saved.
    //
    // And with the last profile gone it set `active_profile` to "FLAT" without
    // creating a FLAT profile, so the name pointed at nothing. The list came
    // back empty, the status still said "FLAT", and it survived a restart
    // because start-up does not recreate the missing profile. An empty active
    // name also makes `sync_profile_audio` return early, so later live edits
    // were never stored in any profile.

    /// The dangling case is now refused instead of produced: the default config
    /// always has a profile, and the GUI has no empty-state row, so a profile
    /// list of zero has no valid active name to point at.
    #[test]
    fn deleting_the_only_profile_is_refused() {
        let profiles = vec![profile("FLAT", 80)];
        assert_eq!(
            delete_plan(&profiles, "FLAT", "FLAT"),
            DeleteOutcome::RefuseLast
        );
    }

    #[test]
    fn deleting_an_unknown_profile_is_not_found() {
        let profiles = vec![profile("FLAT", 80), profile("MUSIC", 50)];
        assert_eq!(
            delete_plan(&profiles, "FLAT", "NOPE"),
            DeleteOutcome::NotFound
        );
    }

    #[test]
    fn deleting_a_background_profile_keeps_the_active_one() {
        let profiles = vec![profile("FLAT", 80), profile("MUSIC", 50)];
        assert_eq!(
            delete_plan(&profiles, "FLAT", "MUSIC"),
            DeleteOutcome::KeptActive
        );
    }

    #[test]
    fn deleting_the_active_profile_switches_to_a_fallback() {
        let profiles = vec![profile("MUSIC", 50), profile("FLAT", 80)];
        assert_eq!(
            delete_plan(&profiles, "MUSIC", "MUSIC"),
            DeleteOutcome::SwitchedActive
        );
    }

    /// The shared profile contract says names are case-insensitive, and the
    /// shipped config uses "FLAT" while others in the wild use "Flat". An exact
    /// compare meant `DeleteProfile { name: "flat" }` found nothing at all, and
    /// deleting "FLAT" while `active_profile` said "Flat" removed the profile
    /// without switching away from it.
    #[test]
    fn profile_matching_ignores_case() {
        let profiles = vec![profile("FLAT", 80), profile("MUSIC", 50)];
        // A case-only spelling must still find the profile, and must recognise
        // that it is the active one — the old exact compares did neither.
        assert_eq!(
            delete_plan(&profiles, "flat", "FLAT"),
            DeleteOutcome::SwitchedActive
        );
        assert_eq!(
            delete_plan(&profiles, "MUSIC", "flat"),
            DeleteOutcome::KeptActive,
            "deleting a case-only spelling of a background profile leaves the active one"
        );
        // And the last profile is still recognised through a case difference,
        // rather than slipping past as "not found".
        let only = vec![profile("FLAT", 80)];
        assert_eq!(
            delete_plan(&only, "flat", "FLAT"),
            DeleteOutcome::RefuseLast
        );
    }

    // ─── Taking over the IPC socket path ─────────────────────
    //
    // The old code unlinked whatever was at the path and then bound. Nothing
    // asked whether anything was listening, so starting a second daemon
    // silently pulled the Unix socket out from under the first one: the first
    // kept its workers and its claim on the headset, the second took the
    // socket, and neither said anything. Two daemons then fought over one
    // device.

    /// A live daemon is never displaced. The caller has to be told, because
    /// the alternative — unlinking and binding anyway — is what produced two
    /// daemons sharing the headset.
    #[test]
    fn a_live_socket_is_never_unlinked() {
        let action = socket_precondition(ExistingSocket::Live, Path::new("/run/x.sock"))
            .expect_err("a live socket must stop the second daemon");
        assert!(
            action.to_string().contains("already"),
            "the reason must name the conflict: {action}"
        );
    }

    /// A socket left behind by a daemon that died is safe to remove, or the
    /// daemon could never start again after a crash or a hard reboot.
    #[test]
    fn a_stale_socket_is_removed_and_bound_over() {
        assert_eq!(
            socket_precondition(ExistingSocket::Stale, Path::new("/run/x.sock"))
                .expect("a stale socket is recoverable"),
            SocketAction::Replace
        );
    }

    /// Nothing at the path is the normal first-start case.
    #[test]
    fn an_absent_socket_is_bound_fresh() {
        assert_eq!(
            socket_precondition(ExistingSocket::Absent, Path::new("/run/x.sock"))
                .expect("an absent socket is the normal case"),
            SocketAction::Bind
        );
    }

    // ─── What gets written to disk ───────────────────────────
    //
    // The save gate tested `matches!(response, Response::Ok)`, which quietly
    // excluded `ToggleMode`: that handler answers `Response::Mode` so the GUI
    // can move its toggle without polling. Switching stereo/7.1 therefore
    // changed the LED and the reported mode, and a restart reverted it —
    // while `SetMode`, the dropdown beside the same button, went through a
    // handler answering `Ok` and did persist. Measured on the running daemon:
    // the toggle left the file at `stereo` while the daemon reported
    // `surround71`.

    fn mode_toggle() -> Request {
        Request::ToggleMode
    }

    /// The regression: the toggle must reach disk, and the rule is now keyed on
    /// the request rather than on the shape of its reply, so the two cannot
    /// drift apart again.
    #[test]
    fn the_mode_toggle_is_persisted() {
        assert!(should_persist(&mode_toggle()));
    }

    /// A failure to apply no longer means the edit is dropped from disk. The
    /// handler already changed `state.config`; persisting it keeps RAM, disk
    /// and the reply from disagreeing, and lets a restart retry what a busy
    /// device refused once.
    #[test]
    fn a_mutation_is_persisted_even_when_its_reply_is_an_error() {
        assert!(should_persist(&Request::SetMicGain { gain: 50 }));
    }

    /// The GUI polls GetStatus three times a second; read-only traffic must
    /// never churn the file.
    #[test]
    fn read_only_requests_never_persist() {
        assert!(!should_persist(&Request::GetStatus));
        assert!(!should_persist(&Request::GetMode));
        assert!(!should_persist(&Request::GetProfiles));
    }

    /// `Reload` reads the file as its source of truth. Writing anything back on
    /// any of its paths would overwrite the file the user asked it to read —
    /// including one they were in the middle of editing.
    #[test]
    fn reload_never_writes_back() {
        assert!(!should_persist(&Request::Reload));
    }
}
