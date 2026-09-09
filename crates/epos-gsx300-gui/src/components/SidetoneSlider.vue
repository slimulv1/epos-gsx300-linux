<script setup lang="ts">
defineProps<{
  enabled: boolean;
  level: number;
}>();

const emit = defineEmits<{
  update: [enabled: boolean, level: number];
}>();
</script>

<template>
  <div class="sidetone">
    <div class="control-row">
      <label class="toggle">
        <input
          type="checkbox"
          :checked="enabled"
          @change="emit('update', ($event.target as HTMLInputElement).checked, level)"
        />
        <span class="slider" />
      </label>
      <span class="label">{{ enabled ? 'On' : 'Off' }}</span>
    </div>
    <input
      type="range"
      :min="0"
      :max="100"
      :value="Math.round(level * 100)"
      :disabled="!enabled"
      class="range"
      @input="emit('update', enabled, ($event.target as HTMLInputElement).valueAsNumber / 100)"
    />
    <span class="value">{{ Math.round(level * 100) }}%</span>
  </div>
</template>

<style scoped>
.sidetone {
  background: #12121a;
  border-radius: 8px;
  padding: 16px;
}

.control-row {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 12px;
}

.label {
  font-size: 13px;
  color: #888;
}

.range {
  width: 100%;
  accent-color: #00d4aa;
}

.value {
  font-size: 12px;
  color: #666;
  display: block;
  text-align: right;
  margin-top: 4px;
}

.toggle {
  position: relative;
  width: 36px;
  height: 20px;
  cursor: pointer;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle .slider {
  position: absolute;
  inset: 0;
  background: #333;
  border-radius: 20px;
  transition: 0.3s;
}

.toggle .slider::before {
  content: '';
  position: absolute;
  width: 16px;
  height: 16px;
  left: 2px;
  top: 2px;
  background: #888;
  border-radius: 50%;
  transition: 0.3s;
}

.toggle input:checked + .slider {
  background: #00d4aa44;
}

.toggle input:checked + .slider::before {
  transform: translateX(16px);
  background: #00d4aa;
}
</style>
