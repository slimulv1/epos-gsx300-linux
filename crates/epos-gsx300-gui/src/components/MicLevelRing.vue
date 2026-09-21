<script setup lang="ts">
/* ============================================================================
   MicLevelRing.vue  -  LIVE MIC-INPUT LEVEL BAR  (FAIL-CLOSED, FROM SCRATCH).

   LOAD-BEARING INVARIANT (NEVER RELAX, never "optimize away"):
   --------------------------------------------------------------------------
   THE BAR LIGHTS ONLY WHEN `active === true`. `active` is set EXCLUSIVELY
   from the Tauri backend "mic-level" payload (`active` field). The Rust
   daemon flips active=true ONLY AFTER ITS OWN TWO-GATE CONFIRMATION of REAL
   audio:
       gate A = RMS/energy gate (real dB above the noise floor),
       gate B = spectral-flatness gate (SFM) - it is TALK/SPECTRUM, not
                hiss, not a flat silent floor,
       plus a loud bypass (>= 15 dB peaks are loud enough on their own).
   A silent / no-mic / hiss-only / no-device input:
       - NEVER lights a single segment,
       - NEVER shows a peak dot / trail,
       - NEVER shows any peak or clip hold,
       - readout stays "NO SIGNAL".
   With no Tauri runtime (pure browser dev) the bar is DARK and reads
   "NO SIGNAL". There is NO mock, NO mockActive, NO mock timer, NO
   self-animation, NO fabrication - not even under a fake label, not even
   "for browser dev", not even "for demo". This component NEVER fabricates
   audio - it does not even accept a fabricated signal as input.

   FAIL-CLOSED STRUCTURE (why this can never produce a fake bar):
   - `active` starts false and becomes true ONLY from a real payload.apply.
   - Every derived value (frac, segments, peak dot, peak trail, leds,
     db readout) is computed FROM `active` FIRST: if !active, everything is
     0 / off / "NO SIGNAL". Nothing is ever self-generated.
   - prefers-reduced-motion is honored (no continuous self-animation; the
     only movement is backend-driven easing on CONFIRMED real audio).
   Logo / meter content is pure non-color semantics where a landmark is
   needed: landmark seams at -6 / -3 dB use NON-color signaling (dashed
   ticks + text labels), so color never carries meaning on its own.

   PURE ASCII BY POLICY: a stray em-dash or CJK byte in this file breaks
   vue-tsc, which used to keep the OBSOLETE mock dist embedded in the GUI
   binary - exactly the "fake audio" you saw. Staying 100% ASCII guarantees
   every fresh `npm run build` regenerates a fail-closed dist and every
   fresh `cargo build` embeds it, permanently ending the fake bar.
   ========================================================================== */

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
/* Mic preamp gain readout under the bar (real daemon value, 0-100%). */
const gain = computed(() => Math.round(store.audio?.mic_gain ?? 0));

/*  Geometry: HORIZONTAL bar, 30 segments, viewBox 0 0 276 64.
       Track x 4..272, band y 36..68.  */
const BAR_X0 = 4;   /* left edge of track */
const BAR_W = 268;  /* track width (viewBox = 276 wide, 4px margins) */
const BAR_H = 32;   /* bar height */
const BAR_Y = 36;   /* bar top */
const SEGMENTS = 30;
const SEG_W = BAR_W / SEGMENTS;   /* ~8.93px per slot */
const SEG_SWEEP = 8.2;            /* lit span per segment (~0.73px notch) */
const RANGE_DB = 18;  /* 18 dB above the noise floor = full bar */
const GATE_DB = 3.5;  /* ambience below this NEVER lights the bar */
const LANDMARK_DB = [6, 3]; /* seams at -6 / -3 dB (NON-color signaling) */

function landmarkX(dbBelowScale: number): number {
  const frac = (RANGE_DB - Math.max(GATE_DB, dbBelowScale)) / RANGE_DB;
  return BAR_X0 + Math.min(1, Math.max(0, frac)) * BAR_W;
}
function segRect(i: number): { x: number; y: number; w: number; h: number } {
  return { x: BAR_X0 + i * SEG_W, y: BAR_Y, w: SEG_SWEEP, h: BAR_H };
}

