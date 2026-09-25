mod audio;
mod config;
mod devices;
mod hid;
mod hid_io;
mod hwinfo;
mod ipc;
mod led;
mod streams;
mod sync;

use crate::audio::AudioPipeline;
use crate::audio::{run_status, COMMAND_BUDGET};
use crate::hid::{HidEvent, HidHandler};
use crate::ipc::IpcState;
use crate::led::LedController;
use crate::sync::lock;
use anyhow::Result;
use epos_shared::config::AudioMode;
use epos_shared::config::SmartButtonAction;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Notify, RwLock};
use tracing::{debug, error, info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    //
    // A constant directive, so it parses - but `unwrap` here would be a panic
    // during start-up, before the daemon has done anything at all, reachable only
    // by a future edit to the string. `unwrap_or_default` degrades to the default
    // filter instead: the same behaviour today, and a quiet daemon rather than a
    // dead one if it ever stops parsing.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "epos_gsx300d=info,warn".parse().unwrap_or_default()),
        )
        .init();

    info!("epos-gsx300d v{} starting...", env!("CARGO_PKG_VERSION"));

    // Load config
    let config = config::load()?;
    // Seed the daemon's belief about the file with what it actually contains,
    // before anything can save. Without this the first save would find no
    // belief to contradict and would overwrite an edit made between the load
    // and that save.
    let config_bytes_on_disk = std::fs::read(config::config_path()).ok();
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
    if config::resolve_active_profile_audio(&mut config) {
        info!(
            "Applying audio from active profile '{}' (top-level copy was stale)",
            config.active_profile
        );
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
    // Dark until the first routing poll says playback is on the EPOS. Lighting
    // it at boot would claim a mode for a headset nothing is going through,
    // which is the same "on but not working" this ring was just stopped from
    // doing.
    if let Some(ref mut led_ctrl) = led {
        if let Err(e) = led_ctrl.set_indicator(led::LedIndicator::Unused) {
            warn!("Failed to set initial LED state: {}", e);
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
        volume_by_sink: std::sync::Mutex::new(std::collections::BTreeMap::new()),
        mic_watch: std::sync::Arc::new(std::sync::Mutex::new(Default::default())),
        last_written: std::sync::Mutex::new(config_bytes_on_disk),
        volume_save_notify: Arc::new(Notify::new()),
        smart_button_seq: std::sync::atomic::AtomicU64::new(0),
    }));

    // Background task: handle smart button presses (mode sync) according to
    // the configured SmartButtonAction (default: toggle stereo ⇄ 7.1).
    let s = state.clone();
    tokio::spawn(async move {
        while let Some(evt) = hid_rx.recv().await {
            match evt {
                // The device reports a long press as its own value (0x04, measured
                // as >2s on hardware), so it is a separate gesture rather than a
                // longer version of the click. It used to fall into the arm below
                // and do exactly what a click does, which made the two
                // indistinguishable and left nothing to act on.
                //
                // It leaves the EPOS: put playback back on a device that is not
                // ours, which by the route rule takes the EQ out of the path and
                // darkens the ring.
                HidEvent::LongPress => {
                    // A toggle, not an exit. Asking to leave while already away was
                    // the state this button was in for its whole life: five presses
                    // in a row logged "playback is already off the EPOS" and the
                    // ring stayed dark, because the only way onto the EPOS was the
                    // GUI. The direction is decided by `toggle_epos` from
                    // `is_epos_sink`, never from the cached in-use flag, which lags
                    // a poll.
                    //
                    // Read lock, not write. This branch runs pactl with a 5s budget
                    // per command while holding it, and a write lock here freezes
                    // every IPC request, the config watcher and the volume watcher
                    // for the duration. Nothing here writes the state: the sequence
                    // counter is an atomic and the LED sync reads the pipeline.
                    let result = {
                        let st = s.read().await;
                        st.smart_button_seq
                            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        st.audio.toggle_epos().await
                    };
                    let audio::ToggleResult { outcome, moves } = result;
                    // Leaving the EPOS has to move the audio with it: changing the
                    // default only steers streams opened afterwards, and a stream
                    // the rescue pinned with `move-sink-input` keeps playing on the
                    // EPOS hardware. Measured: the EPOS DAC carried a louder signal
                    // than the speakers the user was listening on.
                    //
                    // The moves run here with no lock held, for the same reason the
                    // EQ rescue's do: one subprocess per stream at a 5s budget, and a
                    // read lock held across that freezes IPC and every watcher. The
                    // scope above has closed, so taking a lock again cannot nest.
                    if let Some(plan) = moves {
                        let moved = AudioPipeline::run_stream_moves(&plan).await;
                        let st = s.read().await;
                        st.audio.record_stream_moves(&plan, moved);
                    }
                    // The ring is the answer to "am I on the EPOS", so it is
                    // corrected here rather than up to a poll later. That needs the
                    // write lock, so it is a second short scope after the first has
                    // closed - the two must not nest, or the queued writer deadlocks
                    // against the read this task is still holding.
                    if matches!(
                        outcome,
                        audio::ToggleOutcome::Entered(_) | audio::ToggleOutcome::Left(_)
                    ) {
                        let mut st = s.write().await;
                        st.sync_led();
                    }
                    match outcome {
                        audio::ToggleOutcome::Entered(sink) => {
                            info!("Smart button long press: entered the EPOS via {sink}");
                        }
                        audio::ToggleOutcome::Left(sink) => {
                            info!("Smart button long press: left the EPOS for {sink}");
                        }
                        audio::ToggleOutcome::AlreadyHere => {
                            info!("Smart button long press: playback is already on the EPOS")
                        }
                        audio::ToggleOutcome::AlreadyAway => {
                            info!("Smart button long press: playback is already off the EPOS")
                        }
                        audio::ToggleOutcome::NoDevice => warn!(
                            "Smart button long press: no EPOS sink name is known, so the \
                             default was left alone"
                        ),
                        audio::ToggleOutcome::NotPublished(sink) => warn!(
                            "Smart button long press: the EPOS sink {sink} is not published, \
                             so the default was left alone"
                        ),
                        audio::ToggleOutcome::NowhereToGo => warn!(
                            "Smart button long press: no other published sink to go to, \
                             so the default was left alone"
                        ),
                        audio::ToggleOutcome::CouldNotDecide => warn!(
                            "Smart button long press: the sink list could not be read, \
                             so the default was left alone"
                        ),
                    }
                }
                HidEvent::ModeChanged(_) => {
                    let mut st = s.write().await;
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
                            st.sync_led();
                            info!("Smart button: mode → {:?} (LED sync)", new_mode);
                            emit_smart_notify(&st);
                        }
                        SmartButtonAction::ToggleEq => {
                            st.smart_button_seq
                                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            let enabled = !st.config.audio.eq.enabled;
                            st.config.audio.eq.enabled = enabled;
                            // The active profile is "the profile you are editing".
                            // The IPC setters mirror live edits into it; the
                            // button used not to, so switching profiles and back
                            // silently undid whatever the button just toggled.
                            crate::ipc::sync_active_profile(&mut st);
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
                                    .position(|n| {
                                        crate::ipc::profile_name_matches(
                                            n.trim(),
                                            st.config.active_profile.trim(),
                                        )
                                    })
                                {
                                    Some(i) => (i + 1) % names.len(),
                                    None => 0,
                                };
                                Some(names[next_idx].clone())
                            };
                            if let Some(name) = next {
                                if let Some(profile) = st
                                    .config
                                    .profiles
                                    .iter()
                                    .find(|p| {
                                        crate::ipc::profile_name_matches(p.name.trim(), &name)
                                    })
                                {
                                    let profile_audio = profile.audio.clone();
                                    let profile_mode = profile.mode;
                                    st.config.audio = profile_audio;
                                    st.config.active_profile = name.clone();
                                    st.config.mode = profile_mode;
                                    st.sync_led();
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
                            // The active profile is "the profile you are editing".
                            // The IPC setters mirror live edits into it; the
                            // button used not to, so switching profiles and back
                            // silently undid whatever the button just toggled.
                            crate::ipc::sync_active_profile(&mut st);
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
                            // The active profile is "the profile you are editing".
                            // The IPC setters mirror live edits into it; the
                            // button used not to, so switching profiles and back
                            // silently undid whatever the button just toggled.
                            crate::ipc::sync_active_profile(&mut st);
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
                    let _ = led.set_indicator(led::LedIndicator::Unused);
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

    // These two were joined as `let (_, _) =`, so both `Result`s were dropped:
    // a failed bind was invisible. If the Unix socket cannot be served the GUI
    // has no way in at all, and the daemon is not doing its job, so that ends
    // the process and lets systemd report it. The HTTP bridge only exists for
    // the vite dev server, so failing to take port 9898 — a leftover listener,
    // or a second daemon — must not take the real interface down with it, but it
    // must not pass unnoticed either.
    let (ipc_result, bridge_result) =
        tokio::join!(ipc::run_server(state.clone()), ipc::run_http_bridge(state.clone()),);

    if let Err(e) = &bridge_result {
        error!(
            "HTTP dev bridge unavailable on 127.0.0.1:9898: {e}. \
             The Unix socket interface is unaffected; only the browser dev GUI \
             cannot connect."
        );
    }

    // Both listeners ended. That is never a healthy steady state: the Unix
    // server returns `Ok` only by erroring out, so reaching this point means the
    // daemon cannot serve its one real interface and should not keep holding
    // the headset, the HID device and the capture element.
    //
    // Exit here rather than returning `Err`, because returning would drop the
    // tokio runtime, and dropping the runtime waits for its blocking tasks and
    // worker threads — some of which sit in synchronous HID I/O. Measured: a
    // second daemon that refused to start kept running past 25s instead of
    // exiting, still holding `/dev/hidraw3`. A fatal start-up condition has to
    // release the hardware on the spot and let systemd report the failure.
    let fatal = match &ipc_result {
        Err(e) => format!("IPC server failed: {e}"),
        Ok(()) => "IPC server stopped unexpectedly".to_string(),
    };
    error!("{fatal} — daemon cannot serve requests, exiting");
    std::process::exit(1);
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
    let level = percent.clamp(0, 100);
    let pct = format!("{level}%");
    // Remember it as this sink's own level before the write is attempted. The
    // watcher only re-applies a level it has been told about, and the dial is
    // one of the two ways the user sets a level, so a dial turn that was not
    // remembered would be lost the next time playback moved away and back.
    {
        let st = state.read().await;
        lock(&st.volume_by_sink).insert(sink.clone(), level);
    }
    // Capped like every other external command: a wedged `pactl` must not
    // leave the knob handler waiting forever. The dial's tracked value is
    // written after this returns either way, and the watcher corrects it
    // from the real sink level if the write did not land.
    if let Err(detail) = run_status("pactl", &["set-sink-volume", &sink, &pct], COMMAND_BUDGET).await
    {
        warn!("pactl set-sink-volume {} {} failed: {}", sink, pct, detail);
    }
}

/// Is the EQ chain the thing the user is listening to right now?
///
/// Decided by what the daemon follows for volume, because that is already the
/// answer to "which sink is carrying audio" and asking a second question would
/// be asking the graph twice for one answer. The anchor is the EQ sink, so
/// playback sitting on either the anchor or the raw EPOS hardware means the
/// chain is in use and the anchor's volume is a stage the signal passes through.
async fn st_chain_in_use(state: &Arc<RwLock<IpcState>>) -> bool {
    let followed = volume_sink(state).await;
    if followed.is_empty() {
        return false;
    }
    if followed == crate::audio::EQ_SINK_NAME {
        return true;
    }
    // The followed sink is the hardware end of the chain, so the chain is in
    // use exactly when the sink is one of ours. The node name comes from the
    // state's own cache, the same source `volume_sink` used a line above.
    let epos = {
        let st = state.read().await;
        st.pipewire_nodes
            .as_ref()
            .map(|(sink, _)| sink.clone())
            .unwrap_or_default()
    };
    crate::audio::is_epos_sink(&followed, &epos)
}

/// Read a sink's volume straight from PipeWire/PulseAudio.
///
/// Returns the level **unclamped**. It used to clamp here, and that is exactly
/// why the 100% cap could not be enforced: by the time the watcher saw a value,
/// 150% and 100% were the same number, so there was nothing left to notice and
/// nothing left to correct. The ceiling is applied by [`cap_sink_volume`], where
/// the fact that the sink was over it is still available.
///
/// Returns None if the sink is gone or the query fails.
async fn read_sink_volume(sink: &str) -> Option<i32> {
    if sink.is_empty() {
        return None;
    }
    // A hung `pactl` returns None here, so the watcher skips this tick and
    // tries again, rather than the loop waiting on it forever.
    let text = run_status("pactl", &["get-sink-volume", sink], COMMAND_BUDGET)
        .await
        .ok()?;
    // "Volume: front-left: 19660 / 30% / -31.37 dB,   front-right: ... / 30% / ..."
    // Take the first "/ N%" token — works for mono and stereo sinks.
    text.split('/')
        .nth(1)?
        .trim()
        .trim_end_matches('%')
        .parse::<i32>()
        .ok()
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
        let raw = match read_sink_volume(&sink).await {
            Some(v) => v,
            None => continue,
        };

        // The second stage of the chain is held at unity, and independently of
        // which sink the watcher follows. The anchor is the user's control - it
        // is the default sink, and it is what the daemon writes to now - so it is
        // left alone. The raw EPOS sink is the other factor in the product, and
        // it was found at 26% (-35 dB) across restarts with the user's own
        // control at 100%.
        if st_chain_in_use(&state).await {
            let raw_sink = {
                let st = state.read().await;
                st.pipewire_nodes
                    .as_ref()
                    .map(|(sink, _)| sink.clone())
                    .unwrap_or_default()
            };
            if !raw_sink.is_empty() {
                if let Some(now) = read_sink_volume(&raw_sink).await {
                    if let Some(want) = chain_stage_to_apply(now) {
                        debug!("chain's second stage is at {now}%, holding it at {want}%");
                        if let Err(detail) = run_status(
                            "pactl",
                            &["set-sink-volume", &raw_sink, &format!("{want}%")],
                            COMMAND_BUDGET,
                        )
                        .await
                        {
                            warn!("could not hold the second stage at {want}%: {detail}");
                        }
                    }
                }
            }
        }

        // The ceiling, over every volume-bearing object in the system rather
        // than only the sinks.
        //
        // The sink-only version of this looked complete and was not: measured on
        // this machine with it in place, 4 of 4 sink-inputs and 9 of 9 sources
        // were at 150% with the daemon silent, because `pactl list sinks` does
        // not mention either. The user asked for the ceiling to cover the whole
        // audio system and it covered a quarter of it.
        //
        // **Outside the state lock, deliberately.** These four listings and their
        // writes are four subprocesses at a five-second budget each, and the lock
        // is the one every IPC request, the config watcher, the hotplug loop and
        // the volume dial queue behind. The pattern of holding it across a
        // subprocess predates this function by one `set-sink-volume`; four more
        // multiplied it.
        //
        // Measured, 300 IPC samples against a 1 s tick: median 4.3 ms, p99 7.1 ms,
        // and a single 18.8 ms outlier - which is the four listings' own runtime,
        // to the millisecond. So an unlucky request waits exactly as long as the
        // lock is held and no longer. The 20 s tail is arithmetic, not something
        // observed here: zero commands hit their budget in five minutes.
        //
        // Nothing below needs the state. The writes address a sink by name; the
        // trackers are the followed sink's, and that is handled separately.
        for kind in [
            VolumeKind::Sink,
            VolumeKind::SinkInput,
            VolumeKind::Source,
            VolumeKind::SourceOutput,
        ] {
            let listing =
                run_status("pactl", &["list", kind.list_arg()], COMMAND_BUDGET).await;
            let Ok(listing) = listing else { continue };
            for (id, level) in over_cap_in(kind, &listing) {
                if kind == VolumeKind::Sink && id == sink {
                    continue;
                }
                info!(
                    "{} {id} is at {level}%, above the 100% ceiling — pulling it back",
                    kind.list_arg()
                );
                if let Err(detail) = run_status(
                    "pactl",
                    &[kind.set_verb(), &id, "100%"],
                    COMMAND_BUDGET,
                )
                .await
                {
                    warn!("could not pull {} {id} back to 100%: {detail}", kind.list_arg());
                }
            }
        }

        let st = state.write().await;

        // The followed sink's own cap, kept separate because it also has to
        // update the trackers below: a sink pushed past 100% by pavucontrol, the
        // DE applet or `wpctl` has to come back, and `apply_volume` only clamps
        // what the daemon itself writes.
        let capped = cap_sink_volume(raw);
        if let Some(level) = capped.write {
            info!("sink {sink} is at {raw}%, above the 100% ceiling — pulling it back");
            if let Err(detail) =
                run_status("pactl", &["set-sink-volume", &sink, &format!("{level}%")], COMMAND_BUDGET)
                    .await
            {
                warn!("could not pull {sink} back to {level}%: {detail}");
            }
            st.last_volume_target
                .store(level, std::sync::atomic::Ordering::Relaxed);
        }
        let actual = capped.report;

        // A different sink means the tracked number belonged to another device
        // entirely, so this is not somebody adjusting the volume: it is playback
        // moving.
        //
        // What the new sink should then BE is not the same question as what the
        // old one was. A sink this daemon has set before gets its own level
        // back; a sink heard for the first time is adopted. Adopting
        // unconditionally is what kept the EPOS at 24% — a level the speakers
        // had left lying around — across every session.
        let target_changed = {
            let mut last = lock(&st.last_volume_sink);
            let changed = !last.is_empty() && last.as_str() != sink.as_str();
            *last = sink.clone();
            changed
        };
        if target_changed {
            let remembered = lock(&st.volume_by_sink).get(&sink).copied();
            let verdict = volume_on_target_change(remembered, actual);
            if let Some(level) = verdict.write {
                info!("restoring {sink} to its own level, {level}% (found at {actual}%)");
                if let Err(detail) =
                    run_status("pactl", &["set-sink-volume", &sink, &format!("{level}%")], COMMAND_BUDGET)
                        .await
                {
                    warn!("could not restore {sink} to {level}%: {detail}");
                }
            }
            st.volume
                .store(verdict.report, std::sync::atomic::Ordering::Relaxed);
            st.last_volume_target
                .store(verdict.report, std::sync::atomic::Ordering::Relaxed);
            lock(&st.volume_by_sink).insert(sink.clone(), verdict.report);
            tracing::info!(
                "Volume now follows {} — showing its level, {}%",
                sink,
                verdict.report
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
        // continuous with reality, and remember it as this sink's level so the
        // next time playback comes back here it is restored rather than found.
        st.volume
            .store(actual, std::sync::atomic::Ordering::Relaxed);
        st.last_volume_target
            .store(actual, std::sync::atomic::Ordering::Relaxed);
        lock(&st.volume_by_sink).insert(sink.clone(), actual);
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
                if let Some(ref mut led) = st.led {
                    if let Err(e) = led.reopen() {
                        warn!("Failed to reopen LED device: {}", e);
                    }
                }
                st.sync_led();
                info!("LED re-synced after reconnect");
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
                    // When start-up detection missed the headset, `main` never
                    // gave the pipeline a device and never applied the config,
                    // and the reconnect branch is skipped because
                    // `was_connected` was already seeded true. Close that here.
                    // Done before `st.device = device` so the device can be
                    // borrowed from the local, not from the guard.
                    if first_connect_owes_pipeline(st.audio.has_device()) {
                        info!("Pipeline had no device at boot - applying config now");
                        if let Some(ref d) = device {
                            st.audio.set_device(d);
                        }
                        let audio_cfg = st.config.audio.clone();
                        st.audio.update_config(&audio_cfg);
                        match st.audio.apply_full().await {
                            Ok(true) => debug!("audio conf changed - instance restart enqueued"),
                            Ok(false) => debug!("audio conf unchanged - no instance restart"),
                            Err(e) => warn!("Failed to apply config on first connect: {}", e),
                        }
                    }
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
            // Everything below is read-only on the pipeline, and all four calls
            // take `&self`: they mutate only their own atomics and the mutexes
            // inside `AudioPipeline` (`eq_chain_missing_polls`, `role_health`,
            // `mic_watch`, `restarts`). None of them needs the write lock, yet
            // holding it froze the entire control surface for as long as they
            // ran -- measured at 2.7 seconds every 63 seconds, which is
            // `maintain_mic_signal` capturing microphone audio with the global
            // write lock held. GetStatus, the config watcher, the volume watcher
            // and every IPC request all queue behind that one lock.
            //
            // A read lock lets the 3s status poll run alongside.
            drop(st);
            let plan = {
                let st = state.read().await;
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
            // silence with no diagnostic. eq_plan() verifies the chain,
            // falls back to raw hardware if it is gone, and asks for a restart.
            // back, falls back to raw hardware if it is gone, asks for a restart,
            // and hands back the stream moves it wants made.
            let plan = st.audio.eq_plan().await;
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
            plan
            };
            // The moves run here, with no lock held. Each one is a `pactl
            // move-sink-input` at a 5 s budget and there is one per stream
            // playing, so this can be minutes of subprocess time. Under the read
            // lock above, tokio's write-preferring FIFO would park every IPC
            // request, the config watcher and the volume watcher for the whole
            // of it - the same freeze the microphone capture used to cause.
            //
            // Like the capture below, this scope must not nest with the one
            // above: a queued writer cannot be granted while this task holds a
            // read, so a second read taken while the first is alive deadlocks.
            if let Some(plan) = plan {
                let result = AudioPipeline::run_stream_moves(&plan).await;
                let st = state.read().await;
                st.audio.record_stream_moves(&plan, result);
            }
            // The microphone probe captures audio for about 2.7 seconds, and it
            // runs here with no state lock held at all.
            //
            // What it needs is a handle to its own state plus two values, so the
            // snapshot is taken in its own short scope above and the capture
            // happens after every guard has gone. Holding any lock for the
            // capture froze the whole control surface for its duration, and
            // tokio's write-preferring FIFO made it worse: the volume watcher
            // queues a writer every second, so that writer — and every reader
            // behind it — waited out the full capture.
            //
            // The two scopes above and here must not nest, and an earlier
            // version of this did nest a read inside the long read. That is the
            // deadlock tokio documents: a queued writer cannot be granted while
            // this task still holds a read, and the second read cannot be
            // granted while the writer is queued. The daemon answered no request
            // at all until it was restarted.
            let (voice_engaged, source, watch) = {
                let st = state.read().await;
                (
                    st.audio.voice_path_engaged(),
                    st.device.as_ref().map(|d| d.pipewire_source.clone()),
                    std::sync::Arc::clone(&st.mic_watch),
                )
            };
            audio::AudioPipeline::maintain_mic_signal(voice_engaged, source, &watch).await;
        }

        was_connected = is_connected;
    }
}

/// Whether the first-connect branch still owes the pipeline its device.
///
/// `main` leaves the node cache empty even when its own start-up scan found the
/// headset, so this branch runs on the first poll either way. On a normal boot
/// the pipeline was already given a device and the whole config applied, and
/// doing that again would re-write every instance conf for nothing.
///
/// It is genuinely owed only when start-up detection missed: `main` saw no
/// device, so it never called `set_device` or `apply_full`, and the loop's own
/// scan found the headset moments later with `was_connected` already seeded
/// true — which skips the reconnect branch, the only other place that hands the
/// pipeline a device. Without this, the routing below ran against fallback node
/// names with a config that had never been applied, and nothing said so.
fn first_connect_owes_pipeline(pipeline_has_device: bool) -> bool {
    !pipeline_has_device
}

/// What the volume watcher should do about a sink it has just started following.
///
/// Two decisions that used to be one guess. Separated because they answer
/// different questions and the answers conflict: "what should this sink read" is
/// about the ceiling, "what should this sink be set to" is about whose level
/// applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VolumeVerdict {
    /// The level the daemon should report and track.
    pub report: i32,
    /// The level to write back, or `None` to leave the sink alone.
    pub write: Option<i32>,
}

/// Keep a sink at or below 100%.
///
/// PipeWire allows a sink to 150% and pavucontrol, DE applet and `wpctl` all
/// offer it, so a cap inside `apply_volume` alone is not a cap: it only bounds
/// what the daemon writes. The sink can still be pushed past 100% from outside
/// and stay there, which is the state the user had to reach before the EPOS was
/// audible at all.
///
/// The write is unconditional rather than "only if it changed", because the
/// value arriving here has already lost the information that it was over - the
/// old read clamped on the way in, so 150% and 100% were indistinguishable and
/// the cap could not be enforced at all.
pub fn cap_sink_volume(raw: i32) -> VolumeVerdict {
    if raw > 100 {
        VolumeVerdict {
            report: 100,
            write: Some(100),
        }
    } else {
        VolumeVerdict {
            report: raw.max(0),
            write: None,
        }
    }
}

/// What to do when playback moves to a different sink.
///
/// `remembered` is the level this daemon last set on the sink being moved to,
/// and `on_disk` is what the sink is sitting at right now.
///
/// The old behaviour adopted `on_disk` unconditionally. That is right for a
/// device being heard for the first time and wrong for one heard before: the
/// level sitting on a sink between sessions is whatever was left there, and the
/// EPOS sink was found at 24% - 37 dB - session after session, adopted each
/// time, and then written back to `device.volume` by the save worker. The value
/// the user actually chose for the headset was overwritten by the level of a
/// speaker that had been playing moments earlier, and the headset stayed quiet.
///
/// So a sink we have set before gets its own level back. A sink we have not is
/// still adopted, because nothing else knows what a newly plugged device wants.
pub fn volume_on_target_change(remembered: Option<i32>, on_disk: i32) -> VolumeVerdict {
    match remembered {
        Some(level) => {
            let capped = level.clamp(0, 100);
            VolumeVerdict {
                report: capped,
                // Only write when the sink is actually somewhere else, or this
                // fires a pactl call every second for every sink it follows.
                write: if on_disk == capped { None } else { Some(capped) },
            }
        }
        None => VolumeVerdict {
            report: on_disk.clamp(0, 100),
            write: None,
        },
    }
}

/// The level a stage of the chain must be held at, or `None` to leave it alone.
///
/// Used for the one stage the user does **not** control. The chain is
/// `app -> anchor -> EQ -> raw EPOS -> DAC`, and both nodes have a volume, so
/// they multiply. The anchor is the default sink, which is what the desktop
/// applet, pavucontrol, `wpctl` and the media keys all write to, and the daemon
/// now follows it as well - so the anchor is the user's control and is left
/// alone. The raw EPOS sink is the other half of the product, and two
/// independently adjustable attenuators is one too many: the user can only
/// account for the one they are looking at.
///
/// Unity is the only defensible value for the unaccounted-for half. Anything
/// else is a level nobody chose and nobody can see. It was found at 26% -
/// -35 dB - after a restart, with the hardware end the user had set to 100%.
///
/// The "not already there" check is what keeps this from being a `pactl` call
/// every second for the life of the daemon.
pub fn chain_stage_to_apply(current: i32) -> Option<i32> {
    if current == 100 {
        None
    } else {
        Some(100)
    }
}

/// What kind of object a volume belongs to, and the `pactl` verb that writes it.
///
/// The ceiling was applied to sinks only, and the rest of the system was found
/// by measurement rather than assumed: with four sink-inputs and nine sources
/// sitting at 150% and the daemon reporting a clean bill of health. A cap on
/// sinks is not a cap on the machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeKind {
    Sink,
    SinkInput,
    Source,
    SourceOutput,
}

impl VolumeKind {
    /// The `pactl list` sub-command that produces this kind's listing.
    pub fn list_arg(self) -> &'static str {
        match self {
            VolumeKind::Sink => "sinks",
            VolumeKind::SinkInput => "sink-inputs",
            VolumeKind::Source => "sources",
            VolumeKind::SourceOutput => "source-outputs",
        }
    }

    /// The `pactl set-*-volume` verb that writes it. Verified by running each
    /// one against the live graph rather than read off `--help`, which does not
    /// list them on this pactl at all.
    pub fn set_verb(self) -> &'static str {
        match self {
            VolumeKind::Sink => "set-sink-volume",
            VolumeKind::SinkInput => "set-sink-input-volume",
            VolumeKind::Source => "set-source-volume",
            VolumeKind::SourceOutput => "set-source-output-volume",
        }
    }

    /// How a block in this kind's listing is identified.
    fn id_field(self) -> IdField {
        match self {
            VolumeKind::Sink | VolumeKind::Source => IdField::Named,
            VolumeKind::SinkInput | VolumeKind::SourceOutput => IdField::Index,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IdField {
    /// `Name: <name>`, written by name.
    Named,
    /// The numeric id from the `Sink Input #<n>` block header.
    Index,
}

/// Every object of `kind` in `listing` whose level is above 100%, as
/// `(identifier, level)`.
///
/// The identifier is a sink name or a numeric index depending on the kind, and
/// the caller passes the matching half back to `pactl set-*-volume` unchanged.
pub fn over_cap_in(kind: VolumeKind, listing: &str) -> Vec<(String, i32)> {
    let mut out = Vec::new();
    let mut name: Option<String> = None;
    let mut id: Option<String> = None;
    let header = match kind {
        VolumeKind::Sink => "Sink #",
        VolumeKind::SinkInput => "Sink Input #",
        VolumeKind::Source => "Source #",
        VolumeKind::SourceOutput => "Source Output #",
    };
    for raw in listing.lines() {
        let line = raw.trim();
        if let Some(rest) = line.strip_prefix(header) {
            // A new block invalidates whatever the previous one held.
            id = Some(rest.split_whitespace().next().unwrap_or("").to_string());
            name = None;
            continue;
        }
        if let Some(rest) = line.strip_prefix("Name: ") {
            name = Some(rest.trim().to_string());
            continue;
        }
        if !line.starts_with("Volume: ") {
            continue;
        }
        let key = match kind.id_field() {
            IdField::Named => name.clone(),
            IdField::Index => id.clone(),
        };
        let Some(key) = key.filter(|k| !k.is_empty()) else {
            continue;
        };
        let Some(pct) = line
            .strip_prefix("Volume: ")
            .and_then(|rest| rest.split('/').nth(1))
            .map(|s| s.trim().trim_end_matches('%').trim())
            .and_then(|s| s.parse::<i32>().ok())
        else {
            continue;
        };
        if pct > 100 {
            out.push((key, pct));
        }
    }
    out
}

/// Decide whether a hand-edited `device.volume` should be put on the sink.
///
/// Pure. The watcher used to copy the value into `st.config` and nothing else,
/// which left the daemon's trackers saying one thing and the file another. The
/// volume watcher then read the real sink, adopted that level over the edit, and
/// the next save wrote it back — so editing the volume in the file did nothing,
/// with a log line that only said "non-audio settings updated".
///
/// Clamped rather than trusted: the file is hand-editable and the dial's range
/// is 0-100. Unchanged means no action, so a watcher that only re-reads the
/// same file never disturbs the sink.
fn external_volume_to_apply(
    previous: Option<i32>,
    edited: Option<i32>,
) -> Option<i32> {
    let edited = edited?;
    if previous == Some(edited) {
        return None;
    }
    Some(edited.clamp(0, 100))
}

/// Background task: watch config file for external edits and hot-apply them.
///
/// Persists the daemon's config to disk and records the exact bytes written so
/// `config_watch_loop` can recognize the daemon's own atomic `save()` and skip
/// it — instead of reloading and reverting newer in-memory state.
/// Whether the config file may be written right now.
#[derive(Debug, Clone, PartialEq, Eq)]
enum SaveDecision {
    /// The file is what we believe it to be, so writing is safe.
    Proceed,
    /// Somebody else changed it. `reason` is what to tell the user.
    Refuse { reason: String },
}

/// Decide whether a save may go ahead, from what the daemon believes is on disk
/// and what is actually there.
///
/// The daemon owns the config in memory and rewrites the whole file, so an edit
/// made outside it is only picked up once the 2s watcher notices. Until then any
/// save overwrites it. That was observed on this machine: a volume save landed
/// between two writes of a test and the second one was lost.
///
/// Re-checking immediately before the write and failing closed when ownership
/// cannot be established is the guidance for daemon-authored files, and the
/// belief is kept honest in both directions: `save_config` records what it
/// wrote, and the watcher records what it adopted.
fn save_decision(believed: Option<&[u8]>, on_disk: Option<&[u8]>) -> SaveDecision {
    let (Some(believed), Some(on_disk)) = (believed, on_disk) else {
        // Nothing of ours to protect: either we have never written, or there is
        // no file there to overwrite.
        return SaveDecision::Proceed;
    };
    if believed == on_disk {
        return SaveDecision::Proceed;
    }
    SaveDecision::Refuse {
        reason: format!(
            "config.json changed outside the daemon \
             ({} bytes on disk, {} bytes expected) - not overwriting it; \
             the edit will be loaded shortly",
            on_disk.len(),
            believed.len()
        ),
    }
}

fn save_config(st: &IpcState) -> Result<(), anyhow::Error> {
    // Check before writing, not after: after the fact the edit is already gone.
    let on_disk = std::fs::read(config::config_path()).ok();
    let believed = lock(&st.last_written).clone();
    if let SaveDecision::Refuse { reason } = save_decision(believed.as_deref(), on_disk.as_deref())
    {
        // Surfaced to the client as a failure, and logged, so the edit is
        // visible rather than silently losing a race.
        warn!("{reason}");
        return Err(anyhow::anyhow!("{reason}"));
    }

    config::save(&st.config)?;
    if let Ok(bytes) = std::fs::read(config::config_path()) {
        *lock(&st.last_written) = Some(bytes);
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
    // The mtime whose read or parse last failed. Retrying is what lets an edit
    // survive being caught mid-write, but warning on every 2s tick would bury
    // the log for as long as a file stays broken, so the warning is per change.
    let mut last_failed_mtime: Option<std::time::SystemTime> = None;

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let mtime = std::fs::metadata(&path).and_then(|m| m.modified()).ok();
        if mtime == last_mtime {
            continue;
        }
        // `last_mtime` is advanced only once this change is settled — the
        // daemon's own write, or an external edit that parsed. It used to be
        // assigned here, before the read and the parse, so a read that failed
        // or a file caught mid-write marked the change as seen and it was
        // never looked at again: the edit was silently dropped, and the only
        // thing that could revive it was an unrelated later change.

        // Read the raw file bytes before parsing so we can recognize the
        // daemon's own atomic write. If they match what we last wrote, this
        // mtime change came from a daemon `save()` — skip it. This prevents a
        // spurious hot-reload from reverting in-memory state that has advanced
        // past the on-disk snapshot (e.g. two rapid profile switches inside the
        // 2s poll window, or a knob turn queued behind a save).
        let disk_bytes = match std::fs::read(&path) {
            Ok(b) => b,
            // Unsettled: leave `last_mtime` alone so the next tick retries.
            Err(e) => {
                debug!("Config read failed, will retry: {}", e);
                last_failed_mtime = mtime;
                continue;
            }
        };
        {
            let s = state.read().await;
            if *lock(&s.last_written) == Some(disk_bytes.clone()) {
                // This mtime change came from the daemon's own atomic `save()`;
                // skip it so we don't revert in-memory state that has advanced
                // past the on-disk snapshot.
                last_mtime = mtime;
                last_failed_mtime = None;
                continue;
            }
        }

        let mut new_config = match config::load_existing() {
            Ok(c) => c,
            Err(e) => {
                // File may be mid-write or malformed; keep current config and
                // keep watching, so the edit lands once the file is whole. Warn
                // once for this change rather than on every retry.
                if last_failed_mtime != mtime {
                    warn!("Config reload failed, keeping current config: {}", e);
                    last_failed_mtime = mtime;
                }
                continue;
            }
        };

        // Settled: this external edit is now in memory.
        last_mtime = mtime;
        last_failed_mtime = None;

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

        // A hand-edited `device.volume` has to reach the sink, not just the
        // config: otherwise the 1s volume watcher reads the real sink, adopts
        // that level over the edit, and the next save writes it back — the edit
        // does nothing, silently. Computed before the config is replaced so the
        // previous value is the one actually in force.
        let edited_volume = external_volume_to_apply(
            st.config.device.volume,
            new_config.device.volume,
        );
        if let Some(percent) = edited_volume {
            st.volume
                .store(percent, std::sync::atomic::Ordering::Relaxed);
            st.last_volume_target
                .store(percent, std::sync::atomic::Ordering::Relaxed);
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
            st.sync_led();
            info!("Config hot-reload: LED synced to {:?}", desired_mode);
        }
        if !audio_changed && !mode_changed {
            info!("Config hot-reload: non-audio settings updated");
        }
        *lock(&st.last_written) = Some(disk_bytes);
        // Release the lock before touching the sink: the write costs a `pactl`
        // pair, and a writer resolving the sink under the global lock is exactly
        // what this file has been removing.
        drop(st);
        if let Some(percent) = edited_volume {
            apply_volume(&state, percent).await;
            info!(
                "Config hot-reload: volume set to {}% from the edited file",
                percent
            );
        }
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
        st.sync_led();
        if st.led.as_ref().is_some_and(|led| led.write_failing()) {
            debug!("LED heartbeat: the indicator could not be re-asserted");
            // The hotplug loop only calls reopen() when the device drops off the
            // bus entirely. This is the other case: still enumerated, still
            // refusing writes. Reopen the hidraw fd a bounded number of times — a
            // plain close/open, never USB power/reset, which is what wedged the
            // endpoint originally.
            if let Some(ref mut led) = st.led {
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
        let mut last = lock(&LAST);
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

#[cfg(test)]
mod tests {
    use super::*;

    // ─── House rules, checked rather than remembered ─────────

    /// Every `.rs` file under `dir`, skipping build output and VCS metadata.
    ///
    /// `target` and `node_modules` are skipped by name because walking into
    /// them is the difference between a test that runs in a millisecond and one
    /// that walks a few hundred thousand files. The GUI crate has a
    /// `src-tauri/target` of its own, so this is not hypothetical.
    fn rust_sources(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if matches!(name.as_ref(), "target" | ".git" | "node_modules") {
                continue;
            }
            let path = entry.path();
            if path.is_dir() {
                rust_sources(&path, out);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                out.push(path);
            }
        }
    }

    /// The workspace root, found by shape rather than by counting `..` segments.
    ///
    /// A relative hop count is a thing that breaks when the tree is rearranged,
    /// and the symptom would be a house-rule test that silently scans an empty
    /// directory and passes. Looking for the directory that has both a manifest
    /// and a `crates` says what is meant instead.
    fn workspace_root() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .find(|dir| dir.join("Cargo.toml").is_file() && dir.join("crates").is_dir())
            .expect("some ancestor holds the workspace manifest and the crates dir")
            .to_path_buf()
    }

    /// Nothing silences `dead_code` or `unused` with an `allow`.
    ///
    /// This codebase carried three `#[allow(dead_code)]` functions and about
    /// fifty lines of genuinely unreachable code, and the allow attributes are
    /// why none of it ever showed up as a warning. In a binary crate the
    /// attribute is almost never "this is fine as is" - it is either "only the
    /// tests reach this", which means the shipped daemon carries code nothing
    /// calls, or "I meant to delete this and did not". Both are decisions, and
    /// the attribute is a way of taking them silently.
    ///
    /// That is the argument for checking the whole workspace rather than just
    /// the crate: `epos-shared` is a library, where the attribute means a third
    /// thing - public API kept for a caller that does not exist yet - and it is
    /// still worth having to say so out loud.
    #[test]
    fn no_source_file_silences_dead_code_or_unused_with_an_allow() {
        let root = workspace_root();
        let mut files = Vec::new();
        rust_sources(&root, &mut files);
        files.sort();

        // First: make sure the walk found anything. A test that scans nothing
        // and finds nothing is indistinguishable from a clean tree, and the
        // earlier version of the probe test in `audio.rs` failed for exactly
        // this reason - it counted its own text. Named files, not a count, so
        // adding a module does not break this.
        for expected in ["crates/epos-shared/src/lib.rs", "crates/epos-gsx300d/src/main.rs"] {
            assert!(
                files.iter().any(|f| f.ends_with(expected)),
                "the walk missed {expected}, so the rule below is vacuous"
            );
        }

        let mut offenders = Vec::new();
        let mut scanned_lines = 0usize;
        for file in &files {
            let text = std::fs::read_to_string(file).unwrap_or_default();
            // Only the part above the first `#[cfg(test)]`. This is not tidiness:
            // without it the scan reads this test's own doc comment, which quotes
            // the attribute in order to explain it, and its own pattern string,
            // and then fails on the two. That is the third time in this crate
            // that a text-matching test has caught itself - the first two are
            // `audio.rs`'s probe test and its doc-pairing test - so the rule is
            // now: if the check is textual, the check has to exclude its own
            // text, and that exclusion has to be visible here.
            let production = match text.split_once("#[cfg(test)]") {
                Some((before, _tests)) => {
                    assert!(
                        before.len() < text.len(),
                        "split_once gave back the whole file; the marker moved"
                    );
                    before
                }
                None => text.as_str(),
            };
            scanned_lines += production.lines().count();
            for (n, line) in production.lines().enumerate() {
                let squished: String = line.chars().filter(|c| !c.is_whitespace()).collect();
                if squished.contains("allow(dead_code") || squished.contains("allow(unused") {
                    offenders.push(format!("{}:{}: {}", file.display(), n + 1, line.trim()));
                }
            }
        }
        assert!(
            scanned_lines > 1000,
            "only {scanned_lines} production line(s) were read; the walk or the \
             split is not finding the sources"
        );
        assert!(
            offenders.is_empty(),
            "{} line(s) silence a lint with an allow. Delete the code, call it \
             from where it is used, or argue for it here:\n  {}",
            offenders.len(),
            offenders.join("\n  ")
        );
    }

    // ─── Whether a save may go ahead ─────────────────────────
    //
    // The daemon keeps the authoritative config in memory and rewrites the whole
    // file. If the user edits that file while the daemon is running, the edit is
    // only picked up by the 2s watcher — and until it is, a save from an IPC
    // request or the volume worker overwrites it. That was observed on this
    // machine: a legitimate volume save landed between two writes of a test and
    // the second one was lost.
    //
    // Research on daemon-authored files points the same way: re-check the
    // current state immediately before the write, and if ownership cannot be
    // established, fail closed rather than clobber. So the daemon remembers what
    // it believes is on disk, and refuses to write over anything else.

    /// Writing is fine when the file is what we think it is.
    #[test]
    fn a_save_proceeds_when_the_file_is_what_we_wrote() {
        assert!(matches!(
            save_decision(Some(b"ours"), Some(b"ours")),
            SaveDecision::Proceed
        ));
    }

    /// The case that lost the edit: the file no longer matches our belief, so
    /// somebody changed it behind our back.
    #[test]
    fn a_save_is_refused_when_somebody_else_changed_the_file() {
        let outcome = save_decision(Some(b"ours"), Some(b"theirs"));
        let reason = match outcome {
            SaveDecision::Refuse { reason } => reason,
            SaveDecision::Proceed => panic!("a foreign edit must not be overwritten"),
        };
        assert!(
            reason.contains("changed") || reason.contains("edit"),
            "the reason must name the conflict: {reason}"
        );
    }

    /// Never having written is not a conflict — there is nothing of ours to
    /// protect. The startup path seeds the belief from the file it loaded, so
    /// this is only the first-write case.
    #[test]
    fn a_first_save_is_not_a_conflict() {
        assert!(matches!(
            save_decision(None, Some(b"whatever")),
            SaveDecision::Proceed
        ));
    }

    /// A file that has gone missing has nothing to overwrite.
    #[test]
    fn a_missing_file_does_not_block_a_save() {
        assert!(matches!(
            save_decision(Some(b"ours"), None),
            SaveDecision::Proceed
        ));
    }

    /// The reason names the sizes, so a log reader can tell a one-field edit
    /// from a wholesale replacement without opening both files.
    #[test]
    fn the_conflict_reason_says_how_different_the_file_is() {
        let reason = match save_decision(Some(b"1234"), Some(b"12345678")) {
            SaveDecision::Refuse { reason } => reason,
            SaveDecision::Proceed => panic!("expected a refusal"),
        };
        assert!(reason.contains('4'), "expected the expected size in: {reason}");
        assert!(reason.contains('8'), "expected the actual size in: {reason}");
    }

    //
    // The watcher adopted a hand-edited `device.volume` into `st.config` and
    // left the daemon's own trackers alone. The value therefore went nowhere:
    // the 1s volume watcher read the real sink, saw the tracked level differ,
    // adopted the sink's value over it, and the next save wrote that back. The
    // edit was undone silently, and the log said only "non-audio settings
    // updated".

    // ─── Volume arriving from an edited config file ──────────
    //
    // The watcher used to copy a hand-edited `device.volume` into `st.config`
    // and nothing else. The value went nowhere: the 1s volume watcher read the
    // real sink, adopted that level over the edit, and the next save wrote it
    // back — so editing the volume in the file did nothing, and the log said
    // only "non-audio settings updated".

    // ─── The sink itself, and whose level applies to it ──────────
    //
    // Two things the old watcher got wrong, and they are separate:
    //
    //   - the cap. `apply_volume` clamps what it writes, but the sink can be
    //     pushed past 100% from outside and the read clamped on the way in, so
    //     150% and 100% looked identical and nothing ever pulled it back;
    //   - ownership. `device.volume` is one number, and whichever device was
    //     playing last owned it. Moving from speakers at 20% onto the headset
    //     handed the headset the speakers' level, and the save worker wrote it
    //     back, so the next boot began "Volume restored to 20% at boot".

    /// The ceiling covers per-application volumes and capture volumes too.
    ///
    /// Measured on this machine with the sink-only cap in place: 4 of 4
    /// sink-inputs and 9 of 9 sources sitting at 150%, and the daemon quiet,
    /// because `pactl list sinks` never mentions either. A ceiling that leaves
    /// the microphone and every application's own volume unlimited is not a
    /// ceiling on the machine.
    ///
    /// Both listings are captured verbatim from this machine. The shapes differ
    /// in the way that matters: a `Sink Input` block is identified only by the
    /// number in its header and has no `Name:`, while a `Source` block has a
    /// `Name:` and is written by name. Reading either with the other's rules
    /// finds nothing, and finding nothing looks exactly like a clean system.
    #[test]
    fn application_and_capture_volumes_are_covered_too() {
        let sink_inputs = "\
Sink Input #35
\tDriver: protocol-native
\tSink: 26125
\tMute: no
\tVolume: front-left: 98304 / 150% / 10.57 dB,   front-right: 98304 / 150% / 10.57 dB
\tapplication.name = \"firefox\"
Sink Input #2536
\tSink: 26125
\tMute: no
\tVolume: front-left: 32768 / 50% / -18.06 dB,   front-right: 32768 / 50% / -18.06 dB
\tapplication.name = \"opencode\"
";
        assert_eq!(
            over_cap_in(VolumeKind::SinkInput, sink_inputs),
            vec![("35".to_string(), 150)],
            "a sink input is found by the number in its header, and one under \
             the ceiling is left out"
        );

        let sources = "\
Source #36
\tName: alsa_output.usb-Generic_USB_Audio-00.HiFi_7_1__SPDIF__sink.monitor
\tMute: no
\tVolume: mono: 98304 / 150% / 10.57 dB
\tDescription: Monitor of SPDIF
Source #37
\tName: alsa_input.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.mono-fallback
\tMute: no
\tVolume: mono: 65536 / 100% / 0.00 dB
";
        assert_eq!(
            over_cap_in(VolumeKind::Source, sources),
            vec![(
                "alsa_output.usb-Generic_USB_Audio-00.HiFi_7_1__SPDIF__sink.monitor"
                    .to_string(),
                150
            )],
            "a source is found by name, including the monitor sources"
        );
    }

    /// A block with a volume but no usable identifier is skipped rather than
    /// written with an empty one, and a new header clears the previous block's
    /// identity so a source cannot be written with a sink input's number.
    #[test]
    fn a_block_with_no_usable_identifier_is_skipped() {
        let listing = "\
Source Output #7
\tDriver: protocol-native
\tSource: 36
\tVolume: mono: 98304 / 150% / 10.57 dB
\tapplication.name = \"some_recorder\"
";
        assert_eq!(
            over_cap_in(VolumeKind::SourceOutput, listing),
            vec![("7".to_string(), 150)],
            "a source output is found by the number in its header"
        );
        // The same text read as a source has no Name, so nothing is found - and
        // finding nothing is the correct answer, not a silent pass over it.
        assert!(over_cap_in(VolumeKind::Source, listing).is_empty());
    }

    /// Every kind maps to a `pactl` verb that exists, and the verbs are
    /// distinct - two kinds sharing one verb would mean one of them was never
    /// being written.
    #[test]
    fn every_kind_has_its_own_write_verb() {
        let kinds = [
            VolumeKind::Sink,
            VolumeKind::SinkInput,
            VolumeKind::Source,
            VolumeKind::SourceOutput,
        ];
        let mut verbs: Vec<&str> = kinds.iter().map(|k| k.set_verb()).collect();
        let before = verbs.len();
        verbs.sort_unstable();
        verbs.dedup();
        assert_eq!(verbs.len(), before, "two kinds share a write verb");
        assert_eq!(
            VolumeKind::Sink.set_verb(),
            "set-sink-volume",
            "the verb a cap depends on has to be the real one"
        );
    }

    /// The ceiling covers every sink, not only the one playing.
    ///
    /// The sink path of the parser that actually runs.
    ///
    /// These four tests used to call a sink-only copy, `sinks_over_cap`, while
    /// the watcher called `over_cap_in`. Making `over_cap_in` return nothing for
    /// `VolumeKind::Sink` - which disables the ceiling for every sink on the
    /// machine - left the whole suite green. The coverage was on the copy and
    /// not on the code, and the copy existed only to be covered.
    ///
    /// So the assertions are here now, against the path production takes. Same
    /// listings, captured from this machine.
    #[test]
    fn every_sink_over_the_ceiling_is_found_not_just_the_one_playing() {
        let listing = "\
Sink #36
\tState: IDLE
\tName: games_sink
\tVolume: front-left: 65536 / 100% / 0.00 dB,   front-right: 65536 / 100% / 0.00 dB
\tBase Volume: 65536 / 100% / 0.00 dB
Sink #60
\tName: alsa_output.usb-Generic_USB_Audio-00.HiFi_7_1__SPDIF__sink
\tVolume: front-left: 98304 / 150% / 10.61 dB,   front-right: 98304 / 150% / 10.61 dB
Sink #61
\tName: alsa_output.usb-Generic_USB_Audio-00.HiFi_7_1__Speaker__sink
\tVolume: front-left: 72089 / 110% / 1.64 dB,   front-right: 65536 / 100% / 0.00 dB
Sink #26125
\tName: epos-eq-processed
\tVolume: front-left: 65536 / 100% / 0.00 dB,   front-right: 65536 / 100% / 0.00 dB
";
        assert_eq!(
            over_cap_in(VolumeKind::Sink, listing),
            vec![
                (
                    "alsa_output.usb-Generic_USB_Audio-00.HiFi_7_1__SPDIF__sink".to_string(),
                    150
                ),
                (
                    "alsa_output.usb-Generic_USB_Audio-00.HiFi_7_1__Speaker__sink".to_string(),
                    110
                ),
            ],
            "both over-ceiling devices must be found, whatever is playing"
        );
    }

    /// A sink exactly at the ceiling is left alone: a ceiling, not a pin.
    ///
    /// `>=` instead of `>` here is the mutation that would turn the whole feature
    /// into what the user rejected - every sink dragged up to 100% on every
    /// tick, and no way to turn anything down.
    #[test]
    fn a_sink_at_the_ceiling_is_not_treated_as_over_it() {
        let listing = "\
Sink #0
\tName: at_ceiling
\tVolume: front-left: 65536 / 100% / 0.00 dB,   front-right: 65536 / 100% / 0.00 dB
Sink #1
\tName: below
\tVolume: front-left: 32768 / 50% / -18.06 dB,   front-right: 32768 / 50% / -18.06 dB
";
        assert!(over_cap_in(VolumeKind::Sink, listing).is_empty());
    }

    /// Every over-ceiling sink is written, not only the followed one.
    ///
    /// The exclusion the watcher applies is a `continue` on the followed name,
    /// and this pins the list it filters: the two idle devices over the line must
    /// both be in it, and the quiet one must not.
    #[test]
    fn every_over_ceiling_sink_but_the_followed_one_is_written() {
        let listing = "\
Sink #0
\tName: speakers
\tVolume: front-left: 98304 / 150% / 10.61 dB
Sink #1
\tName: headphones
\tVolume: front-left: 72089 / 110% / 1.64 dB
Sink #2
\tName: playing_now
\tVolume: front-left: 98304 / 130% / 5.00 dB
Sink #3
\tName: quiet_one
\tVolume: front-left: 32768 / 50% / -18.06 dB
";
        let to_write = |followed: &str| -> Vec<String> {
            over_cap_in(VolumeKind::Sink, listing)
                .into_iter()
                .filter(|(n, _)| n != followed)
                .map(|(n, _)| n)
                .collect()
        };
        assert_eq!(
            to_write("playing_now"),
            vec!["speakers".to_string(), "headphones".to_string()],
            "both idle devices over the ceiling must be written"
        );
        assert_eq!(
            to_write("quiet_one"),
            vec![
                "speakers".to_string(),
                "headphones".to_string(),
                "playing_now".to_string(),
            ]
        );
    }

    /// An unreadable volume is skipped, and cannot capture the next sink's name.
    ///
    /// The name is consumed rather than copied so a block whose `Volume:` failed
    /// to parse cannot leave its name behind for the next reading. That is
    /// defensive rather than observable - every block `pactl list sinks` prints
    /// carries its own `Name:` line, which overwrites whatever was held - so
    /// consuming and copying are equivalent on real output, and a mutation
    /// between them is expected to survive. It is written this way so a future
    /// caller that reads the two fields separately cannot reintroduce the leak.
    #[test]
    fn an_unreadable_volume_is_skipped_and_cannot_capture_the_next_sink() {
        let listing = "\
Sink #0
\tName: broken
\tVolume: not-a-number
Sink #1
\tName: over_the_line
\tVolume: front-left: 98304 / 150% / 10.61 dB
";
        assert_eq!(
            over_cap_in(VolumeKind::Sink, listing),
            vec![("over_the_line".to_string(), 150)],
            "the over-ceiling sink must be reported under its own name"
        );
    }

    /// The chain's unaccounted-for stage is held at unity, and only when it is
    /// not already there.
    ///
    /// The watcher runs every second, so writing unconditionally would be a
    /// `pactl` call per second for the life of the daemon.
    #[test]
    fn the_second_stage_of_the_chain_is_held_at_unity() {
        assert_eq!(
            chain_stage_to_apply(26),
            Some(100),
            "the raw EPOS sink was found at 26% -35 dB - with the user's own \
             control at 100%, so the headset was a third of full scale"
        );
        assert_eq!(chain_stage_to_apply(0), Some(100));
        assert_eq!(chain_stage_to_apply(150), Some(100));
        assert_eq!(
            chain_stage_to_apply(100),
            None,
            "already at unity: no write"
        );
    }

    /// A sink above 100% is pulled back, and the daemon says 100.
    ///
    /// This is the cap the user asked for, and it is not the one `apply_volume`
    /// already had. That clamp bounds what the daemon writes; this bounds the
    /// sink. The difference is everything outside the daemon — pavucontrol, the
    /// DE volume applet, `wpctl set-volume`, media keys — and the sink was
    /// measured sitting above 100% more than once while the daemon believed it
    /// was at 100.
    #[test]
    fn a_sink_past_one_hundred_percent_is_pulled_back_to_it() {
        assert_eq!(
            cap_sink_volume(150),
            VolumeVerdict { report: 100, write: Some(100) },
            "150% must be written back as 100, not merely reported as 100"
        );
        // 100 exactly is the ceiling, not a value to correct.
        assert_eq!(
            cap_sink_volume(100),
            VolumeVerdict { report: 100, write: None }
        );
        // Below the ceiling nothing is written: the watcher runs every second
        // and a write per tick on an untouched sink is a pactl call per second.
        assert_eq!(
            cap_sink_volume(68),
            VolumeVerdict { report: 68, write: None }
        );
        // A negative reading is a parsing artefact, not a level.
        assert_eq!(
            cap_sink_volume(-3),
            VolumeVerdict { report: 0, write: None }
        );
    }

    /// A sink the daemon has set before gets its own level back, not whatever
    /// level the previous device left lying around.
    ///
    /// This is the bug as it was measured. Playback on the speakers at 20%, then
    /// the headset: the EPOS sink was found at 24%, adopted, saved into
    /// `device.volume`, and restored there on the next boot, so the headset
    /// stayed at -37 dB whatever the user asked for.
    #[test]
    fn moving_to_a_sink_we_have_set_restores_that_sinks_own_level() {
        // The remembered EPOS level was 100; the EPOS sink was found at 24.
        assert_eq!(
            volume_on_target_change(Some(100), 24),
            VolumeVerdict { report: 100, write: Some(100) },
            "the headset must get its own level back, not the one it was found at"
        );
        // Already there: adopt, and do not write every second.
        assert_eq!(
            volume_on_target_change(Some(100), 100),
            VolumeVerdict { report: 100, write: None }
        );
        // A sink heard for the first time is still adopted, because nothing else
        // knows what a newly plugged device wants.
        assert_eq!(
            volume_on_target_change(None, 20),
            VolumeVerdict { report: 20, write: None }
        );
        // A remembered level over the ceiling comes back capped, not at 150.
        assert_eq!(
            volume_on_target_change(Some(150), 40),
            VolumeVerdict { report: 100, write: Some(100) }
        );
    }

    /// A remembered level is capped on the way back out, so a value that got
    /// past the old read cannot be stored and re-applied on a later session.
    #[test]
    fn a_remembered_level_is_capped_before_it_can_be_written_back() {
        for (remembered, expected) in [(150, 100), (250, 100), (-5, 0), (68, 68)] {
            let v = volume_on_target_change(Some(remembered), 10);
            assert_eq!(
                v.write,
                Some(expected),
                "remembered {remembered}% must be written as {expected}%"
            );
        }
    }

    /// An edited volume is applied, and clamped rather than trusted.
    #[test]
    fn an_edited_volume_is_taken_and_clamped() {
        assert_eq!(external_volume_to_apply(Some(30), Some(45)), Some(45));
        assert_eq!(
            external_volume_to_apply(Some(30), Some(500)),
            Some(100),
            "a hand-edited value must be clamped"
        );
        assert_eq!(external_volume_to_apply(Some(30), Some(-20)), Some(0));
    }

    /// Unchanged, or no longer specified: nothing to do. Returning a value here
    /// would put the sink to a level nobody asked for.
    #[test]
    fn an_unchanged_or_absent_volume_is_left_alone() {
        assert_eq!(external_volume_to_apply(Some(30), Some(30)), None);
        assert_eq!(external_volume_to_apply(Some(30), None), None);
    }

    /// A value appearing where the file previously had none is a real edit: the
    /// daemon falls back to 100 when the field is absent, so 30 in the file is
    /// the user asking for 30, not a no-op.
    #[test]
    fn a_volume_appearing_where_there_was_none_is_applied() {
        assert_eq!(external_volume_to_apply(None, Some(30)), Some(30));
    }

    /// The silent-wrong-state case. Start-up detection missed the headset, so
    /// `main` never called `set_device` or `apply_full`; the loop's own scan
    /// found it a moment later with `was_connected` already true, which skips
    /// the reconnect branch — the only other place that hands the pipeline a
    /// device. The pipeline was left with `device: None`, routing ran against
    /// fallback node names, and the persisted config was never applied.
    #[test]
    fn a_pipeline_without_a_device_is_initialised_on_first_connect() {
        assert!(first_connect_owes_pipeline(false));
    }

    /// The other half is what stops the fix costing a second full apply on
    /// every normal boot: `main` already did it, so the branch must not.
    #[test]
    fn a_pipeline_that_already_has_a_device_is_left_alone() {
        assert!(!first_connect_owes_pipeline(true));
    }
}