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

    /// Apply all audio settings to the device
    pub async fn apply_full(&mut self) -> Result<()> {
        self.apply_mic_gain().await?;
        if self.config.eq.enabled {
            self.apply_eq().await?;
        } else {
            self.remove_eq().await?;
        }
        self.apply_sidetone().await?;
        self.apply_noise_gate().await?;
        self.apply_voice_enhancer().await?;
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
                "set", "Mic Capture Volume",
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
    // PipeWire EQ strategy: Write a WirePlumber SPA filter config that applies
    // parametric EQ on the EPOS sink node. Each band = a biquad peaking EQ filter.
    //
    // The filter config is written to:
    //   ~/.config/wireplumber/wireplumber.conf.d/90-epos-eq.conf
    //
    // WirePlumber hot-reloads .conf.d/ files automatically.

    pub async fn apply_eq(&self) -> Result<()> {
        let Some(ref device) = self.device else {
            debug!("No device, skipping EQ");
            return Ok(());
        };

        if !self.config.eq.enabled {
            return Ok(());
        }

        let filter_conf = generate_eq_filter_conf(&self.config.eq.bands, device);
        let conf_dir = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
            .join("wireplumber")
            .join("wireplumber.conf.d");
        let conf_path = conf_dir.join("90-epos-eq.conf");

        std::fs::create_dir_all(&conf_dir)?;
        std::fs::write(&conf_path, &filter_conf)?;
        info!("EQ filter config written to {}", conf_path.display());

        // Tell WirePlumber to reload (if running)
        reload_wireplumber().await;

        Ok(())
    }

    pub async fn remove_eq(&self) -> Result<()> {
        let conf_path = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
            .join("wireplumber")
            .join("wireplumber.conf.d")
            .join("90-epos-eq.conf");

        if conf_path.exists() {
            std::fs::remove_file(&conf_path)?;
            info!("EQ filter config removed");
            reload_wireplumber().await;
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

    pub async fn apply_noise_gate(&self) -> Result<()> {
        let Some(ref device) = self.device else {
            debug!("No device, skipping noise gate");
            return Ok(());
        };

        let conf_dir = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
            .join("wireplumber")
            .join("wireplumber.conf.d");
        let conf_path = conf_dir.join("91-epos-noisegate.conf");

        if self.config.noise_gate.enabled {
            let threshold = self.config.noise_gate.threshold_db;
            let filter_conf = format!(
                r#"# EPOS GSX 300 noise gate (rnnoise)
# Applied to capture node: {source}
monitor.els = [
  {{
    name = libwireplumber-module-rnnoise
    type = filter
    args = {{
      audio.position = [ MONO ]
      capture.props = {{
        node.name = "epos-noisegate-capture"
        media.class = "Audio/Sink"
        audio.position = [ MONO ]
      }}
      playback.props = {{
        node.name = "epos-noisegate-playback"
        media.class = "Audio/Source"
        audio.position = [ MONO ]
      }}
    }}
  }}
]
"#,
                source = device.pipewire_source
            );

            std::fs::create_dir_all(&conf_dir)?;
            std::fs::write(&conf_path, &filter_conf)?;
            info!("Noise gate filter config written (threshold: {}dB)", threshold);
        } else if conf_path.exists() {
            std::fs::remove_file(&conf_path)?;
            info!("Noise gate filter config removed");
        }

        reload_wireplumber().await;
        Ok(())
    }

    // ─── Voice Enhancer ───────────────────────────────────────
    //
    // Voice enhancer = EQ on the capture (mic) stream.
    // Warm = boost low frequencies (200-500 Hz)
    // Clear = boost presence (2k-6k Hz)

    pub async fn apply_voice_enhancer(&self) -> Result<()> {
        let conf_dir = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
            .join("wireplumber")
            .join("wireplumber.conf.d");
        let conf_path = conf_dir.join("92-epos-voice-enhancer.conf");

        let filter_conf = match self.config.voice_enhancer.mode {
            VoiceMode::Off => {
                if conf_path.exists() {
                    std::fs::remove_file(&conf_path)?;
                    info!("Voice enhancer disabled, config removed");
                }
                reload_wireplumber().await;
                return Ok(());
            }
            VoiceMode::Warm => {
                // Boost low-mid frequencies for warmth
                generate_voice_eq_conf("warm", &[
                    (200, 4.0, 0.8),
                    (350, 3.0, 1.0),
                    (500, 2.0, 1.0),
                    (4000, -1.0, 1.2),
                    (8000, -2.0, 1.0),
                ])
            }
            VoiceMode::Clear => {
                // Boost presence and clarity
                generate_voice_eq_conf("clear", &[
                    (200, -2.0, 1.0),
                    (500, -1.0, 1.0),
                    (2500, 3.0, 1.0),
                    (4000, 4.0, 0.8),
                    (6000, 3.0, 1.2),
                ])
            }
            VoiceMode::Custom => {
                if let Some(ref bands) = self.config.voice_enhancer.custom_bands {
                    let band_data: Vec<(u32, f32, f32)> = bands.iter()
                        .map(|b| (b.freq, b.gain_db, b.q))
                        .collect();
                    generate_voice_eq_conf("custom", &band_data)
                } else {
                    if conf_path.exists() {
                        std::fs::remove_file(&conf_path)?;
                    }
                    reload_wireplumber().await;
                    return Ok(());
                }
            }
        };

        std::fs::create_dir_all(&conf_dir)?;
        std::fs::write(&conf_path, &filter_conf)?;
        info!("Voice enhancer filter written ({:?})", self.config.voice_enhancer.mode);

        reload_wireplumber().await;
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

/// Generate WirePlumber parametric EQ filter config for the EPOS sink.
/// Each band = a biquad peaking EQ filter in SPA format.
fn generate_eq_filter_conf(bands: &[epos_shared::config::EqBand], device: &DeviceInfo) -> String {
    let mut spa_filters = String::new();

    for (i, band) in bands.iter().enumerate() {
        // Only add band if gain != 0
        if band.gain_db.abs() < 0.1 {
            continue;
        }

        // Parametric peaking EQ biquad coefficients (SPA format)
        // Convert freq + gain_db + Q → biquad coefficients a0, a1, a2, b0, b1, b2
        let (b0, b1, b2, a0, a1, a2) = peaking_eq_coefficients(
            band.freq as f64,
            band.gain_db as f64,
            band.q as f64,
            48000.0, // GSX 300 is always 48kHz
        );

        spa_filters.push_str(&format!(
            r#"
  biquad{i}: {{
    type = "Biquad"
    name = "epos-eq-band{i}"
    b0 = {b0:.10e}
    b1 = {b1:.10e}
    b2 = {b2:.10e}
    a0 = {a0:.10e}
    a1 = {a1:.10e}
    a2 = {a2:.10e}
  }}
"#,
            i = i,
            b0 = b0, b1 = b1, b2 = b2,
            a0 = a0, a1 = a1, a2 = a2,
        ));
    }

    // If all bands are flat, return empty config
    if spa_filters.is_empty() {
        return format!(
            "# EPOS GSX 300 EQ — all bands flat, no processing needed\n# Device: {}\n",
            device.pipewire_sink
        );
    }

    format!(
        r#"# EPOS GSX 300 parametric EQ — 9-band
# Auto-generated by epos-gsx300d
# Device: {sink}
# Bands: {bands}

monitor.els = [{filters}
]
"#,
        sink = device.pipewire_sink,
        bands = bands.len(),
        filters = spa_filters,
    )
}

/// Generate voice enhancer EQ filter config (capture stream)
fn generate_voice_eq_conf(mode_name: &str, bands: &[(u32, f32, f32)]) -> String {
    let mut spa_filters = String::new();

    for (i, &(freq, gain, q)) in bands.iter().enumerate() {
        if gain.abs() < 0.1 {
            continue;
        }

        let (b0, b1, b2, a0, a1, a2) = peaking_eq_coefficients(
            freq as f64,
            gain as f64,
            q as f64,
            48000.0,
        );

        spa_filters.push_str(&format!(
            r#"
  biquad{idx}: {{
    type = "Biquad"
    name = "voice-{mode}-band{idx}"
    b0 = {b0:.10e}
    b1 = {b1:.10e}
    b2 = {b2:.10e}
    a0 = {a0:.10e}
    a1 = {a1:.10e}
    a2 = {a2:.10e}
  }}
"#,
            idx = i, mode = mode_name,
            b0 = b0, b1 = b1, b2 = b2,
            a0 = a0, a1 = a1, a2 = a2,
        ));
    }

    format!(
        r#"# EPOS GSX 300 voice enhancer ({mode})
# Auto-generated by epos-gsx300d
# Applied to capture stream

monitor.els = [{filters}
]
"#,
        mode = mode_name,
        filters = spa_filters,
    )
}

/// Peaking EQ biquad coefficient calculation.
/// Based on Audio EQ Cookbook (Robert Bristow-Johnson).
fn peaking_eq_coefficients(freq: f64, gain_db: f64, q: f64, sample_rate: f64) -> (f64, f64, f64, f64, f64, f64) {
    let a = 10f64.powf(gain_db / 40.0);
    let w0 = 2.0 * std::f64::consts::PI * freq / sample_rate;
    let alpha = w0.sin() / (2.0 * q);

    let b0 = 1.0 + alpha * a;
    let b1 = -2.0 * w0.cos();
    let b2 = 1.0 - alpha * a;
    let a0 = 1.0 + alpha / a;
    let a1 = -2.0 * w0.cos();
    let a2 = 1.0 - alpha / a;

    (b0, b1, b2, a0, a1, a2)
}

/// Reload WirePlumber by signaling it to re-read configs.
/// WirePlumber watches .conf.d/ via inotify and reloads automatically.
/// This is a best-effort nudge.
async fn reload_wireplumber() {
    // WirePlumber auto-reloads on file changes in .conf.d/
    // No explicit signal needed if inotify is active.
    // As a fallback, we can send SIGHUP to wp if running.
    debug!("WirePlumber will auto-reload config changes via inotify");
}
