<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useDaemonStore } from "../stores/daemon";
import { invoke } from "@tauri-apps/api/core";
import { Info, ExternalLink } from "lucide-vue-next";

const store = useDaemonStore();

/* ─── Autostart ─── */
const autostartEnabled = ref(false);
onMounted(async () => {
  try {
    autostartEnabled.value = await invoke("autostart_get");
  } catch { /* ignore */ }
});
async function onAutostartToggle() {
  const next = !autostartEnabled.value;
  try {
    await invoke("autostart_set", { enabled: next });
    autostartEnabled.value = next;
  } catch (e) {
    console.error("Autostart toggle failed:", e);
  }
}

/* ─── Status ─── */
const daemonVersion = computed(() => store.status?.daemon_version ?? "—");
const activeProfile = computed(() => store.status?.active_profile ?? "—");
const deviceConnected = computed(() => store.status?.device_connected ?? false);
</script>

<template>
  <div class="settings-view">
    <!-- Status -->
    <div class="section-card">
      <h3 class="section-title">STATUS</h3>
      <div class="info-grid">
        <div class="info-row">
          <span class="info-label">Daemon</span>
          <span class="info-value">v{{ daemonVersion }}</span>
        </div>
        <div class="info-row">
          <span class="info-label">Device</span>
          <span class="info-value" :class="deviceConnected ? 'ok' : 'err'">
            {{ deviceConnected ? 'Connected' : 'Disconnected' }}
          </span>
        </div>
        <div class="info-row">
          <span class="info-label">Profile</span>
          <span class="info-value">{{ activeProfile }}</span>
        </div>
      </div>
    </div>

    <!-- System -->
    <div class="section-card">
      <h3 class="section-title">SYSTEM</h3>
      <div class="toggle-row">
        <span class="toggle-label">Show on startup</span>
        <label class="toggle-wrap">
          <input
            type="checkbox"
            :checked="autostartEnabled"
            @change="onAutostartToggle"
          />
          <span class="toggle-track"><span class="toggle-thumb" /></span>
        </label>
      </div>
    </div>

    <!-- About -->
    <div class="section-card">
      <h3 class="section-title">ABOUT</h3>
      <div class="about-content">
        <Info :size="16" class="about-icon" />
        <div class="about-text">
          <p>EPOS GSX 300 Linux</p>
          <p class="muted">Open-source Linux replacement for EPOS Gaming Suite.</p>
          <a
            class="about-link"
            href="https://github.com/slimulv1/epos-gsx300-linux"
            target="_blank"
            rel="noopener"
          >
            <ExternalLink :size="12" />
            github.com/slimulv1/epos-gsx300-linux
          </a>
        </div>
      </div>
      <p class="license">MIT License</p>
    </div>
  </div>
</template>

<style scoped>
.settings-view {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

.section-card {
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: var(--space-3);
}
.section-title {
  font-size: var(--fs-xs);
  color: var(--muted);
  font-weight: 600;
  letter-spacing: 0.5px;
  text-transform: uppercase;
  margin-bottom: var(--space-2);
}

/* Info grid */
.info-grid {
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.info-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px solid var(--border);
}
.info-row:last-child {
  border-bottom: none;
}
.info-label {
  font-size: var(--fs-sm);
  color: var(--muted);
}
.info-value {
  font-size: var(--fs-sm);
  color: var(--text);
}
.info-value.ok {
  color: var(--accent);
}
.info-value.err {
  color: var(--danger);
}

/* Toggle row */
.toggle-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.toggle-label {
  font-size: var(--fs-sm);
  color: var(--text);
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

/* About */
.about-content {
  display: flex;
  gap: 12px;
  margin-bottom: 12px;
}
.about-icon {
  color: var(--accent);
  flex-shrink: 0;
  margin-top: 2px;
}
.about-text p {
  font-size: var(--fs-sm);
  color: var(--text);
  line-height: 1.5;
}
.about-text .muted {
  color: var(--muted);
  font-size: var(--fs-xs);
}
.about-link {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  color: var(--accent);
  font-size: var(--fs-xs);
  text-decoration: none;
  margin-top: 4px;
}
.about-link:hover {
  text-decoration: underline;
}
.license {
  font-size: var(--fs-xs);
  color: var(--faint);
}
</style>