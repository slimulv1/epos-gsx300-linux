<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  config: {
    mic_gain: number;
    noise_gate: { enabled: boolean; threshold_db: number };
  };
}>();

const emit = defineEmits<{
  "update:micGain": [gain: number];
  "update:noiseGate": [enabled: boolean, threshold: number];
}>();

const gain = computed(() => props.config.mic_gain);
const gateEnabled = computed(() => props.config.noise_gate.enabled);
const gateThreshold = computed(() => props.config.noise_gate.threshold_db);
</script>

<template>
  <div class="mic-settings">
    <div class="section">
      <h2>Mic Gain</h2>
      <div class="slider-row">
        <input
          type="range"
          :min="0"
          :max="100"
          :value="gain"
          class="range"
          @input="emit('update:micGain', ($event.target as HTMLInputElement).valueAsNumber)"
        />
        <span class="value">{{ gain }}%</span>
      </div>
    </div>

    <div class="section">
      <h2>Noise Gate</h2>
      <div class="control-row">
        <label class="toggle">
          <input
            type="checkbox"
            :checked="gateEnabled"
            @change="emit('update:noiseGate', ($event.target as HTMLInputElement).checked, gateThreshold)"
          />
          <span class="slider" />
        </label>
        <span class="label">{{ gateEnabled ? 'On' : 'Off' }}</span>
      </div>
      <div class="slider-row">
        <input
          type="range"
          :min="-60"
          :max="0"
          :value="gateThreshold"
          :disabled="!gateEnabled"
          class="range"
          @input="emit('update:noiseGate', gateEnabled, ($event.target as HTMLInputElement).valueAsNumber)"
        />
        <span class="value">{{ gateThreshold }}dB</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.mic-settings {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.section {
  background: #12121a;
  border-radius: 8px;
  padding: 16px;
}

.section h2 {
  font-size: 13px;
  font-weight: 600;
  color: #888;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 12px;
}

.slider-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.range {
  flex: 1;
  accent-color: #00d4aa;
}

.value {
  font-size: 12px;
  color: #666;
  min-width: 40px;
  text-align: right;
}

.control-row {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 12px;
}

.label {
  font-size: 13px;
  color: #888;
}

.toggle {
  position: relative;
  width: 36px;
  height: 20px;
  cursor: pointer;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle .slider {
  position: absolute;
  inset: 0;
  background: #333;
  border-radius: 20px;
  transition: 0.3s;
}

.toggle .slider::before {
  content: '';
  position: absolute;
  width: 16px;
  height: 16px;
  left: 2px;
  top: 2px;
  background: #888;
  border-radius: 50%;
  transition: 0.3s;
}

.toggle input:checked + .slider {
  background: #00d4aa44;
}

.toggle input:checked + .slider::before {
  transform: translateX(16px);
  background: #00d4aa;
}
</style>
