<script setup lang="ts">
defineProps<{
  status: {
    daemon_version: string;
    device_connected: boolean;
    active_profile: string;
  } | null;
}>();
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
  padding: 8px 0;
  border-bottom: 1px solid #1e1e2e;
}

.info-row:last-child {
  border-bottom: none;
}

.label {
  color: #888;
  font-size: 13px;
}

.value {
  color: #e0e0e0;
  font-size: 13px;
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

.link {
  color: #00d4aa;
  margin-top: 8px;
}
</style>
