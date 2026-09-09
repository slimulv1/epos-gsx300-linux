<script setup lang="ts">
import { ref, onMounted } from "vue";
import EqCurve from "./components/EqCurve.vue";
import PresetList from "./components/PresetList.vue";
import SidetoneSlider from "./components/SidetoneSlider.vue";
import VoiceEnhancer from "./components/VoiceEnhancer.vue";
import MicSettings from "./components/MicSettings.vue";
import DeviceInfo from "./components/DeviceInfo.vue";
import { useDaemon } from "./composables/useDaemon";

const { connected, status, audio, profiles, fetchStatus, fetchAudio, fetchProfiles, setActiveProfile } = useDaemon();

const activeTab = ref<"playback" | "mic" | "settings">("playback");

onMounted(async () => {
  await fetchStatus();
  await fetchAudio();
  await fetchProfiles();
});
</script>

<template>
  <div class="app">
    <!-- Header -->
    <header class="header">
      <div class="header-left">
        <svg class="logo" viewBox="0 0 24 24" width="28" height="28">
          <circle cx="12" cy="12" r="10" fill="none" stroke="currentColor" stroke-width="2" />
          <path d="M8 12 L11 15 L16 9" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
        </svg>
        <h1>EPOS GSX 300</h1>
      </div>
      <div class="header-right">
        <span :class="['status-dot', connected ? 'connected' : 'disconnected']" />
        <span class="status-text">{{ connected ? "Connected" : "Disconnected" }}</span>
      </div>
    </header>

    <!-- Tabs -->
    <nav class="tabs">
      <button
        :class="['tab', activeTab === 'playback' && 'active']"
        @click="activeTab = 'playback'"
      >
        <svg viewBox="0 0 24 24" width="18" height="18">
          <path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3c0-1.77-1.02-3.29-2.5-4.03v8.05c1.48-.73 2.5-2.25 2.5-4.02z" fill="currentColor"/>
        </svg>
        Playback
      </button>
      <button
        :class="['tab', activeTab === 'mic' && 'active']"
        @click="activeTab = 'mic'"
      >
        <svg viewBox="0 0 24 24" width="18" height="18">
          <path d="M12 14c1.66 0 3-1.34 3-3V5c0-1.66-1.34-3-3-3S9 3.34 9 5v6c0 1.66 1.34 3 3 3zm5.91-3c-.49 0-.9.36-.98.85C16.52 14.2 14.47 16 12 16s-4.52-1.8-4.93-4.15c-.08-.49-.49-.85-.98-.85-.61 0-1.09.54-1 1.14.49 3 2.89 5.35 5.91 5.78V20c0 .55.45 1 1 1s1-.45 1-1v-2.08c3.02-.43 5.42-2.78 5.91-5.78.1-.6-.39-1.14-1-1.14z" fill="currentColor"/>
        </svg>
        Microphone
      </button>
      <button
        :class="['tab', activeTab === 'settings' && 'active']"
        @click="activeTab = 'settings'"
      >
        <svg viewBox="0 0 24 24" width="18" height="18">
          <path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58a.49.49 0 00.12-.61l-1.92-3.32a.49.49 0 00-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54a.484.484 0 00-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96a.49.49 0 00-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.07.62-.07.94s.02.64.07.94l-2.03 1.58a.49.49 0 00-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6A3.6 3.6 0 1115.6 12 3.6 3.6 0 0112 15.6z" fill="currentColor"/>
        </svg>
        Settings
      </button>
    </nav>

    <!-- Content -->
    <main class="content">
      <!-- Playback Tab -->
      <div v-if="activeTab === 'playback'" class="tab-content">
        <div class="section">
          <h2>Equalizer</h2>
          <EqCurve v-if="audio" :bands="audio.eq.bands" @update="onEqUpdate" />
        </div>

        <div class="section">
          <h2>Presets</h2>
          <PresetList
            :profiles="profiles"
            :active="status?.active_profile || 'Flat'"
            @select="setActiveProfile"
          />
        </div>

        <div class="row">
          <div class="section">
            <h2>Sidetone</h2>
            <SidetoneSlider
              v-if="audio"
              :enabled="audio.sidetone.enabled"
              :level="audio.sidetone.level"
              @update="onSidetoneUpdate"
            />
          </div>

          <div class="section">
            <h2>Voice Enhancer</h2>
            <VoiceEnhancer
              v-if="audio"
              :mode="audio.voice_enhancer.mode"
              @update="onVoiceUpdate"
            />
          </div>
        </div>
      </div>

      <!-- Microphone Tab -->
      <div v-if="activeTab === 'mic'" class="tab-content">
        <MicSettings v-if="audio" :config="audio" @update:micGain="onMicGainUpdate" />
      </div>

      <!-- Settings Tab -->
      <div v-if="activeTab === 'settings'" class="tab-content">
        <DeviceInfo :status="status" />
      </div>
    </main>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  methods: {
    onEqUpdate(bands: any[]) {
      const { setEqBands } = useDaemon();
      setEqBands(bands);
    },
    onSidetoneUpdate(enabled: boolean, level: number) {
      const { setSidetone } = useDaemon();
      setSidetone(enabled, level);
    },
    onVoiceUpdate(mode: string) {
      // TODO: send to daemon
      console.log("Voice mode:", mode);
    },
    onMicGainUpdate(gain: number) {
      const { setMicGain } = useDaemon();
      setMicGain(gain);
    },
  },
});
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #0a0a0f;
  color: #e0e0e0;
  overflow: hidden;
}

.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 20px;
  background: #12121a;
  border-bottom: 1px solid #1e1e2e;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.header-left h1 {
  font-size: 16px;
  font-weight: 600;
  color: #fff;
}

.logo {
  color: #00d4aa;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.status-dot.connected {
  background: #00d4aa;
  box-shadow: 0 0 6px #00d4aa66;
}

.status-dot.disconnected {
  background: #ff4444;
  box-shadow: 0 0 6px #ff444466;
}

.status-text {
  font-size: 12px;
  color: #888;
}

.tabs {
  display: flex;
  gap: 4px;
  padding: 8px 20px;
  background: #12121a;
  border-bottom: 1px solid #1e1e2e;
}

.tab {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: #888;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.tab:hover {
  background: #1e1e2e;
  color: #ccc;
}

.tab.active {
  background: #1e1e2e;
  color: #00d4aa;
}

.content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.tab-content {
  max-width: 800px;
  margin: 0 auto;
}

.section {
  margin-bottom: 24px;
}

.section h2 {
  font-size: 13px;
  font-weight: 600;
  color: #888;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 12px;
}

.row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
}
</style>
