mod audio;
mod config;
mod devices;
mod hid;
mod hwinfo;
mod ipc;
mod led;

use crate::audio::AudioPipeline;
use crate::hid::{HidEvent, HidHandler};
use crate::ipc::IpcState;
use crate::led::LedController;
use anyhow::Result;
use epos_shared::config::AudioMode;
use epos_shared::config::SmartButtonAction;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Notify, RwLock};
use tracing::{info, warn};

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

    // Probe firmware version + chip ID over the read-only memory bus
    // (best-effort — the daemon runs fine without it). Pure read: report
    // 0x04 with bit6 (EEPROM write) clear; never touches the flash protocol.
    let hw_info = device
        .as_ref()
        .and_then(|d| d.hidraw.as_ref())
        .map(hwinfo::probe)
        .unwrap_or_default();
    if let Some(ref v) = hw_info.firmware_version {
        info!("Firmware version: {}", v);
    }
    if let Some(id) = hw_info.chip_id {
        info!("Chip ID: {:#04x}", id);
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
        volume: std::sync::atomic::AtomicI32::new(
            config.device.volume.unwrap_or(100).clamp(0, 100),
        ),
        last_volume_target: std::sync::atomic::AtomicI32::new(
            config.device.volume.unwrap_or(100).clamp(0, 100),
        ),
        hw_info,
        device: None,
        pipewire_nodes: None,
        last_written: std::sync::Mutex::new(None),
        reload_notify: Arc::new(Notify::new()),
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
                            // The device toggles its own physical state when the
                            // button is pressed, then reports the NEW state via
                            // ModeChanged. Trust the readback absolutely — never
                            // double-toggle, or daemon and hardware drift apart by
                            // one step every press (web shows N while LED shows ~N).
                            let new_mode = match pressed_mode {
                                Some(m) => m,
                                // Long press carries no mode byte → toggle explicitly.
                                None => match st.config.mode {
                                    AudioMode::Stereo => AudioMode::Surround71,
                                    AudioMode::Surround71 => AudioMode::Stereo,
                                },
                            };
                            st.config.mode = new_mode;
                            if let Err(e) = save_config(&st) {
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
                            if let Err(e) = save_config(&st) {
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
                                    match st.audio.apply_full().await {
                                        Ok(changed) => {
                                            if changed {
                                                st.reload_notify.notify_one();
                                            }
                                        }
                                        Err(e) => warn!("Failed to apply profile: {}", e),
                                    }
                                    if let Err(e) = save_config(&st) {
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
                            if let Err(e) = save_config(&st) {
                                warn!("Failed to save config: {}", e);
                            }
                            info!(
                                "Smart button: sidetone {}",
                                if enabled { "ON" } else { "OFF" }
                            );
                        }
                        SmartButtonAction::ToggleNoiseGate => {
                            let enabled = !st.config.audio.noise_gate.enabled;
                            st.config.audio.noise_gate.enabled = enabled;
                            let audio_cfg = st.config.audio.clone();
                            st.audio.update_config(&audio_cfg);
                            if let Err(e) = st.audio.apply_noise_gate().await {
                                warn!("Failed to toggle noise gate: {}", e);
                            }
                            if let Err(e) = save_config(&st) {
                                warn!("Failed to save config: {}", e);
                            }
                            info!(
                                "Smart button: noise gate {}",
                                if enabled { "ON" } else { "OFF" }
                            );
                        }
                    }
                }
                HidEvent::VolumeChanged(dir) => {
                    // Device applies gain locally; daemon tracks detents
                    // host-side because the HID descriptor exposes only
                    // incremental consumer detents (no absolute readback).
                    // Volume is reported via GetStatus for the GUI.
                    // Each detent = 2% (measured on hardware, 2026-09-14).
                    // Resolve the EPOS sink name + current tracked value under a
                    // short read lock, then drop it so the awaiting pactl call
                    // never holds the state lock.
                    let (sink, cur) = {
                        let st = s.read().await;
                        let cur = st.volume.load(std::sync::atomic::Ordering::Relaxed);
                        let sink = st
                            .pipewire_nodes
                            .as_ref()
                            .map(|(snk, _)| snk.clone())
                            .unwrap_or_default();
                        (sink, cur)
                    };
                    let next = (cur + dir * 2).clamp(0, 100);
                    // Record the value we are about to command so the volume
                    // watcher does not misclassify the sink landing as an
                    // *external* change and clobber it.
                    {
                        let st = s.write().await;
                        st.last_volume_target
                            .store(next, std::sync::atomic::Ordering::Relaxed);
                    }
                    // Mirror the dial onto the real PipeWire sink so the displayed
                    // value always matches the actual output level.
                    apply_sink_volume(&sink, next).await;
                    // Update the tracked value only AFTER the sink has been
                    // commanded, so a watcher poll that runs mid-apply sees
                    // "actual == tracked" (both still old) instead of a false
                    // mismatch that would revert the dial.
                    {
                        let st = s.write().await;
                        st.volume.store(next, std::sync::atomic::Ordering::Relaxed);
                    }
                    tracing::info!(
                        "Volume knob: {} → {}%",
                        if dir > 0 { "up" } else { "down" },
                        next
                    );
                    // Persist the dial position host-side after a short debounce.
                    // The device keeps no NVM record of the volume (verified in
                    // firmware RE / NVM-PERSISTENCE-REPORT) and exposes no
                    // absolute readback, so ~/.config is the only source of truth
                    // across daemon restarts.
                    let s = s.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        let mut st = s.write().await;
                        let v = st.volume.load(std::sync::atomic::Ordering::Relaxed);
                        st.config.device.volume = Some(v);
                        if let Err(e) = save_config(&st) {
                            warn!("Failed to save volume to config: {}", e);
                        }
                    });
                }
            }
        }
    });

    // Start device hotplug watcher (background task)
    let state_clone = state.clone();
    let _hotplug_handle = tokio::spawn(async move {
        device_hotplug_loop(state_clone).await;
    });

    // Start config watcher — hot-reload external config edits (background task)
    let state_clone = state.clone();
    let _config_handle = tokio::spawn(async move {
        config_watch_loop(state_clone).await;
    });

    // Start LED heartbeat — re-assert the config mode on the physical LED ring
    // every 2s. The GSX 300's smart button toggles the device's *own* LED state
    // and sends an input readback; if that readback is swallowed (device
    // debounce quirk), the daemon never learns about the hardware change and
    // config (what the web GUI shows) drifts apart from the physical LED.
    // Re-asserting every 2s forces the LED back into agreement with config, so
    // the web GUI and the physical LED can never stay desynced for more than
    // one heartbeat interval.
    let state_clone = state.clone();
    let _led_heartbeat_handle = tokio::spawn(async move {
        led_heartbeat_loop(state_clone).await;
    });

    // Keep the tracked volume synced with the real EPOS sink level so external
    // volume changes (keyboard, DE controls, wpctl, apps) are reflected in the
    // GUI dial and don't get reverted by the next dial turn.
    let state_clone = state.clone();
    let _volume_watch_handle = tokio::spawn(async move {
        volume_watch_loop(state_clone).await;
    });

    // Debounced PipeWire-reload worker: any handler that changed on-disk audio
    // config notifies it; the actual `systemctl restart pipewire` runs here, off
    // the IPC lock, so audio tweaks never block the GUI's GetStatus poll.
    let state_clone = state.clone();
    let _reload_handle = tokio::spawn(async move {
        pipewire_reload_worker(state_clone).await;
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
                // Kill the sidetone loopback child — std::process::exit bypasses
                // Drop, so without this the orphaned pw-loopback keeps mixing mic.
                st.audio.kill_sidetone().await;
                std::process::exit(0);
            }
        });
    }

    // Run Unix socket IPC server and HTTP bridge (web dev GUI) in parallel
    info!("Daemon ready, starting IPC server...");
    let (_, _) = tokio::join!(ipc::run_server(state.clone()), ipc::run_http_bridge(state),);

    Ok(())
}

