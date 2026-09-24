<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useDaemonStore } from "../stores/daemon";
import { VOICE_MODE_NAMES, type VoiceModeName } from "../stores/daemon";

const store = useDaemonStore();
const disconnected = computed(() => !store.status?.device_connected);

/* ─── Voice Enhancer ─── */
const voiceModes = VOICE_MODE_NAMES;
const voiceMode = computed(() => store.audio?.voice_enhancer?.mode ?? "off");

/* A mode can only be applied once the daemon has delivered the audio config:
 * setVoiceEnhancer returns early while `audio` is null, so a click before
 * that point used to be a silent no-op on a button that still looked live. */
const audioReady = computed(() => store.audio !== null);

/* "Custom" copies the current playback EQ. With no non-flat band the backend
 * would build an empty filter chain (passthrough) while the button showed as
 * selected, i.e. an active control that does nothing. Block it instead. */
const hasCustomBands = computed(() =>
  (store.audio?.eq?.bands ?? []).some((b) => Math.abs(b.gain_db) >= 0.1)
);

function voiceDisabled(mode: VoiceModeName): boolean {
  if (disconnected.value || !audioReady.value) return true;
  return mode === "custom" && !hasCustomBands.value;
}

function voiceHint(mode: VoiceModeName): string {
  if (disconnected.value) return "Device not connected";
  if (!audioReady.value) return "Waiting for audio state from daemon";
  if (mode === "custom" && !hasCustomBands.value) {
    return "Custom needs at least one EQ band above 0.1 dB (set one in Playback > EQ)";
  }
  return "";
}

async function onVoiceSelect(mode: VoiceModeName) {
  if (mode === "custom") {
    await store.setVoiceEnhancer("custom", store.audio?.eq?.bands);
  } else {
    await store.setVoiceEnhancer(mode);
  }
}

/* ─── Mic gain ─── */
const micGain = computed(() => store.audio?.mic_gain ?? 50);
function onMicGainChange(gain: number) {
  store.setMicGain(gain);
}

/* ─── Noise gate ─── */
const gateEnabled = computed(() => store.audio?.noise_gate?.enabled ?? false);
const gateThreshold = computed(() => store.audio?.noise_gate?.threshold_db ?? 0);
function onGateToggle(enabled: boolean) {
  store.setNoiseGate(enabled, gateThreshold.value);
}
function onGateThresholdChange(threshold: number) {
  store.setNoiseGate(gateEnabled.value, threshold);
}

/* ─── Slider fill tracking (fix: fill must hug the thumb exactly) ───
 * Two independent defects caused the fill to detach from the knob:
 *
 *  1) The CSS read `var(--fill, 50%)` but NOTHING ever assigned --fill,
 *     so the gradient was permanently pinned at the 50% fallback.
 *     We now bind --fill from the real slider value.
 *
 *  2) A native range input's background paints in the PADDING box while
 *     the thumb is laid out in the CONTENT box, so even a correctly
 *     assigned percentage lands a few pixels away from the knob. We move
 *     the gradient onto ::-webkit-slider-runnable-track, which is the
 *     very box the thumb travels in, so fill edge == thumb center.
 *
 * The local refs below mirror the value instantly on drag. The daemon
 * round-trip is async; without these the fill would visibly lag behind
 * the knob while dragging. They are re-synced from the store whenever
 * the device reports a new value. */
const micGainDrag = ref<number | null>(null);
const gateDrag = ref<number | null>(null);

const micGainShown = computed(() => micGainDrag.value ?? micGain.value);
const gateShown = computed(() => gateDrag.value ?? gateThreshold.value);

/* Normalised 0..1 position of each slider, for the gradient stop. */
const micGainFill = computed(() => {
  const min = 0;
  const max = 100;
  const pct = ((micGainShown.value - min) / (max - min)) * 100;
  return Math.min(100, Math.max(0, pct));
});
const gateFill = computed(() => {
  const min = -60;
  const max = 0;
  const pct = ((gateShown.value - min) / (max - min)) * 100;
  return Math.min(100, Math.max(0, pct));
});

/* Keep the optimistic drag value in sync with the store so an external
 * change (profile switch, daemon restore) is never masked by a stale
 * local value that the user is no longer dragging. */
watch(
  () => store.audio?.mic_gain,
  (v) => {
    if (v !== undefined && micGainDrag.value !== null && v !== micGainDrag.value) {
      micGainDrag.value = null;
    }
  }
);
watch(
  () => store.audio?.noise_gate?.threshold_db,
  (v) => {
    if (v !== undefined && gateDrag.value !== null && v !== gateDrag.value) {
      gateDrag.value = null;
    }
  }
);

function onMicGainDrag(gain: number) {
  micGainDrag.value = gain;
  onMicGainChange(gain);
}
function onGateDrag(threshold: number) {
  gateDrag.value = threshold;
  onGateThresholdChange(threshold);
}

