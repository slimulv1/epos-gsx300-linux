<script setup lang="ts">
defineProps<{
  profiles: { name: string }[];
  active: string;
}>();

const emit = defineEmits<{
  select: [name: string];
  add: [];
  delete: [name: string];
}>();
</script>

<template>
  <div class="presets">
    <button
      v-for="profile in profiles"
      :key="profile.name"
      :class="['preset-btn', active === profile.name && 'active']"
      @click="emit('select', profile.name)"
    >
      {{ profile.name }}
      <span
        v-if="profile.name !== 'Flat'"
        class="preset-del"
        title="Delete preset"
        @click.stop="emit('delete', profile.name)"
      >
        ×
      </span>
    </button>
    <button class="preset-btn add" @click="$emit('add')">
      +
    </button>
  </div>
</template>

<style scoped>
.presets {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.preset-btn {
  padding: 8px 16px;
  background: #1e1e2e;
  border: 1px solid #2a2a3a;
  border-radius: 6px;
  color: #888;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.preset-btn:hover {
  background: #2a2a3a;
  color: #ccc;
}

.preset-btn.active {
  background: #00d4aa22;
  border-color: #00d4aa;
  color: #00d4aa;
}

.preset-btn.add {
  font-size: 16px;
  padding: 8px 12px;
}

.preset-del {
  margin-left: 8px;
  color: #ff6b6b;
  font-size: 14px;
  line-height: 1;
}

.preset-del:hover {
  color: #ff4444;
}
</style>