/// Apply a host-side dial volume (0-100) to the EPOS PipeWire sink so the
/// hardware dial value and the real sink output level always agree.
async fn apply_sink_volume(sink: &str, percent: i32) {
    if sink.is_empty() {
        return;
    }
    let pct = format!("{}%", percent.clamp(0, 100));
    match tokio::process::Command::new("pactl")
        .args(["set-sink-volume", sink, &pct])
        .output()
        .await
    {
        Ok(out) if !out.status.success() => {
            warn!(
                "pactl set-sink-volume {} {}: {}",
                sink,
                pct,
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        Err(e) => warn!("pactl set-sink-volume {} failed: {}", sink, e),
        _ => {}
    }
}

/// Read the current EPOS sink volume (0-100) straight from PipeWire/PulseAudio.
/// Returns None if the sink is gone or the query fails.
async fn read_sink_volume(sink: &str) -> Option<i32> {
    if sink.is_empty() {
        return None;
    }
    let out = tokio::process::Command::new("pactl")
        .args(["get-sink-volume", sink])
        .output()
        .await
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    // "Volume: front-left: 19660 / 30% / -31.37 dB,   front-right: ... / 30% / ..."
    // Take the first "/ N%" token — works for mono and stereo sinks.
    let pct = text
        .split('/')
        .nth(1)?
        .trim()
        .trim_end_matches('%')
        .parse::<i32>()
        .ok()?;
    Some(pct.clamp(0, 100))
}

/// Background task: keep the daemon's tracked volume in sync with the *actual*
/// EPOS PipeWire sink level.
///
/// The hardware dial only emits incremental detents with no absolute readback,
/// so the daemon tracks volume host-side (`volume`). But other actors — keyboard
/// media keys, DE volume controls, pavucontrol, wpctl, applications — can change
/// the sink volume directly without going through the daemon. If we ignored
/// those, `GetStatus.volume` would drift from reality, and the next dial turn
/// (which SETs an absolute % from the stale tracked value) would jump the real
/// volume back to the stale number.
///
/// This loop polls the sink every second and adopts any change it did not make
/// itself, so the dial display always reflects the true output level and the
/// dial stays continuous with external adjustments.
async fn volume_watch_loop(state: Arc<RwLock<IpcState>>) {
    let interval = tokio::time::Duration::from_secs(1);
    loop {
        tokio::time::sleep(interval).await;
        let sink = {
            let st = state.read().await;
            st.pipewire_nodes
                .as_ref()
                .map(|(s, _)| s.clone())
                .unwrap_or_default()
        };
        if sink.is_empty() {
            continue;
        }
        let actual = match read_sink_volume(&sink).await {
            Some(v) => v,
            None => continue,
        };
        let st = state.write().await;
        let tracked = st.volume.load(std::sync::atomic::Ordering::Relaxed);
        if actual == tracked {
            continue;
        }
        let last_target = st
            .last_volume_target
            .load(std::sync::atomic::Ordering::Relaxed);
        if actual == last_target {
            // The sink just landed on a value we commanded ourselves (or is still
            // settling). Align the tracker without treating it as external.
            st.volume
                .store(actual, std::sync::atomic::Ordering::Relaxed);
            continue;
        }
        // Genuine external change — adopt it so the dial + next dial turn stay
        // continuous with reality.
        st.volume
            .store(actual, std::sync::atomic::Ordering::Relaxed);
        st.last_volume_target
            .store(actual, std::sync::atomic::Ordering::Relaxed);
        let s = state.clone();
        let v = actual;
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            let mut st = s.write().await;
            st.config.device.volume = Some(v);
            if let Err(e) = save_config(&st) {
                warn!("Failed to save volume to config: {}", e);
            }
        });
        tracing::info!(
            "Volume externally set to {}% — synced daemon tracker",
            actual
        );
    }
}

