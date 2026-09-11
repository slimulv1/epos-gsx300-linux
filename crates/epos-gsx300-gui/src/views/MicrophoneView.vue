<script setup lang="ts">
import { computed } from "vue";
import { useDaemonStore } from "../stores/daemon";

const store = useDaemonStore();
const disconnected = computed(() => !store.status?.device_connected);

/* ─── Voice enhancer ─── */
const voiceModes = ["off", "warm", "clear", "custom"] as const;
const voiceMode = computed(() => store.audio?.voice_enhancer?.mode ?? "off");
function onVoiceSelect(mode: string) {
  if (mode === "custom") {
    store.setVoiceEnhancer("custom", store.audio?.eq?.bands);
  } else {
    store.setVoiceEnhancer(mode);
  }
}

/* ─── Mic gain ─── */
const micGain = computed(() => store.audio?.mic_gain ?? 50);
function onMicGainChange(gain: number) {
  store.setMicGain(gain);
}

/* ─── Noise gate ─── */
const gateEnabled = computed(() => store.audio?.noise_gate?.enabled ?? false);
const gateThreshold = computed(() => store.audio?.noise_gate?.threshold_db ?? -30);
function onGateToggle(enabled: boolean) {
  store.setNoiseGate(enabled, gateThreshold.value);
}
function onGateThresholdChange(threshold: number) {
  store.setNoiseGate(gateEnabled.value, threshold);
}
</script>

<template>
  <div class="mic-view" :class="{ disabled: disconnected }">
    <!-- Voice Enhancer -->
    <div class="section-card">
      <h3 class="section-title">VOICE ENHANCER</h3>
      <div class="voice-grid">
        <button
          v-for="m in voiceModes"
          :key="m"
          :class="['voice-btn', voiceMode === m && 'active']"
          @click="onVoiceSelect(m)"
          :disabled="disconnected"
        >
          {{ m.charAt(0).toUpperCase() + m.slice(1) }}
        </button>
      </div>
    </div>

    <!-- Mic Gain -->
    <div class="section-card">
      <h3 class="section-title">MIC GAIN</h3>
      <div class="slider-row">
        <input
          type="range"
          class="accent-range"
          :min="0"
          :max="100"
          :value="micGain"
          :disabled="disconnected"
          @input="onMicGainChange(($event.target as HTMLInputElement).valueAsNumber)"
        />
        <span class="value-badge">{{ micGain }}%</span>
      </div>
    </div>

    <!-- Noise Gate -->
    <div class="section-card">
      <div class="section-header">
        <h3 class="section-title">NOISE GATE</h3>
        <label class="toggle-wrap">
          <input
            type="checkbox"
            :checked="gateEnabled"
            :disabled="disconnected"
            @change="onGateToggle(($event.target as HTMLInputElement).checked)"
          />
          <span class="toggle-track"><span class="toggle-thumb" /></span>
        </label>
      </div>
      <div class="slider-row">
        <input
          type="range"
          class="accent-range"
          :min="-60"
          :max="0"
          :value="gateThreshold"
          :disabled="!gateEnabled || disconnected"
          @input="onGateThresholdChange(($event.target as HTMLInputElement).valueAsNumber)"
        />
        <span class="value-badge">{{ gateThreshold }}dB</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.mic-view {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}
.mic-view.disabled {
  opacity: 0.45;
  pointer-events: none;
}

.section-card {
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: var(--space-3);
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-2);
}

.section-title {
  font-size: var(--fs-xs);
  color: var(--muted);
  font-weight: 600;
  letter-spacing: 0.5px;
  text-transform: uppercase;
  margin-bottom: var(--space-2);
}
.section-header .section-title {
  margin-bottom: 0;
}

/* Voice buttons */
.voice-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}
.voice-btn {
  padding: 10px;
  background: var(--panel-2);
  border: 1px solid var(--border);
  border-radius: 8px;
  color: var(--muted);
  font-size: var(--fs-sm);
  font-family: var(--font-ui);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--time-fast) var(--ease);
}
.voice-btn:hover {
  border-color: var(--accent-dim);
  color: var(--text);
}
.voice-btn.active {
  background: var(--accent-glow);
  border-color: var(--accent);
  color: var(--accent);
  box-shadow: 0 0 12px var(--accent-glow);
}

/* Slider */
.slider-row {
  display: flex;
  align-items: center;
  gap: 12px;
}
.accent-range {
  flex: 1;
  accent-color: var(--accent);
}
.value-badge {
  font-size: var(--fs-xs);
  color: var(--muted);
  min-width: 40px;
  text-align: right;
}

/* Toggle */
.toggle-wrap {
  position: relative;
  display: inline-block;
  cursor: pointer;
}
.toggle-wrap input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}
.toggle-track {
  display: block;
  width: 36px;
  height: 20px;
  background: var(--grid);
  border-radius: 20px;
  transition: background var(--time-fast) var(--ease);
}
.toggle-thumb {
  display: block;
  width: 16px;
  height: 16px;
  background: var(--muted);
  border-radius: 50%;
  margin: 2px;
  transition: all var(--time-fast) var(--ease);
}
.toggle-wrap input:checked + .toggle-track {
  background: var(--accent-dim);
}
.toggle-wrap input:checked + .toggle-track .toggle-thumb {
  transform: translateX(16px);
  background: var(--accent);
}

@media (max-width: 500px) {
  .voice-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}
</style>