/* ─── Mic meter lifecycle: keep-alive while this tab is mounted ───
 * mic_meter_start is idempotent (returns true when already running), so we
 * simply re-assert it every 2s. This self-heals all failure modes:
 *  - device absent at first open  → resolve fails, retried next tick
 *  - device unplugged mid-meter   → pw-record EOFs, backend clears child,
 *                                   next tick spawns a fresh one when back
 *  - tab switched mid-start       → late start is stopped on unmount
 */
let meterTimer: ReturnType<typeof setInterval> | null = null;
let meterDestroyed = false;

async function ensureMeter() {
  if (meterDestroyed) return;
  try {
    const ok = await invoke<boolean>("mic_meter_start");
    if (meterDestroyed && ok) {
      // Start resolved after unmount — stop it so no capture leaks.
      try {
        await invoke("mic_meter_stop");
      } catch {
        /* ignore */
      }
    }
  } catch {
    // No Tauri runtime (browser dev): status stays "NO SIGNAL".
    // It NEVER fabricates audio — there is no mock anywhere in the frontend.
  }
}

/* ─── Fail-closed mic status text (replaces the old level ring) ───
 * Subscribes to the backend's REAL "mic-level" stream. The static readout
 * shows "MIC ACTIVE + dB" ONLY when a payload confirms active===true (real,
 * two-gate-validated audio). Silent / hiss / no-mic keeps "NO SIGNAL" -
 * the text NEVER animates, NEVER self-generates, NEVER fabricates. There is
 * no bar, no ring, no mock, no self-animation anywhere in the frontend. */
const micMeterActive = ref(false);
const micMeterDb = ref(0);
let levelUnlisten: UnlistenFn | null = null;

async function wireLevel() {
  try {
    if (levelUnlisten) return;
    levelUnlisten = await listen<{ active: boolean; db: number }>(
      "mic-level",
      (e) => {
        const p = e.payload;
        if (!p.active) {
          // Fail-closed: any not-active payload (silent/hiss/no-mic) keeps
          // the text dark and reads "NO SIGNAL". Never fabricates.
          micMeterActive.value = false;
          micMeterDb.value = 0;
          return;
        }
        micMeterActive.value = true;
        micMeterDb.value = p.db;
      }
    );
  } catch {
    // No Tauri runtime (browser dev): readout stays "NO SIGNAL" forever.
  }
}

onMounted(() => {
  ensureMeter();
  meterTimer = setInterval(ensureMeter, 2000);
  void wireLevel();
});

onUnmounted(async () => {
  meterDestroyed = true;
  if (meterTimer) {
    clearInterval(meterTimer);
    meterTimer = null;
  }
  if (levelUnlisten) {
    levelUnlisten();
    levelUnlisten = null;
  }
  try {
    await invoke("mic_meter_stop");
  } catch {
    /* ignore */
  }
});
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
          :disabled="voiceDisabled(m)"
          :title="voiceHint(m)"
        >
          {{ m.charAt(0).toUpperCase() + m.slice(1) }}
        </button>
      </div>
    </div>

    <!-- Mic gain + fail-closed static status (no bar, no ring, no mock) -->
    <div class="section-card">
      <div class="section-header">
        <h3 class="section-title">MIC GAIN</h3>
        <span
          class="status-readout"
          :class="{ ok: micMeterActive && !disconnected }"
        >
          {{ micMeterActive && !disconnected ? "MIC ACTIVE" : "NO SIGNAL" }}
        </span>
      </div>
      <div class="slider-row mic-slider">
        <input
          type="range"
          class="accent-range"
          :style="{ '--fill': `${micGainFill}%` }"
          :min="0"
          :max="100"
          step="1"
          :value="micGainShown"
          :disabled="disconnected"
          @input="onMicGainDrag(($event.target as HTMLInputElement).valueAsNumber)"
        />
        <span class="value-badge">
          {{ micMeterActive && !disconnected ? micMeterDb.toFixed(1) + " dB" : "" }}
        </span>
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
          :style="{ '--fill': `${gateFill}%` }"
          :min="-60"
          :max="0"
          step="1"
          :value="gateShown"
          :disabled="disconnected"
          @input="onGateDrag(($event.target as HTMLInputElement).valueAsNumber)"
        />
        <span class="value-badge">{{ gateShown }}dB</span>
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
  opacity: 0.55;
  pointer-events: none;
}
.section-card {
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: var(--space-3);
}
.section-title {
  margin: 0;
  font-size: var(--fs-xs);
  font-weight: 700;
  letter-spacing: 0.8px;
  color: var(--muted);
}
.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}
.voice-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}
.voice-btn {
  padding: 8px 0;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--panel-2);
  color: var(--muted);
  font-size: var(--fs-sm);
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  cursor: pointer;
  transition:
    border-color var(--time-fast) var(--ease),
    color var(--time-fast) var(--ease),
    background var(--time-fast) var(--ease);
}
.voice-btn:hover:not(:disabled) {
  border-color: var(--accent-dim);
  color: var(--text);
}
.voice-btn.active {
  border-color: var(--accent);
  color: var(--accent);
  background: var(--accent-glow);
}
.voice-btn:disabled {
  cursor: not-allowed;
}
.slider-row {
  display: flex;
  align-items: center;
  gap: 10px;
}
.slider-row .accent-range {
  flex: 1;
}
.mic-slider {
  margin-top: 2px;
}
/* Fail-closed static status text: DARK by default ("NO SIGNAL"); lights
 * ONLY when the real backend mic-level payload confirms active===true.
 * It is a plain state label - it never animates, never self-generates,
 * never fabricates, and never moves on its own. */
