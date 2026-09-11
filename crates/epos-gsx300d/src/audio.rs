use anyhow::Result;
use tokio::process::Child;
use tracing::{info, warn, debug};
use epos_shared::config::{AudioConfig, VoiceMode};
use epos_shared::device::DeviceInfo;

/// Audio pipeline — manages PipeWire audio processing via subprocesses
///
/// Strategy (based on PipeWire research):
/// - EQ: WirePlumber parametric filter via wpctl / SPA config hot-reload
/// - Sidetone: pw-loopback capture→playback with volume control
/// - Noise gate: WirePlumber rnnoise filter
/// - Voice enhancer: Capture stream EQ (warm=bass boost, clear=presence boost)
/// - Mic gain: amixer ALSA mixer control
pub struct AudioPipeline {
    config: AudioConfig,
    device: Option<DeviceInfo>,
    sidetone_proc: Option<Child>,
    #[allow(dead_code)]
    eq_filter_path: Option<std::path::PathBuf>,
}

impl AudioPipeline {
    pub fn new(config: &AudioConfig) -> Self {
        Self {
            config: config.clone(),
            device: None,
            sidetone_proc: None,
            eq_filter_path: None,
        }
    }

    /// Store device reference for future operations
    pub fn set_device(&mut self, device: &DeviceInfo) {
        self.device = Some(device.clone());
    }

    /// Sync latest config into the pipeline before applying filters
    pub fn update_config(&mut self, config: &AudioConfig) {
        self.config = config.clone();
    }

    /// Apply all audio settings to the device
    pub async fn apply_full(&mut self) -> Result<()> {
        self.apply_mic_gain().await?;
        let mut changed = false;
        changed |= self.write_eq_conf()?;
        self.apply_sidetone().await?;
        changed |= self.write_noise_gate_conf()?;
        changed |= self.write_voice_conf()?;
        if changed {
            reload_pipewire().await;
        }
        Ok(())
    }

    /// Apply configuration and device together
    #[allow(dead_code)]
    pub async fn apply(&mut self, config: &AudioConfig, device: &DeviceInfo) -> Result<()> {
        self.config = config.clone();
        self.device = Some(device.clone());
        self.apply_full().await
    }

    // ─── Mic Gain ─────────────────────────────────────────────

    pub async fn apply_mic_gain(&self) -> Result<()> {
        let Some(ref device) = self.device else {
            debug!("No device, skipping mic gain");
            return Ok(());
        };

        let gain = self.config.mic_gain;
        let card = device.alsa_card;

        // Map 0-100% → amixer range. The GSX 300 mic has numid=4 "Mic Capture Volume"
        // Range: -30dB to +5dB. amixer handles the percentage mapping.
        debug!("Setting mic gain to {}% on card {}", gain, card);

        let output = tokio::process::Command::new("amixer")
            .args([
                "-c", &card.to_string(),
                "cset", "name='Mic Capture Volume'",
                &format!("{}%", gain),
            ])
            .output()
            .await;

        match output {
            Ok(o) if o.status.success() => {
                info!("Mic gain set to {}%", gain);
            }
            Ok(o) => {
                let stderr = String::from_utf8_lossy(&o.stderr);
                warn!("amixer mic gain failed: {}", stderr.trim());
            }
            Err(e) => {
                warn!("Failed to run amixer: {}", e);
            }
        }
        Ok(())
    }

    // ─── 9-Band EQ ────────────────────────────────────────────
    //
    // PipeWire EQ strategy: Write a filter-chain module config that
    // applies parametric EQ on the EPOS sink node. Uses PipeWire's
    // built-in bq_peaking filters (Audio EQ Cookbook) with Freq/Q/Gain.
    //
    // The config is written to:
    //   ~/.config/pipewire/pipewire.conf.d/50-epos-eq.conf
    //
    // Loaded by the main pipewire instance; requires a pipewire restart.

    /// Write or remove the EQ filter-chain config. Returns true if the
    /// on-disk config actually changed (caller decides whether to reload).
    fn write_eq_conf(&self) -> Result<bool> {
        let conf_dir = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
            .join("pipewire")
            .join("pipewire.conf.d");
        let conf_path = conf_dir.join("50-epos-eq.conf");

        if !self.config.eq.enabled {
            if conf_path.exists() {
                std::fs::remove_file(&conf_path)?;
                info!("EQ filter config removed");
                return Ok(true);
            }
            return Ok(false);
        }

        let Some(ref device) = self.device else {
            debug!("No device, skipping EQ");
            return Ok(false);
        };

        let filter_conf = generate_eq_filter_conf(&self.config.eq.bands, device);

        // All bands flat → remove config instead of writing a stub that
        // PipeWire rejects ("Invalid argument" → crash loop).
        if filter_conf.is_empty() {
            if conf_path.exists() {
                std::fs::remove_file(&conf_path)?;
                info!("EQ filter config removed (all bands flat)");
                return Ok(true);
            }
            return Ok(false);
        }

        std::fs::create_dir_all(&conf_dir)?;
        std::fs::write(&conf_path, &filter_conf)?;
        info!("EQ filter config written to {}", conf_path.display());

        Ok(true)
    }

