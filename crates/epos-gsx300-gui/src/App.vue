<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useDaemonStore } from "./stores/daemon";
import {
  SlidersHorizontal,
  Headphones,
  Mic,
  Settings,
  WifiOff,
} from "lucide-vue-next";
import PlaybackView from "./views/PlaybackView.vue";
import MicrophoneView from "./views/MicrophoneView.vue";
import DeviceView from "./views/DeviceView.vue";
import SettingsView from "./views/SettingsView.vue";

const store = useDaemonStore();

type TabId = "playback" | "microphone" | "device" | "settings";
const activeTab = ref<TabId>("playback");

const tabs: { id: TabId; icon: typeof SlidersHorizontal; label: string }[] = [
  { id: "playback", icon: SlidersHorizontal, label: "EQ" },
  { id: "device", icon: Headphones, label: "Device" },
  { id: "microphone", icon: Mic, label: "Mic" },
  { id: "settings", icon: Settings, label: "Settings" },
];

onMounted(() => {
  store.startPolling();
});
</script>

<template>
  <div class="app-shell">
    <!-- Header -->
    <header class="header">
      <div class="header-left">
        <span class="wordmark">EPOS</span>
        <span class="divider" />
        <span class="device-name">EPOS GSX 300</span>
      </div>

      <div class="header-right">
        <span
          class="status-dot"
          :class="store.connected ? 'on' : 'off'"
          :title="store.connected ? 'Connected' : 'Daemon unreachable'"
        />
      </div>
    </header>

    <!-- Disconnected banner -->
    <div v-if="store.status && !store.status.device_connected" class="disconnected-banner">
      <WifiOff :size="14" />
      <span>Device disconnected — controls disabled</span>
    </div>

    <!-- Content -->
    <main class="content">
      <PlaybackView v-if="activeTab === 'playback'" />
      <DeviceView v-else-if="activeTab === 'device'" />
      <MicrophoneView v-else-if="activeTab === 'microphone'" />
      <SettingsView v-else />
    </main>

    <!-- Bottom nav -->
    <nav class="bottom-nav" role="tablist" aria-label="Navigation">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        role="tab"
        :aria-selected="activeTab === tab.id"
        :class="['nav-btn', activeTab === tab.id && 'active']"
        @click="activeTab = tab.id"
      >
        <component :is="tab.icon" :size="18" />
        <span class="nav-label">{{ tab.label }}</span>
      </button>
    </nav>
  </div>
</template>

<style scoped>
.app-shell {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100%;
  overflow: hidden;
  background: var(--bg);
}

/* ─── Header ─── */
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 48px;
  padding: 0 var(--space-3);
  background: var(--panel);
  border-bottom: 1px solid var(--border);
  flex-shrink: 0;
  min-height: 40px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.wordmark {
  font-size: var(--fs-lg);
  font-weight: 700;
  color: var(--accent);
  letter-spacing: 2px;
}

.divider {
  width: 1px;
  height: 18px;
  background: var(--grid);
}

.device-name {
  font-size: var(--fs-sm);
  color: var(--muted);
  font-weight: 500;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 10px;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}
.status-dot.on {
  background: var(--accent);
  box-shadow: 0 0 6px var(--accent-dim);
}
.status-dot.off {
  background: var(--danger);
}

/* ─── Disconnected banner ─── */
.disconnected-banner {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 6px;
  background: rgba(231, 76, 94, 0.12);
  color: var(--danger);
  font-size: var(--fs-xs);
  font-weight: 500;
  border-bottom: 1px solid rgba(231, 76, 94, 0.2);
  flex-shrink: 0;
}

/* ─── Content ─── */
.content {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: var(--space-3);
}

/* ─── Bottom nav ─── */
.bottom-nav {
  display: flex;
  align-items: stretch;
  height: 48px;
  min-height: 40px;
  background: var(--footer);
  border-top: 1px solid var(--border);
  flex-shrink: 0;
}

.nav-btn {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 2px;
  background: transparent;
  border: none;
  color: var(--muted);
  font-size: 10px;
  font-family: var(--font-ui);
  cursor: pointer;
  transition:
    color var(--time-fast) var(--ease),
    background var(--time-fast) var(--ease);
  position: relative;
}
.nav-btn:hover {
  color: var(--text);
  background: rgba(255, 255, 255, 0.03);
}
.nav-btn.active {
  color: var(--accent);
}
.nav-btn.active::after {
  content: "";
  position: absolute;
  top: 0;
  left: 20%;
  right: 20%;
  height: 2px;
  background: var(--accent);
  border-radius: 0 0 2px 2px;
}

.nav-label {
  font-weight: 500;
  letter-spacing: 0.3px;
}

/* ─── Responsive: narrow layout (dwm tile) ─── */
@media (max-width: 700px) {
  .header {
    padding: 0 var(--space-2);
  }
  .wordmark {
    font-size: var(--fs-md);
  }
  .device-name {
    display: none;
  }
  .divider {
    display: none;
  }
}
</style>