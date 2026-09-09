<script setup lang="ts">
import { ref, onMounted, watch } from "vue";

interface EqBand {
  freq: number;
  gain_db: number;
  q: number;
}

const props = defineProps<{ bands: EqBand[] }>();
const emit = defineEmits<{ update: [bands: EqBand[]] }>();

const canvas = ref<HTMLCanvasElement | null>(null);
const dragging = ref<number | null>(null);

const MIN_DB = -12;
const MAX_DB = 12;
const PADDING = { top: 20, bottom: 30, left: 40, right: 20 };

function freqToX(freq: number, width: number): number {
  const logMin = Math.log10(20);
  const logMax = Math.log10(20000);
  const logFreq = Math.log10(freq);
  const usable = width - PADDING.left - PADDING.right;
  return PADDING.left + ((logFreq - logMin) / (logMax - logMin)) * usable;
}

function dbToY(db: number, height: number): number {
  const usable = height - PADDING.top - PADDING.bottom;
  const ratio = (db - MIN_DB) / (MAX_DB - MIN_DB);
  return PADDING.top + (1 - ratio) * usable;
}

function yToDb(y: number, height: number): number {
  const usable = height - PADDING.top - PADDING.bottom;
  const ratio = 1 - (y - PADDING.top) / usable;
  return Math.round((MIN_DB + ratio * (MAX_DB - MIN_DB)) * 10) / 10;
}

function draw() {
  const c = canvas.value;
  if (!c) return;
  const ctx = c.getContext("2d");
  if (!ctx) return;

  const w = c.width;
  const h = c.height;

  ctx.clearRect(0, 0, w, h);

  // Grid lines
  ctx.strokeStyle = "#1e1e2e";
  ctx.lineWidth = 1;
  for (let db = MIN_DB; db <= MAX_DB; db += 6) {
    const y = dbToY(db, h);
    ctx.beginPath();
    ctx.moveTo(PADDING.left, y);
    ctx.lineTo(w - PADDING.right, y);
    ctx.stroke();
  }

  // 0 dB line
  ctx.strokeStyle = "#333";
  ctx.lineWidth = 1;
  const zeroY = dbToY(0, h);
  ctx.beginPath();
  ctx.moveTo(PADDING.left, zeroY);
  ctx.lineTo(w - PADDING.right, zeroY);
  ctx.stroke();

  // EQ curve
  ctx.beginPath();
  ctx.strokeStyle = "#00d4aa";
  ctx.lineWidth = 2;

  const points = props.bands.map((b) => ({
    x: freqToX(b.freq, w),
    y: dbToY(b.gain_db, h),
  }));

  if (points.length > 0) {
    ctx.moveTo(points[0].x, points[0].y);
    for (let i = 1; i < points.length; i++) {
      const prev = points[i - 1];
      const curr = points[i];
      const cpx = (prev.x + curr.x) / 2;
      ctx.bezierCurveTo(cpx, prev.y, cpx, curr.y, curr.x, curr.y);
    }
  }
  ctx.stroke();

  // Band dots
  props.bands.forEach((band, i) => {
    const x = freqToX(band.freq, w);
    const y = dbToY(band.gain_db, h);

    ctx.beginPath();
    ctx.arc(x, y, 6, 0, Math.PI * 2);
    ctx.fillStyle = dragging.value === i ? "#fff" : "#00d4aa";
    ctx.fill();
    ctx.strokeStyle = "#00d4aa";
    ctx.lineWidth = 2;
    ctx.stroke();

    // Label
    ctx.fillStyle = "#888";
    ctx.font = "10px sans-serif";
    ctx.textAlign = "center";
    const label = band.freq >= 1000 ? `${band.freq / 1000}k` : `${band.freq}`;
    ctx.fillText(label, x, h - 8);
  });
}

function onPointerDown(e: PointerEvent) {
  if (!canvas.value) return;
  const rect = canvas.value.getBoundingClientRect();
  const x = e.clientX - rect.left;
  const w = canvas.value.width;
  // Find nearest band dot
  let closest = 0;
  let minDist = Infinity;
  props.bands.forEach((band, i) => {
    const bx = freqToX(band.freq, w);
    const dist = Math.abs(x - bx);
    if (dist < minDist) {
      minDist = dist;
      closest = i;
    }
  });
  dragging.value = closest;
  (e.target as HTMLElement).setPointerCapture(e.pointerId);
}

function onPointerMove(e: PointerEvent) {
  if (dragging.value === null || !canvas.value) return;
  const rect = canvas.value.getBoundingClientRect();
  const y = e.clientY - rect.top;
  const h = canvas.value.height;

  const newDb = Math.max(MIN_DB, Math.min(MAX_DB, yToDb(y, h)));
  const newBands = [...props.bands];
  newBands[dragging.value] = { ...newBands[dragging.value], gain_db: newDb };
  emit("update", newBands);
  draw();
}

function onPointerUp() {
  dragging.value = null;
}

function onBandChange(index: number, value: number) {
  const newBands = [...props.bands];
  newBands[index] = { ...newBands[index], gain_db: value };
  emit("update", newBands);
  draw();
}

watch(
  () => props.bands,
  () => { draw(); },
  { deep: true }
);

onMounted(() => {
  if (canvas.value) {
    canvas.value.width = canvas.value.clientWidth * 2;
    canvas.value.height = canvas.value.clientHeight * 2;
    const ctx = canvas.value.getContext("2d");
    if (ctx) ctx.scale(2, 2);
    draw();
  }
});
</script>

<template>
  <div class="eq-curve">
    <canvas
      ref="canvas"
      @pointerdown="onPointerDown"
      @pointermove="onPointerMove"
      @pointerup="onPointerUp"
    />
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
          @input="onBandChange(i, ($event.target as HTMLInputElement).valueAsNumber)"
        />
        <span class="band-value">{{ band.gain_db > 0 ? '+' : '' }}{{ band.gain_db }}dB</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.eq-curve {
  background: #12121a;
  border-radius: 8px;
  padding: 16px;
}

canvas {
  width: 100%;
  height: 200px;
  display: block;
  cursor: pointer;
  border-radius: 4px;
}

.band-controls {
  display: flex;
  justify-content: space-around;
  margin-top: 12px;
}

.band-slider {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.band-slider input[type="range"] {
  writing-mode: vertical-lr;
  direction: rtl;
  width: 24px;
  height: 80px;
  accent-color: #00d4aa;
}

.band-value {
  font-size: 10px;
  color: #666;
}
</style>
