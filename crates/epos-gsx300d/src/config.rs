use anyhow::{Context, Result};
use epos_shared::Config;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use tracing::{info, warn};

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

/// Keys already named to the user, so a typo is reported once and not on
/// every reload.
///
/// The reload watcher fires on any mtime change, and the daemon saves on
/// ordinary setting changes, so a config carrying one bad key would otherwise
/// produce the same line over and over.
fn already_reported() -> &'static Mutex<BTreeSet<String>> {
    static KEYS: OnceLock<Mutex<BTreeSet<String>>> = OnceLock::new();
    KEYS.get_or_init(|| Mutex::new(BTreeSet::new()))
}

/// Name any key this build does not recognise, once per key.
///
/// Preserving an unknown key is only half of it. Preserved, the key still does
/// nothing, so a user who typed `mic_gian` instead of `mic_gain` watches their
/// setting not take effect with nothing anywhere to suggest why. The name is
/// what turns a silent no-op into something they can act on — and it is the
/// only place they would ever find out, since the value is not in any response.
fn keys_not_yet_reported<'a>(
    extra: &'a BTreeMap<String, serde_json::Value>,
    seen: &BTreeSet<String>,
) -> Vec<&'a str> {
    extra
        .keys()
        .filter(|key| !seen.contains(*key))
        .map(String::as_str)
        .collect()
}

