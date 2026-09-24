<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from "vue";
import { useDaemonStore } from "../stores/daemon";
import EqCurve from "../components/EqCurve.vue";
import { Play, Square, ChevronDown } from "lucide-vue-next";
import { invoke } from "@tauri-apps/api/core";

const store = useDaemonStore();

/* ─── Preset (active profile) — dropdown góc trên phải ─── */
const profileName = computed(() => store.status?.active_profile ?? "FLAT");
const menuOpen = ref(false);
const menuRef = ref<HTMLElement | null>(null);

function onDocMouseDown(e: MouseEvent) {
  if (menuRef.value && !menuRef.value.contains(e.target as Node)) {
    menuOpen.value = false;
  }
}
onMounted(() => document.addEventListener("mousedown", onDocMouseDown));
onBeforeUnmount(() => document.removeEventListener("mousedown", onDocMouseDown));

function pickProfile(name: string) {
  store.setActiveProfile(name);
  menuOpen.value = false;
}

/* ─── EQ ─── */
function onEqUpdate(bands: { freq: number; gain_db: number; q: number }[]) {
  store.setEqBands(bands);
}

/* ─── Surround mode ─── */
function onModeToggle() {
  store.toggleMode();
}

/* ─── Sidetone (reverberation mapped) ─── */
const sidetoneEnabled = computed(() => store.audio?.sidetone?.enabled ?? false);
const sidetoneLevel = computed(() => Math.round((store.audio?.sidetone?.level ?? 0) * 100));
function onSidetoneUpdate(enabled: boolean, level: number) {
  store.setSidetone(enabled, level / 100);
}

/* ─── Sound test ─── */
const soundPlaying = ref(false);
async function toggleSoundTest() {
  try {
    soundPlaying.value = (await invoke("play_test_tone", {
      eq_enabled: store.audio?.eq?.enabled ?? false,
    })) as boolean;
  } catch (e) {
    console.error("Sound test failed:", e);
  }
}

const disconnected = computed(() => !store.status?.device_connected);

/* The EQ toggle and the EQ doing something are separate facts. Bands flatter
   than 0.1 dB are dropped when the PipeWire conf is generated, so an EQ that
   is switched on with a flat curve puts no filter in the graph at all. The
   audio is right - a flat curve is transparent - but the old status reported
   only the toggle, so the interface said the EQ was on while nothing was
   filtering and the user edited a curve that did nothing.

   The band count is optional on purpose: without it (an older daemon, or no
   status yet) we say nothing rather than guessing. */
const eqActiveBandCount = computed(() => store.status?.eq_active_bands ?? null);
const eqLooksTransparent = computed(
  () =>
    (store.audio?.eq?.enabled ?? false) &&
    eqActiveBandCount.value !== null &&
    eqActiveBandCount.value === 0,
);

/* ─── Save preset modal ─── */
const saveOpen = ref(false);
const saveName = ref("");
const saveError = ref("");

function openSave() {
  saveName.value = "";
  saveError.value = "";
  saveOpen.value = true;
}

async function confirmSave() {
  const name = saveName.value.trim();
  if (!name) {
    saveError.value = "Name cannot be empty";
    return;
  }
  if (store.profiles.some((p) => p.name === name)) {
    saveError.value = "A preset with this name already exists";
    return;
  }
  const ok = await store.createProfile(name);
  if (ok) {
    saveOpen.value = false;
  } else {
    saveError.value = "Failed to save preset";
  }
}
</script>

