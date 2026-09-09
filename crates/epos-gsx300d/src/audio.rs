use anyhow::Result;
use tracing::info;
use epos_shared::AudioConfig;
use epos_shared::device::DeviceInfo;

/// Audio pipeline — manages PipeWire integration
pub struct AudioPipeline {
    config: AudioConfig,
}

impl AudioPipeline {
    pub fn new(config: &AudioConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// Apply audio configuration to the device
    pub async fn apply(&mut self, device: &DeviceInfo) -> Result<()> {
        info!("Applying audio config to ALSA card {}", device.alsa_card);

        // Apply mic gain
        self.apply_mic_gain(device).await?;

        // Apply EQ if enabled
        if self.config.eq.enabled {
            self.apply_eq(device).await?;
        }

        // Apply sidetone if enabled
        if self.config.sidetone.enabled {
            self.apply_sidetone(device).await?;
        }

        // Apply noise gate if enabled
        if self.config.noise_gate.enabled {
            self.apply_noise_gate(device).await?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn update_config(&mut self, config: AudioConfig, device: Option<&DeviceInfo>) -> Result<()> {
        self.config = config;
        if let Some(dev) = device {
            self.apply(dev).await?;
        }
        Ok(())
    }

    async fn apply_mic_gain(&self, device: &DeviceInfo) -> Result<()> {
        // Set ALSA mixer control for mic gain
        let gain = self.config.mic_gain;
        let card = device.alsa_card;
        info!("Setting mic gain to {}% on card {}", gain, card);

        // amixer -c {card} set 'Mic Capture Volume' {gain}%
        tokio::process::Command::new("amixer")
            .args(["-c", &card.to_string(), "set", "Mic Capture Volume", &format!("{}%", gain)])
            .output()
            .await?;

        Ok(())
    }

    async fn apply_eq(&self, _device: &DeviceInfo) -> Result<()> {
        info!("Applying 9-band EQ");
        // TODO: PipeWire module-eq integration
        // For now, log the EQ settings
        for band in &self.config.eq.bands {
            info!("  {}Hz: {}dB (Q={})", band.freq, band.gain_db, band.q);
        }
        Ok(())
    }

    async fn apply_sidetone(&self, _device: &DeviceInfo) -> Result<()> {
        info!("Setting sidetone to {}", self.config.sidetone.level);
        // TODO: pw-loopback integration
        Ok(())
    }

    async fn apply_noise_gate(&self, _device: &DeviceInfo) -> Result<()> {
        info!("Setting noise gate threshold to {}dB", self.config.noise_gate.threshold_db);
        // TODO: rnnoise integration
        Ok(())
    }
}