/// Name them. The decision of *which* keys is [`keys_not_yet_reported`], split
/// out so it can be tested: asserting on the warning itself would mean standing
/// up a tracing subscriber to capture log output, and a mutation that removed
/// this call from `load_from` passed every test in the crate for exactly that
/// reason — the rule was right and nothing could see it being applied.
fn report_unrecognised_keys(extra: &BTreeMap<String, serde_json::Value>) {
    let mut seen = crate::sync::lock(already_reported());
    for key in keys_not_yet_reported(extra, &seen) {
        warn!(
            "config.json has a key this build does not recognise: {key:?}. \
             It is being kept as-is, so nothing is lost, but it has no effect. \
             If you meant to change a setting, check the spelling."
        );
        seen.insert(key.to_string());
    }
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

    let mut config: Config = serde_json::from_str(&data)
        .with_context(|| format!("Failed to parse config from {}", path.display()))?;

    // A hand-edited config.json is the one input that can carry any number at
    // all, and this is the one place a config enters the process — `load` and
    // `load_existing` both come through here. So the bound goes here, rather
    // than in each caller: two call sites in `main.rs` was two chances to miss
    // one, and the first version of this did exactly that.
    crate::audio::sanitize_audio_config(&mut config.audio);
    report_unrecognised_keys(&config.extra);

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

    /// A key this build does not recognise survives a save.
    ///
    /// The whole point of the `extra` map on `Config`. Before it, a typo was
    /// destroyed without a word: `load_from` parsed straight past the unknown
    /// key, `save` serialised the struct — which has no such field — and the
    /// edit was gone. The user changes a setting, the GUI keeps showing the old
    /// value, and nothing says why.
    ///
    /// The file is read, written and read back through the real functions, so
    /// this is the actual round trip and not a serde detail.
    #[test]
    fn an_unrecognised_key_survives_a_save() {
        let dir = scratch("unknown-key");
        let path = dir.join("config.json");
        std::fs::write(&path, r#"{
            "version": 1,
            "device": {"auto_detect": true, "usb_vid": "1395", "usb_pid": "0098"},
            "audio": {
                "eq": {"enabled": false, "bands": []},
                "sidetone": {"enabled": false, "level": 0.0},
                "noise_gate": {"enabled": false, "threshold_db": -30.0},
                "voice_enhancer": {"mode": "off", "custom_bands": null},
                "mic_gain": 50
            },
            "profiles": [],
            "active_profile": "FLAT",
            "smart_button": {"action": "toggle_mode"},
            "mic_gian": 42
        }"#)
        .expect("write");

        let loaded = load_from(&path).expect("an unknown key is not a parse error");
        assert_eq!(
            loaded.extra.get("mic_gian"),
            Some(&serde_json::json!(42)),
            "the key is kept so it can be handed back"
        );

        save_to(&path, &loaded).expect("save");
        let reread = load_from(&path).expect("re-read");
        assert_eq!(
            reread.extra.get("mic_gian"),
            Some(&serde_json::json!(42)),
            "and it is still there after a save: this is what used to be lost"
        );
        // And the fields that *are* recognised are untouched by the mechanism.
        assert_eq!(reread.audio.mic_gain, 50);
        assert!(reread.extra.len() == 1, "only the one key is unknown");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Saving a config nobody edited adds no key of its own.
    ///
    /// The first version of this asserted that `skip_serializing_if` was
    /// load-bearing, and a mutation removing it passed — so the assertion was
    /// checking an attribute that does nothing. With `flatten` the map's entries
    /// are inlined as siblings of the real fields, so an empty map adds nothing
    /// either way, and there is no stray key to check for.
    ///
    /// What is worth checking is the thing that would actually be wrong: a
    /// `save` that introduced a key of its own would change the file's bytes,
    /// and the daemon compares those bytes against what it last wrote to decide
    /// whether a change on disk was its own. So the top-level key set is pinned
    /// exactly.
    #[test]
    fn saving_an_untouched_config_introduces_no_key_of_its_own() {
        let dir = scratch("no-extra-key");
        let path = dir.join("config.json");
        save_to(&path, &Config::default()).expect("save");
        let text = std::fs::read_to_string(&path).expect("read back");
        let keys: std::collections::BTreeSet<String> =
            serde_json::from_str::<serde_json::Value>(&text)
                .expect("valid json")
                .as_object()
                .expect("an object")
                .keys()
                .cloned()
                .collect();
        let expected: std::collections::BTreeSet<String> = [
            "active_profile",
            "audio",
            "device",
            "led_probe",
            "mode",
            "profiles",
            "smart_button",
            "version",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(keys, expected, "the saved key set must be exactly the known one");
        // And it round-trips to an empty map rather than to a missing field.
        assert!(load_from(&path).expect("re-read").extra.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A key is named once, not on every reload.
    ///
    /// The daemon reloads on any mtime change and saves on ordinary setting
    /// changes, so a config carrying one bad key would otherwise print the same
    /// line over and over until the user got bored of the log.
    ///
    /// Asserted on the decision rather than on the log line: capturing a
    /// `warn!` means standing up a tracing subscriber inside a test, and a
    /// mutation that dropped the call from `load_from` passed everything else
    /// for that reason.
    #[test]
    fn an_unrecognised_key_is_named_once_and_not_repeated() {
        let extra: BTreeMap<String, serde_json::Value> = [
            ("mic_gian".to_string(), serde_json::json!(42)),
            ("typo_two".to_string(), serde_json::json!(1)),
        ]
        .into_iter()
        .collect();

        // Nothing reported yet: both are named.
        let mut seen: BTreeSet<String> = BTreeSet::new();
        assert_eq!(
            keys_not_yet_reported(&extra, &seen),
            vec!["mic_gian", "typo_two"],
            "both are new"
        );

        // One has been named; the other still has not.
        seen.insert("mic_gian".to_string());
        assert_eq!(
            keys_not_yet_reported(&extra, &seen),
            vec!["typo_two"],
            "a key already named must not be named again on the next reload"
        );

        // And an empty map asks for nothing, so a clean config is silent.
        assert!(keys_not_yet_reported(&BTreeMap::new(), &seen).is_empty());
    }

    /// A hand-edited config cannot enter the process carrying a number that
    /// nothing else would produce.
    ///
    /// The bound lives in `load_from` rather than in `main.rs` because that is
    /// the one function both `load` and `load_existing` come through, and
    /// `load_existing` is the runtime `Reload` path. With the call in each caller
    /// instead, one of them was missed — the mutation that removed it from
    /// `main.rs` failed no test at all, because every other test reached the rule
    /// directly rather than through this door.
    #[test]
    fn a_config_read_off_disk_is_bounded_on_the_way_in() {
        let dir = scratch("bounded-load");
        let path = dir.join("config.json");
        // A hand edit: every number here is outside what the pipeline can express.
        let mut config = Config::default();
        config.audio.mic_gain = 900_000;
        config.audio.sidetone.level = 4.0;
        config.audio.noise_gate.threshold_db = 25.0;
        config.audio.eq.bands = vec![
            epos_shared::config::EqBand { freq: 0, gain_db: 900.0, q: 1.0 },
        ];
        std::fs::write(&path, serde_json::to_string(&config).expect("serialise")).expect("write");

        let loaded = load_from(&path).expect("the file is well formed");
        assert_eq!(loaded.audio.mic_gain, 100);
        assert_eq!(loaded.audio.sidetone.level, 1.0);
        assert_eq!(loaded.audio.noise_gate.threshold_db, 0.0);
        assert!(
            loaded.audio.eq.bands.is_empty(),
            "a band at 0 Hz cannot be expressed, so it must not survive the load: {:?}",
            loaded.audio.eq.bands
        );
        let _ = std::fs::remove_dir_all(&dir);
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
            let mut config = Config { active_profile: active.to_string(), ..Config::default() };
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