    /// Apply EQ from config — write config + restart PipeWire if changed.
    pub async fn apply_eq(&mut self) -> Result<()> {
        if self.write_eq_conf()? {
            // PipeWire needs a restart to load the new filter-chain module
            reload_pipewire().await;
        }
        Ok(())
    }

    // ─── Sidetone ─────────────────────────────────────────────
    //
    // Sidetone = mix mic capture into playback so user hears themselves.
    // Strategy: spawn pw-loopback from EPOS source → EPOS sink.
    // Kill existing process when settings change.

    pub async fn apply_sidetone(&mut self) -> Result<()> {
        // Kill existing sidetone process
        if let Some(mut proc) = self.sidetone_proc.take() {
            let _ = proc.start_kill();
            info!("Killed old sidetone process");
        }

        if !self.config.sidetone.enabled {
            info!("Sidetone disabled");
            return Ok(());
        }

        let Some(ref device) = self.device else {
            debug!("No device, skipping sidetone");
            return Ok(());
        };

        let level = self.config.sidetone.level;

        // Build pw-loopback command:
        // pw-loopback captures from mic source and plays to headset sink
        // The volume prop controls the sidetone level
        let mut cmd = tokio::process::Command::new("pw-loopback");
        cmd.arg("--capture").arg(&device.pipewire_source);
        cmd.arg("--playback").arg(&device.pipewire_sink);
        cmd.arg("--capture-props");
        cmd.arg(format!(
            "audio.position=[MONO] stream.dont-remix=true node.passive=true"
        ));
        cmd.arg("--playback-props");
        // Set volume for sidetone level (0.0 to 1.0)
        let vol = level.clamp(0.0, 1.0);
        cmd.arg(format!(
            "audio.position=[MONO] channelmix.normalize=false volume={:.2}",
            vol
        ));

        match cmd.spawn() {
            Ok(child) => {
                self.sidetone_proc = Some(child);
                info!("Sidetone started at level {:.0}%", level * 100.0);
            }
            Err(e) => {
                warn!("Failed to spawn pw-loopback for sidetone: {}", e);
            }
        }

        Ok(())
    }

    // ─── Noise Gate ───────────────────────────────────────────
    //
    // Noise gate via WirePlumber rnnoise filter on the capture node.
    // Writes a filter config that WirePlumber loads automatically.

    /// Write or remove the noise-gate filter-chain config.
    /// Returns true if the on-disk config actually changed.
    fn write_noise_gate_conf(&self) -> Result<bool> {
        let conf_dir = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
            .join("pipewire")
            .join("pipewire.conf.d");
        let conf_path = conf_dir.join("93-epos-noisegate.conf");

        if !self.config.noise_gate.enabled {
            if conf_path.exists() {
                std::fs::remove_file(&conf_path)?;
                info!("Noise gate filter config removed");
                return Ok(true);
            }
            return Ok(false);
        }

        let Some(ref device) = self.device else {
            debug!("No device, skipping noise gate");
            return Ok(false);
        };

        if self.config.noise_gate.enabled {
            // Map threshold_db (-60..0) → VAD Threshold % (50..100):
            // lower dB threshold = more aggressive suppression.
            let vad_threshold = 50.0 + ((-self.config.noise_gate.threshold_db).clamp(0.0, 60.0) / 60.0) * 50.0;
            let filter_conf = format!(
                r#"# EPOS GSX 300 noise gate (rnnoise via LADSPA filter-chain)
# Applied to capture node: {source}
# Requires librnnoise_ladspa.so in LADSPA_PATH (e.g. ~/.local/lib/ladspa)
context.modules = [
    {{
        name = libpipewire-module-filter-chain
        flags = [ nofail ]
        args = {{
            node.description = "EPOS GSX 300 Noise Gate"
            media.name       = "EPOS GSX 300 Noise Gate"
            filter.graph = {{
                nodes = [
                    {{
                        type   = ladspa
                        name   = rnnoise
                        plugin = "librnnoise_ladspa"
                        label  = noise_suppressor_stereo
                        control = {{
                            "VAD Threshold (%)" {vad_threshold:.1}
                        }}
                    }}
                ]
            }}
            audio.position = [ FL FR ]
            capture.props = {{
                node.name   = "epos-noisegate-capture"
                target.object = "{source}"
                node.passive = true
            }}
            playback.props = {{
                node.name   = "epos-noisegate-output"
                media.class = Audio/Source
            }}
        }}
    }}
]
"#,
                source = device.pipewire_source
            );

            std::fs::create_dir_all(&conf_dir)?;
            std::fs::write(&conf_path, &filter_conf)?;
            info!("Noise gate filter written (rnnoise, capture: {})", device.pipewire_source);
        }

        Ok(true)
    }

