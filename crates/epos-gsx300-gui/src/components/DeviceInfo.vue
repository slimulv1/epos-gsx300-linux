<script setup lang="ts">
import type { DeviceInfo } from "../composables/useDaemon";

defineProps<{
  status: {
    daemon_version: string;
    device_connected: boolean;
    active_profile: string;
    smart_button_action?: string;
  } | null;
  device: DeviceInfo | null;
  smartButtonAction: string;
}>();

const emit = defineEmits<{
  (e: "update:smartButton", action: string): void;
}>();

// Label map for smart button actions (mirrors Rust SmartButtonAction)
const actionLabels: Record<string, string> = {
  toggle_mode: "Toggle stereo ⇄ 7.1 + LED (EPOS default)",
  toggle_eq: "Toggle EQ on/off",
  cycle_preset: "Cycle through presets",
  toggle_sidetone: "Toggle sidetone",
  toggle_noise_gate: "Toggle noise gate",
};
</script>

<template>
  <div class="device-info">
    <div class="info-card">
      <div class="info-row">
        <span class="label">Device</span>
        <span class="value">{{ status?.device_connected ? 'EPOS GSX 300' : 'Not connected' }}</span>
      </div>
      <div class="info-row">
        <span class="label">Daemon</span>
        <span class="value">v{{ status?.daemon_version || '—' }}</span>
      </div>
      <div class="info-row">
        <span class="label">Profile</span>
        <span class="value">{{ status?.active_profile || '—' }}</span>
      </div>
      <template v-if="device">
        <div class="info-row">
          <span class="label">USB</span>
          <span class="value">bus {{ device.usb_bus }} · addr {{ device.usb_addr }}</span>
        </div>
        <div class="info-row">
          <span class="label">ALSA card</span>
          <span class="value">card {{ device.alsa_card }}</span>
        </div>
        <div class="info-row">
          <span class="label">Sink</span>
          <span class="value mono" :title="device.pipewire_sink">{{ device.pipewire_sink }}</span>
        </div>
        <div class="info-row">
          <span class="label">Source</span>
          <span class="value mono" :title="device.pipewire_source">{{ device.pipewire_source }}</span>
        </div>
        <div class="info-row">
          <span class="label">HID</span>
          <span class="value mono">{{ device.hidraw || '—' }}</span>
        </div>
        <div class="info-row">
          <span class="label">Firmware</span>
          <span class="value">{{ device.firmware_version || 'unknown' }}</span>
        </div>
      </template>
    </div>

    <div class="info-card">
      <h3>Smart Button</h3>
      <p class="hint">What happens when you press the dial (smart button) on the GSX 300.</p>
      <select
        class="action-select"
        :value="smartButtonAction"
        @change="emit('update:smartButton', ($event.target as HTMLSelectElement).value)"
      >
        <option v-for="(label, key) in actionLabels" :key="key" :value="key">
          {{ label }}
        </option>
      </select>
    </div>

    <div class="info-card">
      <h3>About</h3>
      <p>Open-source Linux replacement for EPOS Gaming Suite.</p>
      <p class="link">github.com/slimulv1/epos-gsx300-linux</p>
    </div>
  </div>
</template>

<style scoped>
.device-info {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.info-card {
  background: #12121a;
  border-radius: 8px;
  padding: 16px;
}

.info-row {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  gap: 12px;
  padding: 8px 0;
  border-bottom: 1px solid #1e1e2e;
}

.info-row:last-child {
  border-bottom: none;
}

.label {
  color: #888;
  font-size: 13px;
  flex-shrink: 0;
}

.value {
  color: #e0e0e0;
  font-size: 13px;
  text-align: right;
  word-break: break-all;
}

.mono {
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 12px;
}

h3 {
  font-size: 14px;
  color: #fff;
  margin-bottom: 8px;
}

p {
  font-size: 13px;
  color: #888;
  line-height: 1.5;
}

.hint {
  margin-bottom: 12px;
}

.action-select {
  width: 100%;
  padding: 10px 12px;
  background: #1e1e2e;
  border: 1px solid #2a2a3a;
  border-radius: 6px;
  color: #e0e0e0;
  font-size: 13px;
  cursor: pointer;
}

.action-select:hover {
  border-color: #3a3a4a;
}

.action-select:focus {
  outline: none;
  border-color: #00d4aa;
}

.link {
  color: #00d4aa;
  margin-top: 8px;
}
</style>