mod ipc_client;
mod mic_meter;

use ipc_client::DaemonClient;
use std::io::Write;
use std::process::{Child, Command};
use std::sync::Mutex;
use tauri::Manager;
use tauri::State;
use tauri_plugin_autostart::ManagerExt;

/// Tauri command: send a JSON request to the daemon and return the response.
/// Frontend calls this via invoke("daemon_request", { request: {...} }).
#[tauri::command]
async fn daemon_request(
    client: State<'_, DaemonClient>,
    request: String,
) -> Result<String, String> {
    client.send_request(&request).await
}

/// Embedded 440 Hz test tone (generated at build time, checked into resources/).
const TEST_TONE: &[u8] = include_bytes!("../resources/epos-test-tone.wav");

/// Tracks the running pw-play child so SOUND TEST can stop it.
struct ToneState(Mutex<Option<Child>>);

/// Play the test tone through the EQ filter chain (or stop it if playing).
/// Returns the NEW state: true = now playing, false = stopped.
#[tauri::command]
fn play_test_tone(eq_enabled: bool, state: State<'_, ToneState>) -> Result<bool, String> {
    let mut guard = state.0.lock().map_err(|_| "tone state poisoned".to_string())?;

    // Already playing → stop it
    if let Some(mut child) = guard.take() {
        let _ = child.kill();
        let _ = child.wait();
        return Ok(false);
    }

    // Write the embedded tone to a temp file, then play it via PipeWire.
    let path = std::env::temp_dir().join("epos-test-tone.wav");
    let mut f = std::fs::File::create(&path).map_err(|e| e.to_string())?;
    f.write_all(TEST_TONE).map_err(|e| e.to_string())?;
    drop(f);

    // Route through the EQ filter chain when EQ is active, else default sink.
    let mut cmd = Command::new("pw-play");
    if eq_enabled {
        cmd.args(["--target", "epos-eq-input"]);
    }
    let child = cmd
        .arg(&path)
        .spawn()
        .map_err(|e| format!("pw-play failed (pipewire-utils installed?): {e}"))?;

    *guard = Some(child);
    Ok(true)
}

#[tauri::command]
fn autostart_get(app: tauri::AppHandle) -> Result<bool, String> {
    app.autolaunch().is_enabled().map_err(|e| e.to_string())
}

#[tauri::command]
fn autostart_set(app: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    if enabled {
        app.autolaunch().enable().map_err(|e| e.to_string())
    } else {
        app.autolaunch().disable().map_err(|e| e.to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .invoke_handler(tauri::generate_handler![
            daemon_request,
            play_test_tone,
            mic_meter::mic_meter_start,
            mic_meter::mic_meter_stop,
            autostart_get,
            autostart_set
        ])
        .setup(|app| {
            let client = DaemonClient::new();
            app.manage(client);
            app.manage(ToneState(Mutex::new(None)));
            app.manage(mic_meter::MicMeterState::default());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
