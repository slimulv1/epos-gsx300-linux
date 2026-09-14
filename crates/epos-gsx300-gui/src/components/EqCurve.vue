<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";

interface EqBand {
  freq: number;
  gain_db: number;
  q: number;
}

const props = defineProps<{ bands: EqBand[] }>();
const emit = defineEmits<{ update: [bands: EqBand[]] }>();

const container = ref<HTMLDivElement | null>(null);
const canvas = ref<HTMLCanvasElement | null>(null);
const dragging = ref<number | null>(null);
const hovered = ref<number | null>(null);
const tooltip = ref<{ x: number; y: number; freq: number; db: number } | null>(null);

// Matches the EPOS Gaming Suite scale: ±6 dB every 3 dB, 9 bands 64..16k
const MIN_DB = -6;
const MAX_DB = 6;
// EPOS style: ±6 dB every 3 dB, 9 bands 64..16k, linear spacing.
// PADDING.left/right inset the outermost bands (64 / 16k) well clear of the
// frame — the running-light glow then ends ~30px inside the padding, never
// touching the border. PADDING.bottom is reduced so the dB ladder on the
// left stretches taller (more vertical room between the numbers).
const PADDING = { top: 16, bottom: 50, left: 170, right: 174 };
const DOT_RADIUS = 5;
const DISPLAY_STEP = 0.1; // min gain change that counts as a real edit
const EMIT_DEBOUNCE_MS = 250;

// Source of truth for drawing. Mirrors props.bands but updates immediately
// during drag/slider/keyboard so the curve never lags, while actual
// "update" emissions are debounced (see scheduleEmit).
const display = ref<EqBand[]>([]);
watch(
  () => props.bands,
  (b) => {
    if (dragging.value === null) display.value = b;
  },
  { deep: true, immediate: true }
);

// ── Debounced emit ──────────────────────────────────────────────
// Dragging an EQ band fires pointermove at 60-120 Hz; each "update"
// would trigger SetEq IPC → PipeWire config write → pipewire restart.
// Debounce so the daemon only reloads once per gesture.
let emitTimer: ReturnType<typeof setTimeout> | null = null;
let pending: EqBand[] | null = null;

function scheduleEmit(bands: EqBand[]) {
  pending = bands;
  if (emitTimer) clearTimeout(emitTimer);
  emitTimer = setTimeout(() => {
    emitTimer = null;
    if (pending) {
      emit("update", pending);
      pending = null;
    }
  }, EMIT_DEBOUNCE_MS);
}

function flushEmit() {
  if (emitTimer) {
    clearTimeout(emitTimer);
    emitTimer = null;
  }
  if (pending) {
    emit("update", pending);
    pending = null;
  }
}

// Shared mutation path: update display immediately (smooth), emit debounced.
function updateBand(index: number, value: number) {
  if (Math.abs(value - display.value[index].gain_db) < DISPLAY_STEP) return;
  const next = [...display.value];
  next[index] = { ...next[index], gain_db: value };
  display.value = next;
  scheduleEmit(next);
  draw();
}

function resizeCanvas() {
  const c = canvas.value;
  const ct = container.value;
  if (!c || !ct) return;
  const dpr = window.devicePixelRatio || 1;
  const w = ct.clientWidth;
  c.width = w * dpr;
  c.style.width = w + "px";
  // CRITICAL: never set style.height from the container's clientHeight.
  // The container's height is derived from the canvas (height: auto +
  // padding + band-controls), so feeding it back as the canvas height
  // creates a positive feedback loop where each ResizeObserver round
  // grows the EQ section by ~109px — "infinite height growth".
  // Keep the CSS-declared height (180px) and only mirror it into the
  // backing store with the device-pixel-ratio applied.
  const cssH = parseFloat(getComputedStyle(c).height) || 180;
  c.height = cssH * dpr;
  const ctx = c.getContext("2d");
  if (ctx) ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  draw();
}

let ro: ResizeObserver | null = null;
onMounted(() => {
  resizeCanvas();
  ro = new ResizeObserver(resizeCanvas);
  if (container.value) ro.observe(container.value);
});
onUnmounted(() => {
  ro?.disconnect();
  if (emitTimer) clearTimeout(emitTimer);
});

