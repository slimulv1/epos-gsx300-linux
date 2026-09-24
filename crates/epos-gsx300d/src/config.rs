use anyhow::{Context, Result};
use epos_shared::Config;
use std::path::{Path, PathBuf};
use tracing::info;

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

/// Load the config for daemon startup.
///
/// This is the only caller allowed to author a config file, and only when the
/// file is genuinely absent — at first boot there is nothing to lose.
pub fn load() -> Result<Config> {
    load_or_create(&config_path())
}

/// Load the config for a runtime reload: read what is on disk, never author it.
///
/// Both runtime callers need this. `Reload` is a user asking to re-read the
/// file, and the watcher reacts to a change in a file it has already seen — in
/// neither case is an absent file an instruction to write a default over the
/// real path.
pub fn load_existing() -> Result<Config> {
    load_from(&config_path())
}

/// Load the config at `path`, creating a default file if it is absent.
///
/// Bootstrap only. At runtime an absent file means something removed it, and
/// answering that by writing `Config::default()` over the real path would turn
/// a transient gap into a factory reset the user never asked for.
pub fn load_or_create(path: &Path) -> Result<Config> {
    if !path.exists() {
        info!("No config found at {}, creating default", path.display());
        let config = Config::default();
        save_to(path, &config)?;
        return Ok(config);
    }
    load_from(path)
}

/// Load the config at `path`, treating an absent file as an error.
///
/// A file that is missing, unreadable, empty or malformed is reported and left
/// exactly as found: a runtime reload must never author or repair the file, or
/// a momentary gap would silently discard the user's profiles.
pub fn load_from(path: &Path) -> Result<Config> {
    let data = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config from {}", path.display()))?;

    let config: Config = serde_json::from_str(&data)
        .with_context(|| format!("Failed to parse config from {}", path.display()))?;

    Ok(config)
}

/// Save config to disk atomically
pub fn save(config: &Config) -> Result<()> {
    save_to(&config_path(), config)
}

/// Write `config` to `path` by writing a sibling temp file and renaming it, so
/// a concurrent reader sees either the previous file or the new one and never
/// a half-written mixture.
pub fn save_to(path: &Path, config: &Config) -> Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)
            .with_context(|| format!("Failed to create config dir {}", dir.display()))?;
    }

    let tmp_path = temp_path(path);

    let data = serde_json::to_string_pretty(config)?;
    std::fs::write(&tmp_path, &data)
        .with_context(|| format!("Failed to write config to {}", tmp_path.display()))?;

    std::fs::rename(&tmp_path, path).with_context(|| {
        format!(
            "Failed to atomically replace {} from {}",
            path.display(),
            tmp_path.display()
        )
    })?;

    info!("Config saved to {}", path.display());
    Ok(())
}

