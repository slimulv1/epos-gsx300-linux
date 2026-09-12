use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};
use tokio::sync::RwLock;
use tracing::{info, error, warn};
use epos_shared::Config;
use epos_shared::config::AudioMode;
use epos_shared::config::AudioConfig;
use epos_shared::ipc::{Request, Response};
use crate::audio::AudioPipeline;
use crate::config;
use crate::devices;
use crate::led::LedController;
use crate::hwinfo::HwInfo;

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

pub struct IpcState {
    pub config: Config,
    pub audio: AudioPipeline,
    pub led: Option<LedController>,
    /// Most recent volume-dial position (0-100, tracked from detents: device
    /// reports incremental up/down only — no absolute readback exists).
    /// Initialized to 100 = device power-on default (full volume).
    pub volume: std::sync::atomic::AtomicI32,
    /// Firmware/board identity probed once from the read-only memory bus
    /// at startup (firmware version string + chip ID).
    pub hw_info: HwInfo,
}

pub async fn run_server(state: Arc<RwLock<IpcState>>) -> Result<()> {
    let runtime_dir = dirs::runtime_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"));
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
                        error!("Client handler error: {}", e);
                    }
                });
            }
            Err(e) => {
                error!("Accept error: {}", e);
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

        let response = {
            let mut st = state.write().await;
            handle_request(request, &mut st).await
        };

        send_response(&mut writer, &response).await?;

        // Persist config only after mutating requests. Read-only requests
        // (the GUI polls GetStatus every 3s) must not churn the file.
        if matches!(response, Response::Ok) && is_mutation {
            let st = state.read().await;
            if let Err(e) = config::save(&st.config) {
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

async fn handle_request(request: Request, state: &mut IpcState) -> Response {
    match request {
        // --- Status ---
        Request::GetStatus => {
            let device = devices::detect().await;
            Response::Status {
                daemon_version: env!("CARGO_PKG_VERSION").into(),
                device_connected: device.is_some(),
                eq_active: state.config.audio.eq.enabled,
                active_profile: state.config.active_profile.clone(),
                mode: state.config.mode,
                smart_button_action: serde_json::to_string(&state.config.smart_button.action)
                    .unwrap_or_else(|_| "\"toggle_mode\"".into())
                    .trim_matches('"')
                    .into(),
                volume: state.volume.load(std::sync::atomic::Ordering::Relaxed),
            }
        }
        Request::GetDevice => {
            let mut device = devices::detect().await;
            // Merge firmware identity probed read-only from the memory bus at
            // startup (firmware version string + chip ID). Keep raw USB info
            // from the fresh detect.
            if let Some(ref mut d) = device {
                d.firmware_version = state.hw_info.firmware_version.clone();
            }
            Response::Device(device)
        }

        // --- EQ ---
        Request::GetEq => Response::Eq(state.config.audio.clone()),
        Request::SetEq { eq } => {
            state.config.audio.eq = eq.eq;
            state.audio.update_config(&state.config.audio);
            if let Err(e) = state.audio.apply_eq().await {
                warn!("Failed to apply EQ: {}", e);
            }
            Response::Ok
        }

        // --- Sidetone ---
        Request::SetSidetone { enabled, level } => {
            state.config.audio.sidetone.enabled = enabled;
            state.config.audio.sidetone.level = level;
            state.audio.update_config(&state.config.audio);
            if let Err(e) = state.audio.apply_sidetone().await {
                warn!("Failed to apply sidetone: {}", e);
            }
            Response::Ok
        }

        // --- Noise Gate ---
        Request::SetNoiseGate { enabled, threshold_db } => {
            state.config.audio.noise_gate.enabled = enabled;
            state.config.audio.noise_gate.threshold_db = threshold_db;
            state.audio.update_config(&state.config.audio);
            if let Err(e) = state.audio.apply_noise_gate().await {
                warn!("Failed to apply noise gate: {}", e);
            }
            Response::Ok
        }

        // --- Voice Enhancer ---
        Request::SetVoiceEnhancer { mode, custom_bands } => {
            use epos_shared::config::VoiceMode;
            let voice_mode = match mode.as_str() {
                "warm" => VoiceMode::Warm,
                "clear" => VoiceMode::Clear,
                "custom" => VoiceMode::Custom,
                _ => VoiceMode::Off,
            };
            state.config.audio.voice_enhancer.mode = voice_mode;
            state.config.audio.voice_enhancer.custom_bands = custom_bands;
            state.audio.update_config(&state.config.audio);
            if let Err(e) = state.audio.apply_voice_enhancer().await {
                warn!("Failed to apply voice enhancer: {}", e);
            }
            Response::Ok
        }

        // --- Mic ---
        Request::SetMicGain { gain } => {
            state.config.audio.mic_gain = gain;
            // Sync into the pipeline's config copy — apply_mic_gain reads
            // self.config.mic_gain, and without this the handler applied the
            // previous value instead of the requested one.
            let audio_cfg = state.config.audio.clone();
            state.audio.update_config(&audio_cfg);
            if let Err(e) = state.audio.apply_mic_gain().await {
                warn!("Failed to apply mic gain: {}", e);
            }
            Response::Ok
        }

        // --- Audio Mode / LED ---
        Request::GetMode => Response::Mode(state.config.mode),
        Request::SetMode { mode } => {
            state.config.mode = mode;
            if let Err(e) = config::save(&state.config) {
                warn!("Failed to save config: {}", e);
            }
            // Update LED color
            if let Some(ref mut led) = state.led {
                if let Err(e) = led.set_mode(mode) {
                    warn!("Failed to set LED mode: {}", e);
                }
            }
            info!("Audio mode changed to {} (LED: {})", mode.display_name(), match mode {
                AudioMode::Stereo => "blue",
                AudioMode::Surround71 => "red",
            });
            Response::Ok
        }
        Request::ToggleMode => {
            let new_mode = match state.config.mode {
                AudioMode::Stereo => AudioMode::Surround71,
                AudioMode::Surround71 => AudioMode::Stereo,
            };
            state.config.mode = new_mode;
            if let Err(e) = config::save(&state.config) {
                warn!("Failed to save config: {}", e);
            }
            // Update LED color
            if let Some(ref mut led) = state.led {
                if let Err(e) = led.set_mode(new_mode) {
                    warn!("Failed to toggle LED mode: {}", e);
                }
            }
            info!("Audio mode toggled to {} (LED: {})", new_mode.display_name(), match new_mode {
                AudioMode::Stereo => "blue",
                AudioMode::Surround71 => "red",
            });
            Response::Mode(new_mode)
        }

        // --- Profiles ---
        Request::GetProfiles => Response::Profiles(state.config.profiles.clone()),
        Request::SetActiveProfile { name } => {
            if let Some(profile) = state.config.profiles.iter().find(|p| p.name == name) {
                state.config.audio = profile.audio.clone();
                state.config.active_profile = name;
                if let Err(e) = state.audio.apply_full().await {
                    warn!("Failed to apply profile: {}", e);
                }
                Response::Ok
            } else {
                Response::Error {
                    message: format!("Profile '{}' not found", name),
                }
            }
        }
        Request::CreateProfile { name, audio } => {
            let now = chrono_now();
            state.config.profiles.push(epos_shared::Profile {
                name: name.clone(),
                audio,
                created_at: now,
            });
            info!("Created profile '{}'", name);
            Response::Ok
        }
        Request::DeleteProfile { name } => {
            let before = state.config.profiles.len();
            state.config.profiles.retain(|p| p.name != name);
            if state.config.profiles.len() < before {
                // If we deleted the active profile, fall back to Flat (or first remaining).
                if state.config.active_profile == name {
                    let fallback = state
                        .config
                        .profiles
                        .iter()
                        .find(|p| p.name == "Flat")
                        .or_else(|| state.config.profiles.first())
                        .cloned();
                    if let Some(profile) = fallback {
                        state.config.audio = profile.audio.clone();
                        state.config.active_profile = profile.name.clone();
                        let audio_cfg = state.config.audio.clone();
                        state.audio.update_config(&audio_cfg);
                        if let Err(e) = state.audio.apply_full().await {
                            warn!("Failed to apply fallback profile: {}", e);
                        }
                        info!("Deleted active profile '{}' → fallback to '{}'", name, profile.name);
                    } else {
                        // No profiles left: reset to defaults.
                        state.config.audio = AudioConfig::default();
                        state.config.active_profile = String::from("Flat");
                        let audio_cfg = state.config.audio.clone();
                        state.audio.update_config(&audio_cfg);
                        if let Err(e) = state.audio.apply_full().await {
                            warn!("Failed to apply default audio: {}", e);
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
            match config::load() {
                Ok(new_config) => {
                    state.config = new_config;
                    if let Err(e) = state.audio.apply_full().await {
                        warn!("Failed to apply reloaded config: {}", e);
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
                        error!("HTTP handler error: {}", e);
                    }
                });
            }
            Err(e) => {
                error!("HTTP accept error: {}", e);
            }
        }
    }
}

async fn handle_http_client(
    mut stream: TcpStream,
    state: Arc<RwLock<IpcState>>,
) -> Result<()> {
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
            let _ = stream.write_all(
                b"HTTP/1.1 400 Bad Request\r\nConnection: close\r\nContent-Length: 0\r\n\r\n",
            ).await;
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

    let response = {
        let mut st = state.write().await;
        handle_request(request, &mut st).await
    };

    // Persist config after mutations (same rule as Unix socket)
    if matches!(response, Response::Ok) && is_mutation {
        let st = state.read().await;
        if let Err(e) = config::save(&st.config) {
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
    haystack
        .windows(needle.len())
        .position(|w| w == needle)
}
