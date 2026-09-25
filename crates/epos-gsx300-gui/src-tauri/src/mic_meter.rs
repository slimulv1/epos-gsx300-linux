use pipewire as pw;
use pw::spa::param::format::{MediaSubtype, MediaType};
use pw::spa::param::format_utils;
use pw::spa::pod::Pod;
use serde::Serialize;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use tauri::Emitter;

/// Holds the live PipeWire capture stream worker.
///
/// Direct PipeWire capture stream (Audio/Capture) targetting the EPOS source
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
///
/// All levels are computed from the chunk **RMS** (not peak) — the EPOS
/// CX21988 preamp delivers a *non-stationary* hiss (chunk peaks wander from
/// -48 dBFS to -9 dBFS with **no mic attached**), so peak-based metering
/// happily reports "audio" where there is only electronics noise. RMS with a
/// noise-center floor makes real sound the only thing that moves the bar.
///
/// Detecting that real sound is a two-tier decision per 30 ms chunk:
///   1. *level* gate — chunk-RMS must sit >= noise floor + `SIG_GATE_DB`, and
///   2. *spectral* gate — the chunk's sub-band spectral flatness (SFM) must
///      be <= `SFM_GATE`. Real voice is harmonically structured (SFM
///      ~0.32-0.44 at ANY gain), while the GSX 300 preamp hiss is broad/flat
///      (SFM ~0.56+). Only chunks passing BOTH may feed the confidence
///      integrator. Loud broadband audio (claps, music, shouts — relative
///      level reaching `LOUD_BYPASS_DB`) bypasses the spectral gate: loud is
///      real even when it is not voice.
///
/// `db` = chunk-RMS relative to the adaptive noise floor, gated: 0 until a
/// real signal has been confirmed (see `ENERGY_ENGAGE`), so a silent mic is
/// reported as `active:false`/`db:0` — never a fake reading.
/// `peak_db` = relative peak-hold of `db` with ~16 dB/s falloff,
/// `clip` = true only when **confirmed audio** reaches >= -6 dBFS (absolute),
/// `active` = false when no real signal is present (device unplugged, meter
/// stopped, OR the mic is connected but truly silent — no hiss = no bar).
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

/// How long `pw-dump` may take before we stop waiting for it.
const PW_DUMP_BUDGET: std::time::Duration = std::time::Duration::from_secs(2);

/// Find the raw EPOS GSX 300 mic source node via pw-dump.
/// Matches by prefix/suffix (NOT the hardcoded serial) so a device swap
/// still resolves correctly. Returns Ok(None) when the device is absent.
async fn resolve_epos_source() -> Result<Option<String>, String> {
    resolve_source_with("pw-dump", PW_DUMP_BUDGET).await
}

