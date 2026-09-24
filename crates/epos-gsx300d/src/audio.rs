use anyhow::{Context, Result};
use epos_shared::config::{AudioConfig, VoiceMode};
use epos_shared::device::DeviceInfo;
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::Notify;
use tracing::{debug, info, warn};

/// Option B (3i): EPOS DSP lives in per-role software-only PipeWire instances
/// (`pipewire-epos@eq`, `pipewire-epos@voice`, `pipewire-epos@sidetone`).
/// The daemon regenerates each instance's own `pipewire.conf` and restarts
/// ONLY that instance — the main pipewire graph (A2+ speakers) is never
/// touched at runtime.
///
/// Evidence (2026-09-16, verified live):
/// - filter-chain with module-level `remote.name = "pipewire-0"` publishes
///   its capture/playback nodes into the MAIN instance (cross-instance works)
/// - playback side auto-links to the EPOS hardware sink via WirePlumber
/// - fail-closed: killing an instance vanishes its sink → pinned streams stay
///   silent, nothing leaks to A2+
/// - sidetone = module-loopback inside the instance (volume prop)
/// - rnnoise LADSPA port names are "Input"/"Output" (not In/Out)
pub struct AudioPipeline {
    config: AudioConfig,
    device: Option<DeviceInfo>,
    pub restarts: Arc<RestartBus>,
    /// Monotonic counter bumped by every `apply_full()`. The delayed mic-gain
    /// reapply task captures its epoch and aborts if a newer apply has since
    /// started, so a stale task can never overwrite a newer gain value.
    gain_epoch: Arc<AtomicU64>,
}

/// Debounced restart bus: audio handlers record which epos instance(s) changed
/// and notify; the worker in main.rs performs the actual restarts (union,
/// dedup, and **concurrently** — see `pipewire_reload_worker`) off the IPC
/// lock. Every restart is time-bounded internally, so a slow or unresponsive
/// role can never starve the others.
pub struct RestartBus {
    pub notify: Notify,
    pending: Mutex<BTreeSet<String>>,
}

impl RestartBus {
    pub fn new() -> Self {
        Self {
            notify: Notify::new(),
            pending: Mutex::new(BTreeSet::new()),
        }
    }

    /// Request a restart of one epos instance ("eq" | "voice" | "sidetone").
    pub fn request(&self, name: &str) {
        assert!(matches!(name, "eq" | "voice" | "sidetone"));
        self.pending.lock().unwrap().insert(name.to_string());
        self.notify.notify_one();
    }

    /// Drain a deduplicated snapshot; requests arriving later remain pending.
    pub fn drain(&self) -> BTreeSet<String> {
        std::mem::take(&mut *self.pending.lock().unwrap())
    }
}

impl Default for RestartBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Static fail-closed null-sink in the MAIN graph, installed once by
/// `40-epos-eq-virtualsink.conf`. The `pipewire-epos@eq` instance captures
/// its monitor and plays the EQ'd result to the raw hardware sink.
pub const EQ_SINK_NAME: &str = "epos-eq-input";

/// Which sink new playback streams should attach to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputRoute {
    /// `epos-eq-input` — playback goes through the 9-band EQ.
    Processed,
    /// The EPOS hardware sink — playback bypasses the EQ.
    Raw,
}

/// Decide which sink should be the PipeWire default, given the EQ toggle and
/// whether the device is present.
///
/// `None` means "do not touch the default sink". While the EPOS is absent there
/// is no `Raw` sink to point at, and forcibly claiming the default would yank
/// audio away from whatever device the user is actually using.
///
/// Pure policy, deliberately free of I/O so it can be tested directly.
pub fn desired_output_route(eq_enabled: bool, device_connected: bool) -> Option<OutputRoute> {
    if !device_connected {
        return None;
    }
    Some(if eq_enabled {
        OutputRoute::Processed
    } else {
        OutputRoute::Raw
    })
}

/// Hard limits for every EQ / voice band interpolated into a generated
/// PipeWire config. They are deliberately conservative: the values are written
/// verbatim into a config file that PipeWire parses, so a malformed, stale, or
/// hostile config must never be able to inject NaN/Inf, a zero/negative
/// frequency, a runaway Q, or enough bands to exhaust the per-instance
/// `MemoryMax=64M / TasksMax=32` budget and silence the audio path.
pub const BAND_FREQ_MIN_HZ: f32 = 20.0;
pub const BAND_FREQ_MAX_HZ: f32 = 20_000.0;
pub const BAND_Q_MIN: f32 = 0.1;
pub const BAND_Q_MAX: f32 = 30.0;
pub const BAND_GAIN_LIMIT_DB: f32 = 24.0;
/// Bands flatter than this are no-ops and are dropped entirely.
pub const BAND_GAIN_EPSILON_DB: f32 = 0.1;
pub const MAX_EQ_BANDS: usize = 32;

/// Normalise a band list into the `(freq, gain_db, q)` tuples that are safe to
/// interpolate into a generated PipeWire config.
///
/// - non-finite gain/Q are dropped (NaN/Inf would be written out verbatim)
/// - frequencies outside the audible band are dropped
/// - gain and Q are clamped into safe ranges rather than dropped
/// - flat bands and bands past `MAX_EQ_BANDS` are dropped
pub fn sanitize_bands(bands: &[epos_shared::config::EqBand]) -> Vec<(u32, f32, f32)> {
    let mut out = Vec::with_capacity(bands.len().min(MAX_EQ_BANDS));
    for b in bands.iter().take(MAX_EQ_BANDS) {
        if !b.gain_db.is_finite() || !b.q.is_finite() {
            continue;
        }
        let freq = b.freq as f32;
        if freq < BAND_FREQ_MIN_HZ || freq > BAND_FREQ_MAX_HZ {
            continue;
        }
        let gain = b.gain_db.clamp(-BAND_GAIN_LIMIT_DB, BAND_GAIN_LIMIT_DB);
        if gain.abs() < BAND_GAIN_EPSILON_DB {
            continue;
        }
        let q = b.q.clamp(BAND_Q_MIN, BAND_Q_MAX);
        out.push((b.freq, gain, q));
    }
    out
}

/// Bound a mic gain to the 0..=100 percentage that `amixer` is given.
///
/// `amixer` accepts `5000%` and silently saturates at full scale, so an
/// unbounded value both lies about the requested gain and pins the hardware at
/// maximum. Applied in [`AudioPipeline::update_config`] so every route into the
/// pipeline is covered — direct IPC, a stored profile, a hand-edited
/// `config.json`, and `Reload` — not just the IPC edge.
fn sanitize_mic_gain(gain: u32) -> u32 {
    gain.min(100)
}

/// The ALSA card index to hand to `amixer`, or an error when it is not known.
///
/// An unknown card is deliberately an error rather than a fallback to 0. ALSA
/// card 0 is a real sound card, so defaulting to it made an EPOS
/// unplug/replug race write the EPOS mic gain onto an unrelated device.
/// Card 0 stays valid when it is genuinely the answer.
fn usable_alsa_card(card: Option<u8>) -> Result<u8> {
    card.context(
        "ALSA card for the GSX 300 is not known yet (USB enumerated before \
         /proc/asound) — refusing to touch an arbitrary card; the hotplug poll \
         retries",
    )
}

