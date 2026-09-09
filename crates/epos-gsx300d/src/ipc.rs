use anyhow::Result;
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tracing::{info, error};
use epos_shared::{Config, DeviceInfo};
use epos_shared::ipc::{Request, Response};
use crate::audio::AudioPipeline;
use crate::hid::HidHandler;

pub async fn run_server(
    _config_path: PathBuf,
    current_config: &mut Config,
    current_device: &mut Option<DeviceInfo>,
    _audio: &mut AudioPipeline,
    _hid: HidHandler,
) -> Result<()> {
    // Determine socket path
    let runtime_dir = dirs::runtime_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    let socket_path = runtime_dir.join("epos-gsx300d.sock");

    // Remove stale socket
    if socket_path.exists() {
        std::fs::remove_file(&socket_path)?;
    }

    let listener = UnixListener::bind(&socket_path)?;
    info!("IPC server listening on {}", socket_path.display());

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                let config = current_config.clone();
                let device = current_device.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream, config, device).await {
                        error!("Client error: {}", e);
                    }
                });
            }
            Err(e) => {
                error!("Accept error: {}", e);
            }
        }
    }
}

async fn handle_client(
    stream: UnixStream,
    config: Config,
    device: Option<DeviceInfo>,
) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next_line().await? {
        let request: Request = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let resp = Response::Error {
                    message: format!("Invalid request: {}", e),
                };
                let json = serde_json::to_string(&resp)?;
                writer.write_all(json.as_bytes()).await?;
                writer.write_all(b"\n").await?;
                continue;
            }
        };

        let response = match request {
            Request::GetStatus => Response::Status {
                daemon_version: env!("CARGO_PKG_VERSION").into(),
                device_connected: device.is_some(),
                eq_active: config.audio.eq.enabled,
                active_profile: config.active_profile.clone(),
            },
            Request::GetDevice => Response::Device(device.clone()),
            Request::GetEq => Response::Eq(config.audio.clone()),
            Request::GetProfiles => Response::Profiles(config.profiles.clone()),
            Request::Quit => {
                info!("Quit requested");
                std::process::exit(0);
            }
            _ => Response::Ok,
        };

        let json = serde_json::to_string(&response)?;
        writer.write_all(json.as_bytes()).await?;
        writer.write_all(b"\n").await?;
    }

    Ok(())
}