.status-readout {
  font-size: var(--fs-xs);
  font-weight: 700;
  letter-spacing: 0.6px;
  color: var(--danger);
  font-family: var(--font-mono);
}
.status-readout.ok {
  color: var(--accent);
}
/* ── Slider: the fill must land exactly on the thumb centre ──
 * The element's own background paints the PADDING box, but the native
 * thumb travels inside the CONTENT box, so a gradient on the element is
 * always offset from the knob. The fill therefore lives on
 * ::-webkit-slider-runnable-track, the same box the thumb is positioned
 * against, and the element background is left transparent.
 * --fill is bound from the live slider value (see script above); the
 * 50% fallback is never reached. */
.accent-range {
  --thumb-size: 14px;
  --track-size: 4px;
  appearance: none;
  -webkit-appearance: none;
  width: 100%;
  margin: 0;
  padding: 0;
  border: 0;
  background: transparent;
  cursor: pointer;
}
.accent-range:disabled {
  cursor: not-allowed;
}
.accent-range::-webkit-slider-runnable-track {
  width: 100%;
  height: var(--track-size);
  border: 0;
  border-radius: 999px;
  background: linear-gradient(
    to right,
    var(--accent) 0%,
    var(--accent) var(--fill, 50%),
    var(--grid) var(--fill, 50%),
    var(--grid) 100%
  );
}
.accent-range::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: var(--thumb-size);
  height: var(--thumb-size);
  margin-top: calc((var(--track-size) - var(--thumb-size)) / 2);
  border-radius: 50%;
  background: var(--accent);
  border: 2px solid var(--panel);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.45);
  cursor: pointer;
  transition: transform var(--time-fast) var(--ease);
}
.accent-range:hover::-webkit-slider-thumb {
  transform: scale(1.12);
}
.accent-range:active::-webkit-slider-thumb {
  transform: scale(1.02);
}
.accent-range:focus-visible::-webkit-slider-thumb {
  box-shadow: 0 0 0 3px var(--accent-glow);
}
.accent-range:disabled::-webkit-slider-thumb {
  background: var(--faint);
  cursor: not-allowed;
}
/* Firefox: ::-moz-range-progress is the native fill and already tracks the
 * thumb centre correctly, so it is used directly instead of a gradient. */
.accent-range::-moz-range-track {
  width: 100%;
  height: var(--track-size);
  border: 0;
  border-radius: 999px;
  background: var(--grid);
}
.accent-range::-moz-range-progress {
  height: var(--track-size);
  border-radius: 999px;
  background: var(--accent);
}
.accent-range::-moz-range-thumb {
  width: var(--thumb-size);
  height: var(--thumb-size);
  border-radius: 50%;
  background: var(--accent);
  border: 2px solid var(--panel);
  cursor: pointer;
}
.accent-range:disabled::-moz-range-thumb {
  background: var(--faint);
  cursor: not-allowed;
}
.value-badge {
  min-width: 48px;
  text-align: right;
  font-size: var(--fs-xs);
  font-weight: 600;
  font-family: var(--font-mono);
  color: var(--muted);
  font-variant-numeric: tabular-nums;
}
.toggle-wrap {
  display: inline-flex;
  cursor: pointer;
}
.toggle-wrap input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}
.toggle-track {
  width: 36px;
  height: 20px;
  border-radius: 999px;
  background: var(--grid);
  position: relative;
  transition: background var(--time-fast) var(--ease);
}
.toggle-wrap input:checked + .toggle-track {
  background: var(--accent, #2f81f7);
}
.toggle-thumb {
  position: absolute;
  top: 3px;
  left: 3px;
  width: 14px;
  height: 14px;
  border-radius: 50%;
  background: var(--text);
  transition: transform var(--time-fast) var(--ease);
}
.toggle-wrap input:checked + .toggle-track .toggle-thumb {
  transform: translateX(16px);
}
</style>