/// A fresh sibling path for one save.
///
/// Every writer shares one config path, and a fixed temp name let concurrent
/// saves consume each other's file: one rename would move the other's bytes
/// into place and the second rename would fail with the temp file already
/// gone. The name carries the pid and a per-process counter, so two saves can
/// never be handed the same path.
fn temp_path(path: &Path) -> PathBuf {
    static SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let sequence = SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "config.json".to_string());
    path.with_file_name(format!("{name}.tmp.{}.{sequence}", std::process::id()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    /// A private directory per test case, so these never touch the real
    /// `~/.config/epos-gsx300/` and never need to mutate the environment
    /// (which the other tests in this binary share).
    fn scratch(name: &str) -> PathBuf {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let dir = std::env::temp_dir().join(format!(
            "epos-config-test-{}-{}-{name}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create scratch dir");
        dir
    }

    /// The destructive case. `Reload` used to call the same `load()` that
    /// bootstrap uses, and bootstrap answers a missing file by writing
    /// `Config::default()`. So a config that was momentarily absent — an
    /// editor's remove-then-write, a `git checkout` of a deleted file — was
    /// turned into a factory reset that overwrote the user's profiles.
    /// A reload must read the file or fail, never author one.
    #[test]
    fn a_missing_config_is_an_error_for_a_reload_and_nothing_is_written() {
        let dir = scratch("missing");
        let path = dir.join("config.json");

        let outcome = load_from(&path);

        assert!(outcome.is_err(), "a missing file must not load as defaults");
        assert!(
            !path.exists(),
            "a failed reload must not create a config file"
        );
    }

    /// Bootstrap is the one caller that should author a default, and only when
    /// the file is genuinely absent.
    #[test]
    fn a_missing_config_is_created_at_bootstrap() {
        let dir = scratch("bootstrap");
        let path = dir.join("config.json");

        let loaded = load_or_create(&path).expect("bootstrap must create a config");

        assert!(path.exists(), "bootstrap must leave a config file behind");
        // `Config` has no `PartialEq`, so compare what actually landed on disk.
        let on_disk = load_from(&path).expect("reload what bootstrap wrote");
        assert_eq!(
            serde_json::to_string(&on_disk).expect("serialize"),
            serde_json::to_string(&Config::default()).expect("serialize"),
            "bootstrap must write exactly the default config"
        );
    }

    /// A corrupt file is reported, not replaced. The daemon refusing to start
    /// is recoverable; silently overwriting the user's settings is not.
    #[test]
    fn a_corrupt_config_is_preserved_byte_for_byte() {
        let dir = scratch("corrupt");
        let path = dir.join("config.json");
        let garbage = "{ this is not json";
        std::fs::write(&path, garbage).expect("seed corrupt config");

        assert!(load_from(&path).is_err(), "corrupt JSON must not load");
        assert_eq!(
            std::fs::read_to_string(&path).expect("file must still be there"),
            garbage,
            "the unreadable file must be left for the user to inspect"
        );
    }

    /// An editor that truncates before writing leaves a zero-byte file. That
    /// must read as a failure, not as "no settings, use defaults".
    #[test]
    fn an_empty_config_is_an_error_and_is_left_alone() {
        let dir = scratch("empty");
        let path = dir.join("config.json");
        std::fs::write(&path, "").expect("seed empty config");

        assert!(load_from(&path).is_err(), "an empty file must not load as defaults");
        assert_eq!(
            std::fs::read_to_string(&path).expect("file must still be there"),
            "",
            "the empty file must not be backfilled with defaults"
        );
    }

    /// Unknown fields are ignored on load, so a config written by a newer build
    /// still works — and what is saved back is what this build understands.
    #[test]
    fn a_saved_config_round_trips() {
        let dir = scratch("roundtrip");
        let path = dir.join("config.json");
        let mut config = Config::default();
        config.device.volume = Some(37);

        save_to(&path, &config).expect("save");
        let loaded = load_from(&path).expect("load back what was saved");

        assert_eq!(loaded.device.volume, Some(37));
    }

    /// Saving is atomic at the pathname, so a reader sees either the old file
    /// or the new one, never a half-written one — and no temp file is left
    /// behind for a later run to trip over.
    #[test]
    fn saving_leaves_no_temp_file_behind() {
        let dir = scratch("notemp");
        let path = dir.join("config.json");

        save_to(&path, &Config::default()).expect("save");

        let leftovers: Vec<_> = std::fs::read_dir(&dir)
            .expect("read scratch dir")
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|name| name != "config.json")
            .collect();
        assert!(leftovers.is_empty(), "unexpected files left: {leftovers:?}");
    }

    /// Every writer in the daemon shares one config path: the IPC handlers, the
    /// volume-save worker and the smart-button path all run as separate tasks.
    /// A single fixed temp name let two of them consume each other's file, so
    /// one rename moved the other's bytes into place and the second rename
    /// failed outright.
    #[test]
    fn temp_paths_never_collide() {
        let dir = scratch("tempnames");
        let path = dir.join("config.json");

        let mut seen = std::collections::HashSet::new();
        for _ in 0..1000 {
            assert!(
                seen.insert(temp_path(&path)),
                "two saves were handed the same temp path"
            );
        }
    }

    /// The temp file has to be a sibling of the target, or the rename is a
    /// cross-device copy and stops being atomic.
    #[test]
    fn temp_path_stays_beside_its_target() {
        let path = Path::new("/home/someone/.config/epos-gsx300/config.json");
        assert_eq!(temp_path(path).parent(), path.parent());
    }

    /// The consequence of a shared temp name: with enough writers in flight,
    /// some saves fail outright and the file can be left holding another
    /// writer's bytes. Every save must succeed and the result must parse.
    #[test]
    fn concurrent_saves_all_succeed_and_leave_a_readable_config() {
        let dir = scratch("concurrent");
        let path = dir.join("config.json");
        save_to(&path, &Config::default()).expect("seed");

        let mut handles = Vec::new();
        for worker in 0..16 {
            let path = path.clone();
            handles.push(std::thread::spawn(move || {
                for round in 0..8 {
                    let mut config = Config::default();
                    config.device.volume = Some(worker * 8 + round);
                    save_to(&path, &config)
                        .unwrap_or_else(|e| panic!("save {worker}/{round} failed: {e}"));
                }
            }));
        }
        for handle in handles {
            handle.join().expect("writer thread must not panic");
        }

        load_from(&path).expect("the surviving config must parse");
        let leftovers: Vec<_> = std::fs::read_dir(&dir)
            .expect("read scratch dir")
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|name| name != "config.json")
            .collect();
        assert!(leftovers.is_empty(), "temp files were left behind: {leftovers:?}");
    }
}

