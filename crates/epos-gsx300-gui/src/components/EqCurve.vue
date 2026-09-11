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

const MIN_DB = -12;
const MAX_DB = 12;
const PADDING = { top: 16, bottom: 24, left: 36, right: 16 };
const DOT_RADIUS = 5;
const HOVER_RADIUS = 8;

function resizeCanvas() {
  const c = canvas.value;
  const ct = container.value;
  if (!c || !ct) return;
  const dpr = window.devicePixelRatio || 1;
  const w = ct.clientWidth;
  const h = ct.clientHeight;
  c.width = w * dpr;
  c.height = h * dpr;
  c.style.width = w + "px";
  c.style.height = h + "px";
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

  // Dotted grid
  ctx.fillStyle = "rgba(42, 58, 63, 0.6)";
  for (let db = MIN_DB; db <= MAX_DB; db += 3) {
    const y = dbToY(db, h);
    for (let x = PADDING.left; x <= w - PADDING.right; x += 12) {
      ctx.beginPath();
      ctx.arc(x, y, 1, 0, Math.PI * 2);
      ctx.fill();
    }
  }

  // 0 dB line
  ctx.strokeStyle = "rgba(42, 58, 63, 1)";
  ctx.lineWidth = 1;
  ctx.setLineDash([4, 4]);
  ctx.beginPath();
  ctx.moveTo(PADDING.left, dbToY(0, h));
  ctx.lineTo(w - PADDING.right, dbToY(0, h));
  ctx.stroke();
  ctx.setLineDash([]);

  // Band labels (x-axis)
  ctx.fillStyle = "rgba(138, 155, 160, 0.7)";
  ctx.font = "10px var(--font-ui)";
  ctx.textAlign = "center";
  props.bands.forEach((b) => {
    const x = freqToX(b.freq, w);
    const label = b.freq >= 1000 ? `${b.freq / 1000}k` : `${b.freq}`;
    ctx.fillText(label, x, h - 4);
  });

  // EQ curve with glow
  const points = props.bands.map((b) => ({
    x: freqToX(b.freq, w),
    y: dbToY(b.gain_db, h),
  }));

  if (points.length > 1) {
    // Glow layer
    ctx.save();
    ctx.shadowColor = "rgba(78, 205, 196, 0.4)";
    ctx.shadowBlur = 10;
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

  // Band dots
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

    ctx.beginPath();
    ctx.arc(x, y, r, 0, Math.PI * 2);
    ctx.fillStyle = dragging.value === i ? "#ffffff" : "#4ecdc4";
    ctx.fill();
    ctx.strokeStyle = "#4ecdc4";
    ctx.lineWidth = 2;
    ctx.stroke();
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
    const newBands = [...props.bands];
    newBands[dragging.value] = { ...newBands[dragging.value], gain_db: newDb };
    emit("update", newBands);
    updateTooltip(e);
  }
  draw();
}

function onPointerUp() {
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
  const band = props.bands[dragging.value];
  const rect = canvas.value.getBoundingClientRect();
  tooltip.value = {
    x: e.clientX - rect.left,
    y: e.clientY - rect.top - 24,
    freq: band.freq,
    db: band.gain_db,
  };
}

function onBandChange(index: number, value: number) {
  const newBands = [...props.bands];
  newBands[index] = { ...newBands[index], gain_db: value };
  emit("update", newBands);
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