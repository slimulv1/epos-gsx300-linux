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
use tracing::{debug, info, warn};

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
            d.usb_bus,
            d.usb_addr,
            d.alsa_card
                .map(|c| c.to_string())
                .unwrap_or_else(|| "not enumerated yet".to_string())
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

    // Resolve the live audio from the active profile before building the
    // pipeline, the same way `SetActiveProfile` does.
    //
    // The config file carries the audio twice: the top-level `audio` block and
    // a copy inside each profile, and `sync_active_profile` keeps the two in
    // step at runtime. On disk the profile is the one a user edits, so it has
    // to win at startup. Otherwise a band changed in the profile was applied
    // by the config watcher, then silently reverted on the next daemon start
    // from the stale top-level copy — verified before this change.
    let mut config = config;
    if !config.active_profile.is_empty() {
        if let Some(p) = config
            .profiles
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(&config.active_profile))
        {
            if serde_json::to_value(&p.audio).ok() != serde_json::to_value(&config.audio).ok() {
                info!(
                    "Applying audio from active profile '{}' (top-level copy was stale)",
                    p.name
                );
                config.audio = p.audio.clone();
            }
        }
    }

    // Initialize audio pipeline
    let mut audio = AudioPipeline::new(&config.audio);
    if let Some(ref d) = device {
        audio.set_device(d);
        if let Err(e) = audio.apply_full().await {
            warn!("Failed to apply initial audio config: {}", e);
        }
        // Make the running instances a function of the config, deterministically.
        //
        // `write_instance_conf` only requests a restart when the file bytes
        // changed, which cannot detect an instance running an OLDER conf whose
        // file already matches — the state a lost restart leaves behind, and the
        // one the watchdog cannot see either, since the node is published either
        // way. Each instance records the conf it was last verified to have
        // loaded, and anything that does not match what is on disk is restarted
        // now.
        //
        // This replaces an unconditional restart of `eq` and `voice`, which was
        // a guess rather than a check and never covered `sidetone` at all, so a
        // lost sidetone restart stayed lost for good.
        audio.request_stale_instance_restarts();
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
        // Say plainly whether the ring is actually being driven, so the state
        // is never assumed to match the requested mode. EPROTO here means the
        // USB endpoint is wedged, not that the unit lacks LED support — the
        // same report succeeds on the same unit after a physical replug.
        if led_ctrl.write_failing() {
            warn!(
                "LED ring is NOT being driven: HID output writes fail (errno 71 \
                 EPROTO) while the device stays enumerated. Audio is unaffected. \
                 Unplug the GSX 300 for ~10s and plug it back in — the daemon \
                 re-syncs the ring on reconnect. See \
                 docs/reverse-engineering/LED-HID-WEDGED-ENDPOINT.md"
            );
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
        last_volume_sink: std::sync::Mutex::new(String::new()),
        last_written: std::sync::Mutex::new(None),
        volume_save_notify: Arc::new(Notify::new()),
        smart_button_seq: std::sync::atomic::AtomicU64::new(0),
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
                            st.smart_button_seq
                                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
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
                            emit_smart_notify(&st);
                        }
                        SmartButtonAction::ToggleEq => {
                            st.smart_button_seq
                                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
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
                            emit_smart_notify(&st);
                        }
                        SmartButtonAction::CyclePreset => {
                            st.smart_button_seq
                                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            let names: Vec<String> =
                                st.config.profiles.iter().map(|p| p.name.clone()).collect();
                            let next = if names.is_empty() {
                                None
                            } else {
                                // An unknown/missing active_profile must fall
                                // back to the FIRST entry. Using usize::MAX
                                // here and adding 1 overflowed: panic in debug
                                // builds, wrap-to-0 in release.
                                let next_idx = match names
                                    .iter()
                                    .position(|n| *n == st.config.active_profile)
                                {
                                    Some(i) => (i + 1) % names.len(),
                                    None => 0,
                                };
                                Some(names[next_idx].clone())
                            };
                            if let Some(name) = next {
                                if let Some(profile) =
                                    st.config.profiles.iter().find(|p| p.name == name)
                                {
                                    let profile_audio = profile.audio.clone();
                                    let profile_mode = profile.mode;
                                    st.config.audio = profile_audio;
                                    st.config.active_profile = name.clone();
                                    st.config.mode = profile_mode;
                                    if let Some(ref mut led) = st.led {
                                        if let Err(e) = led.set_mode(profile_mode) {
                                            warn!("Failed to set LED after smart button: {}", e);
                                        }
                                    }
                                    let audio_cfg = st.config.audio.clone();
                                    st.audio.update_config(&audio_cfg);
                                    match st.audio.apply_full().await {
                                        // apply_full() already enqueues any required instance restart on the
                                        // RestartBus; `changed` only reports whether a conf actually differed.
                                        Ok(true) => debug!("audio conf changed - instance restart enqueued"),
                                        Ok(false) => debug!("audio conf unchanged - no instance restart"),
                                        Err(e) => warn!("Failed to apply profile: {}", e),
                                    }
                                    if let Err(e) = save_config(&st) {
                                        warn!("Failed to save config: {}", e);
                                    }
                                    info!("Smart button: profile → {}", name);
                                    info!(
                                        "Smart button: profile → {} · LED {} (sync)",
                                        name,
                                        match profile_mode {
                                            AudioMode::Stereo => "blue",
                                            AudioMode::Surround71 => "red",
                                        }
                                    );
                                    emit_smart_notify(&st);
                                }
                            }
                        }
                        SmartButtonAction::ToggleSidetone => {
                            st.smart_button_seq
                                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
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
                            emit_smart_notify(&st);
                        }
                        SmartButtonAction::ToggleNoiseGate => {
                            st.smart_button_seq
                                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
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
                            emit_smart_notify(&st);
                        }
                    }
                }
                HidEvent::VolumeChanged(dir) => {
                    // Device applies gain locally; daemon tracks detents
                    // host-side because the HID descriptor exposes only
                    // incremental consumer detents (no absolute readback).
                    // Volume is reported via GetStatus for the GUI.
                    // Each detent = 2% (measured on hardware, 2026-09-14).
                    // Resolve the tracked value under a short read lock, then drop
                    // it so the awaiting pactl call never holds the state lock.
                    let cur = {
                        let st = s.read().await;
                        st.volume.load(std::sync::atomic::Ordering::Relaxed)
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
                    // The writer resolves the sink itself, outside the state lock:
                    // it costs a `pactl get-default-sink`, and holding the global
                    // lock across a subprocess is how the hotplug poll and the IPC
                    // handlers get stalled by one slow call.
                    apply_volume(&s, next).await;
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
                    // Persist the dial position host-side, debounced. Notify the
                    // shared volume-save worker instead of spawning a per-event
                    // task — a fast knob drag would otherwise queue one save task
                    // per detent (audit F5).
                    s.read().await.volume_save_notify.notify_one();
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

    // Keep the tracked volume synced with the level of the sink that is actually
    // playing, so external volume changes (keyboard, DE controls, wpctl, apps)
    // are reflected in the GUI dial and don't get reverted by the next dial turn.
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

    // Debounced volume-save worker: the knob handler and the external-volume
    // watcher notify it; the actual config write runs here, coalesced, so a burst
    // of dial turns produces one save instead of one-per-detent (audit F5).
    let state_clone = state.clone();
    let _volume_save_handle = tokio::spawn(async move {
        volume_save_worker(state_clone).await;
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
                // Nothing to reap here: the sidetone lives inside the
                // `pipewire-epos@sidetone` systemd instance, not a
                // daemon-owned child, so `kill_sidetone` is intentionally a
                // no-op (see AudioPipeline). The old comment here described a
                // spawned pw-loopback child that no longer exists and
                // contradicted the implementation.
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

/// The sink whose volume the dial and the GUI reflect: whichever one is actually
/// carrying audio right now.
///
/// This used to be the EPOS hardware sink unconditionally, which is correct only
/// while playback goes through the headset. With audio on the speakers the dial
/// still moved the headset, so turning it did nothing audible while the
/// interface displayed a number for a device that was not playing — three
/// different volumes on screen at once and none of them the one being heard.
async fn volume_sink(state: &Arc<RwLock<IpcState>>) -> String {
    let epos = {
        let st = state.read().await;
        st.pipewire_nodes
            .as_ref()
            .map(|(sink, _)| sink.clone())
            .unwrap_or_default()
    };
    let default = audio::AudioPipeline::read_default_sink()
        .await
        .unwrap_or_default();
    audio::volume_target_sink(&default, &epos, audio::EQ_SINK_NAME)
}

/// Apply a host-side dial volume (0-100) to the sink that is actually playing.
///
/// Takes the shared state rather than a sink name **on purpose**. The bug this
/// replaces was a caller passing whichever sink it happened to have in hand,
/// which is correct only while the headset is what the user is listening to: the
/// boot path did exactly that on every daemon start, writing the tracked level
/// onto a device nobody was hearing. Resolving inside the writer means there is
/// no way to reach `pactl set-sink-volume` without going through the same policy
/// the reader uses, so the two cannot drift apart.
async fn apply_volume(state: &Arc<RwLock<IpcState>>, percent: i32) {
    let sink = volume_sink(state).await;
    if sink.is_empty() {
        debug!("Volume write skipped: no sink is currently carrying audio");
        return;
    }
    let pct = format!("{}%", percent.clamp(0, 100));
    match tokio::process::Command::new("pactl")
        .args(["set-sink-volume", &sink, &pct])
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

/// Read a sink's volume (0-100) straight from PipeWire/PulseAudio.
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
        // Follow whichever sink is actually carrying audio. Hardcoding the EPOS
        // sink made the dial move the headset while the speakers were playing,
        // and the reported level belonged to a device nobody could hear.
        let sink = volume_sink(&state).await;
        if sink.is_empty() {
            continue;
        }
        let actual = match read_sink_volume(&sink).await {
            Some(v) => v,
            None => continue,
        };
        let st = state.write().await;

        // A different sink means the tracked number belonged to another device
        // entirely, so this is not somebody adjusting the volume: it is playback
        // moving. Say which, and adopt the new sink's real level.
        let target_changed = {
            let mut last = st.last_volume_sink.lock().unwrap();
            let changed = !last.is_empty() && last.as_str() != sink.as_str();
            *last = sink.clone();
            changed
        };
        if target_changed {
            st.volume
                .store(actual, std::sync::atomic::Ordering::Relaxed);
            st.last_volume_target
                .store(actual, std::sync::atomic::Ordering::Relaxed);
            tracing::info!(
                "Volume now follows {} — showing its level, {}%",
                sink,
                actual
            );
            continue;
        }
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
        // Notify the shared debounced volume-save worker instead of spawning a
        // per-event save task (audit F5).
        st.volume_save_notify.notify_one();
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

        // Reuse cached PipeWire node names — detect without spawning pw-dump.
        // A fresh `pw-dump` runs once per (re)connect, and now also whenever the
        // cached names are no longer published in the main graph.
        //
        // That second case is not defensive extra. On 2026-09-24 the daemon
        // started while the main graph was still coming up, cached a pair of
        // names that were never node names, wrote them into the generated confs,
        // and then never revisited them: this loop only re-applies on a
        // connect/disconnect transition, and it passed the cache back with
        // `needs_fresh = false`, so it never even re-read the graph. The result
        // was an EQ and a voice chain that stayed dead through every restart the
        // watchdog performed.
        let cached_nodes = state.read().await.pipewire_nodes.clone();
        // Checked against the live graph only while the device is on the bus, so
        // an unplugged headset costs a sysfs read and nothing else. A probe that
        // could not run reports "not stale" — it is not evidence about any node,
        // and treating it as evidence would spawn a `pw-dump` on every tick.
        let names_stale = if cached_nodes.is_some() && devices::usb_present() {
            let node_list = audio::AudioPipeline::main_node_list().await;
            audio::cached_nodes_stale(cached_nodes.as_ref(), node_list.as_deref().unwrap_or(""))
        } else {
            false
        };
        let device = if (cached_nodes.is_none() || names_stale) && devices::usb_present() {
            if names_stale {
                info!("Cached EPOS node names are not published — resolving them again");
            }
            devices::detect().await
        } else {
            devices::detect_with_nodes(cached_nodes.clone(), false).await
        };
        let is_connected = device.is_some();

        // Names that moved are a re-enumeration as far as the generated confs are
        // concerned: those still carry the old targets, so they have to be
        // rewritten. Handled by the same path as a reconnect rather than a second
        // one that could drift away from it.
        let names_changed = match (&cached_nodes, &device) {
            (Some(_), Some(d)) => audio::node_names_changed(
                cached_nodes.as_ref(),
                &(d.pipewire_sink.clone(), d.pipewire_source.clone()),
            ),
            _ => false,
        };

        if is_connected && (!was_connected || names_changed) {
            if names_changed {
                info!("EPOS node names changed — regenerating the instance confs");
            } else {
                info!("EPOS GSX 300 connected — applying config");
            }
            // Re-probe hardware info BEFORE taking the write lock: hwinfo::probe
            // does a blocking HID read (~tens of ms) and the lock guards all IPC,
            // so doing it inside would stall GetStatus/GetDevice for the whole
            // probe (audit F4). `device` is owned by this loop, not the lock.
            let hw = device
                .as_ref()
                .and_then(|d| d.hidraw.as_ref())
                .map(hwinfo::probe)
                .unwrap_or_default();
            let vol = state.read().await.volume.load(std::sync::atomic::Ordering::Relaxed);
            let mut st = state.write().await;
            // Names come from the fresh scan above (cache was empty when we
            // entered this branch).
            if let Some(ref d) = device {
                st.pipewire_nodes = Some((d.pipewire_sink.clone(), d.pipewire_source.clone()));
                // Fill in version/chip-ID from the pre-lock probe above (the
                // boot-time probe can return empty if it ran during the udev ACL
                // race; a reconnect is the right moment to fill it in).
                if hw.firmware_version.is_some() || hw.chip_id.is_some() {
                    info!("Hardware info refreshed after reconnect");
                    st.hw_info = hw;
                }
                st.audio.set_device(d);
                match st.audio.apply_full().await {
                    // apply_full() already enqueues any required instance restart on the
                    // RestartBus; `changed` only reports whether a conf actually differed.
                    Ok(true) => debug!("audio conf changed - instance restart enqueued"),
                    Ok(false) => debug!("audio conf unchanged - no instance restart"),
                    Err(e) => warn!("Failed to apply audio config on connect: {}", e),
                }
                // Re-assert the output route: the sink node names can change on
                // re-enumeration, so the default may now point at a stale node.
                if let Err(e) = st.audio.route_output().await {
                    warn!("Failed to route output on reconnect: {}", e);
                }
                // Same for the capture side: the voice chain is rebuilt on
                // reconnect, so re-point the default source at the fresh node.
                if let Err(e) = st.audio.route_input().await {
                    warn!("Failed to route input on reconnect: {}", e);
                }
                // Restore host-side dial volume onto whatever is actually playing,
                // so the knob position matches the real output level after a
                // replug. The tracked value is read here, but the write itself
                // happens after the lock is released: the writer resolves the sink
                // with a `pactl` call, and holding the global lock across a
                // subprocess stalls every IPC handler behind it.
                st.last_volume_target
                    .store(vol, std::sync::atomic::Ordering::Relaxed);
            }
            st.device = device;
            drop(st);
            apply_volume(&state, vol).await;
            info!("Volume restored to {}% after reconnect", vol);
            {
                let mut st = state.write().await;
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
                    // First connect after boot: push the saved dial volume onto
                    // whichever sink is actually playing, so display == actual
                    // output. This used to write to the EPOS sink unconditionally,
                    // which on every daemon start put the tracked level onto a
                    // device the user was not listening to — the original report,
                    // still present in this one path after the others were fixed.
                    let vol = st.volume.load(std::sync::atomic::Ordering::Relaxed);
                    st.last_volume_target
                        .store(vol, std::sync::atomic::Ordering::Relaxed);
                    drop(st);
                    apply_volume(&state, vol).await;
                    tracing::info!("Volume restored to {}% at boot", vol);
                    let mut st = state.write().await;
                    st.device = device;
                    // Boot with the device already plugged: the steady-state
                    // branch never calls apply_full, so the output route has to
                    // be asserted here or a persisted EQ=on would be ignored
                    // until the next toggle.
                    if let Err(e) = st.audio.route_output().await {
                        warn!("Failed to route output on boot: {}", e);
                    }
                    // And the capture route, for the same reason: a persisted
                    // voice mode must be live from the first poll, not only
                    // after the user toggles something.
                    if let Err(e) = st.audio.route_input().await {
                        warn!("Failed to route input on boot: {}", e);
                    }
                    continue;
                }
            }
            if st.device != device {
                st.device = device;
            }
            // Re-assert the capture route on every poll. The processed mic
            // node only exists once the voice instance has restarted and
            // published it, which happens after this branch's earlier work, so
            // a one-shot call at connect time would find nothing and never be
            // retried. route_input() exits cheaply when the default is already
            // correct, so the steady-state cost is one `pactl get-default-source`.
            if let Err(e) = st.audio.route_input().await {
                warn!("Failed to route input: {}", e);
            }
            // Watchdog for the EQ. route_output() only ran on connect and on
            // boot, so a chain that died in between left the default sink on
            // `epos-eq-input` — a null-sink that still accepts streams, i.e.
            // silence with no diagnostic. maintain_eq() verifies the chain,
            // falls back to raw hardware if it is gone, and asks for a restart.
            st.audio.maintain_eq().await;
            // Watchdog for the roles the EQ watcher does not cover. A MAIN
            // `pipewire.service` restart drops every cross-daemon instance's
            // link, and `voice`/`sidetone` previously stayed silent-but-apparently
            // healthy forever afterwards: nodes gone, units still `active`,
            // nothing logged. One shared node listing covers both.
            st.audio.maintain_instances().await;
            // Microphone signal watchdog. Self-rate-limited to one short capture
            // a minute, and only while a voice feature is engaged, because it
            // opens the capture device. It reports rather than repairs: a muted
            // capture element or a stale ALSA source belongs to the capture path,
            // not to the DSP instances, so the honest thing to do is stop
            // claiming that a microphone which is not delivering audio is a
            // healthy one.
            st.audio.maintain_mic_signal().await;
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

/// Background worker: debounced restart of the per-role epos instances.
///
/// Handlers that change on-disk epos instance confs record the role on the
/// [`AudioPipeline`] restart bus (via `write_instance_conf`) instead of
/// restarting anything inline. This worker coalesces those notifications for
/// 250 ms, drains the deduplicated role set, and restarts only those
/// instances — never the main pipewire graph (A2+ stays untouched).
async fn pipewire_reload_worker(state: Arc<RwLock<IpcState>>) {
    loop {
        // Clone the bus notify under the read lock, then release it before
        // awaiting — a long idle wait must not hold the IPC lock.
        let bus = {
            let st = state.read().await;
            st.audio.restarts.clone()
        };
        bus.notify.notified().await;
        // Coalesce rapid changes (e.g. an EQ slider drag) into one restart:
        // keep waiting up to 250 ms for more notifications before firing.
        while let Ok(()) =
            tokio::time::timeout(Duration::from_millis(250), bus.notify.notified()).await
        {}
        let pending = state.read().await.audio.restarts.drain();
        if pending.is_empty() {
            continue;
        }
        info!(
            "EPOS instance restart (debounced) for: {}",
            pending.iter().cloned().collect::<Vec<_>>().join(", ")
        );
        // Restart the roles concurrently, not in sequence. A role that is slow
        // or wedged must not delay the others: with a sequential loop one stuck
        // role silently cancelled every restart behind it, which is how a
        // voice instance ended up running a 68-minute-old conf. Each
        // `restart_epos_instance` is internally time-bounded, and spawning
        // them together means a failure in one cannot starve the rest.
        let mut handles = Vec::with_capacity(pending.len());
        for role in pending {
            handles.push(tokio::spawn(async move {
                let name = role.clone();
                let ok = audio::restart_epos_instance(&role).await;
                if !ok {
                    warn!("EPOS instance {name} did not come up cleanly");
                }
            }));
        }
        for h in handles {
            let _ = h.await;
        }
    }
}

/// Background worker: debounced persist of the dial volume to config.
///
/// Both the knob handler (`VolumeChanged`) and the external-volume watcher call
/// `volume_save_notify.notify_one()` instead of each spawning their own 2s-
/// debounced save task. A fast knob drag or a held media key would otherwise
/// spawn dozens of overlapping `save_config` tasks writing `config.json`
/// concurrently (audit F5). This coalesces them into a single save after 2s of
/// quiet, always committing the latest volume.
async fn volume_save_worker(state: Arc<RwLock<IpcState>>) {
    let notify = state.read().await.volume_save_notify.clone();
    loop {
        notify.notified().await;
        // Coalesce rapid changes (fast knob drag / held media key) into a single
        // save: keep consuming notifications for 2s before committing.
        while let Ok(()) = tokio::time::timeout(Duration::from_secs(2), notify.notified()).await {}
        let mut st = state.write().await;
        let v = st.volume.load(std::sync::atomic::Ordering::Relaxed);
        st.config.device.volume = Some(v);
        if let Err(e) = save_config(&st) {
            warn!("Failed to save volume to config: {}", e);
        }
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

        let mut new_config = match config::load() {
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

        // A change confined to the ACTIVE PROFILE's audio is an audio change
        // too. Comparing only the top-level `audio` block meant editing a
        // profile's EQ on disk was classified as a non-audio edit and never
        // applied: the watcher logged "non-audio settings updated" while the
        // generated EQ conf kept the old curve. This is the common way a user
        // edits an EQ.
        let active_before = st.config.active_profile.clone();
        let profile_audio = |cfg: &epos_shared::config::Config, name: &str| {
            cfg.profiles
                .iter()
                .find(|p| p.name.eq_ignore_ascii_case(name))
                .and_then(|p| serde_json::to_value(&p.audio).ok())
        };
        let active_profile_changed = active_before != new_config.active_profile
            || profile_audio(&st.config, &active_before)
                != profile_audio(&new_config, &new_config.active_profile);

        // Resolve the live audio from the active profile, the same way
        // `SetActiveProfile` does, so a profile edit actually reaches the
        // pipeline instead of the top-level snapshot winning.
        if active_profile_changed && !active_before.is_empty() {
            if let Some(p) = new_config
                .profiles
                .iter()
                .find(|p| p.name.eq_ignore_ascii_case(&new_config.active_profile))
            {
                new_config.audio = p.audio.clone();
            }
        }

        st.config = new_config;

        if audio_changed || active_profile_changed {
            let audio_cfg = st.config.audio.clone();
            st.audio.update_config(&audio_cfg);
            match st.audio.apply_full().await {
                // apply_full() already enqueues any required instance restart on the
                // RestartBus; `changed` only reports whether a conf actually differed.
                Ok(true) => debug!("audio conf changed - instance restart enqueued"),
                Ok(false) => debug!("audio conf unchanged - no instance restart"),
                Err(e) => warn!("Failed to apply reloaded audio config: {}", e),
            }
            info!(
                "Config hot-reload: audio settings applied{}",
                if active_profile_changed {
                    " (from the active profile)"
                } else {
                    ""
                }
            );
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
            // LedController already rate-limits its own warning to the
            // failure TRANSITION, so repeating it here produced a second
            // warning every 2s (~1400/hour) on top of the one it emitted
            // itself. Keep this at debug so a wedged LED path costs a single
            // visible line instead of flooding the journal.
            if led.set_mode(desired).is_err() {
                debug!("LED heartbeat: could not re-assert {:?}", desired);
                // The hotplug loop only calls reopen() when the device drops
                // off the bus entirely. This is the other case: still
                // enumerated, still refusing writes. Reopen the hidraw fd a
                // bounded number of times — a plain close/open, never USB
                // power/reset, which is what wedged the endpoint originally.
                led.recover_if_needed();
            }
        }
    }
}

/// Daemon-owned desktop notification for a smart-button action — fires even
/// with the GUI shut, because the daemon is the single source of truth for
/// button state and the GUI no longer emits its own toast.
///
/// Body is byte-parity with the GUI (`notifySmartButton`): `<profile> · <mode>`.
/// Deliberately a *sync* fn: it only reads already-committed `st.config` fields
/// and spawns `notify-send` detached, so it never holds the async state lock or
/// blocks the audio path. Debounced (~800ms) so the device's twin-press HID
/// readback (~0.5ms apart) yields exactly one toast, never two.
fn emit_smart_notify(st: &IpcState) {
    // Config gate — lets users disable just the button toast without touching
    // the rest of the smart-button behavior. Denies politely: no toast.
    if !st.config.smart_button.notify_enabled {
        return;
    }

    // Debounce: the GSX 300 reports one readback per physical press via its own
    // in-device debounce, but the HID twin-press can still surface twice
    // ~0.5ms apart (observed 12:29:43.871/43.871). Collapse to one toast.
    static LAST: std::sync::Mutex<Option<std::time::Instant>> =
        std::sync::Mutex::new(None);
    {
        let mut last = LAST.lock().unwrap();
        let now = std::time::Instant::now();
        if last
            .map(|t| now.duration_since(t) < std::time::Duration::from_millis(800))
            .unwrap_or(false)
        {
            return;
        }
        *last = Some(now);
    }

    // Build the body byte-for-byte like the GUI: `${active_profile ?? "Flat"} ·
    // ${modeLabel}` where modeLabel is "7.1" for surround/7.1 else "Stereo".
    let profile = if st.config.active_profile.is_empty() {
        epos_shared::config::FLAT_PROFILE_NAME.to_string()
    } else {
        st.config.active_profile.clone()
    };
    let mode_label = match st.config.mode {
        AudioMode::Surround71 => "7.1",
        AudioMode::Stereo => "Stereo",
    };
    let body = format!("{} · {}", profile, mode_label);

    // Detached spawn — best-effort, warn-only on failure (never blocks the
    // audio path, never returns an error up into the button handler).
    let res = std::process::Command::new("notify-send")
        .arg("-u")
        .arg("normal")
        .arg("-a")
        .arg("epos-gsx300")
        .arg("EPOS GSX 300")
        .arg(&body)
        .spawn();
    match res {
        Ok(mut child) => {
            // Give notify-send a moment to deliver, then let it detach fully.
            // Drop the handle: we don't await it (no async here by design).
            let _ = child.try_wait();
        }
        Err(e) => warn!("Smart-button notify: notify-send failed: {}", e),
    }
}
