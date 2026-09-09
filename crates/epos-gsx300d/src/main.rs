mod audio;
mod config;
mod devices;
mod hid;
mod ipc;

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".parse().unwrap()),
        )
        .init();

    info!("epos-gsx300d starting...");

    // Load config
    let config = config::load()?;
    info!("Config loaded from {}", config::config_path().display());

    // Initialize device detection
    let device = devices::detect().await;
    if let Some(ref d) = device {
        info!("Device detected: ALSA card {}", d.alsa_card);
    } else {
        info!("No EPOS GSX 300 detected, waiting...");
    }

    // Initialize audio pipeline
    let mut audio = audio::AudioPipeline::new(&config.audio);
    if let Some(ref d) = device {
        audio.apply(&d).await?;
    }

    // Initialize HID handler
    let hid_handler = hid::HidHandler::new();

    // Start IPC server
    let config_path = config::config_path();
    let mut current_config = config.clone();
    let mut current_device = device;

    ipc::run_server(config_path, &mut current_config, &mut current_device, &mut audio, hid_handler).await?;

    Ok(())
}