/*  Live level state (fed ONLY by the backend "mic-level" payload;
       fail-closed: everything starts dark / zero / NO SIGNAL)  */
const active = ref(false);   /* TRUE only when backend CONFIRMED real audio */
const displayDb = ref(0);    /* eased copy that drives the bar (anti-jitter) */
const peakDb = ref(0);       /* relative peak-hold (backend-smoothed) */
const clipHold = ref(false); /* 500ms visual hold after last clip frame */
const clipTimer: ReturnType<typeof setTimeout> | null = null;

/*  Derived level (relative to gate) - FAIL-CLOSED  */
const frac = computed(() => {
  if (!active.value) return 0;              /* fail-closed: dark */
  return Math.min(1, Math.max(0, (displayDb.value - GATE_DB) / RANGE_DB));
});
const peakFrac = computed(() => {
  if (!active.value) return 0;
  return Math.min(1, Math.max(0, peakDb.value / RANGE_DB));
});
const segments = computed(() => {
  const litN = Math.ceil(frac.value * SEGMENTS);
  const peakN = Math.ceil(peakFrac.value * SEGMENTS);
  return Array.from({ length: SEGMENTS }, (_, i) => {
    const r = segRect(i);
    const dbBelow = RANGE_DB - ((i + 1) / SEGMENTS) * RANGE_DB;
    let cls: string = "off";
    if (i < litN) {
      cls = dbBelow >= 6 ? "zone-d" : dbBelow >= 3 ? "zone-w" : "lit";
    } else if (i < peakN) {
      cls = "peak-trail";
    }
    return { ...r, cls };
  });
});
const fracPos = computed(() => {
  if (!active.value || frac.value <= 0) {
    return { x: BAR_X0, y: BAR_Y + BAR_H / 2 };
  }
  const litN = Math.max(1, Math.ceil(frac.value * SEGMENTS));
  return { x: BAR_X0 + litN * SEG_W, y: BAR_Y + BAR_H / 2 };
});
const dbText = computed(() => {
  if (!active.value) return "NO SIGNAL";
  if (displayDb.value <= GATE_DB) return "0.0 dB";
  return displayDb.value.toFixed(1) + " dB";
});

/*  LED state machine (fail-closed)  */
type Led = "idle" | "ok" | "clip";
const led = computed<Led>(() => {
  if (!active.value) return "idle";
  if (clipHold.value) return "clip";
  if (displayDb.value <= GATE_DB) return "idle";
  return "ok";
});

let unlisten: UnlistenFn | null = null;

function apply(p: MicPayload) {
  if (!p.active) {
    /* NO confirmed real audio -> FAIL-CLOSED: zero EVERYTHING. The bar
       stays dark, no peak, no trail, no clip, readout "NO SIGNAL".
       Silent / hiss / no-mic input NEVER lights - never fabricates. */
    active.value = false;
    displayDb.value = 0;
    peakDb.value = 0;
    clipHold.value = false;
    return;
  }
  /* Real CONFIRMED audio -> ease toward it (rise fast, fall slow). This
     ONLY smooths REAL backend data; it never fabricates a signal. */
  active.value = true;
  const target = Math.max(0, Math.max(GATE_DB, p.db));
  const k = target > displayDb.value ? 0.5 : 0.12;
  displayDb.value = displayDb.value + (target - displayDb.value) * k;
  peakDb.value = Math.max(peakDb.value, p.peak_db);
  if (p.clip) {
    clipHold.value = true;
    if (clipTimer) clearTimeout(clipTimer);
    setTimeout(() => { clipHold.value = false; }, 500);
  }
}

onMounted(async () => {
  try {
    unlisten = await listen<MicPayload>("mic-level", (e) => apply(e.payload));
  } catch {
    /* No Tauri runtime (browser dev): component stays FAIL-CLOSED - the
       bar stays DARK, readout "NO SIGNAL". It NEVER fabricates audio -
       not under any label. */
  }
});

onUnmounted(() => {
  if (unlisten) unlisten();
  if (clipTimer) clearTimeout(clipTimer);
});
</script>

