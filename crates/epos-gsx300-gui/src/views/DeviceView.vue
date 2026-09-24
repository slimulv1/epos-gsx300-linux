<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";
import { useDaemonStore } from "../stores/daemon";
import { Usb, MonitorSpeaker, Radio, Cpu, Volume2, ChevronDown } from "lucide-vue-next";

const store = useDaemonStore();

const actionLabels: Record<string, string> = {
  toggle_mode: "Toggle stereo ⇄ 7.1 (EPOS default)",
  toggle_eq: "Toggle EQ on/off",
  cycle_preset: "Cycle through presets",
  toggle_sidetone: "Toggle sidetone",
  toggle_noise_gate: "Toggle noise gate",
};

const menuOpen = ref(false);
const actionMenuRef = ref<HTMLElement | null>(null);

function currentAction(): string {
  return store.status?.smart_button_action ?? "toggle_mode";
}

function pickAction(key: string) {
  store.setSmartButton(key);
  menuOpen.value = false;
}

function toggleMenu() {
  menuOpen.value = !menuOpen.value;
}

function onDocClick(e: MouseEvent) {
  if (menuOpen.value && actionMenuRef.value && !actionMenuRef.value.contains(e.target as Node)) {
    menuOpen.value = false;
  }
}

onMounted(() => document.addEventListener("mousedown", onDocClick));
onBeforeUnmount(() => document.removeEventListener("mousedown", onDocClick));
</script>

<template>
  <div class="device-view">
    <!-- Product card -->
    <div class="product-card">
      <div class="product-icon">
        <svg viewBox="0 0 80 80" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect x="8" y="20" width="64" height="40" rx="8" stroke="var(--accent)" stroke-width="2" fill="none" />
          <circle cx="40" cy="40" r="14" stroke="var(--accent)" stroke-width="2" fill="none" />
          <circle cx="40" cy="40" r="6" fill="var(--accent)" opacity="0.3" />
          <line x1="4" y1="40" x2="8" y2="40" stroke="var(--muted)" stroke-width="2" />
          <line x1="72" y1="40" x2="76" y2="40" stroke="var(--muted)" stroke-width="2" />
        </svg>
      </div>
      <div class="product-info">
        <span class="product-name">EPOS GSX 300</span>
        <span class="product-sub">External Gaming DAC · USB</span>
        <span class="product-mode">
          {{ store.mode === 'surround71' ? '7.1 Surround' : '2.0 Stereo' }}
        </span>
      </div>
    </div>

    <!-- Device info -->
    <div class="section-card" v-if="store.device">
      <h3 class="section-title">DEVICE INFO</h3>
      <div class="info-grid">
        <div class="info-row">
          <Usb :size="14" class="info-icon" />
          <span class="info-label">USB</span>
          <span class="info-value">bus {{ store.device.usb_bus }} · addr {{ store.device.usb_addr }}</span>
        </div>
        <div class="info-row">
          <MonitorSpeaker :size="14" class="info-icon" />
          <span class="info-label">ALSA</span>
          <span class="info-value">card {{ store.device.alsa_card ?? "not enumerated" }}</span>
        </div>
        <div class="info-row">
          <Radio :size="14" class="info-icon" />
          <span class="info-label">HID</span>
          <span class="info-value mono">{{ store.device.hidraw || '—' }}</span>
        </div>
        <div class="info-row">
          <Cpu :size="14" class="info-icon" />
          <span class="info-label">Firmware</span>
          <span class="info-value">{{ store.device.firmware_version || 'unknown' }}</span>
        </div>
        <div class="info-row">
          <Volume2 :size="14" class="info-icon" />
          <span class="info-label">Volume dial</span>
          <span class="info-value">{{ store.status?.volume ?? '—' }}%</span>
        </div>
      </div>
    </div>

    <!-- Smart button -->
    <div class="section-card">
      <h3 class="section-title">SMART BUTTON</h3>
      <p class="hint">What happens when you press the dial on the GSX 300.</p>
      <div class="action-menu" :class="{ open: menuOpen }" ref="actionMenuRef">
        <button type="button" class="action-btn" @click="toggleMenu">
          <span>{{ actionLabels[currentAction()] }}</span>
          <ChevronDown :size="14" class="action-caret" />
        </button>
        <transition name="drop">
          <div v-if="menuOpen" class="action-list">
            <button
              v-for="(label, key) in actionLabels"
              :key="key"
              type="button"
              class="action-item"
              :class="{ active: key === currentAction() }"
              @click="pickAction(key)"
            >
              {{ label }}
            </button>
          </div>
        </transition>
      </div>
    </div>
  </div>
