use serde::Serialize;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use tauri::{Emitter, Manager};

/// Holds the running `pw-record` child used for live mic level metering.
/// The reader thread also takes this child out on EOF (device unplugged),
/// so every state transition is serialized through the same mutex.
#[derive(Default)]
pub struct MicMeterState {
    pub child: Mutex<Option<Child>>,
}

/// Event payload emitted to the frontend (~47x/s):
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

/// Find the raw EPOS GSX 300 mic source node via pw-dump.
/// Matches by prefix/suffix (NOT the hardcoded serial) so a device swap
/// still resolves correctly. Returns Ok(None) when the device is absent.
fn resolve_epos_source() -> Result<Option<String>, String> {
    let out = Command::new("pw-dump")
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

/// Start the mic level meter. Returns:
///   Ok(true)  — meter running (or already running)
///   Ok(false) — EPOS mic not present; caller should retry later
///   Err(msg)  — pw-dump/pw-record unavailable or spawn failed
#[tauri::command]
pub async fn mic_meter_start(
    app: tauri::AppHandle,
    state: tauri::State<'_, MicMeterState>,
) -> Result<bool, String> {
    {
        let guard = state.child.lock().map_err(|e| e.to_string())?;
        if guard.is_some() {
            return Ok(true); // already running
        }
    }

    let Some(src) = resolve_epos_source()? else {
        return Ok(false);
    };

    let mut child = Command::new("pw-record")
        .args([
            "--target",
            &src,
            "--channels",
            "1",
            "--rate",
            "48000",
            "--format",
            "f32",
            "--latency",
            "50ms",
            "-",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("pw-record failed (pipewire-utils installed?): {e}"))?;

    let stdout = child.stdout.take().ok_or("pw-record: no stdout")?;
    *state.child.lock().map_err(|e| e.to_string())? = Some(child);

    let app2 = app; // moved into the reader thread
    std::thread::spawn(move || {
        use std::io::Read;
        let mut reader = std::io::BufReader::new(stdout);
        let mut buf = [0u8; 4096]; // 1024 mono f32 samples per chunk
        let mut peak_db: f32 = 0.0; // relative peak-hold
        let mut quiet_db: f32 = -100.0; // adaptive noise-floor reference (absolute dBFS)
        let mut boot: Vec<f32> = Vec::with_capacity(60); // bootstrap samples
        let mut started = false;
        loop {
            let n = match reader.read(&mut buf) {
                Ok(0) | Err(_) => break, // EOF → device gone or meter stopped
                Ok(n) => n,
            };
            let mut data = &buf[..n];
            if !started {
                started = true;
                // pw-record prefixes a 24-byte Sun Audio (.snd) header on stdout:
                // magic ".snd", data offset, size, encoding 6=f32, rate, chans.
                // Skip it so header bytes are never decoded as samples.
                if data.len() >= 24 && &data[..4] == b".snd" {
                    data = &data[24..];
                }
            }
            if data.is_empty() {
                continue;
            }
            let mut peak: f32 = 0.0;
            for c in data.chunks_exact(4) {
                let a = f32::from_le_bytes([c[0], c[1], c[2], c[3]]).abs();
                if a > peak {
                    peak = a;
                }
            }
            if peak == 0.0 {
                peak = 1e-9;
            }
            let db_full = (20.0 * peak.log10()).max(-60.0); // absolute dBFS
            // Bootstrap: the first ~1.8s of the stream is unreliable (pw-record
            // starts with a few hundred ms of digital silence, which would pin
            // the reference at -60 dB and inflate every level by ~25 dB).
            // Use the MEDIAN of the first 60 chunks as the initial reference.
            if boot.len() < 60 {
                boot.push(db_full);
                if boot.len() == 60 {
                    let mut b = boot.clone();
                    b.sort_by(|a, c| a.partial_cmp(c).unwrap());
                    quiet_db = b[30];
                }
                continue;
            }
            // Adaptive noise-floor follower: learns the mic's idle hiss level.
            // The raw USB preamp is always "alive" (~-35 dBFS here), so without
            // this reference the ring would swing 30-60% on pure silence.
            if db_full < quiet_db + 6.0 {
                quiet_db = quiet_db * 0.98 + db_full * 0.02; // ~1.5 s time constant
            }
            let db = (db_full - quiet_db).max(0.0); // 0 dB = at noise floor
            // Peak-hold (relative): rise instantly, fall ~16 dB/s (0.35 dB per chunk).
            if db >= peak_db {
                peak_db = db;
            } else {
                peak_db = (peak_db - 0.35).max(db);
            }
            let clip = db_full >= -6.0;
            let _ = app2.emit(
                "mic-level",
                MicLevel {
                    db,
                    peak_db,
                    clip,
                    active: true,
                },
            );
        }
        // EOF: release the child and tell the frontend we went silent.
        // (app2 is owned by this closure, so its state borrow cannot escape.)
        if let Some(mut c) = app2
            .state::<MicMeterState>()
            .child
            .lock()
            .ok()
            .and_then(|mut g| g.take())
        {
            let _ = c.kill();
            let _ = c.wait();
        }
        let _ = app2.emit(
            "mic-level",
            MicLevel {
                db: 0.0,
                peak_db: 0.0,
                clip: false,
                active: false,
            },
        );
    });

    Ok(true)
}

/// Stop the mic level meter (kills the pw-record child if running).
#[tauri::command]
pub async fn mic_meter_stop(state: tauri::State<'_, MicMeterState>) -> Result<(), String> {
    if let Some(mut c) = state.child.lock().map_err(|e| e.to_string())?.take() {
        let _ = c.kill();
        let _ = c.wait();
    }
    Ok(())
}
