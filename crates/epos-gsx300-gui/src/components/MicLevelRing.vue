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
const db = ref(-60);
const peakDb = ref(-60);
const active = ref(false);
const clipHold = ref(false); // 500ms visual hold after last clip frame

/* ─── Lifecycle handles ─── */
let unlisten: UnlistenFn | null = null;
let mockTimer: ReturnType<typeof setInterval> | null = null;
let clipTimer: ReturnType<typeof setTimeout> | null = null;

/* ─── Geometry: 28 segments, 0deg = 12 o'clock, clockwise ─── */
const CX = 44;
const CY = 44;
const R = 34;
const SEGMENTS = 28;
const SEG_DEG = 360 / SEGMENTS; // 12.857
const SEG_SWEEP = 10.4; // ~2.45deg gap between segments
const FALLBACK_OFF = -60; // dB floor mapped to 0%

function polar(angleDeg: number): [number, number] {
  const a = ((angleDeg - 90) * Math.PI) / 180; // 0deg => top; positive = clockwise
  return [CX + R * Math.cos(a), CY + R * Math.sin(a)];
}

function arcPath(i: number): string {
  const a0 = i * SEG_DEG;
  const a1 = a0 + SEG_SWEEP;
  const [x0, y0] = polar(a0);
  const [x1, y1] = polar(a1);
  return `M ${x0.toFixed(2)} ${y0.toFixed(2)} A ${R} ${R} 0 0 1 ${x1.toFixed(2)} ${y1.toFixed(2)}`;
}

/* ─── Level rendering ─── */
const gain = computed(() => store.audio?.mic_gain ?? 50);
const frac = computed(() => Math.min(1, Math.max(0, (db.value - FALLBACK_OFF) / 60)));
const peakFrac = computed(() => Math.min(1, Math.max(0, (peakDb.value - FALLBACK_OFF) / 60)));

const segments = computed(() =>
  Array.from({ length: SEGMENTS }, (_, i) => ({
    d: arcPath(i),
    cls: i < frac.value * SEGMENTS ? `lit zone-${i < 22 ? "a" : i < 26 ? "w" : "d"}` : "off",
    hold: i < peakFrac.value * SEGMENTS ? "hold" : "",
  })),
);

const peakPos = computed(() => {
  const [x, y] = polar(peakFrac.value * 360);
  return { x: x.toFixed(2), y: y.toFixed(2) };
});

const dbText = computed(() => {
  if (!active.value) return "NO SIGNAL";
  if (db.value <= -59.5) return "-∞ dB";
  return `${db.value.toFixed(1)} dB`;
});

/* ─── LED state machine ─── */
type LedState = "idle" | "ok" | "clip";
const led = computed<LedState>(() => {
  if (!active.value) return "idle";
  if (clipHold.value) return "clip";
  return "ok";
});

/* ─── Apply a payload from backend (or mock) ─── */
function apply(p: MicLevelPayload) {
  db.value = p.db;
  peakDb.value = p.peak_db;
  active.value = p.active;
  if (p.clip) {
    clipHold.value = true;
    if (clipTimer) clearTimeout(clipTimer);
    clipTimer = setTimeout(() => {
      clipHold.value = false;
    }, 500);
  } else if (!p.active) {
    clipHold.value = false;
  }
}

/* ─── Browser-dev mock (when no Tauri runtime) ─── */
let mockT = 0;
function startMock() {
  let peak = -60;
  mockTimer = setInterval(() => {
    mockT += 0.35;
    const v = Math.sin(mockT) * 0.5 + Math.sin(mockT * 2.1) * 0.35 + Math.sin(mockT * 4.3) * 0.15;
    const d = -50 + ((v + 1) / 2) * 44; // sweep -50..-6
    peak = Math.max(d, peak - 0.8); // simple peak decay (0.8dB / 120ms)
    apply({ db: d, peak_db: peak, clip: d >= -6, active: true });
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
        :class="[s.cls, s.hold ? 'hold' : '']"
        fill="none"
        stroke-width="6"
        stroke-linecap="butt"
      />
      <circle :cx="peakPos.x" :cy="peakPos.y" r="3.4" class="peak-dot" />
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
path.hold {
  opacity: 1;
}

.peak-dot {
  fill: var(--text);
  filter: drop-shadow(0 0 3px var(--accent));
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