async fn device_hotplug_loop(state: Arc<RwLock<IpcState>>) {
    // Seed with current state so the first poll doesn't re-apply config.
    let mut was_connected = devices::detect().await.is_some();

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        // Reuse cached PipeWire node names — Detect without spawning pw-dump.
        // A fresh `pw-dump` runs exactly once per (re)connect: when the device
        // is on the USB bus (cheap sysfs check) and we don't know node names
        // yet. With a cache present, or device absent, zero subprocesses.
        let cached_nodes = state.read().await.pipewire_nodes.clone();
        let device = if cached_nodes.is_none() && devices::usb_present() {
            devices::detect().await
        } else {
            devices::detect_with_nodes(cached_nodes, false).await
        };
        let is_connected = device.is_some();

        if is_connected && !was_connected {
            info!("EPOS GSX 300 connected — applying config");
            let mut st = state.write().await;
            // Names come from the fresh scan above (cache was empty when we
            // entered this branch).
            if let Some(ref d) = device {
                st.pipewire_nodes = Some((d.pipewire_sink.clone(), d.pipewire_source.clone()));
                // Re-probe hardware info: the boot-time probe can return
                // empty if it ran during the udev ACL race; a reconnect is
                // the right moment to fill in version/chip-ID.
                if let Some(ref hid) = d.hidraw {
                    let hw = hwinfo::probe(hid);
                    if hw.firmware_version.is_some() || hw.chip_id.is_some() {
                        info!("Hardware info refreshed after reconnect");
                        st.hw_info = hw;
                    }
                }
                st.audio.set_device(d);
                match st.audio.apply_full().await {
                    Ok(changed) => {
                        if changed {
                            st.reload_notify.notify_one();
                        }
                    }
                    Err(e) => warn!("Failed to apply audio config on connect: {}", e),
                }
                // Restore host-side dial volume onto the real sink so the
                // knob position matches the actual output level after (re)plug.
                let vol = st.volume.load(std::sync::atomic::Ordering::Relaxed);
                let sink = st
                    .pipewire_nodes
                    .as_ref()
                    .map(|(snk, _)| snk.clone())
                    .unwrap_or_default();
                st.last_volume_target
                    .store(vol, std::sync::atomic::Ordering::Relaxed);
                apply_sink_volume(&sink, vol).await;
                info!("Volume restored to {}% on EPOS sink", vol);
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
            st.device = device;
        } else if !is_connected && was_connected {
            info!("EPOS GSX 300 disconnected");
            let mut st = state.write().await;
            st.device = None;
            // Keep pipewire_nodes cached: names are harmless while absent, and
            // usb_present() re-detection refreshes them on the next reconnect.
            // Kill any running sidetone process
            st.audio.set_device(&epos_shared::DeviceInfo::default());
        } else {
            // Steady state (connected or still absent): refresh the detect
            // cache that GetStatus/GetDevice read. Reuses cached node names /
            // zero pw-dump. Fill the node cache if this scan produced names.
            let mut st = state.write().await;
            if is_connected
                && st.pipewire_nodes.is_none()
                && !device.as_ref().is_some_and(|d| d.pipewire_sink.is_empty())
            {
                if let Some(ref d) = device {
                    st.pipewire_nodes = Some((d.pipewire_sink.clone(), d.pipewire_source.clone()));
                    // First connect after boot: push the saved dial volume
                    // onto the sink once so display == actual output.
                    let vol = st.volume.load(std::sync::atomic::Ordering::Relaxed);
                    let sink = d.pipewire_sink.clone();
                    st.last_volume_target
                        .store(vol, std::sync::atomic::Ordering::Relaxed);
                    drop(st);
                    apply_sink_volume(&sink, vol).await;
                    tracing::info!("Volume restored to {}% on EPOS sink (boot)", vol);
                    let mut st = state.write().await;
                    st.device = device;
                    continue;
                }
            }
            st.device = device;
        }

        was_connected = is_connected;
    }
}