<template>
  <div class="playback-view" :class="{ disabled: disconnected }">
    <!-- Profile dropdown — góc trên bên phải -->
    <div class="profile-menu" ref="menuRef">
      <button
        class="profile-btn"
        :disabled="disconnected"
        @click="menuOpen = !menuOpen"
      >
        {{ profileName }} <ChevronDown :size="12" class="chev" :class="{ open: menuOpen }" />
      </button>
      <div v-if="menuOpen" class="profile-list">
        <button
          v-for="p in store.profiles"
          :key="p.name"
          class="profile-item"
          :class="{ active: p.name === profileName }"
          @click="pickProfile(p.name)"
        >
          {{ p.name }}
        </button>
        <div class="profile-sep" />
        <button class="profile-item profile-save" @click="openSave">+ SAVE PRESET</button>
      </div>
    </div>

    <!-- Mode toggle -->
    <div class="mode-bar">
      <span class="mode-label">AUDIO MODE</span>
      <button class="mode-pill" @click="onModeToggle" :disabled="disconnected">
        <span class="pill-option" :class="{ selected: store.mode === 'stereo' }">2.0</span>
        <span class="pill-option" :class="{ selected: store.mode === 'surround71' }">7.1</span>
      </button>
    </div>

    <!-- EQ graph -->
    <div class="eq-section">
      <h3 class="section-title">EQUALIZER</h3>
      <p v-if="eqLooksTransparent" class="eq-note">
        ON, BUT NOT FILTERING - every band is flat, so the graph has no filter
        in it. Move a band off 0 dB and the change will be audible.
      </p>
      <p v-else-if="(eqActiveBandCount ?? 0) > 0" class="eq-note ok">
        {{ eqActiveBandCount }} band{{ eqActiveBandCount === 1 ? "" : "s" }}
        active.
      </p>
      <EqCurve
        :bands="store.audio?.eq?.bands ?? []"
        @update="onEqUpdate"
      />
    </div>

    <!-- Controls row -->
    <div class="controls-row">
      <!-- Sidetone -->
      <div class="control-card">
        <h3 class="section-title">REVERBERATION</h3>
        <div class="sidetone-row">
          <label class="toggle-wrap">
            <input
              type="checkbox"
              :checked="sidetoneEnabled"
              :disabled="disconnected"
              @change="onSidetoneUpdate(($event.target as HTMLInputElement).checked, sidetoneLevel)"
            />
            <span class="toggle-track"><span class="toggle-thumb" /></span>
          </label>
          <input
            type="range"
            class="accent-range"
            :min="0"
            :max="100"
            :value="sidetoneLevel"
            :disabled="!sidetoneEnabled || disconnected"
            @input="onSidetoneUpdate(sidetoneEnabled, ($event.target as HTMLInputElement).valueAsNumber)"
          />
          <span class="value-badge">{{ sidetoneLevel }}%</span>
        </div>
      </div>

      <!-- Sound test -->
      <div class="control-card">
        <h3 class="section-title">SOUND TEST</h3>
        <button
          class="epos-btn sound-btn"
          :class="{ playing: soundPlaying }"
          @click="toggleSoundTest"
          :disabled="disconnected"
        >
          <Play v-if="!soundPlaying" :size="16" />
          <Square v-else :size="14" />
          {{ soundPlaying ? 'STOP' : 'PLAY' }}
        </button>
      </div>
    </div>

    <!-- Save preset modal -->
    <div v-if="saveOpen" class="modal-overlay" @click.self="saveOpen = false">
      <div class="modal">
        <h3 class="modal-title">SAVE PRESET</h3>
        <input
          v-model="saveName"
          class="modal-input"
          placeholder="Preset name"
          autofocus
          @keydown.enter="confirmSave"
          @keydown.esc="saveOpen = false"
        />
        <p v-if="saveError" class="modal-error">{{ saveError }}</p>
        <div class="modal-actions">
          <button class="epos-btn" @click="saveOpen = false">CANCEL</button>
          <button class="epos-btn epos-btn--accent" @click="confirmSave">SAVE</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.playback-view {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}
.playback-view.disabled {
  opacity: 0.45;
  pointer-events: none;
}

