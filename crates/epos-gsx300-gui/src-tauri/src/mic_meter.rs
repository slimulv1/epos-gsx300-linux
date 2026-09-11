use pipewire as pw;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::{Emitter};
use pw::spa::param::format::{MediaSubtype, MediaType};
use pw::spa::param::format_utils;
use pw::spa::pod::Pod;

/// Holds the live PipeWire capture stream worker.
///
/// Unlike the previous `pw-record` subprocess approach, the meter now opens a
/// direct PipeWire capture stream (Audio/Capture) targetting the EPOS source
/// node via `target.object`. No subprocess, no .snd header, no EOF/keep-alive
/// restart churn: the stream dies cleanly on device unplug and is re-created
/// by the next `mic_meter_start` keep-alive tick.
#[derive(Default)]
pub struct MicMeterState {
    /// Set to true by `mic_meter_stop`. The worker's watchdog timer polls it
    /// (~300 ms) and quits the main loop from inside the loop thread, so
    /// `join()` always returns quickly.
    stop: Arc<AtomicBool>,
    /// Join handle of the worker thread (None = not running).
    join: Mutex<Option<std::thread::JoinHandle<()>>>,
}

/// Event payload emitted to the frontend (~33x/s):
/// `db` = instantaneous mono peak RELATIVE to the adaptive noise floor
///        (0 = idle hiss level; a loud shout reaches ~25-30),
/// `peak_db` = relative peak-hold with ~16 dB/s falloff,
/// `clip` = true when absolute signal >= -6 dBFS,
/// `active` = false when no signal present (device unplugged / meter stopped).
#[derive(Clone, Serialize)]
pub struct MicLevel {
    pub db: f32,
    pub peak_db: f32,
    pub clip: bool,
    pub active: bool,
}

/// Raw pointer to the `pw_main_loop`. Only `pw_main_loop_quit` is ever called
/// through it, and only from the main loop thread itself (watchdog timer or
/// unplug handler), so no Send/Sync requirement leaks into the worker state.
#[derive(Clone, Copy)]
struct QuitHandle(*mut pw::sys::pw_main_loop);

impl QuitHandle {
    fn quit(self) {
        unsafe { pw::sys::pw_main_loop_quit(self.0) };
    }
}

/// Find the raw EPOS GSX 300 mic source node via pw-dump.
/// Matches by prefix/suffix (NOT the hardcoded serial) so a device swap
/// still resolves correctly. Returns Ok(None) when the device is absent.
fn resolve_epos_source() -> Result<Option<String>, String> {
    let out = std::process::Command::new("pw-dump")
        .output()
        .map_err(|e| format!("pw-dump failed (pipewire installed?): {e}"))?;
    if !out.status.success() {
        return Ok(None);
    }
    let data: serde_json::Value = serde_json::from_slice(&out.stdout)
        .map_err(|e| format!("pw-dump parse failed: {e}"))?;
    let arr = data.as_array().ok_or("pw-dump: unexpected root (not an array)")?;
    for obj in arr {
        if obj["type"].as_str() != Some("PipeWire:Interface:Node") {
            continue;
        }
        let name = obj["info"]["props"]["node.name"].as_str().unwrap_or("");
        if name.starts_with("alsa_input.usb-Sennheiser_EPOS_GSX_300_")
            && name.ends_with(".mono-fallback")
        {
            return Ok(Some(name.to_string()));
        }
    }
    Ok(None)
}

/// Live-state of the capture stream, shared by the listener callbacks.
/// The stream is set up WITHOUT `RT_PROCESS`, so every callback runs on the
/// main loop thread -> no locking required inside UserData.
struct MeterData {
    app: tauri::AppHandle,
    quit: QuitHandle,
    /// Negotiated stream format (filled by param_changed).
    format: pw::spa::param::audio::AudioInfoRaw,
    /// Noise-floor reference (absolute dBFS). Fixed init from the measured
    /// EPOS GSX 300 preamp hiss (~-33 dBFS chunk-peak). A bootstrap is not
    /// used: the stream may start with digital silence, and a median over an
    /// all-silence window would pin the reference at -60 dB, inflating every
    /// level by ~25 dB -> ring permanently full. A fixed init heals within a
    /// second no matter what arrives first.
    quiet_db: f32,
    /// Relative peak-hold (rise instantly, fall ~0.35 dB per chunk).
    peak_db: f32,
    /// Samples accumulated toward the next 1440-sample (~30 ms) readout.
    pending: u32,
    /// Peak |sample| within the pending window.
    peak: f32,
    /// True once the stream reaches Streaming (used to distinguish a real
    /// device unplug from the initial Unconnected state).
    was_streaming: bool,
}