/// Background task: watch config file for external edits and hot-apply them.
///
/// Persists the daemon's config to disk and records the exact bytes written so
/// `config_watch_loop` can recognize the daemon's own atomic `save()` and skip
/// it — instead of reloading and reverting newer in-memory state.
fn save_config(st: &IpcState) -> Result<(), anyhow::Error> {
    config::save(&st.config)?;
    if let Ok(bytes) = std::fs::read(config::config_path()) {
        *st.last_written.lock().unwrap() = Some(bytes);
    }
    Ok(())
}

/// Background worker: debounced `systemctl --user restart pipewire`.
///
/// Handlers that change on-disk audio config (EQ / noise gate / voice / profile
/// switches) call `reload_notify.notify_one()` instead of restarting PipeWire
/// inline. This worker coalesces those notifications and performs the restart
/// once, off the IPC lock — so a fast slider drag or a burst of profile
/// switches triggers a single reload, and the GUI's 3s GetStatus poll never
/// stalls waiting on a 1–3s blocking restart.
async fn pipewire_reload_worker(state: Arc<RwLock<IpcState>>) {
    let notify = state.read().await.reload_notify.clone();
    loop {
        notify.notified().await;
        // Coalesce rapid changes (e.g. an EQ slider drag) into a single reload:
        // keep waiting up to 250ms for more notifications before firing.
        while let Ok(()) = tokio::time::timeout(Duration::from_millis(250), notify.notified()).await {}
        info!("PipeWire reload (debounced) triggered by audio config change");
        audio::reload_pipewire().await;
    }
}