/* Profile dropdown — góc trên bên phải (như GSX 300 thật) */
.playback-view {
  position: relative;
}
.profile-menu {
  position: absolute;
  top: 0;
  right: 0;
  z-index: 40;
}
.profile-btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 10px;
  background: transparent;
  border: 1px solid transparent;
  border-radius: 6px;
  color: #fff;
  font-size: var(--fs-sm);
  font-weight: 600;
  letter-spacing: 0.5px;
  text-transform: uppercase;
  font-family: var(--font-ui);
  cursor: pointer;
}
.profile-btn:hover {
  background: rgba(255, 255, 255, 0.06);
  border-color: var(--border);
}
.profile-btn:disabled {
  opacity: 0.4;
  cursor: default;
}
.profile-btn .chev {
  transition: transform var(--time-fast) var(--ease);
}
.profile-btn .chev.open {
  transform: rotate(180deg);
}
.profile-list {
  position: absolute;
  top: calc(100% + 6px);
  right: 0;
  min-width: 180px;
  background: #000;
  border: 1px solid #2a2f35;
  border-radius: 8px;
  padding: 4px;
  box-shadow: 0 12px 32px rgba(0, 0, 0, 0.7);
}
.profile-item {
  display: block;
  width: 100%;
  text-align: left;
  padding: 8px 12px;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: #fff;
  font-size: var(--fs-sm);
  font-family: var(--font-ui);
  letter-spacing: 0.5px;
  cursor: pointer;
}
.profile-item:hover {
  background: #1a1f24;
}
.profile-item.active {
  color: var(--accent);
  background: #101418;
}
.profile-sep {
  height: 1px;
  margin: 4px 8px;
  background: #262b31;
}
.profile-save {
  color: var(--muted);
}
.profile-save:hover {
  color: var(--accent);
  background: #1a1f24;
}

/* Mode pill */
.mode-bar {
  display: flex;
  align-items: center;
  gap: 12px;
}
.mode-label {
  font-size: var(--fs-xs);
  color: var(--muted);
  font-weight: 600;
  letter-spacing: 0.5px;
  text-transform: uppercase;
  flex-shrink: 0;
}
.mode-pill {
  display: flex;
  border: 1px solid var(--border);
  border-radius: 20px;
  overflow: hidden;
  background: var(--panel);
}
.pill-option {
  padding: 6px 18px;
  font-size: var(--fs-sm);
  font-weight: 500;
  color: var(--muted);
  transition: all var(--time-fast) var(--ease);
}
.pill-option.selected {
  background: var(--accent);
  color: var(--bg);
  border-radius: 20px;
}

/* Section titles */
.section-title {
  font-size: var(--fs-xs);
  color: var(--muted);
  font-weight: 600;
  letter-spacing: 0.5px;
  text-transform: uppercase;
  margin-bottom: var(--space-2);
}

.eq-note {
  font-size: var(--fs-xs);
  color: var(--warn);
  margin: 0 0 var(--space-2);
  line-height: 1.4;
}

.eq-note.ok {
  color: var(--muted);
}

/* Controls row */
.controls-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-3);
}
.control-card {
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: var(--space-3);
}

/* Sidetone */
.sidetone-row {
  display: flex;
  align-items: center;
  gap: 12px;
}
.accent-range {
  flex: 1;
  accent-color: var(--accent);
}
.value-badge {
  font-size: var(--fs-xs);
  color: var(--muted);
  min-width: 36px;
  text-align: right;
}

/* Toggle */
.toggle-wrap {
  position: relative;
  display: inline-block;
  cursor: pointer;
}
.toggle-wrap input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}
.toggle-track {
  display: block;
  width: 36px;
  height: 20px;
  background: var(--grid);
  border-radius: 20px;
  transition: background var(--time-fast) var(--ease);
}
.toggle-thumb {
  display: block;
  width: 16px;
  height: 16px;
  background: var(--muted);
  border-radius: 50%;
  margin: 2px;
  transition: all var(--time-fast) var(--ease);
}
.toggle-wrap input:checked + .toggle-track {
  background: var(--accent-dim);
}
.toggle-wrap input:checked + .toggle-track .toggle-thumb {
  transform: translateX(16px);
  background: var(--accent);
}

/* Sound test */
.sound-btn {
  min-width: 100px;
}
.sound-btn.playing {
  border-color: var(--danger);
  color: var(--danger);
}

/* Save preset modal */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(10, 15, 17, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}
.modal {
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: var(--space-4);
  width: min(360px, 90vw);
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}
.modal-title {
  font-size: var(--fs-sm);
  color: var(--text);
  font-weight: 600;
  letter-spacing: 0.5px;
  text-transform: uppercase;
}
.modal-input {
  padding: 8px 12px;
  background: var(--panel-2);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text);
  font-size: var(--fs-sm);
  font-family: var(--font-ui);
}
.modal-input:focus {
  border-color: var(--accent);
  outline: none;
}
.modal-error {
  font-size: var(--fs-xs);
  color: var(--danger);
}
.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

/* Responsive */
@media (max-width: 700px) {
  .controls-row {
    grid-template-columns: 1fr;
  }
}
</style>