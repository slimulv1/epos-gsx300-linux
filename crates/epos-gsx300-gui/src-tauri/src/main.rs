mod ipc_client;

use ipc_client::DaemonClient;
use tauri::State;

/// Tauri command: send a JSON request to the daemon and return the response.
/// Frontend calls this via invoke("daemon_request", { request: {...} }).
#[tauri::command]
async fn daemon_request(
    client: State<'_, DaemonClient>,
    request: String,
) -> Result<String, String> {
    client.send_request(&request).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![daemon_request])
        .setup(|app| {
            let client = DaemonClient::new();
            app.manage(client);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
