<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useDaemonStore } from "../stores/daemon";

interface MicPayload {
  db: number;
  peak_db: number;
  active: boolean;
  clip: boolean;
}

const store = useDaemonStore();
/* GAIN readout under the bar = actual mic preamp gain from the daemon
   (store.audio.mic_gain, 0-100%). Uses `store` → keeps daemon store wired. */
const gain = computed(() => Math.round(store.audio?.mic_gain ?? 0));

/* ─── Live level state (fed by "mic-level" Tauri event) ─── */
const rawDb = ref(0); // relative dB straight from backend (0 = noise floor)
const displayDb = ref(0); // eased copy → drives the bar (anti-jitter)
const peakDb = ref(0); // relative peak-hold (backend-smoothed)
const active = ref(false);
const clipHold = ref(false); // 500ms visual hold after last clip frame

/* ─── Lifecycle handles ─── */
let unlisten: UnlistenFn | null = null;
let mockTimer: ReturnType<typeof setInterval> | null = null;
let clipTimer: ReturnType<typeof setTimeout> | null = null;

/* ─── Geometry: HORIZONTAL bar → 30 segments, fill left→right.
        0 = noise floor (left edge of track); full bar at RANGE_DB above it ─── */
const BAR_X0 = 4; // left edge of track
const BAR_W = 268; // track width (viewBox = 276 wide)
const BAR_H = 32; // bar height
const BAR_Y = 36; // top of bar band
const SEGMENTS = 30;
const SEG_W = BAR_W / SEGMENTS; // ~8.93px per segment slot
const SEG_SWEEP = 8.3; // ~0.63px notch between segments
const RANGE_DB = 18; // 18 dB above the noise floor = full bar
const GATE_DB = 3.5; // small ambience may light 1-2 notches; idle hiss stays below

/* Landmark seams (dB relative to full scale, non-color ticks): -6 / -3 */
const LANDMARK_DB = [6, 3]; // below full scale
function landmarkX(dbBelowScale: number): number {
  const frac = (RANGE_DB - Math.max(GATE_DB, dbBelowScale)) / RANGE_DB;
  return BAR_X0 + Math.min(1, Math.max(0, frac)) * BAR_W;
}

function segRect(i: number): { x: number; y: number; w: number; h: number } {
  return {
    x: BAR_X0 + i * SEG_W,
    y: BAR_Y,
    w: SEG_SWEEP,
    h: BAR_H,
  };
}

/* ─── Level rendering (relative to noise floor) ─── */
const frac = computed(() =>
  Math.min(1, Math.max(0, (displayDb.value - GATE_DB) / RANGE_DB)),
);
const peakFrac = computed(() => Math.min(1, Math.max(0, peakDb.value / RANGE_DB)));

const segments = computed(() => {
  const litN = Math.ceil(frac.value * SEGMENTS); // current-level tip (snapped to segment seam)
  const peakN = Math.ceil(peakFrac.value * SEGMENTS); // peak-hold trail tip
  return Array.from({ length: SEGMENTS }, (_, i) => ({
    ...segRect(i),
    cls:
      i < litN
        ? `lit zone-${i < 23 ? "a" : i < 27 ? "w" : "d"}`
        : i < peakN
          ? "peak-trail"
          : "off",
  }));
});

// White dot rides EXACTLY on the tip of the lit bar (end of last lit segment)
const peakPos = computed(() => {
  const litN = Math.ceil(frac.value * SEGMENTS);
  return { x: (BAR_X0 + litN * SEG_W).toFixed(2), y: BAR_Y + BAR_H / 2 };
});

const dbText = computed(() => {
  if (!active.value) return "NO SIGNAL";
  if (displayDb.value <= GATE_DB) return "0.0 dB";
  return `${displayDb.value.toFixed(1)} dB`;
});

/* ─── LED state machine ─── */
type LedState = "idle" | "ok" | "clip";
const led = computed<LedState>(() => {
  if (!active.value) return "idle";
  if (clipHold.value) return "clip";
  if (displayDb.value <= GATE_DB) return "idle";
  return "ok";
});

/* ─── Apply a payload (from backend or mock) ─── */
function apply(p: MicPayload) {
  rawDb.value = p.db;
  peakDb.value = p.peak_db;
  active.value = p.active;
  // Ease the display toward the raw level: rise fast, fall slowly.
  // Kills preamp-hiss jitter (~±2 dB every 30ms) on a silent mic.
  const target = Math.max(0, p.db);
  const cur = displayDb.value;
  const k = target > cur ? 0.5 : 0.12;
  displayDb.value = cur + (target - cur) * k;
  if (p.clip) {
    clipHold.value = true;
    if (clipTimer) clearTimeout(clipTimer);
    clipTimer = setTimeout(() => {
      clipHold.value = false;
    }, 500);
  } else if (!p.active) {
    clipHold.value = false;
    displayDb.value = 0;
    peakDb.value = 0;
  }
}

/* ─── Browser-dev mock (when no Tauri runtime) ───
   Simulates relative levels: 0 = noise floor, speech ~8-20 dB above it. */
