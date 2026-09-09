mod audio;
mod config;
mod devices;
mod hid;
mod ipc;
mod led;

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use crate::audio::AudioPipeline;
use crate::ipc::IpcState;
use crate::led::LedController;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "epos_gsx300d=info,warn".parse().unwrap()),
        )
        .init();

    info!("epos-gsx300d v{} starting...", env!("CARGO_PKG_VERSION"));

    // Load config
    let config = config::load()?;
    info!("Config loaded from {}", config::config_path().display());

    // Detect device
    let device = devices::detect().await;
    if let Some(ref d) = device {
        info!(
            "EPOS GSX 300 detected: bus {} addr {} ALSA card {}",
            d.usb_bus, d.usb_addr, d.alsa_card
        );
    } else {
        info!("No EPOS GSX 300 detected — daemon will wait for hotplug");
    }

    // Initialize audio pipeline
    let mut audio = AudioPipeline::new(&config.audio);
    if let Some(ref d) = device {
        audio.set_device(d);
        if let Err(e) = audio.apply_full().await {
            warn!("Failed to apply initial audio config: {}", e);
        }
    }

    // Initialize HID handler
    let _hid_handler = hid::HidHandler::new();

    // Initialize LED controller
    let led_config = config.led_probe.clone().unwrap_or_default();
    let mut led = match LedController::new(led_config) {
        Ok(c) => {
            info!("LED controller initialized");
            Some(c)
        }
        Err(e) => {
            warn!("LED controller unavailable: {} (LED control disabled)", e);
            None
        }
    };

    // Apply initial LED state from config
    if let Some(ref mut led_ctrl) = led {
        if let Err(e) = led_ctrl.set_mode(config.mode) {
            warn!("Failed to set initial LED mode: {}", e);
        }
    }

    // Create shared state
    let state = Arc::new(RwLock::new(IpcState {
        config: config.clone(),
        audio,
        led,
    }));

    // Start device hotplug watcher (background task)
    let state_clone = state.clone();
    let _hotplug_handle = tokio::spawn(async move {
        device_hotplug_loop(state_clone).await;
    });

    // Start IPC server (blocking — runs forever)
    info!("Daemon ready, starting IPC server...");
    ipc::run_server(state).await?;

    Ok(())
}

/// Background task: periodically check for device connect/disconnect
async fn device_hotplug_loop(state: Arc<RwLock<IpcState>>) {
    let mut was_connected = false;

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        let device = devices::detect().await;
        let is_connected = device.is_some();

        if is_connected && !was_connected {
            info!("EPOS GSX 300 connected — applying config");
            let mut st = state.write().await;
            if let Some(ref d) = device {
                st.audio.set_device(d);
                if let Err(e) = st.audio.apply_full().await {
                    warn!("Failed to apply audio config on connect: {}", e);
                }
            }
        } else if !is_connected && was_connected {
            info!("EPOS GSX 300 disconnected");
            let mut st = state.write().await;
            // Kill any running sidetone process
            st.audio.set_device(&epos_shared::DeviceInfo::default());
        }

        was_connected = is_connected;
    }
}