function freqToX(freq: number, w: number): number {
  // Linear frequency spacing, matching the physical EPOS GSX 300 UI
  // (measured from device reference screenshots: 9 bands evenly spaced).
  const FREQS = [64, 125, 250, 500, 1000, 2000, 4000, 8000, 16000];
  const i = FREQS.indexOf(freq);
  const usable = w - PADDING.left - PADDING.right;
  return PADDING.left + (Math.max(0, i) / (FREQS.length - 1)) * usable;
}

function dbToY(db: number, h: number): number {
  const usable = h - PADDING.top - PADDING.bottom;
  return PADDING.top + (1 - (db - MIN_DB) / (MAX_DB - MIN_DB)) * usable;
}

function yToDb(y: number, h: number): number {
  const usable = h - PADDING.top - PADDING.bottom;
  return Math.round((MIN_DB + (1 - (y - PADDING.top) / usable) * (MAX_DB - MIN_DB)) * 10) / 10;
}

function draw() {
  const c = canvas.value;
  if (!c) return;
  const ctx = c.getContext("2d");
  if (!ctx) return;
  const w = c.clientWidth;
  const h = c.clientHeight;
  ctx.clearRect(0, 0, w, h);

  // EPOS-style scale ticks: ±6 dB every 3 dB, formatted like the original
  // suite (+06, +03, 00, -03, -06)
  function fmtDb(db: number): string {
    if (db > 0) return `+${String(db).padStart(2, "0")}`;
    if (db < 0) return `-${String(-db).padStart(2, "0")}`;
    return "00";
  }

  // Faint dotted grid (EPOS keeps grid lines barely visible)
  ctx.fillStyle = "rgba(210, 220, 228, 0.08)";
  for (let db = MIN_DB; db <= MAX_DB; db += 3) {
    const y = dbToY(db, h);
    for (let x = PADDING.left; x <= w - PADDING.right; x += 12) {
      ctx.beginPath();
      ctx.arc(x, y, 1, 0, Math.PI * 2);
      ctx.fill();
    }
  }

  // Vertical grid lines at each band position (EPOS-style layout, faint)
  ctx.strokeStyle = "rgba(210, 220, 228, 0.07)";
  ctx.lineWidth = 1;
  ctx.setLineDash([4, 4]);
  props.bands.forEach((b) => {
    const x = freqToX(b.freq, w);
    ctx.beginPath();
    ctx.moveTo(x, PADDING.top);
    ctx.lineTo(x, h - PADDING.bottom);
    ctx.stroke();
  });
  ctx.setLineDash([]);

  // 0 dB line (slightly more visible than the grid, still faint)
  ctx.strokeStyle = "rgba(210, 220, 228, 0.14)";
  ctx.lineWidth = 1;
  ctx.setLineDash([6, 6]);
  ctx.beginPath();
  ctx.moveTo(PADDING.left, dbToY(0, h));
  ctx.lineTo(w - PADDING.right, dbToY(0, h));
  ctx.stroke();
  ctx.setLineDash([]);

  // dB scale ticks on the left (EPOS format: +06 / +03 / 00 / -03 / -06)
  // Drawn at a FIXED column (not PADDING.left - 6) so the curve glow that
  // overhangs band 64 can never light up the numbers.
  ctx.fillStyle = "rgba(226, 236, 244, 0.98)";
  ctx.font = "12px var(--font-ui)";
  ctx.textAlign = "right";
  ctx.textBaseline = "middle";
  for (let db = MIN_DB; db <= MAX_DB; db += 3) {
    ctx.fillText(fmtDb(db), 112, dbToY(db, h));
  }
  ctx.textBaseline = "alphabetic";
  ctx.textAlign = "center";

  // Band labels (x-axis)
  ctx.fillStyle = "rgba(138, 155, 160, 0.7)";
  ctx.font = "10px var(--font-ui)";
  ctx.textAlign = "center";
  props.bands.forEach((b) => {
    const x = freqToX(b.freq, w);
    const label = b.freq >= 1000 ? `${b.freq / 1000}k` : `${b.freq}`;
    ctx.fillText(label, x, h - 4);
  });

  // EQ curve with glow — uniform brightness from 64Hz to 16kHz, plus faint
  // "running-light" tails that extend beyond the outermost bands and taper
  // down to nothing (matches the device's light sweep)
  const points: { x: number; y: number }[] = props.bands.map((b) => ({
    x: freqToX(b.freq, w),
    y: dbToY(b.gain_db, h),
  }));

  if (points.length > 1) {
    const strokeSmooth = (width: number, color: string, shadow: boolean) => {
      ctx.save();
      if (shadow) {
        ctx.shadowColor = "rgba(78, 205, 196, 0.4)";
        ctx.shadowBlur = 7;
      }
      ctx.strokeStyle = color;
      ctx.lineWidth = width;
      ctx.lineJoin = "round";
      ctx.lineCap = "round";
      ctx.beginPath();
      ctx.moveTo(points[0].x, points[0].y);
      for (let i = 1; i < points.length; i++) {
        const cpx = (points[i - 1].x + points[i].x) / 2;
        ctx.bezierCurveTo(cpx, points[i - 1].y, cpx, points[i].y, points[i].x, points[i].y);
      }
      ctx.stroke();
      ctx.restore();
    };

    // 64Hz → 16kHz: uniform, full glow
    strokeSmooth(6.5, "rgba(78, 205, 196, 0.34)", true);
    strokeSmooth(3, "#4ecdc4", false);

    // Tail endpoints: extend in the direction leaving the outermost band,
    // same length both sides (balanced), kept inside the padding
    const TAIL_LEN = 30;
    const p0 = points[0];
    const p1 = points[1];
    const dl = Math.hypot(p0.x - p1.x, p0.y - p1.y) || 1;
    const extL = { x: p0.x + ((p0.x - p1.x) / dl) * TAIL_LEN, y: p0.y + ((p0.y - p1.y) / dl) * TAIL_LEN };
    const pn = points[points.length - 1];
    const pm = points[points.length - 2];
    const dr = Math.hypot(pn.x - pm.x, pn.y - pm.y) || 1;
    const extR = { x: pn.x + ((pn.x - pm.x) / dr) * TAIL_LEN, y: pn.y + ((pn.y - pm.y) / dr) * TAIL_LEN };

    // Draw one tail as 6 sub-strokes fading out toward the free end,
    // with a visibility floor so the tail reads as full-length on both sides
    const drawTail = (far: { x: number; y: number }, band: { x: number; y: number }) => {
      const SUB = 6;
      for (let i = 1; i <= SUB; i++) {
        const f0 = (i - 1) / SUB;
        const f1 = i / SUB;
        const fade = 0.5 + 0.5 * f1; // 1.0 at band → 0.58 at free end (still visible)
        const ax = far.x + (band.x - far.x) * f0;
        const ay = far.y + (band.y - far.y) * f0;
        const bx = far.x + (band.x - far.x) * f1;
        const by = far.y + (band.y - far.y) * f1;
        // Glow layer
        ctx.save();
        ctx.shadowColor = "rgba(78, 205, 196, 0.4)";
        ctx.shadowBlur = 7;
        ctx.strokeStyle = `rgba(78, 205, 196, ${(0.34 * fade * 0.75 + 0.04).toFixed(3)})`;
        ctx.lineWidth = 1.5 + 5 * fade;
        ctx.lineCap = "round";
        ctx.beginPath();
        ctx.moveTo(ax, ay);
        ctx.lineTo(bx, by);
        ctx.stroke();
        ctx.restore();
        // Main line
        ctx.save();
        ctx.strokeStyle = `rgba(78, 205, 196, ${(0.95 * fade).toFixed(3)})`;
        ctx.lineWidth = 0.9 + 1.3 * fade;
        ctx.lineCap = "round";
        ctx.beginPath();
        ctx.moveTo(ax, ay);
        ctx.lineTo(bx, by);
        ctx.stroke();
        ctx.restore();
      }
    };

    drawTail(extL, p0);
    drawTail(extR, pn);
  }

  // No static band dots — hover/drag gets a soft highlight ring instead
  props.bands.forEach((band, i) => {
    if (hovered.value === i || dragging.value === i) {
      const x = freqToX(band.freq, w);
      const y = dbToY(band.gain_db, h);
      ctx.beginPath();
      ctx.arc(x, y, DOT_RADIUS + 6, 0, Math.PI * 2);
      ctx.fillStyle = "rgba(78, 205, 196, 0.15)";
      ctx.fill();
    }
  });
}