/// [`resolve_epos_source`] with the program and the budget as arguments.
///
/// The two are parameters so the test can stand in a program that never exits.
/// That is the case this exists for, and it is the one a real `pw-dump` cannot
/// be asked to reproduce on demand: it blocks the calling thread until the
/// process exits, and nothing in this file bounded that. The frontend calls
/// `mic_meter_start` every 2 s, which returns early while the worker is alive
/// and so only reaches here on the reconnect path - exactly when PipeWire is
/// most likely to be the thing that is stuck.
///
/// `kill_on_drop` is what makes the budget mean anything: without it, timing
/// out would drop the future and leave the child running. tokio's
/// `ChildDropGuard` kills on drop when the flag is set, so the timeout takes
/// the process with it. Reaping afterwards is tokio's best effort, per its own
/// documentation, which is why the budget is long enough that this is a rare
/// path rather than the common one.
async fn resolve_source_with(
    program: &str,
    budget: std::time::Duration,
) -> Result<Option<String>, String> {
    let out = match tokio::time::timeout(
        budget,
        tokio::process::Command::new(program).kill_on_drop(true).output(),
    )
    .await
    {
        Ok(Ok(out)) => out,
        Ok(Err(e)) => return Err(format!("{program} failed (pipewire installed?): {e}")),
        Err(_) => return Err(format!("{program} did not answer within {budget:?}")),
    };
    if !out.status.success() {
        return Ok(None);
    }
    let data: serde_json::Value = serde_json::from_slice(&out.stdout)
        .map_err(|e| format!("{program} parse failed: {e}"))?;
    let arr = data
        .as_array()
        .ok_or(format!("{program}: unexpected root (not an array)"))?;
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

// ─── Metering algorithm constants (validated against the real device) ───
//
// Measured on hardware (2026-09-21), EPOS GSX 300 card, capture `hw:4,0`
// S16_LE mono 48 kHz, NO microphone attached:
//
//   * startup: a burst of digital ADC-settling clicks in the first ~20
//     samples (up to -0.1 dBFS) — must be swallowed, see SETTLE_CHUNKS.
//   * steady state: non-stationary preamp hiss. 30 ms chunk RMS is well
//     behaved (median -37.3, P95 -29.5, max -25.2 dBFS) but isolated
//     transients reach -6.8 dBFS peak — hence a *sustained* gate, never a
//     peak gate, and never an instantaneous decision.
//   * spectrum: the hiss is broad/flat (sub-band spectral flatness SFM
//     ~0.56 median, range 0.47..0.73), while voiced speech — a fundamental
//     with harmonics — reads SFM ~0.32..0.44 at EVERY level from -20 to -44
//     dBFS. SFM is gain-invariant: it separates voice from noise
//     independently of the mic-gain knob.
//
// A chunk is candidate audio when it clears BOTH independent gates —
//   rel >= SIG_GATE_DB (level)  AND  sfm <= SFM_GATE (shape)
// — or when it is simply too loud to be noise:
//   rel >= LOUD_BYPASS_DB (loud broadband audio is still real audio).
// On the 14 s real no-mic recording the two-gate condition fires for at most
// 3 *consecutive* chunks (the level-only gate fired for runs up to 13), so a
// silent mic can never accumulate the 8-chunk engagement.
//
// The floor is the P60 (noise center) of a rolling ~3 s window that only
// ever contains noise-class chunks (see the `!candidate` push condition).
// Real audio therefore can never drag the floor upward; idle hiss keeps the
// floor pegged to the noise center. A silent mic reports active=false
// forever.

const CHUNK_SAMPLES: u32 = 1440; // ~30 ms at 48 kHz -> ~33 emits/s

/// Zero-padded window size for the per-chunk spectral-flatness analysis.
const FFT_BINS: usize = 2048;

/// Chunks skipped right after the stream enters `Streaming` (~240 ms).
/// Swallows the ADC-settling click burst measured at stream start.
const SETTLE_CHUNKS: u32 = 8;

/// Rolling noise-only window length (~3 s of chunks) for the floor estimate.
const FLOOR_WINDOW: usize = 96;
/// The floor becomes live after this many noise chunks (~1 s) so speech
/// right after launch is still measured against a real noise center.
const FLOOR_MIN_SAMPLES: usize = 32;
/// Noise center percentile of the window — immune to rare transients.
const FLOOR_PERCENTILE: f32 = 0.60;
/// Placeholder floor used until the window primes (never emitted).
const FLOOR_INIT: f32 = -33.0;

/// Level gate (dB above the floor). Lower + SFM guard than the old plain
/// 8 dB gate: identical no-false-positive proof, better soft-speech pickup.
const SIG_GATE_DB: f32 = 6.0;

/// Spectral-flatness gate. Voice ~0.32-0.44 (all levels), hiss ~0.56+,
/// clicks ~0.73. 0.50 sits between the two populations.
const SFM_GATE: f32 = 0.50;

/// Loud broadband audio bypass (dB above the floor): sustained at/past this
/// level engages regardless of spectral shape — loud is real, not hiss.
const LOUD_BYPASS_DB: f32 = 15.0;

/// Signal-confidence integrator. Each candidate chunk adds +1 (cap 24),
/// each non-candidate chunk subtracts ENERGY_DECAY. `active` engages when the
/// energy crosses ENERGY_ENGAGE (~240 ms of sustained audio) and releases
/// when it drops below ENERGY_RELEASE (~0.6 s of real silence).
const ENERGY_ENGAGE: f32 = 8.0;
const ENERGY_RELEASE: f32 = 2.0;
const ENERGY_DECAY: f32 = 1.0;
const ENERGY_CAP: f32 = 24.0;

/// Display peak-hold falloff (~16 dB/s, 0.35 dB per chunk).
const PEAK_FALL_PER_CHUNK: f32 = 0.35;
/// Absolute chunk-peak (not RMS) above which a confirmed signal reads CLIP.
const CLIP_DBFS: f32 = -6.0;

/// Live-state of the capture stream, shared by the listener callbacks.
/// The stream is set up WITHOUT `RT_PROCESS`, so every callback runs on the
/// main loop thread -> no locking required inside UserData.
struct MeterData {
    app: tauri::AppHandle,
    quit: QuitHandle,
    /// Negotiated stream format (filled by param_changed).
    format: pw::spa::param::audio::AudioInfoRaw,
    /// Samples accumulated toward the next CHUNK_SAMPLES readout.
    pending: u32,
    /// Peak |sample| within the pending window (CLIP / absolute checks).
    peak: f32,
    /// Sum of squared samples within the pending window (chunk RMS).
    rms_acc: f64,
    /// Last CHUNK_SAMPLES samples (the in-flight window) — the input for the
    /// per-chunk spectral-flatness analysis.
    window: VecDeque<f32>,
    /// Pure signal-processing core (see `MeterCore`).
    core: MeterCore,
}

/// Pure metering state — deliberately free of any Tauri dependency so tests
/// can replay recorded device audio straight through `classify`.
struct MeterCore {
    /// Adaptive noise-center floor (dBFS), P60 of the noise-only window.
    floor_db: f32,
    /// Rolling noise-only chunk-RMS dBFS values (size FLOOR_WINDOW).
    floor_buf: VecDeque<f32>,
    /// Chunks remaining in the post-Streaming settle period.
    settle: u32,
    /// Leaky signal-confidence integrator (see ENERGY_* constants).
    energy: f32,
    /// True once real audio has been confirmed; the "sound present" flag.
    active: bool,
    /// Relative peak-hold of `db` (rise instantly, fall ~0.35 dB/chunk).
    peak_db: f32,
    /// True once the stream reaches Streaming (used to distinguish a real
    /// device unplug from the initial Unconnected state).
    was_streaming: bool,
}

impl Default for MeterCore {
    fn default() -> Self {
        Self {
            floor_db: FLOOR_INIT,
            floor_buf: VecDeque::with_capacity(FLOOR_WINDOW),
            settle: SETTLE_CHUNKS,
            energy: 0.0,
            active: false,
            peak_db: 0.0,
            was_streaming: false,
        }
    }
}

impl MeterCore {
    /// Recompute the noise-center floor from the noise-only window.
    /// Live after FLOOR_MIN_SAMPLES (~1 s); the estimate keeps improving
    /// until the window is full.
    fn refresh_floor(&mut self) {
        if self.floor_buf.len() < FLOOR_MIN_SAMPLES {
            return;
        }
        let mut sorted: Vec<f32> = self.floor_buf.iter().copied().collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let idx = ((sorted.len() - 1) as f32 * FLOOR_PERCENTILE) as usize;
        self.floor_db = sorted[idx];
    }

    /// One finalized 30 ms chunk. Pure decision core (no Tauri dependency)
    /// so regression tests can replay recorded device audio through it.
    /// Returns `None` while the post-Streaming settle window is still open,
    /// otherwise the level payload to emit.
    /// `db_rms` / `db_peak` are the finalized window's RMS and peak in dBFS;
    /// `sfm` is the window's sub-band spectral flatness (0 = harmonic tone,
    /// 1 = flat noise), see `spectral_flatness`.
    fn classify(&mut self, db_rms: f32, db_peak: f32, sfm: f32) -> Option<MicLevel> {
        // Post-Streaming settle: ignore the ADC-settling click burst.
        if self.settle > 0 {
            self.settle -= 1;
            return None;
        }

        let rel = db_rms - self.floor_db;

        // Candidate audio = above the level gate AND spectrally voice-like —
        // or simply too loud to be noise (claps, music, shouting). Hiss can
        // clear the level gate transiently, but never both gates for long.
        let candidate = rel >= LOUD_BYPASS_DB || (rel >= SIG_GATE_DB && sfm <= SFM_GATE);

        if !self.active {
            // Only noise-class chunks may feed the floor estimate. Candidate
            // chunks (and chunks hugging the gate) are NEVER pushed into the
            // floor window, so real speech cannot raise the noise floor over
            // the ~240 ms lead-in.
            if !candidate && rel < SIG_GATE_DB + 2.0 {
                self.floor_buf.push_back(db_rms);
                if self.floor_buf.len() > FLOOR_WINDOW {
                    self.floor_buf.pop_front();
                }
                self.refresh_floor();
            }
            self.energy = if candidate {
                (self.energy + 1.0).min(ENERGY_CAP)
            } else {
                (self.energy - ENERGY_DECAY).max(0.0)
            };
            if self.energy >= ENERGY_ENGAGE {
                self.active = true;
            }
        } else {
            // Confirmed audio: floor frozen, keep confidence alive across
            // normal syllabic pauses, release after real silence.
            self.energy = if candidate {
                (self.energy + 1.0).min(ENERGY_CAP)
            } else {
                (self.energy - ENERGY_DECAY).max(0.0)
            };
            if self.energy < ENERGY_RELEASE {
                self.active = false;
                self.energy = 0.0;
                self.peak_db = 0.0;
                // Re-prime the floor from scratch so the next silence read
                // starts from a fresh noise center, never a stale frozen one.
                self.floor_buf.clear();
            }
        }

        let db = if self.active { rel.max(0.0) } else { 0.0 };
        if self.active {
            // Peak-hold: rise instantly, fall ~16 dB/s (0.35 dB per chunk).
            if db >= self.peak_db {
                self.peak_db = db;
            } else {
                self.peak_db = (self.peak_db - PEAK_FALL_PER_CHUNK).max(db);
            }
        }

        // CLIP only counts against *confirmed* audio — idle preamp transients
        // must never flash the clip LED.
        let clip = self.active && db_peak >= CLIP_DBFS;

        Some(MicLevel {
            db,
            peak_db: self.peak_db,
            clip,
            active: self.active,
        })
    }
}

// ─── Sub-band spectral flatness (SFM) ─────────────────────────────────────
//
// Spectral flatness per 30 ms chunk: for each of 12 log-spaced sub-bands
// from 100 Hz to 8 kHz compute the ratio geometric-mean / arithmetic-mean of
// the power spectrum, then average those ratios. Flat/broadband noise scores
// high (~0.5-0.7), harmonically structured voice scores low (~0.3-0.45), and
// because both means scale identically the measure is invariant to absolute
// level — one threshold (`SFM_GATE`) works at every mic-gain setting.
// See the algorithm block above for the measured device populations.

/// 13 log-spaced band edges, 100 Hz..8 kHz (np.geomspace(100, 8000, 13)).
fn band_edges() -> &'static [f32; 13] {
    static EDGES: OnceLock<[f32; 13]> = OnceLock::new();
    EDGES.get_or_init(|| {
        let mut e = [0.0_f32; 13];
        for (i, v) in e.iter_mut().enumerate() {
            *v = 100.0 * (80.0_f32).powf(i as f32 / 12.0);
        }
        e
    })
}

