use anyhow::{Result, Context};
use std::path::PathBuf;
use tracing::info;
use epos_shared::Config;

/// Get config file path: ~/.config/epos-gsx300/config.json
pub fn config_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("epos-gsx300");
    std::fs::create_dir_all(&config_dir).ok();
    config_dir.join("config.json")
}

/// Load config from disk, or create default
pub fn load() -> Result<Config> {
    let path = config_path();

    if path.exists() {
        let data = std::fs::read_to_string(&path)
            .context("Failed to read config file")?;
        let config: Config = serde_json::from_str(&data)
            .context("Failed to parse config file")?;
        info!("Loaded config from {}", path.display());
        Ok(config)
    } else {
        let config = Config::default();
        save(&config)?;
        info!("Created default config at {}", path.display());
        Ok(config)
    }
}

/// Save config to disk
pub fn save(config: &Config) -> Result<()> {
    let path = config_path();
    let data = serde_json::to_string_pretty(config)
        .context("Failed to serialize config")?;
    std::fs::write(&path, data)
        .context("Failed to write config file")?;
    info!("Saved config to {}", path.display());
    Ok(())
}