    /// Apply noise gate from config — write config + restart PipeWire if changed.
    pub async fn apply_noise_gate(&self) -> Result<()> {
        if self.write_noise_gate_conf()? {
            reload_pipewire().await;
        }
        Ok(())
    }

    // ─── Voice Enhancer ───────────────────────────────────────
    //
    // Voice enhancer = EQ on the capture (mic) stream.
    // Warm = boost low frequencies (200-500 Hz)
    // Clear = boost presence (2k-6k Hz)
    //
    // Uses a PipeWire filter-chain source module (libpipewire-module-filter-chain)
    // written to ~/.config/pipewire/pipewire.conf.d/51-epos-voice-enhancer.conf

    /// Write or remove the voice-enhancer filter-chain config.
    /// Returns true if the on-disk config actually changed.
    fn write_voice_conf(&self) -> Result<bool> {
        let conf_dir = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
            .join("pipewire")
            .join("pipewire.conf.d");
        let conf_path = conf_dir.join("51-epos-voice-enhancer.conf");

        let device_source = self.device.as_ref().map(|d| d.pipewire_source.clone());

        let filter_conf = match self.config.voice_enhancer.mode {
            VoiceMode::Off => None,
            VoiceMode::Warm => {
                // Boost low-mid frequencies for warmth
                Some(generate_voice_eq_conf("warm", device_source.as_deref(), &[
                    (200, 4.0, 0.8),
                    (350, 3.0, 1.0),
                    (500, 2.0, 1.0),
                    (4000, -1.0, 1.2),
                    (8000, -2.0, 1.0),
                ]))
            }
            VoiceMode::Clear => {
                // Boost presence and clarity
                Some(generate_voice_eq_conf("clear", device_source.as_deref(), &[
                    (200, -2.0, 1.0),
                    (500, -1.0, 1.0),
                    (2500, 3.0, 1.0),
                    (4000, 4.0, 0.8),
                    (6000, 3.0, 1.2),
                ]))
            }
            VoiceMode::Custom => {
                if let Some(ref bands) = self.config.voice_enhancer.custom_bands {
                    let active_bands: Vec<(u32, f32, f32)> = bands.iter()
                        .filter(|b| b.gain_db.abs() >= 0.1)
                        .map(|b| (b.freq, b.gain_db, b.q))
                        .collect();
                    if active_bands.is_empty() {
                        // All gains 0 → behave like Off: remove config instead of
                        // writing a useless passthrough filter with no target.
                        None
                    } else {
                        info!("Custom voice: {} active band(s)", active_bands.len());
                        Some(generate_voice_eq_conf("custom", device_source.as_deref(), &active_bands))
                    }
                } else {
                    None
                }
            }
        };

        match filter_conf {
            Some(conf) => {
                std::fs::create_dir_all(&conf_dir)?;
                std::fs::write(&conf_path, &conf)?;
                info!("Voice enhancer filter written ({:?})", self.config.voice_enhancer.mode);
                Ok(true)
            }
            None => {
                if conf_path.exists() {
                    std::fs::remove_file(&conf_path)?;
                    info!("Voice enhancer disabled, config removed");
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }

    /// Apply voice enhancer — write config + restart PipeWire if changed.
    pub async fn apply_voice_enhancer(&self) -> Result<()> {
        if self.write_voice_conf()? {
            reload_pipewire().await;
        }
        Ok(())
    }
}

impl Drop for AudioPipeline {
    fn drop(&mut self) {
        // Kill sidetone process on shutdown
        if let Some(mut proc) = self.sidetone_proc.take() {
            let _ = proc.start_kill();
            info!("Killed sidetone process on shutdown");
        }
    }
}

// ─── Config Generators ────────────────────────────────────────

/// Generate PipeWire filter-chain config for the EPOS sink (playback EQ).
/// Uses PipeWire's built-in bq_peaking filters with Freq/Q/Gain controls.
fn generate_eq_filter_conf(bands: &[epos_shared::config::EqBand], device: &DeviceInfo) -> String {
    let mut nodes = String::new();
    let mut links = String::new();
    let mut prev: Option<String> = None;
    let mut active = 0usize;

    for (i, band) in bands.iter().enumerate() {
        // Only add band if gain != 0
        if band.gain_db.abs() < 0.1 {
            continue;
        }
        let name = format!("eq_band_{i}");
        nodes.push_str(&format!(
            r#"
                    {{
                        type  = builtin
                        name  = "{name}"
                        label = bq_peaking
                        control = {{ "Freq" = {freq} "Q" = {q} "Gain" = {gain} }}
                    }}"#,
            freq = band.freq as u32,
            q = band.q,
            gain = band.gain_db,
        ));
        if let Some(p) = prev.take() {
            links.push_str(&format!(
                r#"
                    {{ output = "{p}:Out" input = "{name}:In" }}"#,
                p = p, name = name
            ));
        }
        prev = Some(name);
        active += 1;
    }

    // If all bands are flat, return empty string so the caller REMOVES the
    // config file. A comment-only stub is REJECTED by PipeWire's conf parser
    // ("Invalid argument") and crashes the whole audio stack.
    if active == 0 {
        return String::new();
    }

    format!(
        r#"# EPOS GSX 300 parametric EQ - 9-band (PipeWire filter-chain)
# Auto-generated by epos-gsx300d
# Device: {sink}
# Bands: {bands}

context.modules = [
    {{
        name = libpipewire-module-filter-chain
        args = {{
            node.description = "EPOS GSX 300 EQ"
            media.name       = "EPOS GSX 300 EQ"
            filter.graph = {{
                nodes = [{nodes}
                ]
                links = [{links}
                ]
            }}
            audio.channels = 2
            audio.position = [ FL FR ]
            capture.props = {{
                node.name   = "epos-eq-input"
                media.class = Audio/Sink
                audio.position = [ FL FR ]
            }}
            playback.props = {{
                node.name   = "epos-eq-output"
                target.object = "{sink}"
                node.passive = true
            }}
        }}
    }}
]
"#,
        sink = device.pipewire_sink,
        bands = active,
        nodes = nodes,
        links = links,
    )
}

/// Generate voice enhancer EQ filter config (capture stream / mic).
/// PipeWire filter-chain source: captures from the EPOS mic source,
/// applies bq_peaking filters, exposes a virtual Audio/Source.
fn generate_voice_eq_conf(mode_name: &str, device_source: Option<&str>, bands: &[(u32, f32, f32)]) -> String {
    let mut nodes = String::new();
    let mut links = String::new();
    let mut prev: Option<String> = None;

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
                    }}"#,
            freq = freq, q = q, gain = gain,
        ));
        if let Some(p) = prev.take() {
            links.push_str(&format!(
                r#"
                    {{ output = "{p}:Out" input = "{name}:In" }}"#,
                p = p, name = name
            ));
        }
        prev = Some(name);
        
    }

    let target = device_source
        .map(|s| format!(r#"                target.object = "{s}""#))
        .unwrap_or_default();

    format!(
        r#"# EPOS GSX 300 voice enhancer ({mode}) - PipeWire filter-chain
# Auto-generated by epos-gsx300d
# Applied to capture stream

context.modules = [
    {{
        name = libpipewire-module-filter-chain
        args = {{
            node.description = "EPOS GSX 300 Voice Enhancer ({mode})"
            media.name       = "EPOS GSX 300 Voice Enhancer ({mode})"
            filter.graph = {{
                nodes = [{nodes}
                ]
                links = [{links}
                ]
            }}
            audio.channels = 2
            audio.position = [ FL FR ]
            capture.props = {{
                node.name   = "epos-voice-capture"
{target}
                node.passive = true
            }}
            playback.props = {{
                node.name   = "epos-voice-output"
                media.class = Audio/Source
            }}
        }}
    }}
]
"#,
        mode = mode_name,
        nodes = nodes,
        links = links,
        target = target,
    )
}

/// Reload PipeWire so filter-chain module configs take effect.
/// Restarts the user pipewire service (fast — <1s) to load new .conf.d files.
async fn reload_pipewire() {
    let status = tokio::process::Command::new("systemctl")
        .args(["--user", "restart", "pipewire"])
        .status()
        .await;
    match status {
        Ok(s) if s.success() => info!("PipeWire restarted to apply filter configs"),
        Ok(s) => warn!("PipeWire restart returned status {:?}", s.code()),
        Err(e) => warn!("Failed to restart pipewire: {}", e),
    }
}