/// In-place iterative radix-2 complex FFT (Cooley–Tukey, bit-reversed input).
fn fft_radix2(re: &mut [f32], im: &mut [f32]) {
    let n = re.len();
    debug_assert!(n.is_power_of_two());
    debug_assert_eq!(im.len(), n);
    let shift = usize::BITS - n.trailing_zeros();
    for i in 0..n {
        let j = i.reverse_bits() >> shift;
        if j > i {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= n {
        let ang = -2.0 * std::f32::consts::PI / len as f32;
        let (wr, wi) = (ang.cos(), ang.sin());
        let half = len / 2;
        for start in (0..n).step_by(len) {
            let (mut cr, mut ci) = (1.0_f32, 0.0_f32);
            for k in 0..half {
                let (ar, ai) = (re[start + k], im[start + k]);
                let (br, bi) = (re[start + k + half], im[start + k + half]);
                let (tr, ti) = (br * cr - bi * ci, br * ci + bi * cr);
                re[start + k] = ar + tr;
                im[start + k] = ai + ti;
                re[start + k + half] = ar - tr;
                im[start + k + half] = ai - ti;
                let nc = cr * wr - ci * wi;
                ci = cr * wi + ci * wr;
                cr = nc;
            }
        }
        len *= 2;
    }
}

/// Sub-band spectral flatness of a chunk (first FFT_BINS samples are used,
/// shorter windows are zero-padded). 0 ≈ single tone, 1 ≈ perfectly flat
/// noise. Digital silence (all sub-band power <= 1e-15) → 1.0, i.e. "definitely
/// noise" — the safe direction for a no-false-positive meter.
fn spectral_flatness<I: IntoIterator<Item = f32>>(samples: I) -> f32 {
    let mut re = [0.0_f32; FFT_BINS];
    let mut im = [0.0_f32; FFT_BINS];
    for (n, v) in samples.into_iter().enumerate() {
        if n >= FFT_BINS {
            break;
        }
        re[n] = v;
    }
    fft_radix2(&mut re, &mut im);

    const BANDS: usize = 12;
    let edges = band_edges();
    let (mut ln_sum, mut am_sum, mut counts) = ([0.0_f64; BANDS], [0.0_f64; BANDS], [0usize; BANDS]);
    let fs = 48000.0_f32;
    // Only the positive-frequency half is needed (input is real).
    for k in 0..=FFT_BINS / 2 {
        let freq = k as f32 * fs / FFT_BINS as f32;
        let p = (re[k] * re[k] + im[k] * im[k]) as f64;
        if p <= 1e-15 || freq < edges[0] || freq >= edges[BANDS] {
            continue;
        }
        for b in 0..BANDS {
            if freq >= edges[b] && freq < edges[b + 1] {
                ln_sum[b] += p.ln();
                am_sum[b] += p;
                counts[b] += 1;
                break;
            }
        }
    }

    // SFM = exp(mean(ln(gm/am + 1e-9))) across populated bands.
    let mut s = 0.0_f64;
    let mut n_bands = 0usize;
    for b in 0..BANDS {
        if counts[b] > 0 {
            let gm = (ln_sum[b] / counts[b] as f64).exp();
            let am = am_sum[b] / counts[b] as f64;
            s += (gm / (am + 1e-18) + 1e-9).ln();
            n_bands += 1;
        }
    }
    if n_bands == 0 {
        return 1.0;
    }
    (s / n_bands as f64).exp() as f32
}

impl MeterData {
    /// 20·log10 of the chunk RMS (dBFS), clamped so digital silence reads -90.
    fn chunk_rms_db(&self) -> f32 {
        let rms = (self.rms_acc / CHUNK_SAMPLES as f64).sqrt();
        let rms = rms.max(f64::from(f32::EPSILON)); // silence -> -90 dBFS
        (20.0 * rms.log10()).max(-90.0) as f32
    }

    /// Absolute peak of the pending window in dBFS (-90 clamp for silence).
    fn chunk_peak_db(&self) -> f32 {
        let p = self.peak.max(f32::EPSILON);
        (20.0 * p.log10()).max(-90.0)
    }

    /// Finalize a 30 ms window: RMS + peak from the accumulators, run the
    /// spectral-flatness gate, classify, and emit the payload.
    fn finalize_chunk(&mut self) {
        let db_rms = self.chunk_rms_db();
        let db_peak = self.chunk_peak_db();
        let sfm = spectral_flatness(self.window.iter().copied());
        if let Some(level) = self.core.classify(db_rms, db_peak, sfm) {
            let _ = self.app.emit("mic-level", level);
        }
        self.pending = 0;
        self.peak = 0.0;
        self.rms_acc = 0.0;
    }
}

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

    let Some(node) = resolve_epos_source().await? else {
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
        pending: 0,
        peak: 0.0,
        rms_acc: 0.0,
        window: VecDeque::new(),
        core: MeterCore::default(),
    };

    let _listener = stream
        .add_local_listener_with_user_data(user_data)
        .state_changed(|_stream, user_data, _old, new| {
            use pw::stream::StreamState;
            match new {
                StreamState::Streaming => {
                    user_data.core.was_streaming = true;
                    // Fresh stream = fresh start: re-arm the settle window so
                    // the ADC click burst at the head of every new stream is
                    // swallowed (device replug, tab reopen, etc).
                    user_data.core.settle = SETTLE_CHUNKS;
                }
                StreamState::Error(_) => {
                    let _ = user_data.app.emit(
                        "mic-level",
                        MicLevel {
                            db: 0.0,
                            peak_db: 0.0,
                            clip: false,
                            active: false,
                        },
                    );
                    user_data.quit.quit();
                }
                StreamState::Unconnected if user_data.core.was_streaming => {
                    // Device unplugged after we were streaming.
                    let _ = user_data.app.emit(
                        "mic-level",
                        MicLevel {
                            db: 0.0,
                            peak_db: 0.0,
                            clip: false,
                            active: false,
                        },
                    );
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
                let s = f32::from_le_bytes([
                    samples[off],
                    samples[off + 1],
                    samples[off + 2],
                    samples[off + 3],
                ]);
                let a = s.abs();
                if a > user_data.peak {
                    user_data.peak = a;
                }
                user_data.rms_acc += f64::from(s) * f64::from(s);
                user_data.window.push_back(s);
                if user_data.window.len() > CHUNK_SAMPLES as usize {
                    user_data.window.pop_front();
                }
                user_data.pending += 1;
                if user_data.pending >= CHUNK_SAMPLES {
                    user_data.finalize_chunk();
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
    let Some(h) = state.join.lock().map_err(|e| e.to_string())?.take() else {
        return Ok(());
    };
    // `join` blocks until the watchdog notices the flag and quits the loop, so
    // up to one tick - ~300 ms by the comment above. Waiting on it here would
    // hold a runtime worker for all of that, so the waiting goes to the
    // blocking pool. The handle moves in, so the lock is not held across the
    // wait either.
    let _ = tokio::task::spawn_blocking(move || h.join()).await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A stand-in program on disk, so a test can make `resolve_source_with`
    /// run something other than `pw-dump` without touching the environment the
    /// other tests in this binary share.
    fn script_program(name: &str, body: &str) -> String {
        use std::os::unix::fs::PermissionsExt;
        let path = std::env::temp_dir().join(format!("epos-gui-{}-{name}", std::process::id()));
        std::fs::write(&path, body).expect("write script");
        let mut perms = std::fs::metadata(&path).expect("stat script").permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&path, perms).expect("chmod script");
        // No cleanup here: the path is the thing the test then runs, so
        // deleting it on the way out defeats the purpose. Each test removes its
        // own script when it is done.
        path.to_str().expect("utf-8 temp path").to_string()
    }

    /// A program that never exits, which is what a wedged `pw-dump` looks like.
    ///
    /// The name is an argument because two tests need one of these and they run
    /// in parallel: a shared filename means one test rewriting a script the
    /// other is executing, which fails with `ETXTBSY` — a race that shows up as
    /// a flaky test rather than as anything to do with the code under test. The
    /// daemon's equivalent needed retry loops for the same reason; distinct
    /// names need none.
    /// Run a script program, retrying the one failure a freshly written script
    /// produces: `ETXTBSY`.
    ///
    /// Measured, not hypothesised: this failed 1 run in 30 with
    /// `Text file busy (os error 26)`, and the failing test moved between runs,
    /// which is what a resource race looks like rather than a logic error. The
    /// kernel returns it from `exec` when something still holds the file open
    /// for writing, and a script written microseconds earlier is exactly that
    /// situation under a loaded machine.
    ///
    /// The daemon already has this: `devices::capped_dump` retries eight times
    /// for the same fault, in the same words, because it hit the same race. A
    /// test helper that writes a script and immediately runs it has the same
    /// exposure and was missing the same answer.
    async fn run_script(
        program: &str,
        budget: std::time::Duration,
    ) -> Result<Option<String>, String> {
        const ATTEMPTS: u32 = 8;
        let mut last = String::new();
        for attempt in 0..ATTEMPTS {
            match resolve_source_with(program, budget).await {
                Err(why)
                    if why.contains("Text file busy") || why.contains("os error 26") =>
                {
                    last = why;
                    if attempt + 1 == ATTEMPTS {
                        break;
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(
                        20 * (attempt as u64 + 1),
                    ))
                    .await;
                }
                other => return other,
            }
        }
        Err(last)
    }

    /// `exec`, and that is the whole point of the line.
    ///
    /// A plain `sleep 600` makes the shell fork a child, and killing the shell
    /// on timeout leaves that child running. 25 runs of this suite left 239
    /// orphaned `sleep 600` processes behind, which is what finally made a 10 s
    /// budget flaky — the machine got slower as the suite leaked. `exec`
    /// replaces the shell, so there is one process and killing it kills the
    /// sleep. It also means the test that polls `/proc` for the script path is
    /// now looking at the process that actually does the waiting, instead of at
    /// a shell that has already exited.
    fn never_exits(name: &str) -> String {
        script_program(name, "#!/bin/sh\nexec sleep 600\n")
    }

    /// No process with this path in its command line, as far as `/proc` can see.
    fn process_alive(marker: &str) -> bool {
        let Ok(entries) = std::fs::read_dir("/proc") else {
            return false;
        };
        entries.flatten().any(|entry| {
            std::fs::read(entry.path().join("cmdline"))
                .map(|cmdline| String::from_utf8_lossy(&cmdline).contains(marker))
                .unwrap_or(false)
        })
    }

    /// Giving up on the dump takes the process with it.
    ///
    /// Without `kill_on_drop`, timing out drops the future and leaves the child
    /// running — so a wedged `pw-dump` costs a worker for its budget *and*
    /// leaves a process behind, every 2 s, for as long as it stays wedged. The
    /// budget test above cannot see this: it only checks that the wait ended.
    ///
    /// Read out of `/proc` rather than by running `pgrep`, because a killed
    /// child stays visible as a zombie until it is reaped, and tokio reaps
    /// best-effort in the background. Polling for a couple of seconds is what
    /// makes this about "the child is gone" instead of "the child was killed
    /// and immediately reaped".
    #[tokio::test]
    async fn abandoning_a_dump_also_takes_the_process_with_it() {
        let program = never_exits("killed");
        let result =
            run_script(&program, std::time::Duration::from_millis(200)).await;
        assert!(result.is_err(), "the dump never answers, so this must time out");

        let mut still_running = true;
        for _ in 0..40 {
            if !process_alive(&program) {
                still_running = false;
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
        let _ = std::fs::remove_file(&program);
        assert!(
            !still_running,
            "the child outlived the budget; a wedged dump would be leaked every \
             2 s for as long as it stayed wedged"
        );
    }

    /// Waiting for the worker thread must not hold a runtime worker.
    ///
    /// `mic_meter_stop` joins the meter thread, which takes up to one watchdog
    /// tick (~300 ms). Joining it inline held a worker for all of that, and
    /// `#[tokio::test]` is a current-thread runtime — so if the join blocks,
    /// nothing else can run.
    ///
    /// The measurement is *during* the join, and that is the whole difficulty.
    /// The first version of this test spawned a fixed 20-tick ticker and
    /// compared the final count, and it passed with the join inlined — because
    /// `tokio::spawn` does not poll a task until the current one yields, so an
    /// inline join ran before the ticker ever started, and the ticker then
    /// completed its course afterwards either way. A test that passes with the
    /// bug still in it is worse than no test, so the count is sampled before
    /// and after, with a yield in between to make sure the ticker is genuinely
    /// running first.
    /// `mic_meter_stop` waits off the worker, at the call site and not just in
    /// the pattern.
    ///
    /// The test below proves that `spawn_blocking` around a join does not hold a
    /// runtime worker. It says nothing about whether `mic_meter_stop` uses it,
    /// and the original defect lived at the call site - which is exactly where
    /// the `find_hidraw` substring check lived in the backend, and why that one
    /// needed a guard test too. Pinned by text over the production half of the
    /// file only, because this test quotes the very expression it looks for and
    /// would otherwise match itself.
    #[test]
    fn stopping_the_meter_joins_off_the_worker() {
        let needle = concat!("spawn_blocking(move || h", ".join())");
        let source = include_str!("mic_meter.rs");
        let production = match source.split_once("#[cfg(test)]") {
            Some((before, _tests)) => before,
            None => panic!("the test module anchor moved; update this test"),
        };
        assert!(
            production.contains(needle),
            "mic_meter_stop must join the meter thread on the blocking pool: the \
             wait is up to one watchdog tick, and the test below shows what an \
             inline join costs a current-thread runtime"
        );
    }

    #[tokio::test]
    async fn waiting_for_the_worker_does_not_hold_the_runtime() {
        let handle =
            std::thread::spawn(|| std::thread::sleep(std::time::Duration::from_millis(400)));
        let ticks = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let counter = std::sync::Arc::clone(&ticks);
        let ticker = tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(20)).await;
                counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });

        // Yield, so the ticker is actually running before the interesting part.
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        let before = ticks.load(std::sync::atomic::Ordering::Relaxed);
        assert!(before >= 3, "the ticker never got going: {before} ticks");

        // Exactly what `mic_meter_stop` does now.
        let _ = tokio::task::spawn_blocking(move || handle.join()).await;

        let after = ticks.load(std::sync::atomic::Ordering::Relaxed);
        ticker.abort();
        assert!(
            after >= before + 10,
            "only {} ticks ran during a 400 ms join ({before} -> {after}), so the \
             join is holding the only worker",
            after - before
        );
    }


    ///
    /// Before this, the dump was read with `std::process::Command::output()`,
    /// which waits for the process to exit and has no way to be told otherwise.
    /// The frontend calls `mic_meter_start` every 2 s, so one hung `pw-dump`
    /// held a runtime worker for as long as the child lived — unbounded — and
    /// a second one took another worker with it.
    ///
    /// This is the case a real `pw-dump` cannot be asked to reproduce on
    /// demand, which is why the program and the budget are parameters.
    #[tokio::test]
    async fn a_dump_that_never_ends_is_abandoned_at_its_budget() {
        let program = never_exits("abandoned");
        let started = std::time::Instant::now();
        let result =
            run_script(&program, std::time::Duration::from_millis(200)).await;
        let elapsed = started.elapsed();

        assert!(
            matches!(&result, Err(why) if why.contains("did not answer within")),
            "a hung dump must be reported as unanswered, got {result:?}"
        );
        assert!(
            elapsed < std::time::Duration::from_secs(5),
            "must give up near its budget, took {elapsed:?}"
        );
        // The script is named by pid, so a run that leaves it behind leaves a
        // file that no later run will ever reuse. 30 runs of this suite left 32
        // of them in /tmp.
        let _ = std::fs::remove_file(&program);
    }

    /// The budget is not a promise to fail: a program that answers is read.
    ///
    /// The anti-vacuity test. A `resolve_source_with` that rejected everything
    /// would satisfy the one above, and the meter would simply never start.
    #[tokio::test]
    async fn a_dump_that_answers_is_still_parsed() {
        let program = script_program(
            "answer",
            r#"#!/bin/sh
cat <<'JSON'
[
  {"type":"PipeWire:Interface:Node",
   "info":{"props":{"node.name":"alsa_input.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.mono-fallback"}}}
]
JSON
"#,
        );
        let found = run_script(&program, std::time::Duration::from_secs(10))
            .await
            .expect("the dump answered")
            .expect("the node is in there");
        assert_eq!(
            found, "alsa_input.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.mono-fallback"
        );
        let _ = std::fs::remove_file(&program);
    }

    /// A program that exits non-zero means "no device", not "error".
    ///
    /// Pinned because the timeout change moved the status check, and a
    /// non-zero exit is how a missing device shows up rather than an empty
    /// listing.
    #[tokio::test]
    async fn a_dump_that_exits_non_zero_is_an_absent_device_not_a_failure() {
        let program = script_program("fail", "#!/bin/sh\nexit 1\n");
        let found = run_script(&program, std::time::Duration::from_secs(10))
            .await
            .expect("a non-zero exit is an answer, not an error");
        assert!(found.is_none(), "nothing was dumped, so nothing was found");
        let _ = std::fs::remove_file(&program);
    }

    /// Replay helper: feed raw S16_LE mono 48 kHz samples into the meter's
    /// decision core (classify) one 30 ms window at a time, the same way the
    /// live PipeWire callback accumulates them.
    /// Replay one 30 ms window at a time through `classify` (the same training
    /// the live PipeWire callback uses: RMS + peak accumulators, then the
    /// spectral-flatness gate computed from the window samples).
    fn feed(samples: &[i16], core: &mut MeterCore) -> Vec<MicLevel> {
        feed_and_track(samples, core).0
    }

    /// `feed` + the peak confidence energy reached, for asserting the
    /// no-false-positive margin (must stay below ENERGY_ENGAGE).
    fn feed_and_track(samples: &[i16], core: &mut MeterCore) -> (Vec<MicLevel>, f32) {
        let mut out = Vec::new();
        let mut max_energy = 0.0_f32;
        let mut i = 0usize;
        while i + CHUNK_SAMPLES as usize <= samples.len() {
            let w = &samples[i..i + CHUNK_SAMPLES as usize];
            let mut peak: f32 = 0.0;
            let mut acc: f64 = 0.0;
            let mut norm = Vec::with_capacity(w.len());
            for &x in w {
                let f = f32::from(x) / 32768.0;
                norm.push(f);
                peak = peak.max(f.abs());
                acc += f64::from(f) * f64::from(f);
            }
            let rms = (acc / CHUNK_SAMPLES as f64).sqrt();
            let db_rms = if rms > f32::EPSILON as f64 {
                (20.0 * rms.log10()).max(-90.0) as f32
            } else {
                -90.0
            };
            let db_peak = if peak > f32::EPSILON {
                (20.0 * peak.log10()).max(-90.0)
            } else {
                -90.0
            };
            let sfm = spectral_flatness(norm.iter().copied());
            if let Some(level) = core.classify(db_rms, db_peak, sfm) {
                out.push(level);
                max_energy = max_energy.max(core.energy);
            }
            i += CHUNK_SAMPLES as usize;
        }
        (out, max_energy)
    }

    fn load_pcm(name: &str) -> Vec<i16> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name);
        let bytes = std::fs::read(path).expect("fixture missing");
        // `chunks_exact`, not `as_chunks`: `as_chunks` is still unstable on
        // this toolchain and clippy suggests it anyway, so the lint is silenced
        // with the reason written down rather than left to fail a build.
        #[allow(clippy::chunks_exact_to_as_chunks)]
        bytes
            .chunks_exact(2)
            .map(|c| i16::from_le_bytes([c[0], c[1]]))
            .collect()
    }

    /// 2.5 s real capture: startup clicks + hiss + one -6.8 dBFS transient.
    fn fixture() -> Vec<i16> {
        load_pcm("epos-no-mic-48k-s16-mono.pcm")
    }

    /// The full 14 s no-mic session the whole algorithm was validated against
    /// (measured 2026-09-21, EPOS GSX 300 capture `hw:4,0`).
    fn fixture_14s() -> Vec<i16> {
        load_pcm("epos-no-mic-14s-48k-s16-mono.pcm")
    }

    /// Gaussian hiss matching the measured EPOS preamp (chunk RMS ≈ -35 dBFS).
    fn synthetic_hiss(seconds: usize, seed: u64) -> Vec<i16> {
        let n = seconds * 48000;
        let mut rng = 0x9E3779B97F4A7C15u64.wrapping_mul(seed + 1);
        let mut next = move || {
            // xorshift64, enough for noise
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng as f64 / u64::MAX as f64
        };
        let std_dev = 32768.0 * 10f64.powf(-35.0 / 20.0);
        (0..n)
            .map(|_| {
                let u1 = next();
                let u2 = next();
                // Box–Muller
                let g = (-2.0 * u1.max(1e-12).ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
                (g * std_dev).round() as i16
            })
            .collect()
    }

    /// Overlay a sine "speech" tone onto `samples[start_sec..end_sec]` with the
    /// requested chunk-RMS (dBFS). The harmonic (220 + 1100 Hz) mix models
    /// voiced speech: low SFM at any level.
    fn overlay_tone(samples: &mut [i16], start_sec: usize, end_sec: usize, rms_dbfs: f64) {
        let n = samples.len();
        let a0 = (start_sec * 48000).min(n);
        let a1 = (end_sec * 48000).min(n);
        let amp = 32768.0 * 10f64.powf(rms_dbfs / 20.0) * std::f64::consts::SQRT_2;
        for (k, s) in samples[a0..a1].iter_mut().enumerate() {
            let t = (a0 + k) as f64 / 48000.0;
            let v = amp
                * (0.7 * (2.0 * std::f64::consts::PI * 220.0 * t).sin()
                    + 0.3 * (2.0 * std::f64::consts::PI * 1100.0 * t).sin());
            *s = (*s as f64 + v).round().clamp(-32768.0, 32767.0) as i16;
        }
    }

    /// The SFM discriminant itself: harmonic tone -> low, flat noise -> high,
    /// digital silence -> 1.0 (definitely "noise", never signal).
    #[test]
    fn spectral_flatness_separates_tone_from_noise() {
        let fs = 48000.0_f64;
        // Voiced-speech model: fundamental + harmonics spread across the
        // sub-bands (same stimulus the 0.32 reading was validated with) plus
        // a faint noise floor, as real formants always carry.
        let mut tone = Vec::new();
        for k in 0..CHUNK_SAMPLES as usize {
            let t = k as f64 / fs;
            let v = 0.7 * (2.0 * std::f64::consts::PI * 180.0 * t).sin()
                + 0.55 * (2.0 * std::f64::consts::PI * 350.0 * t).sin()
                + 0.4 * (2.0 * std::f64::consts::PI * 900.0 * t).sin()
                + 0.25 * (2.0 * std::f64::consts::PI * 2500.0 * t).sin();
            tone.push((v + (k as f64 * 0.0001).sin() * 0.002) as f32);
        }
        let tone_sfm = spectral_flatness(tone.iter().copied());
        assert!(tone_sfm < 0.45, "harmonic tone reads {tone_sfm:.3}");

        let norm: Vec<f32> = synthetic_hiss(1, 3)
            .iter()
            .map(|&x| f32::from(x) / 32768.0)
            .take(CHUNK_SAMPLES as usize)
            .collect();
        let noise_sfm = spectral_flatness(norm.iter().copied());
        assert!(noise_sfm > 0.50, "flat noise reads {noise_sfm:.3}");

        let silence = spectral_flatness(std::iter::repeat_n(0.0_f32, CHUNK_SAMPLES as usize));
        assert_eq!(silence, 1.0);
    }

    /// Real device recording, NO microphone attached: startup ADC clicks +
    /// non-stationary preamp hiss + a rare -6.8 dBFS transient.
    /// The meter must NEVER report audio for any of it.
    #[test]
    fn real_no_mic_never_reports_audio() {
        let mut core = MeterCore::default();
        let levels = feed(&fixture(), &mut core);

        // All 8 settle chunks swallowed (startup click burst included).
        assert_eq!(core.settle, 0);
        assert!(levels.iter().all(|l| !l.active), "no-mic must stay inactive");
        assert!(levels.iter().all(|l| l.db == 0.0 && l.peak_db == 0.0));
        assert!(levels.iter().all(|l| !l.clip), "idle transient must not clip");
        // ~2.5 s = 83 chunks exceed FLOOR_MIN_SAMPLES, so the floor primes to
        // the real noise center (-37-ish) — and the gates still hold per-chunk.
        assert!(
            core.floor_db > -45.0 && core.floor_db < -30.0,
            "floor {:?}",
            core.floor_db
        );
    }

    /// The -6.8 dBFS idle transient inside the real capture lands in a single
    /// window: it may bump the confidence integrator a little but must never
    /// accumulate into an engagement.
    #[test]
    fn isolated_transient_never_engages() {
        let mut core = MeterCore::default();
        let (levels, max_energy) = feed_and_track(&fixture(), &mut core);
        let max_db = levels.iter().map(|l| l.db).fold(0.0_f32, f32::max);
        assert_eq!(max_db, 0.0);
        assert!(
            max_energy < ENERGY_ENGAGE,
            "isolated transients must never accumulate to engage (energy {max_energy})"
        );
    }

    /// The whole 14 s no-mic session: startup clicks, ~460 chunks of
    /// non-stationary hiss, thousands of transient kicks and the -6.8 dBFS
    /// peak. Even with the gate lowered to 6 dB, two-gate + integrator means
    /// confidence never reaches ENERGY_ENGAGE anywhere in the file.
    #[test]
    fn real_no_mic_14s_never_engages_even_with_6db_gate() {
        let mut core = MeterCore::default();
        let (levels, max_energy) = feed_and_track(&fixture_14s(), &mut core);
        assert!(levels.iter().all(|l| !l.active), "no-mic 14s must stay inactive");
        assert!(levels.iter().all(|l| l.db == 0.0 && l.peak_db == 0.0));
        assert!(levels.iter().all(|l| !l.clip), "idle transients must not clip");
        assert!(
            max_energy < ENERGY_ENGAGE,
            "confidence reached {max_energy} (needs {ENERGY_ENGAGE} to engage)"
        );
        assert!(
            core.floor_db > -45.0 && core.floor_db < -30.0,
            "floor {:?}",
            core.floor_db
        );
    }

    /// Real speech must engage quickly, read a sane relative level, hold the
    /// peak, and release promptly once the sound stops.
    #[test]
    fn speech_engages_and_releases() {
        let mut hiss = synthetic_hiss(10, 7);
        overlay_tone(&mut hiss, 4, 6, -22.0); // 2 s of "speech" at -22 dBFS RMS

        let mut core = MeterCore::default();
        let levels = feed(&hiss, &mut core);

        // Find the first engagement.
        let first = levels.iter().position(|l| l.active).expect("speech must engage");
        let t_engage = first as f32 / 33.0;
        assert!((3.0..=6.0).contains(&t_engage), "engage at {t_engage}s");

        // Floor should have primed near the hiss center (-38 ± 5 dBFS).
        assert!(core.floor_db > -43.0 && core.floor_db < -33.0, "floor {:?}", core.floor_db);

        // While the tone is playing, readout is a positive relative level.
        let during: Vec<&MicLevel> = levels.iter().filter(|l| l.active && l.db > 0.0).collect();
        assert!(!during.is_empty());
        let peak_rel = during.iter().map(|l| l.db).fold(0.0_f32, f32::max);
        assert!((10.0..=20.0).contains(&peak_rel), "speech rel level {peak_rel:.1} dB");

        // After the tone ends it must release (well inside the trailing noise).
        let last_active = levels.iter().rposition(|l| l.active).unwrap();
        let seconds_of = (levels.len() - last_active) as f32 / 33.0;
        assert!(seconds_of <= 6.0, "still active {}s after last traffic", seconds_of);
        assert!(!core.active, "must have released by EOF");
    }

    /// Quiet speech over the REAL device hiss: -28 dBFS RMS tone (rel ~9 dB,
    /// above the 6 dB level gate but below the 15 dB loud bypass) must still
    /// engage — exactly the case the spectral gate unlocks.
    #[test]
    fn quiet_speech_over_real_hiss_engages_via_spectral_gate() {
        let mut buf = fixture_14s();
        overlay_tone(&mut buf, 6, 8, -28.0); // 2 s of quiet "speech"

        let mut core = MeterCore::default();
        let levels = feed(&buf, &mut core);

        let first = levels.iter().position(|l| l.active).expect("quiet speech must engage");
        let t_engage = first as f32 / 33.0;
        assert!(
            (5.0..=8.2).contains(&t_engage),
            "quiet speech engage {t_engage:.1}s (tone at 6-8s)"
        );
        let peak_rel = levels.iter().map(|l| l.db).fold(0.0_f32, f32::max);
        assert!((4.0..=14.0).contains(&peak_rel), "readout {peak_rel:.1} dB");
        assert!(!core.active, "must have released by EOF");
    }

    /// Loud broadband audio (a "clap"): SFM is flat but rel blows past the
    /// loud bypass — loud is real audio even when not voice-like.
    #[test]
    fn loud_broadband_bypass_engages() {
        let mut buf = synthetic_hiss(10, 13);
        // A loud uniform-noise burst, RMS ≈ -16 dBFS -> rel ≈ 19 dB above the
        // -35 floor, far past the 15 dB loud bypass. SFM reads ~0.6 (flat),
        // so ONLY the bypass can engage it.
        let amp = 9000.0_f64; // uniform ±amp -> RMS = amp/√3 ≈ -15.8 dBFS
        let (a0, a1) = (4 * 48000, 6 * 48000);
        let mut rng = 0x9E3779B97F4A7C15u64.wrapping_mul(77);
        for s in buf[a0..a1].iter_mut() {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            let v = ((rng as f64 / u64::MAX as f64) * 2.0 - 1.0) * amp;
            *s = (*s as f64 + v).round().clamp(-32768.0, 32767.0) as i16;
        }

        let mut core = MeterCore::default();
        let levels = feed(&buf, &mut core);
        let first = levels.iter().position(|l| l.active).expect("loud broadband must engage");
        let t_engage = first as f32 / 33.0;
        assert!(
            (3.5..=6.0).contains(&t_engage),
            "loud burst engage {t_engage:.1}s"
        );
    }

    /// CLIP is only ever reported for confirmed audio, and even then only
    /// when the absolute peak really reaches -6 dBFS.
    #[test]
    fn clip_requires_confirmed_loud_audio() {
        let mut hiss = synthetic_hiss(10, 11);
        overlay_tone(&mut hiss, 4, 6, -22.0);
        // Make ONE loud voiced region inside the speech burst peak at -3 dBFS.
        let peak_amp = 32768.0 * 10f64.powf(-3.0 / 20.0);
        let center = (5 * 48000) + 720; // mid-speech
        for k in 0..20usize {
            hiss[center + k] = (peak_amp * (-1.0_f64).powi(k as i32)).round() as i16;
        }

        let mut core = MeterCore::default();
        let levels = feed(&hiss, &mut core);

        let clips: Vec<f32> = levels.iter().filter(|l| l.clip).map(|l| l.db).collect();
        assert!(!clips.is_empty(), "loud confirmed speech should clip");
        // And every clip frame must be active (no clip from idle transients).
        assert!(levels.iter().all(|l| !l.clip || l.active));
    }
}