function findNearestBand(clientX: number): number | null {
  const c = canvas.value;
  if (!c) return null;
  const rect = c.getBoundingClientRect();
  const x = clientX - rect.left;
  const w = c.clientWidth;
  let closest = 0;
  let minDist = Infinity;
  props.bands.forEach((band, i) => {
    const dist = Math.abs(x - freqToX(band.freq, w));
    if (dist < minDist) {
      minDist = dist;
      closest = i;
    }
  });
  return minDist < 30 ? closest : null;
}

function onPointerDown(e: PointerEvent) {
  const idx = findNearestBand(e.clientX);
  if (idx === null) return;
  dragging.value = idx;
  (e.target as HTMLElement).setPointerCapture(e.pointerId);
  updateTooltip(e);
}

function onPointerMove(e: PointerEvent) {
  const idx = findNearestBand(e.clientX);
  hovered.value = idx;

  if (dragging.value !== null && canvas.value) {
    const rect = canvas.value.getBoundingClientRect();
    const y = e.clientY - rect.top;
    const newDb = Math.max(MIN_DB, Math.min(MAX_DB, yToDb(y, canvas.value.clientHeight)));
    updateBand(dragging.value, newDb);
    updateTooltip(e);
  }
  draw();
}

function onPointerUp() {
  if (dragging.value !== null) flushEmit(); // send final position immediately
  dragging.value = null;
  tooltip.value = null;
  draw();
}

