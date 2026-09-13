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
const PADDING = { top: 16, bottom: 24, left: 72, right: 24 };
const DOT_RADIUS = 5;
const HOVER_RADIUS = 8;
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
  const logMin = Math.log10(20);
  const logMax = Math.log10(20000);
  const usable = w - PADDING.left - PADDING.right;
  return PADDING.left + ((Math.log10(freq) - logMin) / (logMax - logMin)) * usable;
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
  ctx.fillStyle = "rgba(184, 198, 208, 0.85)";
  ctx.font = "12px var(--font-ui)";
  ctx.textAlign = "right";
  ctx.textBaseline = "middle";
  for (let db = MIN_DB; db <= MAX_DB; db += 3) {
    ctx.fillText(fmtDb(db), PADDING.left - 6, dbToY(db, h));
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

  // EQ curve with glow — 9 band points (dots sit exactly on each band,
  // leftmost dot = 64Hz, rightmost = 16kHz, matching the EPOS suite)
  const points: { x: number; y: number }[] = props.bands.map((b) => ({
    x: freqToX(b.freq, w),
    y: dbToY(b.gain_db, h),
  }));

  if (points.length > 1) {
    // Glow layer
    ctx.save();
    ctx.shadowColor = "rgba(78, 205, 196, 0.4)";
    ctx.shadowBlur = 7;
    ctx.strokeStyle = "rgba(78, 205, 196, 0.3)";
    ctx.lineWidth = 6;
    ctx.beginPath();
    ctx.moveTo(points[0].x, points[0].y);
    for (let i = 1; i < points.length; i++) {
      const cpx = (points[i - 1].x + points[i].x) / 2;
      ctx.bezierCurveTo(cpx, points[i - 1].y, cpx, points[i].y, points[i].x, points[i].y);
    }
    ctx.stroke();
    ctx.restore();

    // Main curve
    ctx.strokeStyle = "#4ecdc4";
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(points[0].x, points[0].y);
    for (let i = 1; i < points.length; i++) {
      const cpx = (points[i - 1].x + points[i].x) / 2;
      ctx.bezierCurveTo(cpx, points[i - 1].y, cpx, points[i].y, points[i].x, points[i].y);
    }
    ctx.stroke();
  }

  // White band dots with cyan glow (EPOS-style), 9 dots on the 9 bands
  const c2d = ctx;
  function dot(x: number, y: number) {
    c2d.beginPath();
    c2d.arc(x, y, DOT_RADIUS, 0, Math.PI * 2);
    c2d.save();
    c2d.shadowColor = "rgba(78, 205, 196, 0.8)";
    c2d.shadowBlur = 5;
    c2d.fillStyle = "#ffffff";
    c2d.fill();
    c2d.restore();
  }

  props.bands.forEach((band, i) => {
    const x = freqToX(band.freq, w);
    const y = dbToY(band.gain_db, h);
    const r = hovered.value === i || dragging.value === i ? HOVER_RADIUS : DOT_RADIUS;

    // Outer glow on hover
    if (hovered.value === i || dragging.value === i) {
      ctx.beginPath();
      ctx.arc(x, y, r + 6, 0, Math.PI * 2);
      ctx.fillStyle = "rgba(78, 205, 196, 0.15)";
      ctx.fill();
    }

    dot(x, y);
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
    <div class="band-controls">
      <div
        v-for="(band, i) in bands"
        :key="band.freq"
        class="band-slider"
      >
        <input
          type="range"
          :min="-12"
          :max="12"
          :step="0.5"
          :value="band.gain_db"
          orient="vertical"
          :class="{ active: selectedBand === i }"
          @input="onBandChange(i, ($event.target as HTMLInputElement).valueAsNumber)"
          @change="flushEmit"
          @focus="selectedBand = i"
        />
        <span class="band-value" :class="{ active: selectedBand === i }">
          {{ band.gain_db > 0 ? '+' : '' }}{{ band.gain_db }}
        </span>
      </div>
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
  height: 180px;
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

/* Band sliders */
.band-controls {
  display: flex;
  justify-content: space-around;
  margin-top: 8px;
}
.band-slider {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
}
.band-slider input[type="range"] {
  writing-mode: vertical-lr;
  direction: rtl;
  width: 20px;
  height: 70px;
  accent-color: var(--accent);
}
.band-slider input.active {
  outline: 2px solid var(--accent);
  outline-offset: 1px;
  border-radius: 4px;
}
.band-value {
  font-size: 9px;
  color: var(--faint);
  font-family: var(--font-mono);
}
.band-value.active {
  color: var(--accent);
  font-weight: 600;
}
</style>