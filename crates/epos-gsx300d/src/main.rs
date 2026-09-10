mod audio;
mod config;
mod devices;
mod hid;
mod ipc;
mod led;

use anyhow::Result;
use epos_shared::config::SmartButtonAction;
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

    // Background task: handle smart button presses (mode sync) according to
    // the configured SmartButtonAction (default: toggle stereo ⇄ 7.1).
    let s = state.clone();
    tokio::spawn(async move {
        while let Some(evt) = hid_rx.recv().await {
            match evt {
                HidEvent::ModeChanged(_) | HidEvent::LongPress => {
                    let mut st = s.write().await;
                    // LongPress carries no mode — derive from current config.
                    let pressed_mode = match evt {
                        HidEvent::ModeChanged(m) => Some(m),
                        _ => None,
                    };
                    let action = st.config.smart_button.action.clone();
                    info!("Smart button pressed (action: {:?})", action);

                    match action {
                        SmartButtonAction::ToggleMode => {
                            let new_mode = match pressed_mode {
                                Some(m) if m != st.config.mode => m,
                                // Device said "already in this mode" (debounce quirk)
                                // or it was a long press → toggle explicitly.
                                _ => match st.config.mode {
                                    AudioMode::Stereo => AudioMode::Surround71,
                                    AudioMode::Surround71 => AudioMode::Stereo,
                                },
                            };
                            st.config.mode = new_mode;
                            if let Err(e) = config::save(&st.config) {
                                warn!("Failed to save config: {}", e);
                            }
                            if let Some(ref mut led) = st.led {
                                if let Err(e) = led.set_mode(new_mode) {
                                    warn!("Failed to set LED after smart button: {}", e);
                                }
                            }
                            info!("Smart button: mode → {:?} (LED sync)", new_mode);
                        }
                        SmartButtonAction::ToggleEq => {
                            let enabled = !st.config.audio.eq.enabled;
                            st.config.audio.eq.enabled = enabled;
                            let audio_cfg = st.config.audio.clone();
                            st.audio.update_config(&audio_cfg);
                            if let Err(e) = st.audio.apply_eq().await {
                                warn!("Failed to toggle EQ: {}", e);
                            }
                            if let Err(e) = config::save(&st.config) {
                                warn!("Failed to save config: {}", e);
                            }
                            info!("Smart button: EQ {}", if enabled { "ON" } else { "OFF" });
                        }
                        SmartButtonAction::CyclePreset => {
                            let names: Vec<String> =
                                st.config.profiles.iter().map(|p| p.name.clone()).collect();
                            let next = if names.is_empty() {
                                None
                            } else {
                                let idx = names
                                    .iter()
                                    .position(|n| *n == st.config.active_profile)
                                    .unwrap_or(usize::MAX);
                                let next_idx = (idx + 1) % names.len();
                                Some(names[next_idx].clone())
                            };
                            if let Some(name) = next {
                                if let Some(profile) =
                                    st.config.profiles.iter().find(|p| p.name == name)
                                {
                                    st.config.audio = profile.audio.clone();
                                    st.config.active_profile = name.clone();
                                    let audio_cfg = st.config.audio.clone();
                                    st.audio.update_config(&audio_cfg);
                                    if let Err(e) = st.audio.apply_full().await {
                                        warn!("Failed to apply profile: {}", e);
                                    }
                                    if let Err(e) = config::save(&st.config) {
                                        warn!("Failed to save config: {}", e);
                                    }
                                    info!("Smart button: profile → {}", name);
                                }
                            }
                        }
                        SmartButtonAction::ToggleSidetone => {
                            let enabled = !st.config.audio.sidetone.enabled;
                            st.config.audio.sidetone.enabled = enabled;
                            let audio_cfg = st.config.audio.clone();
                            st.audio.update_config(&audio_cfg);
                            if let Err(e) = st.audio.apply_sidetone().await {
                                warn!("Failed to toggle sidetone: {}", e);
                            }
                            if let Err(e) = config::save(&st.config) {
                                warn!("Failed to save config: {}", e);
                            }
                            info!("Smart button: sidetone {}", if enabled { "ON" } else { "OFF" });
                        }
                        SmartButtonAction::ToggleNoiseGate => {
                            let enabled = !st.config.audio.noise_gate.enabled;
                            st.config.audio.noise_gate.enabled = enabled;
                            let audio_cfg = st.config.audio.clone();
                            st.audio.update_config(&audio_cfg);
                            if let Err(e) = st.audio.apply_noise_gate().await {
                                warn!("Failed to toggle noise gate: {}", e);
                            }
                            if let Err(e) = config::save(&st.config) {
                                warn!("Failed to save config: {}", e);
                            }
                            info!("Smart button: noise gate {}", if enabled { "ON" } else { "OFF" });
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

    // Run Unix socket IPC server and HTTP bridge (web dev GUI) in parallel
    info!("Daemon ready, starting IPC server...");
    let (_, _) = tokio::join!(
        ipc::run_server(state.clone()),
        ipc::run_http_bridge(state),
    );

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
                // Re-open LED hidraw (device may have re-enumerated) and sync mode
                let desired_mode = st.config.mode;
                if let Some(ref mut led) = st.led {
                    if let Err(e) = led.reopen() {
                        warn!("Failed to reopen LED device: {}", e);
                    }
                    if let Err(e) = led.set_mode(desired_mode) {
                        warn!("Failed to sync LED on connect: {}", e);
                    }
                    info!("LED re-synced to {:?} after reconnect", desired_mode);
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
