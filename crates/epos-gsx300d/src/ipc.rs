use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::RwLock;
use tracing::{info, error, warn};
use epos_shared::Config;
use epos_shared::ipc::{Request, Response};
use crate::audio::AudioPipeline;
use crate::config;
use crate::devices;

use anyhow::Result;

pub struct IpcState {
    pub config: Config,
    pub audio: AudioPipeline,
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

        let response = {
            let mut st = state.write().await;
            handle_request(request, &mut st).await
        };

        send_response(&mut writer, &response).await?;

        // Persist config after mutations
        if matches!(response, Response::Ok) {
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
            }
        }
        Request::GetDevice => {
            let device = devices::detect().await;
            Response::Device(device)
        }

        // --- EQ ---
        Request::GetEq => Response::Eq(state.config.audio.clone()),
        Request::SetEq { eq } => {
            state.config.audio.eq = eq.eq;
            if let Err(e) = state.audio.apply_eq().await {
                warn!("Failed to apply EQ: {}", e);
            }
            Response::Ok
        }

        // --- Sidetone ---
        Request::SetSidetone { enabled, level } => {
            state.config.audio.sidetone.enabled = enabled;
            state.config.audio.sidetone.level = level;
            if let Err(e) = state.audio.apply_sidetone().await {
                warn!("Failed to apply sidetone: {}", e);
            }
            Response::Ok
        }

        // --- Noise Gate ---
        Request::SetNoiseGate { enabled, threshold_db } => {
            state.config.audio.noise_gate.enabled = enabled;
            state.config.audio.noise_gate.threshold_db = threshold_db;
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
            if let Err(e) = state.audio.apply_voice_enhancer().await {
                warn!("Failed to apply voice enhancer: {}", e);
            }
            Response::Ok
        }

        // --- Mic ---
        Request::SetMicGain { gain } => {
            state.config.audio.mic_gain = gain;
            if let Err(e) = state.audio.apply_mic_gain().await {
                warn!("Failed to apply mic gain: {}", e);
            }
            Response::Ok
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
                info!("Deleted profile '{}'", name);
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
                "toggle_eq" => SmartButtonAction::ToggleEq,
                "cycle_preset" => SmartButtonAction::CyclePreset,
                "toggle_sidetone" => SmartButtonAction::ToggleSidetone,
                "toggle_noise_gate" => SmartButtonAction::ToggleNoiseGate,
                _ => SmartButtonAction::CyclePreset,
            };
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
