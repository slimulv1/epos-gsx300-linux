use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::{info, warn};
use epos_shared::Config;

/// Returns the config directory path (~/.config/epos-gsx300/)
pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("epos-gsx300")
}

/// Returns the config file path (~/.config/epos-gsx300/config.json)
pub fn config_path() -> PathBuf {
    config_dir().join("config.json")
}

/// Load config from disk, falling back to defaults
pub fn load() -> Result<Config> {
    let path = config_path();
    if !path.exists() {
        info!("No config found at {}, creating default", path.display());
        let config = Config::default();
        save(&config)?;
        return Ok(config);
    }

    let data = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config from {}", path.display()))?;

    let config: Config = serde_json::from_str(&data)
        .with_context(|| format!("Failed to parse config from {}", path.display()))?;

    Ok(config)
}

/// Save config to disk atomically
pub fn save(config: &Config) -> Result<()> {
    let dir = config_dir();
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create config dir {}", dir.display()))?;

    let path = config_path();
    let tmp_path = path.with_extension("json.tmp");

    let data = serde_json::to_string_pretty(config)?;
    std::fs::write(&tmp_path, &data)
        .with_context(|| format!("Failed to write config to {}", tmp_path.display()))?;

    std::fs::rename(&tmp_path, &path)
        .with_context(|| "Failed to atomically replace config file")?;

    info!("Config saved to {}", path.display());
    Ok(())
}

/// Watch config file for external changes (hot-reload)
#[allow(dead_code)]
pub fn watch_config(config_dir: &Path) -> Result<()> {
    // Placeholder for notify-based file watcher
    // Will be implemented with inotify/notify crate
    warn!("Config file watcher not yet implemented for {}", config_dir.display());
    Ok(())
}
