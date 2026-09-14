use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

pub struct DaemonClient {
    socket_path: PathBuf,
}

impl DaemonClient {
    pub fn new() -> Self {
        let runtime_dir = dirs::runtime_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        Self {
            socket_path: runtime_dir.join("epos-gsx300d.sock"),
        }
    }

    pub async fn send_request(&self, request: &str) -> Result<String, String> {
        let mut stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|e| format!("Failed to connect to daemon: {}", e))?;

        // Send request
        stream
            .write_all(request.as_bytes())
            .await
            .map_err(|e| format!("Failed to send: {}", e))?;
        stream
            .write_all(b"\n")
            .await
            .map_err(|e| format!("Failed to send newline: {}", e))?;

        // Read response
        let (reader, _) = stream.into_split();
        let mut lines = BufReader::new(reader).lines();
        if let Some(line) = lines
            .next_line()
            .await
            .map_err(|e| format!("Failed to read: {}", e))?
        {
            Ok(line)
        } else {
            Err("No response from daemon".into())
        }
    }
}