function onPointerLeave() {
  hovered.value = null;
  tooltip.value = null;
  draw();
}

function updateTooltip(e: PointerEvent) {
  if (dragging.value === null || !canvas.value) return;
  const band = display.value[dragging.value];
  const rect = canvas.value.getBoundingClientRect();
  tooltip.value = {
    x: e.clientX - rect.left,
    y: e.clientY - rect.top - 24,
    freq: band.freq,
    db: band.gain_db,
  };
}

function onBandChange(index: number, value: number) {
  updateBand(index, value);
  draw();
}

// Keyboard support
const selectedBand = ref<number | null>(null);
function onKeyDown(e: KeyboardEvent) {
  if (selectedBand.value === null) return;
  const step = e.shiftKey ? 3 : 1;
  if (e.key === "ArrowUp") {
    e.preventDefault();
    const band = props.bands[selectedBand.value];
    const newDb = Math.min(MAX_DB, band.gain_db + step);
    onBandChange(selectedBand.value, newDb);
  } else if (e.key === "ArrowDown") {
    e.preventDefault();
    const band = props.bands[selectedBand.value];
    const newDb = Math.max(MIN_DB, band.gain_db - step);
    onBandChange(selectedBand.value, newDb);
  } else if (e.key === "ArrowLeft") {
    e.preventDefault();
    selectedBand.value = Math.max(0, selectedBand.value - 1);
  } else if (e.key === "ArrowRight") {
    e.preventDefault();
    selectedBand.value = Math.min(props.bands.length - 1, selectedBand.value + 1);
  }
}

watch(
  () => props.bands,
  () => draw(),
  { deep: true }
);
</script>

<template>
  <div class="eq-curve" ref="container">
    <canvas
      ref="canvas"
      tabindex="0"
      role="slider"
      aria-label="Equalizer curve — use arrow keys to adjust selected band"
      @pointerdown="onPointerDown"
      @pointermove="onPointerMove"
      @pointerup="onPointerUp"
      @pointerleave="onPointerLeave"
      @keydown="onKeyDown"
      @click="selectedBand = findNearestBand($event.clientX)"
    />
    <div v-if="tooltip" class="tooltip" :style="{ left: tooltip.x + 'px', top: tooltip.y + 'px' }">
      {{ tooltip.freq >= 1000 ? `${tooltip.freq / 1000}k` : tooltip.freq }}Hz · {{ tooltip.db > 0 ? '+' : '' }}{{ tooltip.db }}dB
    </div>
  </div>
</template>

<style scoped>
.eq-curve {
  position: relative;
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 12px;
}

canvas {
  width: 100%;
  height: 300px;
  display: block;
  cursor: pointer;
  border-radius: 6px;
  touch-action: none;
}
canvas:focus {
  outline: 2px solid var(--accent);
  outline-offset: 2px;
}

/* Tooltip */
.tooltip {
  position: absolute;
  pointer-events: none;
  background: var(--panel-2);
  border: 1px solid var(--accent-dim);
  border-radius: 6px;
  padding: 4px 8px;
  font-size: 11px;
  color: var(--accent);
  font-weight: 500;
  white-space: nowrap;
  z-index: 10;
  transform: translateX(-50%);
}
</style>