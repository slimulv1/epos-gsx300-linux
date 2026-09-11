<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useDaemonStore } from "../stores/daemon";

interface MicLevelPayload {
  db: number;
  peak_db: number;
  clip: boolean;
  active: boolean;
}

const store = useDaemonStore();

/* ─── Live level state (fed by "mic-level" Tauri event) ─── */
const rawDb = ref(0); // relative dB straight from backend (0 = noise floor)
const displayDb = ref(0); // eased copy → drives the ring (anti-jitter)
const peakDb = ref(0); // relative peak-hold (backend-smoothed)
const active = ref(false);
const clipHold = ref(false); // 500ms visual hold after last clip frame

/* ─── Lifecycle handles ─── */
let unlisten: UnlistenFn | null = null;
let mockTimer: ReturnType<typeof setInterval> | null = null;
let clipTimer: ReturnType<typeof setTimeout> | null = null;

/* ─── Geometry: 30 segments, open arc (60° structural gap at top),
       0deg = 12 o'clock, clockwise fill from 1h position ─── */
const CX = 44;
const CY = 44;
const R = 34;
const SEGMENTS = 30;
const TRACK_DEG = 300; // opening = 60°
const START_DEG = 30; // first segment at 1h (just past the gap)
const SEG_DEG = TRACK_DEG / SEGMENTS; // 10.0°
const SEG_SWEEP = 8.6; // ~1.4deg notch between segments
const RANGE_DB = 18; // 18 dB above the noise floor = full ring
const GATE_DB = 3.5; // small ambience may light 1-2 notches; idle hiss stays below

function polar(angleDeg: number): [number, number] {
  const a = ((angleDeg - 90) * Math.PI) / 180; // 0deg => top; positive = clockwise
  return [CX + R * Math.cos(a), CY + R * Math.sin(a)];
}

function arcPath(i: number): string {
  const a0 = START_DEG + i * SEG_DEG;
  const a1 = a0 + SEG_SWEEP;
  const [x0, y0] = polar(a0);
  const [x1, y1] = polar(a1);
  return `M ${x0.toFixed(2)} ${y0.toFixed(2)} A ${R} ${R} 0 0 1 ${x1.toFixed(2)} ${y1.toFixed(2)}`;
}

/* ─── Level rendering (relative to noise floor) ─── */
const gain = computed(() => store.audio?.mic_gain ?? 50);
const frac = computed(() =>
  Math.min(1, Math.max(0, (displayDb.value - GATE_DB) / RANGE_DB)),
);
const peakFrac = computed(() => Math.min(1, Math.max(0, peakDb.value / RANGE_DB)));

const segments = computed(() => {
  const litN = Math.ceil(frac.value * SEGMENTS); // current-level tip (snapped to segment seam)
  const peakN = Math.ceil(peakFrac.value * SEGMENTS); // peak-hold trail tip
  return Array.from({ length: SEGMENTS }, (_, i) => ({
    d: arcPath(i),
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
  const [x, y] = polar(START_DEG + litN * SEG_DEG);
  return { x: x.toFixed(2), y: y.toFixed(2) };
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
  if (displayDb.value <= GATE_DB) return "idle"; // silence → no signal light
  return "ok";
});

/* ─── Apply a payload from backend (or mock) ─── */
function apply(p: MicLevelPayload) {
  rawDb.value = p.db;
  peakDb.value = p.peak_db;
  active.value = p.active;
  // Ease the display toward the raw level: rise fast, fall slowly.
  // Kills the preamp-hiss jitter (~±2 dB every 30ms) on a silent mic.
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
    unlisten = await listen<MicLevelPayload>("mic-level", (e) => apply(e.payload));
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
  <div class="ring-meter" :class="{ dead: !active }">
    <svg viewBox="0 0 88 88" role="img" aria-label="Microphone input level">
      <path
        v-for="(s, i) in segments"
        :key="i"
        :d="s.d"
        :class="s.cls"
        fill="none"
        stroke-width="6"
        stroke-linecap="butt"
      />
      <circle v-if="frac > 0" :cx="peakPos.x" :cy="peakPos.y" r="3.4" class="peak-dot" />
      <text x="44" y="45" text-anchor="middle" class="gain-text">{{ gain }}%</text>
      <text x="44" y="57" text-anchor="middle" class="gain-label">GAIN</text>
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
.ring-meter {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  font-family: var(--font-ui);
}
.ring-meter.dead .gain-text {
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
  font-size: 6px;
  font-weight: 600;
  letter-spacing: 1.2px;
}

/* segments */
path.off {
  stroke: var(--grid);
  opacity: 0.6;
}
path.lit {
  stroke: var(--accent);
  filter: drop-shadow(0 0 3px var(--accent-glow));
}
path.zone-w {
  stroke: var(--warn);
  filter: drop-shadow(0 0 3px rgba(224, 164, 88, 0.45));
}
path.zone-d {
  stroke: var(--danger);
  filter: drop-shadow(0 0 4px rgba(231, 76, 94, 0.55));
}
path.peak-trail {
  stroke: var(--accent);
  opacity: 0.32;
  filter: drop-shadow(0 0 2px var(--accent-glow));
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
}
</style>