/// Polls the config file mtime every 2s. When it changes, reloads and applies:
/// - audio changes  → update_config + apply_full (EQ / voice / noise / sidetone / mic gain)
/// - mode changes   → re-sync LED ring
/// - anything else  (e.g. smart button action) → just update in-memory config
///
/// The daemon's own atomic `save()` writes are recognized by comparing the
/// on-disk bytes against the snapshot recorded at write time, so we only react
/// to edits made *outside* the daemon.
async fn config_watch_loop(state: Arc<RwLock<IpcState>>) {
    let path = config::config_path();
    let mut last_mtime = std::fs::metadata(&path).and_then(|m| m.modified()).ok();

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let mtime = std::fs::metadata(&path).and_then(|m| m.modified()).ok();
        if mtime == last_mtime {
            continue;
        }
        last_mtime = mtime;

        // Read the raw file bytes before parsing so we can recognize the
        // daemon's own atomic write. If they match what we last wrote, this
        // mtime change came from a daemon `save()` — skip it. This prevents a
        // spurious hot-reload from reverting in-memory state that has advanced
        // past the on-disk snapshot (e.g. two rapid profile switches inside the
        // 2s poll window, or a knob turn queued behind a save).
        let disk_bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(_) => continue,
        };
        {
            let s = state.read().await;
            if *s.last_written.lock().unwrap() == Some(disk_bytes.clone()) {
                // This mtime change came from the daemon's own atomic `save()`;
                // skip it so we don't revert in-memory state that has advanced
                // past the on-disk snapshot.
                continue;
            }
        }

        let new_config = match config::load() {
            Ok(c) => c,
            Err(e) => {
                // File may be mid-write or malformed; keep current config.
                warn!("Config reload failed, keeping current config: {}", e);
                continue;
            }
        };

        let mut st = state.write().await;

        let audio_changed = serde_json::to_value(&st.config.audio)
            .ok()
            .zip(serde_json::to_value(&new_config.audio).ok())
            .map(|(a, b)| a != b)
            .unwrap_or(true);
        let mode_changed = st.config.mode != new_config.mode;

        st.config = new_config;

        if audio_changed {
            let audio_cfg = st.config.audio.clone();
            st.audio.update_config(&audio_cfg);
            match st.audio.apply_full().await {
                Ok(changed) => {
                    if changed {
                        st.reload_notify.notify_one();
                    }
                }
                Err(e) => warn!("Failed to apply reloaded audio config: {}", e),
            }
            info!("Config hot-reload: audio settings applied");
        }
        if mode_changed {
            let desired_mode = st.config.mode;
            if let Some(ref mut led) = st.led {
                if let Err(e) = led.set_mode(desired_mode) {
                    warn!("Failed to sync LED after config reload: {}", e);
                }
            }
            info!("Config hot-reload: LED synced to {:?}", desired_mode);
        }
        if !audio_changed && !mode_changed {
            info!("Config hot-reload: non-audio settings updated");
        }
        *st.last_written.lock().unwrap() = Some(disk_bytes);
    }
}

/// Background task: periodically re-assert the physical LED ring color from the
/// current config mode.
///
/// The GSX 300's smart button toggles the device's *own* LED state and then
/// reports the new state as an input readback. If the readback is swallowed by
/// the device's debounce quirk, the daemon never learns the hardware state, so
/// config (which the web GUI renders) and the physical LED can drift apart by
/// one step per missed readback.
///
/// This loop keeps the LED converged on config: every interval it writes the
/// configured mode to the ring, overriding any state the device assumed on its
/// own. The web GUI reads config, so after at most one interval, what the GUI
/// shows == what the LED physically displays.
async fn led_heartbeat_loop(state: Arc<RwLock<IpcState>>) {
    let interval = tokio::time::Duration::from_secs(2);
    loop {
        tokio::time::sleep(interval).await;
        let mut st = state.write().await;
        let desired = st.config.mode;
        if let Some(ref mut led) = st.led {
            if let Err(e) = led.set_mode(desired) {
                warn!("LED heartbeat: failed to re-assert {:?}: {}", desired, e);
            }
        }
    }
}