let mockT = 0;
function startMock() {
  let peak = 0;
  mockTimer = setInterval(() => {
    mockT += 0.35;
    const v = Math.sin(mockT) * 0.5 + Math.sin(mockT * 2.1) * 0.35 + Math.sin(mockT * 4.3) * 0.15;
    const d = Math.max(0, 11 + ((v + 1) / 2) * 13); // idle-ish 11-24 → speech
    peak = Math.max(d, peak - 0.8); // simple peak decay (0.8dB / 120ms)
    apply({ db: d, peak_db: peak, clip: d >= 23, active: true });
  }, 120);
}

function stopMock() {
  if (mockTimer) {
    clearInterval(mockTimer);
    mockTimer = null;
  }
}

onMounted(async () => {
  try {
    unlisten = await listen<MicPayload>("mic-level", (e) => apply(e.payload));
  } catch {
    // No Tauri runtime (browser dev) — fall back to simulated signal.
    startMock();
  }
});

onUnmounted(() => {
  if (unlisten) unlisten();
  stopMock();
  if (clipTimer) clearTimeout(clipTimer);
});
</script>

<template>
  <div class="bar-meter" :class="{ dead: !active }">
    <svg viewBox="0 0 276 64" role="img" aria-label="Microphone input level">
      <!-- landmark ticks (-6dB / -3dB): NON-color signaling, texture + text -->
      <line v-for="(l, i) in LANDMARK_DB" :key="i" :x1="landmarkX(l)" :x2="landmarkX(l)" y1="32" y2="72" class="landmark" />
      <text v-for="(l, i) in LANDMARK_DB" :key="'t' + i" :x="landmarkX(l)" y="82" text-anchor="middle" class="landmark-label">{{ -l }}dB</text>

      <!-- 30 horizontal segments, fill left→right -->
      <rect
        v-for="(s, i) in segments"
        :key="i"
        :x="s.x"
        :y="s.y"
        :width="s.w"
        :height="s.h"
        :class="s.cls"
      />
      <!-- white dot rides EXACTLY on the tip of the lit bar -->
      <circle v-if="frac > 0" :cx="peakPos.x" :cy="peakPos.y" r="4" class="peak-dot" />

      <!-- balance text → moved BELOW bar (left side) -->
      <text x="10" y="60" class="gain-text">{{ gain }}%</text>
      <text x="10" y="70" class="gain-label">GAIN</text>
    </svg>

    <div class="meter-meta">
      <span class="db-readout" :class="{ clippy: led === 'clip' }">{{ dbText }}</span>
      <span class="leds" aria-hidden="true">
        <span class="led" :class="{ on: led === 'idle' }">IDLE</span>
        <span class="led" :class="{ on: led === 'ok' }">OK</span>
        <span class="led clip-led" :class="{ on: led === 'clip' }">CLIP</span>
      </span>
    </div>
  </div>
</template>

<style scoped>
.bar-meter {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  font-family: var(--font-ui);
}
.bar-meter.dead .gain-text {
  fill: var(--muted);
}
.gain-text {
  fill: var(--text);
  font-size: 15px;
  font-weight: 700;
  letter-spacing: 0.2px;
}
.gain-label {
  fill: var(--muted);
  font-size: 9px;
  font-weight: 600;
  letter-spacing: 1.2px;
}

/* segments */
rect.off {
  fill: var(--grid);
  opacity: 0.6;
}
rect.lit {
  fill: var(--accent);
  filter: drop-shadow(0 0 3px var(--accent-glow));
}
rect.zone-w {
  fill: var(--warn);
  filter: drop-shadow(0 0 3px rgba(224, 164, 88, 0.45));
}
rect.zone-d {
  fill: var(--danger);
  filter: drop-shadow(0 0 4px rgba(231, 76, 94, 0.55));
}
rect.peak-trail {
  fill: var(--accent);
  opacity: 0.32;
}

/* landmark ticks — non-color (texture + text labels), reduced-motion safe */
line.landmark {
  stroke: var(--muted);
  stroke-width: 1;
  stroke-dasharray: 2 2;
  opacity: 0.55;
}
.landmark-label {
  fill: var(--muted);
  font-size: 8px;
  font-weight: 600;
  letter-spacing: 0.2px;
}

.peak-dot {
  fill: #ffffff;
  stroke: rgba(190, 240, 235, 0.85);
  stroke-width: 0.8;
  filter: drop-shadow(0 0 4px var(--accent)) drop-shadow(0 0 8px rgba(78, 205, 196, 0.45));
}

/* meta row */
.meter-meta {
  display: flex;
  align-items: center;
  gap: 10px;
}
.db-readout {
  font-family: var(--font-mono, monospace);
  font-size: 11px;
  color: var(--text);
  min-width: 74px;
  text-align: right;
  letter-spacing: 0.3px;
}
.db-readout.clippy {
  color: var(--danger);
}
.leds {
  display: flex;
  gap: 7px;
}
.led {
  font-size: 9px;
  font-weight: 600;
  letter-spacing: 0.8px;
  color: var(--grid);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 2px 5px;
  transition: color var(--time-fast) var(--ease), background var(--time-fast) var(--ease);
}
.led.on {
  background: var(--accent-glow);
  border-color: var(--accent-dim);
  color: var(--accent);
}
.led.clip-led.on {
  background: rgba(231, 76, 94, 0.14);
  border-color: rgba(231, 76, 94, 0.5);
  color: var(--danger);
}

@media (prefers-reduced-motion: reduce) {
  .led {
    transition: none;
  }
  .peak-dot,
  rect.lit,
  rect.zone-w,
  rect.zone-d {
    transition: none;
  }
}
</style>