</template>

<style scoped>
.device-view {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

/* Product card */
.product-card {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: var(--space-3);
}
.product-icon {
  flex-shrink: 0;
}
.product-icon svg {
  width: 80px;
  height: 80px;
}
.product-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.product-name {
  font-size: var(--fs-lg);
  font-weight: 700;
  color: var(--text);
}
.product-sub {
  font-size: var(--fs-sm);
  color: var(--muted);
}
.product-mode {
  font-size: var(--fs-xs);
  color: var(--accent);
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

/* Section */
.section-card {
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: var(--space-3);
}
.section-title {
  font-size: var(--fs-xs);
  color: var(--muted);
  font-weight: 600;
  letter-spacing: 0.5px;
  text-transform: uppercase;
  margin-bottom: var(--space-2);
}
.hint {
  font-size: var(--fs-sm);
  color: var(--faint);
  margin-bottom: var(--space-2);
}

/* Info grid */
.info-grid {
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.info-row {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 0;
  border-bottom: 1px solid var(--border);
}
.info-row:last-child {
  border-bottom: none;
}
.info-icon {
  color: var(--muted);
  flex-shrink: 0;
}
.info-label {
  font-size: var(--fs-xs);
  color: var(--muted);
  min-width: 50px;
  text-transform: uppercase;
  font-weight: 600;
  letter-spacing: 0.3px;
}
.info-value {
  font-size: var(--fs-sm);
  color: var(--text);
  text-align: right;
  word-break: break-all;
}
.mono {
  font-family: var(--font-mono);
  font-size: var(--fs-xs);
}

/* Smart button dropdown — black bg, white text */
.action-menu {
  position: relative;
}
.action-btn {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  width: 100%;
  padding: 10px 12px;
  background: #000;
  border: 1px solid var(--border);
  border-radius: 8px;
  color: #fff;
  font-size: var(--fs-sm);
  font-family: var(--font-ui);
  cursor: pointer;
  text-align: left;
}
.action-btn:hover {
  border-color: var(--accent-dim);
}
.action-btn:focus-visible {
  outline: none;
  border-color: var(--accent);
}
.action-caret {
  color: var(--accent);
  flex-shrink: 0;
  transition: transform 0.15s ease;
}
.action-menu.open .action-caret,
.action-btn[aria-expanded="true"] .action-caret {
  transform: rotate(180deg);
}
.action-list {
  position: absolute;
  top: calc(100% + 6px);
  left: 0;
  right: 0;
  z-index: 50;
  background: #000;
  border: 1px solid var(--border);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.7);
  padding: 4px;
  display: flex;
  flex-direction: column;
}
.action-item {
  display: block;
  width: 100%;
  padding: 9px 12px;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: #fff;
  font-size: var(--fs-sm);
  font-family: var(--font-ui);
  text-align: left;
  cursor: pointer;
}
.action-item:hover {
  background: rgba(78, 205, 196, 0.12);
}
.action-item.active {
  color: var(--accent);
  background: rgba(78, 205, 196, 0.08);
}
.drop-enter-active,
.drop-leave-active {
  transition: opacity 0.12s ease, transform 0.12s ease;
}
.drop-enter-from,
.drop-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}

/* Responsive: narrow window */
@media (max-width: 500px) {
  .product-icon svg {
    width: 56px;
    height: 56px;
  }
  .product-name {
    font-size: var(--fs-md);
  }
}
</style>