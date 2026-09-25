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

/// Make the active profile the source of truth for `config.audio`.
///
/// The top-level `audio` is a copy, and copies drift. A live edit writes the
/// active profile and the top level together, but a hand edit to the file
/// usually touches only one of them. Whoever adopts a config has to decide
/// which wins, and the decision has to be the same everywhere: a band changed
/// in the profile was applied by the config watcher and then silently undone on
/// the next daemon start, which read the stale top-level copy instead.
///
/// Returns whether the audio was replaced, so a caller can say so rather than
/// staying quiet. A name matching no profile changes nothing at all — resolving
/// "no active profile" into defaults would be a silent factory reset.
pub fn resolve_active_profile_audio(config: &mut Config) -> bool {
    if config.active_profile.is_empty() {
        return false;
    }
    let Some(profile) = config
        .profiles
        .iter()
        .find(|p| p.name.eq_ignore_ascii_case(&config.active_profile))
    else {
        return false;
    };
    if serde_json::to_value(&profile.audio).ok() == serde_json::to_value(&config.audio).ok() {
        return false;
    }
    config.audio = profile.audio.clone();
    true
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
    ///
    /// The directory is removed when the returned guard is dropped. It used to
    /// be a bare `PathBuf` and nothing ever cleaned it up: 6,320 of these were
    /// sitting in /tmp, roughly thirty per run of the suite, which is both a leak
    /// and a slow accumulation of directories nobody looking at the machine would
    /// know the provenance of.
    fn scratch(name: &str) -> (PathBuf, ScratchDir) {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let dir = std::env::temp_dir().join(format!(
            "epos-config-test-{}-{}-{name}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create scratch dir");
        let guard = ScratchDir(dir.clone());
        (dir, guard)
    }

    /// Removes a scratch directory when the test ends, passed or failed.
    struct ScratchDir(PathBuf);

    impl Drop for ScratchDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// The destructive case. `Reload` used to call the same `load()` that
    /// bootstrap uses, and bootstrap answers a missing file by writing
    /// `Config::default()`. So a config that was momentarily absent — an
    /// editor's remove-then-write, a `git checkout` of a deleted file — was
    /// turned into a factory reset that overwrote the user's profiles.
    /// A reload must read the file or fail, never author one.
    #[test]
    fn a_missing_config_is_an_error_for_a_reload_and_nothing_is_written() {
        let (dir, _guard) = scratch("missing");
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
        let (dir, _guard) = scratch("bootstrap");
        let path = dir.join("config.json");

        let loaded = load_or_create(&path).expect("bootstrap must create a config");

        assert!(path.exists(), "bootstrap must leave a config file behind");
        assert_eq!(
            loaded.active_profile,
            epos_shared::config::FLAT_PROFILE_NAME,
            "bootstrap must hand back the default profile"
        );
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
        let (dir, _guard) = scratch("corrupt");
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
        let (dir, _guard) = scratch("empty");
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
        let (dir, _guard) = scratch("roundtrip");
        let path = dir.join("config.json");
        let mut config = Config::default();
        config.device.volume = Some(37);

        save_to(&path, &config).expect("save");
        let loaded = load_from(&path).expect("load back what was saved");

        assert_eq!(loaded.device.volume, Some(37));
    }

    /// The top-level `audio` is a copy, and copies drift. A live edit writes
    /// the active profile and the top level, but a hand edit to the file
    /// usually touches one of them, so whoever adopts a config has to decide
    /// which wins. The rule is the same everywhere or an edit applied through
    /// one path gets reverted by another — which is what happened: a band
    /// changed in the profile was applied by the watcher, then silently undone
    /// on the next daemon start from the stale top-level copy.
    #[test]
    fn the_active_profile_wins_over_a_stale_top_level_copy() {
        let mut config = Config::default();
        let mut profile = epos_shared::Profile::flat();
        profile.audio.mic_gain = 37;
        config.profiles = vec![profile];
        config.active_profile = "FLAT".to_string();
        config.audio.mic_gain = 100; // the stale copy

        let changed = resolve_active_profile_audio(&mut config);

        assert!(changed, "a stale top-level copy must be replaced");
        assert_eq!(config.audio.mic_gain, 37);
    }

    /// Already in step: nothing to do, and the caller is told so it can stay
    /// quiet instead of logging a change that did not happen.
    #[test]
    fn an_already_resolved_config_is_left_alone() {
        let mut config = Config::default();
        let mut profile = epos_shared::Profile::flat();
        profile.audio.mic_gain = 37;
        config.profiles = vec![profile];
        config.active_profile = "FLAT".to_string();
        config.audio.mic_gain = 37;

        assert!(!resolve_active_profile_audio(&mut config));
        assert_eq!(config.audio.mic_gain, 37);
    }

    /// Matching is case-insensitive, per the shared profile contract: the
    /// shipped config says "FLAT" and others in the wild say "Flat".
    #[test]
    fn the_profile_is_matched_ignoring_case() {
        let mut config = Config::default();
        let mut profile = epos_shared::Profile::flat();
        profile.audio.mic_gain = 42;
        config.profiles = vec![profile];
        config.active_profile = "flat".to_string();
        config.audio.mic_gain = 100;

        assert!(resolve_active_profile_audio(&mut config));
        assert_eq!(config.audio.mic_gain, 42);
    }

    /// A name that matches nothing must leave the audio completely alone. This
    /// is the dangerous direction: resolving "no profile" into defaults would be
    /// a silent factory reset of the user's settings.
    #[test]
    fn a_dangling_active_profile_never_resets_the_audio() {
        for active in ["", "DOES-NOT-EXIST"] {
            let mut config = Config::default();
            config.active_profile = active.to_string();
            config.audio.mic_gain = 88;

            assert!(
                !resolve_active_profile_audio(&mut config),
                "{active:?} must not claim to have resolved anything"
            );
            assert_eq!(config.audio.mic_gain, 88, "{active:?} must not touch audio");
        }
    }

    /// Saving is atomic at the pathname, so a reader sees either the old file
    /// or the new one, never a half-written one — and no temp file is left
    /// behind for a later run to trip over.
    #[test]
    fn saving_leaves_no_temp_file_behind() {
        let (dir, _guard) = scratch("notemp");
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
        let (dir, _guard) = scratch("tempnames");
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
        let (dir, _guard) = scratch("concurrent");
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