const CHUNK_SAMPLES: u32 = 1440; // ~30 ms at 48 kHz -> ~33 emits/s

/// Start the mic level meter. Returns:
///   Ok(true)  — meter running (or already running)
///   Ok(false) — EPOS mic not present; caller should retry later
///   Err(msg)  — pw-dump unavailable / stream setup failed
#[tauri::command]
pub async fn mic_meter_start(
    app: tauri::AppHandle,
    state: tauri::State<'_, MicMeterState>,
) -> Result<bool, String> {
    // Idempotent: if the worker is still alive, it is already metering.
    {
        let guard = state.join.lock().map_err(|e| e.to_string())?;
        if let Some(h) = guard.as_ref() {
            if !h.is_finished() {
                return Ok(true);
            }
        }
    }

    let Some(node) = resolve_epos_source()? else {
        return Ok(false);
    };

    state.stop.store(false, Ordering::SeqCst);

    let stop = state.stop.clone();

    let handle = std::thread::spawn(move || run_meter(app, stop, node));

    *state.join.lock().map_err(|e| e.to_string())? = Some(handle);
    Ok(true)
}

/// Worker body: a direct PipeWire capture stream from the EPOS source node.
/// Runs its own main loop; exits via `pw_main_loop_quit` either from the
/// watchdog timer (external stop) or from the unplug/error states.
fn run_meter(app: tauri::AppHandle, stop: Arc<AtomicBool>, node: String) {
    pw::init();

    let mainloop = match pw::main_loop::MainLoopRc::new(None) {
        Ok(ml) => ml,
        Err(_) => return,
    };
    let quit = QuitHandle(mainloop.as_raw_ptr());

    let context = match pw::context::ContextRc::new(&mainloop, None) {
        Ok(c) => c,
        Err(_) => return,
    };
    let core = match context.connect_rc(None) {
        Ok(c) => c,
        Err(_) => return,
    };

    let mut props = pw::properties::properties! {
        *pw::keys::MEDIA_TYPE => "Audio",
        *pw::keys::MEDIA_CATEGORY => "Capture",
        *pw::keys::MEDIA_ROLE => "Communication",
    };
    props.insert(*pw::keys::TARGET_OBJECT, node);

    let stream = match pw::stream::StreamBox::new(&core, "epos-mic-meter", props) {
        Ok(s) => s,
        Err(_) => return,
    };

    let user_data = MeterData {
        app,
        quit,
        format: Default::default(),
        quiet_db: -33.0,
        peak_db: 0.0,
        pending: 0,
        peak: 0.0,
        was_streaming: false,
    };

    let _listener = stream
        .add_local_listener_with_user_data(user_data)
        .state_changed(|_stream, user_data, _old, new| {
            use pw::stream::StreamState;
            match new {
                StreamState::Streaming => user_data.was_streaming = true,
                StreamState::Error(_) => {
                    let _ = user_data
                        .app
                        .emit("mic-level", MicLevel { db: 0.0, peak_db: 0.0, clip: false, active: false });
                    user_data.quit.quit();
                }
                StreamState::Unconnected if user_data.was_streaming => {
                    // Device unplugged after we were streaming.
                    let _ = user_data
                        .app
                        .emit("mic-level", MicLevel { db: 0.0, peak_db: 0.0, clip: false, active: false });
                    user_data.quit.quit();
                }
                _ => {}
            }
        })
        .param_changed(|_stream, user_data, id, param| {
            let Some(param) = param else { return };
            if id != pw::spa::param::ParamType::Format.as_raw() {
                return;
            }
            let (media_type, media_subtype) = match format_utils::parse_format(param) {
                Ok(v) => v,
                Err(_) => return,
            };
            if media_type != MediaType::Audio || media_subtype != MediaSubtype::Raw {
                return;
            }
            let _ = user_data.format.parse(param);
        })
        .process(|stream, user_data| {
            use std::convert::TryInto;
            let Some(mut buffer) = stream.dequeue_buffer() else {
                return;
            };
            let datas = buffer.datas_mut();
            if datas.is_empty() {
                return;
            }
            let data = &mut datas[0];
            let chunk_size = data.chunk().size() as usize;
            let Some(samples) = data.data() else { return };
            let n_bytes = samples.len().min(chunk_size);
            let n_f32 = n_bytes / 4;
            for i in 0..n_f32 {
                let off = i * 4;
                let a = f32::from_le_bytes(
                    [samples[off], samples[off + 1], samples[off + 2], samples[off + 3]]
                        .try_into()
                        .unwrap(),
                )
                .abs();
                if a > user_data.peak {
                    user_data.peak = a;
                }
                user_data.pending += 1;
                if user_data.pending >= CHUNK_SAMPLES {
                    let mut p = user_data.peak;
                    if p == 0.0 {
                        p = 1e-9;
                    }
                    let db_full = (20.0 * p.log10()).max(-60.0); // absolute dBFS
                    // Adaptive noise-floor follower (both directions): only
                    // chases while the input sits within 8 dB of the reference
                    // (idle hiss fluctuation). Real audio sits > 8 dB above the
                    // floor and must never drag the reference upward.
                    if (db_full - user_data.quiet_db).abs() < 8.0 {
                        user_data.quiet_db += (db_full - user_data.quiet_db) * 0.01; // ~3 s
                    }
                    let db = (db_full - user_data.quiet_db).max(0.0);
                    // Peak-hold: rise instantly, fall ~16 dB/s (0.35 dB per chunk).
                    if db >= user_data.peak_db {
                        user_data.peak_db = db;
                    } else {
                        user_data.peak_db = (user_data.peak_db - 0.35).max(db);
                    }
                    let clip = db_full >= -6.0;
                    let _ = user_data.app.emit(
                        "mic-level",
                        MicLevel {
                            db,
                            peak_db: user_data.peak_db,
                            clip,
                            active: true,
                        },
                    );
                    user_data.pending = 0;
                    user_data.peak = 0.0;
                }
            }
        })
        .register()
        .ok();

    // Request raw mono f32 at the graph rate (leave rate/channels empty so
    // PipeWire negotiates; 48 kHz is the EPOS native rate).
    let mut audio_info = pw::spa::param::audio::AudioInfoRaw::new();
    audio_info.set_format(pw::spa::param::audio::AudioFormat::F32LE);
    let obj = pw::spa::pod::Object {
        type_: pw::spa::utils::SpaTypes::ObjectParamFormat.as_raw(),
        id: pw::spa::param::ParamType::EnumFormat.as_raw(),
        properties: audio_info.into(),
    };
    let values: Vec<u8> = pw::spa::pod::serialize::PodSerializer::serialize(
        std::io::Cursor::new(Vec::new()),
        &pw::spa::pod::Value::Object(obj),
    )
    .unwrap()
    .0
    .into_inner();
    let mut params = [Pod::from_bytes(&values).unwrap()];

    let _ = stream.connect(
        pw::spa::utils::Direction::Input,
        None,
        pw::stream::StreamFlags::AUTOCONNECT | pw::stream::StreamFlags::MAP_BUFFERS,
        &mut params,
    );

    // Watchdog: polls the stop flag every 300 ms and quits from this (loop)
    // thread, so the external stop path never races with the loop internals.
    let _stop = stop;
    let quit2 = quit;
    let _timer = mainloop.loop_().add_timer(move |_expirations| {
        if _stop.load(Ordering::SeqCst) {
            quit2.quit();
        }
    });
    let _ = _timer.update_timer(
        Some(std::time::Duration::from_millis(300)),
        Some(std::time::Duration::from_millis(300)),
    );

    mainloop.run();
}

/// Stop the mic level meter (quits the main loop via the watchdog; join <= ~300 ms).
#[tauri::command]
pub async fn mic_meter_stop(state: tauri::State<'_, MicMeterState>) -> Result<(), String> {
    state.stop.store(true, Ordering::SeqCst);
    if let Some(h) = state.join.lock().map_err(|e| e.to_string())?.take() {
        let _ = h.join();
    }
    Ok(())
}