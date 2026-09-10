mod audio;
mod config;
mod devices;
mod hid;
mod ipc;
mod led;

use anyhow::Result;
use epos_shared::config::AudioMode;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{info, warn};
use crate::audio::AudioPipeline;
use crate::hid::{HidEvent, HidHandler};
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

    // HID event channel: reader thread → async handler task
    let (hid_tx, mut hid_rx) = mpsc::unbounded_channel::<HidEvent>();

    // Initialize HID handler (smart button + volume dial listener)
    let hid_handler = HidHandler::new(hid_tx);
    let _hid_thread = hid_handler.spawn_reader();

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

    // Background task: handle smart button presses (mode sync)
    let s = state.clone();
    tokio::spawn(async move {
        while let Some(evt) = hid_rx.recv().await {
            match evt {
                HidEvent::ModeChanged(mode) => {
                    let mut st = s.write().await;
                    if st.config.mode == mode {
                        continue; // already in this mode — nothing to do
                    }
                    info!("Smart button: mode → {:?} (LED sync)", mode);
                    st.config.mode = mode;
                    if let Err(e) = config::save(&st.config) {
                        warn!("Failed to save config: {}", e);
                    }
                    if let Some(ref mut led) = st.led {
                        if let Err(e) = led.set_mode(mode) {
                            warn!("Failed to set LED after smart button: {}", e);
                        }
                    }
                }
                HidEvent::LongPress => {
                    // Long-press cycles mode (stereo ⇄ 7.1), same as a click.
                    let mut st = s.write().await;
                    let new_mode = match st.config.mode {
                        AudioMode::Stereo => AudioMode::Surround71,
                        AudioMode::Surround71 => AudioMode::Stereo,
                    };
                    info!("Smart button long press: mode → {:?}", new_mode);
                    st.config.mode = new_mode;
                    if let Err(e) = config::save(&st.config) {
                        warn!("Failed to save config: {}", e);
                    }
                    if let Some(ref mut led) = st.led {
                        if let Err(e) = led.set_mode(new_mode) {
                            warn!("Failed to set LED after long press: {}", e);
                        }
                    }
                }
                HidEvent::VolumeChanged(dir) => {
                    // Device applies gain locally; daemon only tracks direction.
                    tracing::debug!(
                        "Volume knob: {}",
                        if dir > 0 { "up" } else { "down" }
                    );
                }
            }
        }
    });

    // Start device hotplug watcher (background task)
    let state_clone = state.clone();
    let _hotplug_handle = tokio::spawn(async move {
        device_hotplug_loop(state_clone).await;
    });

    // Graceful shutdown: SIGTERM/SIGINT → reset LED to blue, then exit.
    // (Drop impls do NOT run on signal kill, so we handle it explicitly.)
    {
        let s = state.clone();
        tokio::spawn(async move {
            let mut signals = match signal_hook::iterator::Signals::new([
                signal_hook::consts::SIGTERM,
                signal_hook::consts::SIGINT,
            ]) {
                Ok(s) => s,
                Err(e) => {
                    warn!("Failed to register signal handler: {}", e);
                    return;
                }
            };
            if let Some(sig) = signals.forever().next() {
                info!("Received signal {} — shutting down", sig);
                let mut st = s.write().await;
                if let Some(ref mut led) = st.led {
                    // Reset to blue (stereo default) so the ring isn't left red
                    // after the daemon stops.
                    let _ = led.set_mode(AudioMode::Stereo);
                }
                std::process::exit(0);
            }
        });
    }

    // Start IPC server (blocking — runs forever)
    info!("Daemon ready, starting IPC server...");
    ipc::run_server(state).await?;

    Ok(())
}

/// Background task: periodically check for device connect/disconnect
async fn device_hotplug_loop(state: Arc<RwLock<IpcState>>) {
    // Seed with current state so the first poll doesn't re-apply config.
    let mut was_connected = devices::detect().await.is_some();

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
