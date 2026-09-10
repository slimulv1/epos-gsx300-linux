<script setup lang="ts">
defineProps<{ mode: string }>();
const emit = defineEmits<{ update: [mode: string] }>();

const modes = ["off", "warm", "clear", "custom"];
</script>

<template>
  <div class="voice">
    <button
      v-for="m in modes"
      :key="m"
      :class="['mode-btn', mode === m && 'active']"
      @click="emit('update', m)"
    >
      {{ m.charAt(0).toUpperCase() + m.slice(1) }}
    </button>
  </div>
</template>

<style scoped>
.voice {
  background: #12121a;
  border-radius: 8px;
  padding: 16px;
  display: flex;
  gap: 8px;
}

.mode-btn {
  flex: 1;
  padding: 10px;
  background: #1e1e2e;
  border: 1px solid #2a2a3a;
  border-radius: 6px;
  color: #888;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.mode-btn:hover {
  background: #2a2a3a;
  color: #ccc;
}

.mode-btn.active {
  background: #00d4aa22;
  border-color: #00d4aa;
  color: #00d4aa;
}
</style>