impl AudioPipeline {
    pub fn new(config: &AudioConfig) -> Self {
        let mut pipeline = Self {
            config: AudioConfig::default(),
            device: None,
            restarts: Arc::new(RestartBus::new()),
            gain_epoch: Arc::new(AtomicU64::new(0)),
        };
        // Same funnel as update_config, so a hand-edited config.json is bounded
        // at startup too and not only on later IPC updates.
        pipeline.update_config(config);
        pipeline
    }

    /// Store device reference for future operations
    pub fn set_device(&mut self, device: &DeviceInfo) {
        self.device = Some(device.clone());
    }

    /// Sync latest config into the pipeline before applying filters
    pub fn update_config(&mut self, config: &AudioConfig) {
        let mut config = config.clone();
        // Single funnel for every config source, so a profile or an edited
        // config.json cannot smuggle an out-of-range gain past the IPC clamp.
        config.mic_gain = sanitize_mic_gain(config.mic_gain);
        self.config = config;
    }

    /// Apply the full audio config (EQ, mic gain, sidetone, noise gate, voice).
    /// Returns which of the three epos instances need a restart.
    pub async fn apply_full(&mut self) -> Result<bool> {
        self.apply_mic_gain().await?;
        let mut changed = false;
        changed |= self.write_eq_conf()?;
        changed |= self.write_voice_conf()?;
        changed |= self.write_sidetone_conf()?;
        // WirePlumber's device.restore-routes replays the saved input
        // channelVolume (stored as 1.0 in ~/.local/state/wireplumber/
        // default-routes) when the ALSA capture node activates, usually a
        // couple of seconds after this setup runs. That overwrites the
        // numid=4 Mic Capture Volume we just set. Re-apply the gain once
        // the node activation has settled so our value wins.
        //
        // Guarded by an epoch: a NEWER apply_full() (a gain change, a profile
        // switch, a reload) bumps the epoch, and this task then aborts. Without
        // it, changing the gain within the 4s window let this stale task
        // re-apply the OLD value, silently reverting the user's newer setting.
        let epoch = self.gain_epoch.fetch_add(1, Ordering::SeqCst) + 1;
        let gain = self.config.mic_gain;
        let device = self.device.clone();
        let gain_epoch = Arc::clone(&self.gain_epoch);
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(4)).await;
            if gain_epoch.load(Ordering::SeqCst) != epoch {
                debug!(
                    "Skipping stale mic-gain reapply (epoch {} superseded by {})",
                    epoch,
                    gain_epoch.load(Ordering::SeqCst)
                );
                return;
            }
            if let Some(dev) = device {
                let _ = apply_mic_gain_oneshot(&dev, gain).await;
            }
        });
        Ok(changed)
    }

    /// Apply configuration and device together
    #[allow(dead_code)]
    pub async fn apply(&mut self, config: &AudioConfig, device: &DeviceInfo) -> Result<()> {
        self.config = config.clone();
        self.device = Some(device.clone());
        self.apply_full().await?;
        Ok(())
    }

    // ─── Mic Gain ─────────────────────────────────────────────

    pub async fn apply_mic_gain(&self) -> Result<()> {
        let Some(ref device) = self.device else {
            debug!("No device, skipping mic gain");
            return Ok(());
        };
        apply_mic_gain_oneshot(device, self.config.mic_gain).await
    }

    // ─── EPOS instance confs (Option B) ───────────────────────
    //
    // Each role owns ONE self-contained pipewire.conf under
    // ~/.config/pipewire-epos/<role>/pipewire.conf. The daemon regenerates the
    // whole file (base skeleton + generated DSP module) atomically, skips the
    // restart when the bytes are unchanged, and restarts only that instance.

    fn instance_conf_path(role: &str) -> std::path::PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
            .join("pipewire-epos")
            .join(role)
            .join("pipewire.conf")
    }

    /// Atomically write a generated instance conf. Returns true if the file
    /// actually changed (caller decides whether to restart that instance).
    fn write_instance_conf(&self, role: &str, conf: &str) -> Result<bool> {
        let path = Self::instance_conf_path(role);
        if let Ok(existing) = std::fs::read(&path) {
            if existing == conf.as_bytes() {
                debug!("{role} instance conf unchanged — skip restart");
                return Ok(false);
            }
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let tmp = path.with_extension("conf.tmp");
        std::fs::write(&tmp, conf)?;
        std::fs::rename(&tmp, &path)?;
        info!("{} instance conf written to {}", role, path.display());
        self.restarts.request(role);
        Ok(true)
    }

    /// Stable device node names for the conf templates. Falls back to the
    /// serial-embedded ALSA patterns when the cache is empty (device absent
    /// at write time — the names are constant across reboots).
    fn node_names(&self) -> (String, String) {
        match &self.device {
            Some(d) if !d.pipewire_sink.is_empty() && !d.pipewire_source.is_empty() => {
                (d.pipewire_sink.clone(), d.pipewire_source.clone())
            }
            _ => (
                "alsa_output.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.analog-stereo"
                    .to_string(),
                "alsa_input.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.mono-fallback"
                    .to_string(),
            ),
        }
    }

    // ─── Output routing (EQ on/off) ───────────────────────────
    //
    // Playback reaches the EQ only if new streams attach to the
    // `epos-eq-input` anchor. Apps otherwise pick the raw EPOS hardware sink
    // and the whole filter-chain is bypassed.
    //
    // WHY THE DEFAULT SINK AND NOT A WIREPLUMBER RULE
    // Both alternatives were tried and measured on this machine
    // (WirePlumber 0.5.17):
    //   * `target.object` in a device-node rule redirects ZERO streams.
    //   * rewriting each Stream's target feeds the `pipewire-epos@eq`
    //     instance's own output back into `epos-eq-input` (feedback loop),
    //     because that output targets the hardware sink by construction.
    // Moving the default sink changes which sink NEW streams attach to and
    // never touches existing stream targets, so the EQ instance's output is
    // never a redirect candidate and no loop is possible.
    //
    // Fail-closed: if the EQ instance is down, audio disappears into the
    // always-present null-sink anchor — silence, never a leak to A2+.

    /// Point the PipeWire default sink at `sink`. No-op on an empty name.
    async fn set_default_sink(sink: &str) -> Result<()> {
        if sink.is_empty() {
            return Ok(());
        }
        let out = tokio::process::Command::new("pactl")
            .args(["set-default-sink", sink])
            .output()
            .await?;
        if !out.status.success() {
            anyhow::bail!(
                "pactl set-default-sink {} failed: {}",
                sink,
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        Ok(())
    }

    /// Current PipeWire default sink name, or `None` if it cannot be read.
    async fn read_default_sink() -> Option<String> {
        let out = tokio::process::Command::new("pactl")
            .args(["get-default-sink"])
            .output()
            .await
            .ok()?;
        if !out.status.success() {
            return None;
        }
        let name = String::from_utf8_lossy(&out.stdout).trim().to_string();
        (!name.is_empty()).then_some(name)
    }

    /// Align the default sink with the EQ toggle. Returns true if it changed.
    ///
    /// Only acts on a difference, so a manual override in pavucontrol is
    /// preserved until the next transition (EQ toggled, or device replugged)
    /// rather than being fought on every poll.
    pub async fn route_output(&self) -> Result<bool> {
        let eq_enabled = self.config.eq.enabled;
        let device_connected = self
            .device
            .as_ref()
            .is_some_and(|d| !d.pipewire_sink.is_empty());
        let Some(route) = desired_output_route(eq_enabled, device_connected) else {
            // Device absent: leave the user's current default alone.
            return Ok(false);
        };
        let (raw_sink, _) = self.node_names();
        let target = match route {
            OutputRoute::Processed => EQ_SINK_NAME.to_string(),
            OutputRoute::Raw => raw_sink,
        };
        if target.is_empty() {
            return Ok(false);
        }
        if Self::read_default_sink().await.as_deref() == Some(target.as_str()) {
            return Ok(false);
        }
        Self::set_default_sink(&target).await?;
        info!(
            "Output routed to {} (EQ {})",
            target,
            if eq_enabled { "on" } else { "off" }
        );
        Ok(true)
    }

    // ─── 9-Band EQ (pipewire-epos@eq) ─────────────────────────
    //
    // The EQ instance ALWAYS runs a filter-chain: flat bands → passthrough
    // (copy) so the epos-eq-input sink keeps existing and headset audio keeps
    // flowing when EQ is off. The chain is published into MAIN as an
    // Audio/Sink named `epos-eq-input`; apps target it; playback auto-links
    // to the EPOS hardware sink. Kill the instance → sink vanishes → pinned
    // streams go silent (fail-closed), A2+ untouched.

    fn write_eq_conf(&self) -> Result<bool> {
        let (sink, _src) = self.node_names();
        let bands = if self.config.eq.enabled {
            self.config.eq.bands.as_slice()
        } else {
            &[]
        };
        let eq_conf = generate_eq_instance_conf(bands, &sink);
        self.write_instance_conf("eq", &eq_conf)
    }

    /// Apply EQ from config — write the eq instance conf.
    ///
    /// Also regenerates the voice instance conf, because voice mode `custom`
    /// mirrors these same bands. Without this the stored custom_bands were
    /// updated correctly while the running voice filter-chain kept the old
    /// values, so the UI showed the new EQ but the mic sounded unchanged.
    ///
    /// Returns true if either on-disk conf changed (restart needed).
    pub async fn apply_eq(&mut self) -> Result<bool> {
        let mut changed = self.write_eq_conf()?;
        if self.config.voice_enhancer.mode == VoiceMode::Custom {
            changed |= self.write_voice_conf()?;
        }
        Ok(changed)
    }

    // ─── Voice + Noise Gate (pipewire-epos@voice) ──────────────
    //
    // One filter-chain owns the whole mic path: capture EPOS source → rnnoise
    // (if enabled) → voice EQ bands (if mode != off) → virtual Audio/Source
    // `epos-voice-output` in MAIN (Discord etc. keep using the same name).
    // Nothing enabled → passthrough copy so the source always exists.

    fn write_voice_conf(&self) -> Result<bool> {
        let (_sink, source) = self.node_names();
        let (noise_gate, voice_mode, voice_bands) = {
            let ng = &self.config.noise_gate;
            let ve = self.config.voice_enhancer.clone();
            let bands = match &ve.mode {
                VoiceMode::Warm => vec![
                    (200u32, 4.0f32, 0.8f32),
                    (350, 3.0, 1.0),
                    (500, 2.0, 1.0),
                    (4000, -1.0, 1.2),
                    (8000, -2.0, 1.0),
                ],
                VoiceMode::Clear => vec![
                    (200, -2.0, 1.0),
                    (500, -1.0, 1.0),
                    (2500, 3.0, 1.0),
                    (4000, 4.0, 0.8),
                    (6000, 3.0, 1.2),
                ],
                VoiceMode::Custom => {
                    // Sanitised: custom bands arrive straight from the client
                    // (and from config.json), so they are untrusted input.
                    sanitize_bands(ve.custom_bands.as_deref().unwrap_or(&[]))
                }
                VoiceMode::Off => Vec::new(),
            };
            (
                ng.enabled,
                ve.mode.clone(),
                bands
                    .into_iter()
                    .filter(|(_f, g, _q)| g.abs() >= 0.1)
                    .collect::<Vec<_>>(),
            )
        };
        let vad_threshold = if noise_gate {
            50.0 + ((-self.config.noise_gate.threshold_db).clamp(0.0, 60.0) / 60.0) * 50.0
        } else {
            0.0
        };
        let voice_conf = generate_voice_instance_conf(
            &source,
            noise_gate,
            vad_threshold,
            &voice_mode,
            &voice_bands,
        );
        self.write_instance_conf("voice", &voice_conf)
    }

    /// Apply noise gate — regenerates the voice instance conf.
    pub async fn apply_noise_gate(&self) -> Result<bool> {
        self.write_voice_conf()
    }

    /// Apply voice enhancer — regenerates the voice instance conf.
    pub async fn apply_voice_enhancer(&self) -> Result<bool> {
        self.write_voice_conf()
    }

    // ─── Sidetone (pipewire-epos@sidetone) ─────────────────────
    //
    // module-loopback inside the instance: capture EPOS source (MONO, no
    // remix) → play EPOS sink (stereo, volume prop). Keep the instance bare
    // (no module) when disabled → zero links, mic untouched.

    fn write_sidetone_conf(&self) -> Result<bool> {
        if !self.config.sidetone.enabled {
            // Disabled → write a bare instance conf (no DSP module).
            let conf = instance_base_conf("sidetone", "");
            return self.write_instance_conf("sidetone", &conf);
        }
        let (sink, source) = self.node_names();
        let level = self.config.sidetone.level.clamp(0.0, 1.0);
        let module = format!(
            r#"
    {{ name = libpipewire-module-loopback
      args = {{
        remote.name = "pipewire-0"
        node.description = "EPOS GSX 300 Sidetone"
        media.name = "EPOS GSX 300 Sidetone"
        capture.props = {{
            node.name = "epos-sidetone-capture"
            target.object = "{source}"
            remote.name = "pipewire-0"
            audio.position = [ MONO ]
            stream.dont-remix = true
            node.passive = true
        }}
        playback.props = {{
            node.name = "epos-sidetone-output"
            target.object = "{sink}"
            remote.name = "pipewire-0"
            audio.position = [ FL FR ]
            channelmix.normalize = false
            volume = {vol:.2}
        }}
      }} }}
"#,
            source = source,
            sink = sink,
            vol = level,
        );
        let conf = instance_base_conf("sidetone", &module);
        self.write_instance_conf("sidetone", &conf)
    }

    /// Apply sidetone — regenerate the sidetone instance conf.
    /// Returns true if the conf changed (restart needed).
    pub async fn apply_sidetone(&mut self) -> Result<bool> {
        self.write_sidetone_conf()
    }

    /// In Option B the sidetone lives inside the pipewire-epos@sidetone
    /// instance (systemd Restart=always), not a daemon-owned child, so there
    /// is nothing to reap on shutdown. Kept as a no-op for call-site
    /// compatibility (`kill_sidetone().await`).
    pub async fn kill_sidetone(&mut self) {}
}

// ─── Config Generators (Option B instances) ──────────────────

/// Shared skeleton for an epos-instance pipewire.conf. `extra_modules` are
/// appended inside the single `context.modules` array (a conf file may only
/// define one such array; generated DSP modules live here).
fn instance_base_conf(role: &str, extra_modules: &str) -> String {
    format!(
        r#"# EPOS GSX 300 — {role} PipeWire instance (auto-generated by epos-gsx300d)
# Option B (3i): software-only instance, runs OUTSIDE the main pipewire graph.
# Restarted by the daemon on {role} changes — the main instance (A2+) is
# never touched at runtime.

context.properties = {{
  core.name = pipewire-epos-{role}
  remote.name = pipewire-epos-{role}
  core.daemon = true
  link-factory.enabled = false
  default.clock.rate         = 48000
  default.clock.allowed-rates = [ 48000 ]
  default.clock.quantum      = 256
  default.clock.min-quantum  = 256
  default.clock.max-quantum  = 256
}}
context.spa-libs = {{
  support.node.driver = support/libspa-support
  support.node        = support/libspa-support
  support.cpu         = support/libspa-support
  support.log         = support/libspa-support
}}
context.modules = [
  {{ name = libpipewire-module-rt }}
  {{ name = libpipewire-module-protocol-native }}
  {{ name = libpipewire-module-metadata }}
  {{ name = libpipewire-module-spa-node-factory }}
  {{ name = libpipewire-module-client-node }}
  {{ name = libpipewire-module-adapter }}{extra_modules}
]
context.objects = [
  {{ factory = spa-node-factory
    args = {{
      factory.name    = support.node.driver
      node.name       = Dummy-Driver
      node.group      = pipewire.dummy
      node.sync-group = sync.dummy
      priority.driver = 200000
    }}
  }}
]
"#,
        role = role,
        extra_modules = extra_modules,
    )
}

/// Generate the `eq` instance conf: 9-band EQ filter-chain that captures the
/// MAIN null-sink monitor `epos-eq-input.monitor` (the static fail-closed
/// anchor installed once at install time) and plays the processed audio to the
/// EPOS hardware sink. Apps keep targeting `epos-eq-input`; if this instance
/// dies, the anchor sink remains → streams stay silent, never routed to A2+.
/// Flat bands → passthrough `copy` node so the path always flows.
fn generate_eq_instance_conf(bands: &[epos_shared::config::EqBand], sink: &str) -> String {
    let mut nodes = String::new();
    let mut links = String::new();
    let mut prev: Option<String> = None;
    let mut active = 0usize;

    // Sanitised: the EQ bands arrive from the client and from config.json.
    for (i, (freq, gain, q)) in sanitize_bands(bands).into_iter().enumerate() {
        let name = format!("eq_band_{i}");
        nodes.push_str(&format!(
            r#"
                    {{
                        type  = builtin
                        name  = "{name}"
                        label = bq_peaking
                        control = {{ "Freq" = {freq} "Q" = {q} "Gain" = {gain} }}
                    }}"#,
            freq = freq,
            q = q,
            gain = gain,
        ));
        if let Some(p) = prev.take() {
            links.push_str(&format!(
                r#"
                    {{ output = "{p}:Out" input = "{name}:In" }}"#,
                p = p,
                name = name
            ));
        }
        prev = Some(name);
        active += 1;
    }

    if active == 0 {
        // Passthrough — EQ off / all flat: keep the path flowing.
        nodes.push_str(
            r#"
                    { type = builtin name = passthrough label = copy }"#,
        );
    } else {
        info!("EQ: {} active band(s)", active);
    }

    let module = format!(
        r#"
    {{ name = libpipewire-module-filter-chain
      args = {{
        node.description = "EPOS GSX 300 EQ"
        media.name = "EPOS GSX 300 EQ"
        filter.graph = {{
            nodes = [{nodes}
            ]
            links = [{links}
            ]
        }}
        audio.channels = 2
        audio.position = [ FL FR ]
        capture.props = {{
            node.name = "epos-eq-capture"
            target.object = "epos-eq-input.monitor"
            remote.name = "pipewire-0"
            node.passive = true
        }}
        playback.props = {{
            node.name = "epos-eq-output"
            target.object = "{sink}"
            remote.name = "pipewire-0"
            node.passive = true
        }}
      }} }}
"#,
        sink = sink,
        nodes = nodes,
        links = links,
    );
    instance_base_conf("eq", &module)
}

/// Generate the `voice` instance conf: ONE filter-chain owning the whole mic
/// path (rnnoise → voice EQ bands → virtual Audio/Source `epos-voice-output`
/// in MAIN). Nothing enabled → single `copy` passthrough so the source always
/// exists. rnnoise LADSPA port names are "Input"/"Output" (verified).
fn generate_voice_instance_conf(
    source: &str,
    noise_gate: bool,
    vad_threshold: f32,
    mode: &VoiceMode,
    bands: &[(u32, f32, f32)],
) -> String {
    let mut nodes = String::new();
    let mut links = String::new();
    let mut prev: Option<String> = None;

    if noise_gate {
        nodes.push_str(&format!(
            r#"
                    {{
                        type   = ladspa
                        name   = rnnoise
                        plugin = "librnnoise_ladspa"
                        label  = noise_suppressor_mono
                        control = {{
                            "VAD Threshold (%)" {vad_threshold:.1}
                        }}
                    }}"#
        ));
        prev = Some("rnnoise".to_string());
    }

    for (i, &(freq, gain, q)) in bands.iter().enumerate() {
        if gain.abs() < 0.1 {
            continue;
        }
        let name = format!("voice_band_{i}");
        nodes.push_str(&format!(
            r#"
                    {{
                        type  = builtin
                        name  = "{name}"
                        label = bq_peaking
                        control = {{ "Freq" = {freq} "Q" = {q} "Gain" = {gain} }}
                    }}"#
        ));
        if let Some(p) = prev.take() {
            // rnnoise LADSPA exposes `:Output`; PipeWire builtin bq_peaking uses `:Out`.
            let out_port = if p == "rnnoise" { "Output" } else { "Out" };
            links.push_str(&format!(
                r#"
                    {{ output = "{p}:{out_port}" input = "{name}:In" }}"#,
                p = p,
                out_port = out_port,
                name = name
            ));
        }
        prev = Some(name);
    }

    if prev.is_none() {
        // Nothing enabled → passthrough.
        nodes.push_str(
            r#"
                    { type = builtin name = passthrough label = copy }"#,
        );
    }

    info!(
        "Voice instance: noise_gate={} mode={:?} bands={}",
        noise_gate,
        mode,
        bands.len()
    );

    let module = format!(
        r#"
    {{ name = libpipewire-module-filter-chain
      args = {{
        node.description = "EPOS GSX 300 Voice Chain"
        media.name = "EPOS GSX 300 Voice Chain"
        filter.graph = {{
            nodes = [{nodes}
            ]
            links = [{links}
            ]
        }}
        audio.channels = 1
        audio.position = [ MONO ]
        capture.props = {{
            node.name = "epos-voice-capture"
            target.object = "{source}"
            media.class = Stream/Input/Audio
            remote.name = "pipewire-0"
            node.passive = true
        }}
        playback.props = {{
            node.name = "epos-voice-output"
            media.class = Audio/Source
            remote.name = "pipewire-0"
        }}
      }} }}
"#,
        source = source,
        nodes = nodes,
        links = links,
    );
    instance_base_conf("voice", &module)
}

/// Restart one epos instance (`systemctl --user restart pipewire-epos@<name>`)
/// and health-check it: the instance must come up AND publish its expected
/// node into MAIN (eq/voice always have one; sidetone only when its conf has
/// the loopback module). Returns success; leaves EPOS silent (fail-closed)
/// but never touches the main instance.
pub(crate) async fn restart_epos_instance(role: &str) -> bool {
    let svc = format!("pipewire-epos@{role}.service");
    info!("Restarting epos instance {role}");
    // Check whether the unit exists at all — a missing unit means the install
    // wasn't completed; fail closed (keep main untouched) and warn loudly.
    // The per-role DSP runs as a template INSTANCE of pipewire-epos@.service
    // (eq, voice, sidetone all pull from the one template unit file). An
    // instance like `pipewire-epos@voice.service` is NOT itself a unit file,
    // so `list-unit-files <instance>` is always empty; the unit file that
    // must exist for the install to be complete is the TEMPLATE. Enable is
    // "indirect" (instances are pulled from graphical-session.wants), which
    // is the complete-install state we care about.
    let template_file = "pipewire-epos@.service";
    let exists = matches!(
        run_probe(
            "systemctl",
            &["--user", "list-unit-files", template_file],
            PROBE_BUDGET
        )
        .await,
        Probe::Ran(out) if out.contains(template_file)
    );
    if !exists {
        warn!("epos instance unit {svc} not found — install not complete, EPOS silent");
        return false;
    }

    // `systemctl restart` legitimately takes a second or two, but it gets a cap
    // for the same reason the probes do: an unbounded await here would stall the
    // reload worker just as effectively as the pw-cli wedge did.
    const RESTART_CMD_BUDGET: Duration = Duration::from_secs(20);
    let restart = tokio::time::timeout(
        RESTART_CMD_BUDGET,
        tokio::process::Command::new("systemctl")
            .args(["--user", "restart", &svc])
            .stdin(std::process::Stdio::null())
            .kill_on_drop(true)
            .status(),
    )
    .await;
    let ok = match restart {
        Ok(Ok(s)) if s.success() => true,
        Ok(Ok(s)) => {
            warn!("epos instance restart {svc} returned {:?}", s.code());
            false
        }
        Ok(Err(e)) => {
            warn!("epos instance restart {svc} failed: {}", e);
            false
        }
        Err(_) => {
            warn!("epos instance restart {svc} exceeded {RESTART_CMD_BUDGET:?} — abandoned");
            false
        }
    };
    if !ok {
        return false;
    }

    // Health check: the instance must be up AND its control socket reachable
    // AND actually publish the DSP node this role exists to provide.
    //
    // Socket-only checking was not enough: an instance can be perfectly
    // reachable while its filter-chain module failed to register (a bad
    // generated conf, a missing LADSPA plugin such as rnnoise, a taken node
    // name). The audio path is then silent while the daemon reports success,
    // which is exactly the "the button does nothing" failure mode. Verifying
    // the node exists closes that gap. Never touches the main instance.
    //
    // The whole retry sequence is bounded: the caller must always get its turn
    // back, even if every attempt stalls.
    let verify = async {
        // A disabled role publishes nothing, so there is no node to demand.
        // An enabled one must publish its node, otherwise the conf was written
        // but the running instance never loaded it — the silent "the button
        // does nothing" failure this check exists to catch.
        let expected = expected_node(role);
        for attempt in 0..3 {
            let up = instance_reachable(role).await;
            let node_ok = match &expected {
                Some(n) => instance_node_present(role, n).await,
                None => true,
            };
            if up && node_ok {
                return true;
            }
            if attempt == 1 {
                warn!(
                    "epos instance {role}: unhealthy after restart \
                     (unit_active={up}, node_published={node_ok}) — retry once"
                );
            }
            tokio::time::sleep(Duration::from_millis(700)).await;
        }
        false
    };
    match tokio::time::timeout(RESTART_VERIFY_BUDGET, verify).await {
        Ok(healthy) => {
            if !healthy {
                warn!("epos instance {role}: still not reachable — EPOS path silent (fail-closed)");
            }
            healthy
        }
        Err(_) => {
            warn!(
                "epos instance {role}: verification exceeded {RESTART_VERIFY_BUDGET:?} \
                 — treating as unhealthy (fail-closed)"
            );
            false
        }
    }
}

/// Wall-clock cap for any external probe this module runs.
///
/// Generous enough for `systemctl is-active` and `pw-dump` (both measured at
/// well under 50 ms), small enough that a wedged tool cannot stall the reload
/// worker for more than a couple of seconds.
const PROBE_BUDGET: Duration = Duration::from_secs(5);

/// Total wall-clock cap for one role's post-restart verification, covering
/// every retry attempt. A role that cannot be verified in this window is
/// reported unhealthy and the worker moves on to the next role.
const RESTART_VERIFY_BUDGET: Duration = Duration::from_secs(12);

/// Result of a capped, non-interactive subprocess probe.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Probe {
    /// The process was started and exited; stdout is captured.
    Ran(String),
    /// The process outlived the budget and was killed.
    TimedOut,
    /// The process could not be started at all.
    SpawnFailed(String),
}

/// Run `program args...` with stdin closed and a hard wall-clock cap.
///
/// Every external probe goes through here. Two properties are load-bearing:
///
/// * **stdin is `/dev/null`.** `pw-cli` validates its arguments and, on a bad
///   one, drops into an interactive command loop instead of exiting. With an
///   inherited stdin that blocks forever — which is exactly how
///   `pw-cli -r … info 0` (rejected with "unknown global '0'") wedged the
///   reload worker permanently on 2026-09-24.
/// * **The wait is capped and the child is killed on drop.** `kill_on_drop`
///   plus `timeout` means a stuck probe leaves no orphan behind.
///
/// The exit status is deliberately not folded into the outcome: callers that
/// care use `systemctl is-active`'s stdout, and a non-zero status is normal
/// for `is-active` on a stopped unit.
async fn run_probe(program: &str, args: &[&str], budget: Duration) -> Probe {
    let child = tokio::process::Command::new(program)
        .args(args)
        .stdin(std::process::Stdio::null())
        .kill_on_drop(true)
        .output();
    match tokio::time::timeout(budget, child).await {
        Ok(Ok(o)) => Probe::Ran(String::from_utf8_lossy(&o.stdout).into_owned()),
        Ok(Err(e)) => Probe::SpawnFailed(e.to_string()),
        Err(_) => {
            warn!("probe `{program}` exceeded {budget:?} — abandoned");
            Probe::TimedOut
        }
    }
}

/// Does a `pw-dump` of the MAIN graph publish `node`?
///
/// `pw-dump` emits JSON, so the name is read out of each object's
/// `props["node.name"]` rather than grepped as text. A substring match would be
/// wrong in both directions: `epos-voice` would "find" `epos-voice-output`,
/// and a node merely mentioned in some other field would count as present.
/// Unparseable output is treated as "not present" so a broken probe can never
/// be mistaken for a healthy instance.
fn dump_has_node(dump: &str, node: &str) -> bool {
    if node.is_empty() {
        return false;
    }
    let root: serde_json::Value = match serde_json::from_str(dump) {
        Ok(v) => v,
        Err(e) => {
            warn!("pw-dump output is not valid JSON ({e}) — node check cannot be trusted");
            return false;
        }
    };
    fn walk(v: &serde_json::Value, node: &str) -> bool {
        match v {
            serde_json::Value::Object(m) => {
                if m
                    .get("props")
                    .and_then(|p| p.get("node.name"))
                    .and_then(|n| n.as_str())
                    == Some(node)
                {
                    return true;
                }
                m.values().any(|x| walk(x, node))
            }
            serde_json::Value::Array(a) => a.iter().any(|x| walk(x, node)),
            _ => false,
        }
    }
    walk(&root, node)
}

/// The MAIN-graph node this role must publish for its DSP path to be usable,
/// or `None` when the role is disabled and so legitimately publishes nothing.
///
/// Read from the generated conf rather than hardcoded, for two reasons:
/// a disabled role has no node, so demanding one would report a permanent
/// false failure; and the sidetone node is `epos-sidetone-output`, not the
/// `epos-sidetone` that was hardcoded before — that name matches nothing, so
/// the check could never succeed.
///
/// `eq` is the exception: `epos-eq-input` is the static null-sink installed in
/// MAIN by `40-epos-eq-virtualsink.conf` and exists whether or not the EQ
/// filter is engaged, so it is always expected.
fn expected_node(role: &str) -> Option<String> {
    if role == "eq" {
        return Some(EQ_SINK_NAME.to_string());
    }
    let marker = match role {
        "voice" => "epos-voice-output",
        "sidetone" => "epos-sidetone-output",
        // RestartBus::request asserts role ∈ {eq, voice, sidetone}; keep a
        // total match so this can never panic on a new role.
        _ => return None,
    };
    let conf = std::fs::read_to_string(AudioPipeline::instance_conf_path(role)).ok()?;
    conf.contains(&format!("node.name = \"{marker}\""))
        .then(|| marker.to_string())
}

/// Is the epos instance up?
///
/// Deliberately does **not** use `pw-cli`: that tool has an interactive
/// fallback and blocked here, wedging the caller indefinitely. The systemd
/// unit being active plus the instance's control socket existing answers the
/// same question using commands that cannot block, and
/// [`instance_node_present`] separately proves the DSP node is actually
/// published where the audio path needs it.
async fn instance_reachable(role: &str) -> bool {
    let unit = format!("pipewire-epos@{role}.service");
    let active = matches!(
        run_probe("systemctl", &["--user", "is-active", &unit], PROBE_BUDGET).await,
        Probe::Ran(out) if out.trim() == "active"
    );
    let socket_present = dirs::runtime_dir()
        .map(|dir| dir.join(format!("pipewire-epos-{role}")).exists())
        .unwrap_or(false);
    if !active || !socket_present {
        debug!("epos instance {role}: unit_active={active} socket={socket_present}");
    }
    active && socket_present
}

/// Does the epos instance actually publish `node` into the main graph?
///
/// Queried against the MAIN instance (`-r pipewire-0`) on purpose: the nodes
/// are published cross-instance by design, so asking the per-role instance
/// would not prove the application-visible path exists.
async fn instance_node_present(role: &str, node: &str) -> bool {
    if node.is_empty() {
        return false;
    }
    // `pw-dump` is a batch tool: no interactive fallback, ~15 ms, and `-r
    // pipewire-0` pins it to the MAIN graph explicitly so an inherited
    // PIPEWIRE_REMOTE cannot point it at a per-role instance instead. This is
    // where the role's node has to appear for the path to be usable.
    match run_probe("pw-dump", &["-r", "pipewire-0"], PROBE_BUDGET).await {
        Probe::Ran(dump) => dump_has_node(&dump, node),
        _ => {
            warn!("epos instance {role}: node check for '{node}' failed — assuming unhealthy");
            false
        }
    }
}

/// Set the GSX 300 capture gain via amixer, standalone so it can be
/// re-applied after WirePlumber restore-routes overwrites the element
/// during node activation (see AudioPipeline::apply_full).
async fn apply_mic_gain_oneshot(device: &DeviceInfo, gain: u32) -> Result<()> {
    // Refuse an unknown card instead of writing to card 0, which is some other
    // device's mixer. The hotplug poll retries once ALSA has enumerated.
    let card = usable_alsa_card(device.alsa_card)?;
    let gain = sanitize_mic_gain(gain);

    // The GSX 300 capture gain is the ALSA mixer element "Mic Capture
    // Volume" (numid=4, 0-35, 100% = 5 dB). Note: `amixer scontrols`
    // shows the SIMPLE name "Mic", but `amixer cset` matches the ELEMENT
    // name — those are different namespaces. cset "Mic" fails, so the
    // element name is the primary; "Mic" is kept as a fallback for
    // firmwares/quirks that rename the element.
    debug!("Setting mic gain to {}% on card {}", gain, card);

    let mut last_stderr = String::new();
    let mut success = false;
    for control in ["Mic Capture Volume", "Mic"] {
        let output = tokio::process::Command::new("amixer")
            .args([
                "-c",
                &card.to_string(),
                "cset",
                &format!("name='{}'", control),
                &format!("{}%", gain),
            ])
            .output()
            .await;

        match output {
            Ok(o) if o.status.success() => {
                info!("Mic gain set to {}% (control '{}')", gain, control);
                success = true;
                break;
            }
            Ok(o) => {
                last_stderr = String::from_utf8_lossy(&o.stderr).trim().to_string();
            }
            Err(e) => {
                last_stderr = e.to_string();
            }
        }
    }

    if !success {
        // Previously this only logged a warning and returned Ok(()), so a
        // failed amixer call was reported to the GUI as a successful gain
        // change and the bad value was persisted. Surface it as an error.
        let detail = if last_stderr.is_empty() {
            "amixer reported no usable capture control".to_string()
        } else {
            last_stderr
        };
        warn!("amixer mic gain failed: {}", detail);
        return Err(anyhow::anyhow!(
            "Failed to set mic gain to {}% on card {}: {}",
            gain,
            card,
            detail
        ));
    }
    Ok(())
}

impl Drop for AudioPipeline {
    fn drop(&mut self) {
        // In Option B the sidetone lives inside the pipewire-epos@sidetone
        // instance (systemd Restart=always), not a daemon-owned child, so there
        // is nothing to reap here. Restarting/maintaining that instance is the
        // RestartBus worker's job.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use epos_shared::config::EqBand;

    fn band(freq: u32, gain_db: f32, q: f32) -> EqBand {
        EqBand { freq, gain_db, q }
    }

    /// A band inside every documented limit must survive untouched, so normal
    /// EQ editing is never altered by the safety layer.
    #[test]
    fn valid_band_is_preserved_verbatim() {
        let out = sanitize_bands(&[band(1000, -3.0, 1.2)]);
        assert_eq!(out, vec![(1000u32, -3.0f32, 1.2f32)]);
    }

    /// The two Warm/Clear presets must survive the sanitiser, otherwise the
    /// built-in voice modes would be silently broken by the safety layer.
    #[test]
    fn built_in_voice_presets_survive_sanitisation() {
        let warm = [
            (200u32, 4.0f32, 0.8f32),
            (350, 3.0, 1.0),
            (500, 2.0, 1.0),
            (4000, -1.0, 1.2),
            (8000, -2.0, 1.0),
        ];
        let bands: Vec<EqBand> = warm.iter().map(|&(f, g, q)| band(f, g, q)).collect();
        assert_eq!(sanitize_bands(&bands), warm.to_vec());

        let clear = [
            (200u32, -2.0f32, 1.0f32),
            (500, -1.0, 1.0),
            (2500, 3.0, 1.0),
            (4000, 4.0, 0.8),
            (6000, 3.0, 1.2),
        ];
        let bands: Vec<EqBand> = clear.iter().map(|&(f, g, q)| band(f, g, q)).collect();
        assert_eq!(sanitize_bands(&bands), clear.to_vec());
    }

    /// A flat (or near-flat) band is a no-op filter; it must be dropped rather
    /// than written into the PipeWire config as a useless bq_peaking node.
    #[test]
    fn flat_bands_are_dropped() {
        assert!(sanitize_bands(&[band(1000, 0.0, 1.0)]).is_empty());
        // 0.05 dB is below the 0.1 dB epsilon.
        assert!(sanitize_bands(&[band(1000, 0.05, 1.0)]).is_empty());
        assert!(sanitize_bands(&[band(1000, -0.05, 1.0)]).is_empty());
    }

    /// freq is a u32, so 0 is representable and WAS reaching the generated
    /// config as `"Freq" = 0`. Sub-audible and ultrasonic bands are dropped.
    #[test]
    fn out_of_band_frequencies_are_dropped() {
        assert!(sanitize_bands(&[band(0, 6.0, 1.0)]).is_empty());
        assert!(sanitize_bands(&[band(19, 6.0, 1.0)]).is_empty());
        assert!(sanitize_bands(&[band(20001, 6.0, 1.0)]).is_empty());
        // Boundaries are inside the allowed range.
        assert_eq!(sanitize_bands(&[band(20, 6.0, 1.0)]), vec![(20u32, 6.0f32, 1.0f32)]);
        assert_eq!(sanitize_bands(&[band(20000, 6.0, 1.0)]), vec![(20000u32, 6.0f32, 1.0f32)]);
    }

    /// Non-finite values would be interpolated verbatim into the PipeWire
    /// config file. They must never survive.
    #[test]
    fn non_finite_values_are_dropped() {
        assert!(sanitize_bands(&[band(1000, f32::NAN, 1.0)]).is_empty());
        assert!(sanitize_bands(&[band(1000, f32::INFINITY, 1.0)]).is_empty());
        assert!(sanitize_bands(&[band(1000, f32::NEG_INFINITY, 1.0)]).is_empty());
        assert!(sanitize_bands(&[band(1000, 6.0, f32::NAN)]).is_empty());
        assert!(sanitize_bands(&[band(1000, 6.0, f32::INFINITY)]).is_empty());
    }

    /// Out-of-range gain/Q are clamped, not dropped: the band still carries the
    /// user's intent, just bounded to a safe range.
    #[test]
    fn gain_and_q_are_clamped_not_dropped() {
        assert_eq!(sanitize_bands(&[band(1000, 100.0, 1.0)]), vec![(1000u32, 24.0f32, 1.0f32)]);
        assert_eq!(sanitize_bands(&[band(1000, -100.0, 1.0)]), vec![(1000u32, -24.0f32, 1.0f32)]);
        assert_eq!(sanitize_bands(&[band(1000, 6.0, 0.0)]), vec![(1000u32, 6.0f32, 0.1f32)]);
        assert_eq!(sanitize_bands(&[band(1000, 6.0, -5.0)]), vec![(1000u32, 6.0f32, 0.1f32)]);
        assert_eq!(sanitize_bands(&[band(1000, 6.0, 1000.0)]), vec![(1000u32, 6.0f32, 30.0f32)]);
    }

    /// An unbounded band list could exhaust the per-instance
    /// MemoryMax=64M/TasksMax=32 budget and silence the audio path.
    #[test]
    fn band_count_is_capped() {
        let many: Vec<EqBand> = (0..500).map(|i| band(100 + i, 3.0, 1.0)).collect();
        assert_eq!(sanitize_bands(&many).len(), MAX_EQ_BANDS);
    }

    /// A mixed list: only the safe, in-range, non-flat bands may appear, in
    /// their original order.
    #[test]
    fn mixed_list_keeps_only_safe_bands_in_order() {
        let out = sanitize_bands(&[
            band(100, 5.0, 1.0),    // keep
            band(0, 5.0, 1.0),      // drop: freq 0
            band(200, 0.0, 1.0),    // drop: flat
            band(300, 50.0, 1.0),   // keep, gain clamped to 24
            band(400, 3.0, 0.0),    // keep, q clamped to 0.1
            band(50000, 3.0, 1.0),  // drop: ultrasonic
        ]);
        assert_eq!(
            out,
            vec![
                (100u32, 5.0f32, 1.0f32),
                (300, 24.0, 1.0),
                (400, 3.0, 0.1),
            ]
        );
    }

    /// Custom mode with no usable bands must produce an empty chain, which the
    /// caller turns into a passthrough `copy` node.
    #[test]
    fn empty_and_all_flat_custom_bands_yield_empty_chain() {
        assert!(sanitize_bands(&[]).is_empty());
        assert!(sanitize_bands(&[band(100, 0.0, 1.0), band(200, 0.0, 1.0)]).is_empty());
    }

    /// With EQ ON and the device present, new playback streams must attach to
    /// the processed anchor, otherwise the filter-chain is bypassed entirely.
    #[test]
    fn eq_on_with_device_routes_to_processed_sink() {
        assert_eq!(
            desired_output_route(true, true),
            Some(OutputRoute::Processed)
        );
    }

    /// With EQ OFF, playback must go straight to the EPOS hardware sink.
    #[test]
    fn eq_off_with_device_routes_to_raw_sink() {
        assert_eq!(desired_output_route(false, true), Some(OutputRoute::Raw));
    }

    /// While the EPOS is unplugged the daemon must NOT claim the default sink.
    /// There is no raw EPOS sink to fall back to, and stealing the default
    /// would yank audio away from whatever device the user is really using.
    #[test]
    fn absent_device_never_touches_the_default_sink() {
        for eq_enabled in [true, false] {
            assert_eq!(
                desired_output_route(eq_enabled, false),
                None,
                "eq_enabled={eq_enabled} must not touch the default while absent"
            );
        }
    }

    /// The anchor name must match the one installed by
    /// 40-epos-eq-virtualsink.conf, otherwise routing points at nothing.
    #[test]
    fn processed_route_uses_the_installed_anchor_name() {
        assert_eq!(EQ_SINK_NAME, "epos-eq-input");
    }

    /// Custom bands are stored verbatim in the config, and the sanitiser is
    /// what stands between an untrusted config file and the generated PipeWire
    /// conf. A custom band list that survived sanitising must reach the filter
    /// graph as the same (freq, gain, q) tuples.
    #[test]
    fn custom_bands_reach_the_graph_unchanged() {
        let custom = vec![band(250, 4.5, 1.0), band(3000, -2.0, 0.7)];
        assert_eq!(
            sanitize_bands(&custom),
            vec![(250u32, 4.5f32, 1.0f32), (3000u32, -2.0f32, 0.7f32)]
        );
    }

    // ── Capped subprocess probes ──────────────────────────────────────────
    //
    // Regression: the post-restart health check ran
    //   pw-cli -r pipewire-epos-<role> info 0
    // through a bare `.output().await`. `pw-cli` rejects the bogus object id
    // ("unknown global '0'") and then falls back to its interactive command
    // loop, blocking on stdin forever. The restart worker awaited that future
    // with no cap, so the worker wedged permanently on the first DSP change
    // and no epos instance was ever restarted again. A live hung `pw-cli`
    // child of the daemon was observed doing exactly this.
    //
    // The contract these tests pin down is the fix: a probe ALWAYS returns,
    // and it can never wait on stdin.

    /// A command that never exits must be abandoned at the budget, not awaited
    /// forever. This is the wedge that silently froze the whole reload worker.
    #[tokio::test]
    async fn probe_gives_up_on_a_command_that_never_exits() {
        let started = std::time::Instant::now();
        let outcome = run_probe("sleep", &["30"], Duration::from_millis(150)).await;
        assert_eq!(outcome, Probe::TimedOut);
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "probe must return near its budget, took {:?}",
            started.elapsed()
        );
    }

    /// stdin is closed, so a tool that tries to read it (or drop into an
    /// interactive prompt) gets EOF and exits instead of blocking.
    #[tokio::test]
    async fn probe_cannot_block_on_stdin() {
        // `cat` with an inherited terminal/stdin would block forever. With a
        // null stdin it sees EOF immediately and returns empty.
        let outcome = run_probe("cat", &[], Duration::from_secs(5)).await;
        match outcome {
            Probe::Ran(stdout) => assert_eq!(stdout, ""),
            other => panic!("expected a clean empty read, got {other:?}"),
        }
    }

    /// The happy path still returns real stdout — the cap must not truncate
    /// or discard data from probes that do answer.
    #[tokio::test]
    async fn probe_returns_stdout_when_the_command_answers() {
        let outcome = run_probe("echo", &["epos-voice-output"], Duration::from_secs(5)).await;
        assert_eq!(outcome, Probe::Ran("epos-voice-output\n".to_string()));
    }

    /// A missing binary is a spawn failure, reported as such — never a hang and
    /// never silently reported as a healthy probe.
    #[tokio::test]
    async fn probe_reports_a_missing_program() {
        let outcome = run_probe(
            "epos-no-such-probe-binary",
            &[],
            Duration::from_secs(5),
        )
        .await;
        assert!(matches!(outcome, Probe::SpawnFailed(_)), "got {outcome:?}");
    }

    /// The node-presence check reads real `pw-dump` output, which is JSON. An
    /// earlier version matched a `node.name = "…"` text shape that `pw-dump`
    /// never emits, so it reported every role as unhealthy. This pins the real
    /// format, and pins that a missing node cannot pass by accident.
    #[test]
    fn node_presence_reads_real_pw_dump_json() {
        // Trimmed from a real `pw-dump -r pipewire-0` run.
        let dump = r#"[ { "info": { "props": { "node.name": "epos-sidetone-output",
                  "media.class": "Stream/Output/Audio" } } },
                { "info": { "props": { "node.name": "epos-eq-input" } } } ]"#;
        assert!(dump_has_node(dump, "epos-sidetone-output"));
        assert!(dump_has_node(dump, "epos-eq-input"));
        // Absent, and a prefix must not match a real node.
        assert!(!dump_has_node(dump, "epos-voice-output"));
        assert!(!dump_has_node(dump, "epos-sidetone"));
    }

    /// Corrupt probe output must read as "not present", never as healthy.
    #[test]
    fn unparseable_dump_is_not_treated_as_healthy() {
        assert!(!dump_has_node("not json at all", "epos-voice-output"));
        assert!(!dump_has_node("[{\"info\":{\"props\":{}}}]", "epos-voice-output"));
    }

    /// `epos-eq-input` is the static null-sink and must be expected whether or
    /// not the EQ is engaged — it is what apps target either way.
    #[test]
    fn eq_role_always_expects_the_anchor_sink() {
        assert_eq!(expected_node("eq").as_deref(), Some("epos-eq-input"));
    }

    /// An unknown role expects nothing rather than panicking. The restart bus
    /// already constrains the role set, but a total match keeps this safe.
    #[test]
    fn unknown_role_expects_no_node() {
        assert_eq!(expected_node("nonsense"), None);
        assert_eq!(expected_node(""), None);
    }

    // ── ALSA card: "not found" must never become card 0 ────────────────────
    //
    // `find_alsa_card(...).unwrap_or(0)` used 0 as a sentinel for "ALSA has not
    // enumerated this device yet". `apply_mic_gain_oneshot` then ran
    // `amixer -c 0 …`, which is a real, different sound card — so an EPOS
    // unplug/replug race could move somebody else's input gain. The card index
    // has to stay unknown until it is actually known.

    /// A missing ALSA card is an error, never a fallback to card 0.
    #[test]
    fn unknown_alsa_card_is_refused() {
        assert!(usable_alsa_card(None).is_err());
    }

    /// Card 0 is a legitimate ALSA index, not a sentinel. This case exists to
    /// stop anyone "fixing" the bug above by rejecting 0, which would break
    /// every user whose headset really is on card 0.
    #[test]
    fn alsa_card_zero_is_a_real_card() {
        assert_eq!(usable_alsa_card(Some(0)).unwrap(), 0);
        assert_eq!(usable_alsa_card(Some(4)).unwrap(), 4);
    }

    // ── mic_gain is a percentage, bounded at the pipeline ──────────────────

    /// `amixer` accepts values above 100% and silently saturates, so an
    /// unclamped gain looks successful while the hardware sits pinned at max.
    /// Every path that can set a gain — direct IPC, a profile, a hand-edited
    /// config.json, Reload — funnels through here.
    #[test]
    fn mic_gain_is_bounded_to_a_percentage() {
        assert_eq!(sanitize_mic_gain(0), 0);
        assert_eq!(sanitize_mic_gain(1), 1);
        assert_eq!(sanitize_mic_gain(100), 100);
        assert_eq!(sanitize_mic_gain(101), 100);
        assert_eq!(sanitize_mic_gain(5_000), 100);
        assert_eq!(sanitize_mic_gain(u32::MAX), 100);
    }

    /// The bound is applied where config enters the pipeline, not only at the
    /// IPC edge — a profile carrying 5000 must not reach `amixer` either.
    #[test]
    fn pipeline_clamps_an_oversized_gain_from_any_source() {
        let mut cfg = AudioConfig::default();
        cfg.mic_gain = 5_000;
        let pipeline = AudioPipeline::new(&cfg);
        assert_eq!(pipeline.config.mic_gain, 100);
    }
}