<template>
  <div class="bar-meter" :class="{ dead: !active }">
    <svg viewBox="0 0 276 64" role="img" aria-label="Microphone input level">
      <!-- landmark seams at -6 / -3 dB -> NON-color signaling: dashed
           ticks + text labels (color never carries the meaning alone) -->
      <line
        v-for="(l, i) in LANDMARK_DB" :key="i"
        :x1="landmarkX(l)" :x2="landmarkX(l)" y1="32" y2="72" class="landmark"
      />
      <text
        v-for="(l, i) in LANDMARK_DB" :key="'t' + i"
        :x="landmarkX(l)" y="82" text-anchor="middle" class="landmark-label"
      >{{ -l }}dB</text>

      <!-- 30 horizontal segments, lit left -> right (fail-closed) -->
      <rect
        v-for="(s, i) in segments" :key="i"
        :x="s.x" :y="s.y" :width="s.w" :height="s.h" :class="s.cls"
      />
      <!-- white peak dot rides exactly on the lit tip -->
      <circle
        v-if="frac > 0" :cx="fracPos.x" :cy="fracPos.y" r="4" class="peak-dot"
      />
    </svg>

    <div class="meter-meta">
      <span class="db-readout" :class="{ clippy: led === 'clip' }">{{ dbText }}</span>
      <span class="leds" aria-hidden="true">
        <span class="led" :class="{ on: led === 'idle' }">IDLE</span>
        <span class="led" :class="{ on: led === 'ok' }">OK</span>
        <span class="led clip-led" :class="{ on: led === 'clip' }">CLIP</span>
      </span>
      <span class="gain-readout">{{ gain }}% GAIN</span>
    </div>
  </div>
</template>

<style scoped>
.bar-meter {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  font-family: var(--font-ui, system-ui, -apple-system, "Segoe UI", sans-serif);
  width: min(100%, 384px);
}
.bar-meter.dead .db-readout { color: var(--muted); }

/* segments - fail-closed: OFF is the default, LIT granted only by backend */
rect.off { fill: var(--grid); opacity: 0.6; }
rect.lit { fill: var(--accent); filter: drop-shadow(0 0 3px var(--accent-glow)); }
rect.zone-w { fill: var(--warn); filter: drop-shadow(0 0 3px rgba(224, 164, 88, 0.45)); }
rect.zone-d { fill: var(--danger); filter: drop-shadow(0 0 4px rgba(231, 76, 94, 0.55)); }
rect.peak-trail { fill: var(--accent); opacity: 0.32; }

/* landmark seams -> NON-color: dashed ticks + text labels */
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
}

.peak-dot {
  fill: #ffffff;
  stroke: rgba(190, 240, 235, 0.85);
  stroke-width: 0.8;
  filter: drop-shadow(0 0 4px var(--accent)) drop-shadow(0 0 8px rgba(78, 205, 196, 0.45));
}

.meter-meta { display: flex; align-items: center; gap: 10px; }
.db-readout {
  font-family: var(--font-mono, monospace);
  font-size: 11px;
  color: var(--text);
  min-width: 74px;
  text-align: right;
  letter-spacing: 0.3px;
}
.db-readout.clippy { color: var(--danger); }
.leds { display: flex; gap: 7px; }
.led {
  font-size: 9px;
  font-weight: 600;
  letter-spacing: 0.8px;
  color: var(--grid);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 2px 5px;
  transition: color 0.12s ease, background 0.12s ease;
}
.led.on { background: rgba(78, 205, 196, 0.14); border-color: var(--accent-dim); color: var(--accent); }
.led.clip-led.on { background: rgba(231, 76, 94, 0.14); border-color: rgba(231, 76, 94, 0.5); color: var(--danger); }
.gain-readout {
  font-family: var(--font-mono, monospace);
  font-size: 9px;
  color: var(--muted);
  letter-spacing: 0.4px;
}

@media (prefers-reduced-motion: reduce) {
  rect.lit,
  rect.zone-w,
  rect.zone-d,
  rect.peak-trail,
  .peak-dot { transition: none; }
}
</style>
