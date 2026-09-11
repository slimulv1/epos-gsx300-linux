import { defineStore } from "pinia";
import { ref } from "vue";

// Tauri invoke — loaded dynamically so browser dev mode can fall back
let tauriInvoke: ((cmd: string, args?: Record<string, unknown>) => Promise<any>) | null = null;
try {
  // @tauri-apps/api is available in Tauri runtime; may throw in plain browser
  const mod = await import("@tauri-apps/api/core");
  tauriInvoke = mod.invoke;
} catch {
  // Running outside Tauri (e.g. plain `vite dev`)
}

// ─── Types (mirror Rust IPC structs) ────────────────────────

export interface EqBand {
  freq: number;
  gain_db: number;
  q: number;
}

export interface AudioConfig {
  eq: {
    enabled: boolean;
    bands: EqBand[];
  };
  sidetone: { enabled: boolean; level: number };
  noise_gate: { enabled: boolean; threshold_db: number };
  voice_enhancer: { mode: string; custom_bands: EqBand[] | null };
  mic_gain: number;
}

export interface Profile {
  name: string;
  audio: AudioConfig;
  created_at: string;
}

export type AudioMode = "stereo" | "surround71";

export interface DeviceInfo {
  usb_bus: number;
  usb_addr: number;
  alsa_card: number;
  pipewire_sink: string;
  pipewire_source: string;
  hidraw: string | null;
  input_event: string | null;
  firmware_version: string | null;
}

export interface DeviceStatus {
  daemon_version: string;
  device_connected: boolean;
  eq_active: boolean;
  active_profile: string;
  mode: AudioMode;
  smart_button_action?: string;
  volume?: number;
}

// ─── Response shape from daemon ─────────────────────────────
interface DaemonResponse {
  type: string;
  payload: any;
}

// Default 9-band EQ (matches daemon epos-shared defaults)
export function defaultBands(): EqBand[] {
  return [
    { freq: 64, gain_db: 0, q: 1 },
    { freq: 125, gain_db: 0, q: 1 },
    { freq: 250, gain_db: 0, q: 1 },
    { freq: 500, gain_db: 0, q: 1 },
    { freq: 1000, gain_db: 0, q: 1 },
    { freq: 2000, gain_db: 0, q: 1 },
    { freq: 4000, gain_db: 0, q: 1 },
    { freq: 8000, gain_db: 0, q: 1 },
    { freq: 16000, gain_db: 0, q: 1 },
  ];
}

export const useDaemonStore = defineStore("daemon", () => {
  // ─── State ────────────────────────────────────────────────
  const connected = ref(false);
  const status = ref<DeviceStatus | null>(null);
  const audio = ref<AudioConfig | null>(null);
  const profiles = ref<Profile[]>([]);
  const mode = ref<AudioMode>("stereo");
  const device = ref<DeviceInfo | null>(null);

  // ─── IPC Layer ────────────────────────────────────────────

  /**
   * Production (Tauri runtime): use invoke("daemon_request") ONLY.
   * No HTTP bridge, no mock — a failed invoke marks the GUI disconnected
   * so the disabled-state banner shows instead of fake data.
   *
   * Browser dev only: try HTTP bridge (127.0.0.1:9898), then mock.
   */
  async function sendRequest(request: Record<string, unknown>): Promise<DaemonResponse | null> {
    if (tauriInvoke) {
      try {
        const jsonResponse: string = await tauriInvoke("daemon_request", {
          request: JSON.stringify(request),
        });
        connected.value = true;
        return JSON.parse(jsonResponse) as DaemonResponse;
      } catch {
        connected.value = false;
        return null;
      }
    }

    // Browser dev mode — HTTP bridge first, then mock
    try {
      const resp = await fetch("http://127.0.0.1:9898/ipc", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(request),
      });
      if (resp.ok) {
        return (await resp.json()) as DaemonResponse;
      }
      const errBody = await resp.json().catch(() => null);
      if (errBody) return errBody as DaemonResponse;
    } catch {
      // Bridge not reachable → mock for this call only
    }
    return mockResponse(request);
  }

  // ─── Mock Data (browser dev fallback only) ────────────────

  let mockMode: AudioMode = "stereo";
  const mockStatus = {
    daemon_version: "0.1.0",
    device_connected: true,
    eq_active: false,
    active_profile: "Flat",
    mode: mockMode as AudioMode,
    smart_button_action: "toggle_mode",
  };

  function mockResponse(request: Record<string, unknown>): DaemonResponse {
    switch (request.type) {
      case "GetStatus":
        return { type: "Status", payload: { ...mockStatus, mode: mockMode } };
      case "GetMode":
        return { type: "Mode", payload: mockMode };
      case "ToggleMode":
        mockMode = mockMode === "stereo" ? "surround71" : "stereo";
        return { type: "Mode", payload: mockMode };
      case "SetMode": {
        const m = (request as { payload?: { mode?: AudioMode } }).payload?.mode;
        if (m) mockMode = m;
        return { type: "Ok", payload: null };
      }
      case "SetSmartButton": {
        const a = (request as { payload?: { action?: string } }).payload?.action;
        if (a) mockStatus.smart_button_action = a;
        return { type: "Ok", payload: null };
      }
      case "GetEq":
        return {
          type: "Eq",
          payload: {
            eq: { enabled: false, bands: defaultBands() },
            sidetone: { enabled: false, level: 0 },
            noise_gate: { enabled: false, threshold_db: -30 },
            voice_enhancer: { mode: "off", custom_bands: null },
            mic_gain: 80,
          },
        };
      case "GetProfiles":
        return {
          type: "Profiles",
          payload: [
            { name: "Flat", audio: {}, created_at: "2026-09-09" },
            { name: "Music", audio: {}, created_at: "2026-09-09" },
            { name: "Movie", audio: {}, created_at: "2026-09-09" },
            { name: "eSport", audio: {}, created_at: "2026-09-09" },
          ],
        };
      case "GetDevice":
        return {
          type: "Device",
          payload: {
            usb_bus: 1,
            usb_addr: 5,
            alsa_card: 2,
            pipewire_sink: "alsa_output.usb-Sennheiser_EPOS_GSX_300-00.analog-stereo",
            pipewire_source: "alsa_input.usb-Sennheiser_EPOS_GSX_300-00.analog-stereo",
            hidraw: "/dev/hidraw3",
            input_event: "/dev/input/event7",
            firmware_version: null,
          },
        };
      default:
        return { type: "Ok", payload: null };
    }
  }

  // ─── Actions ──────────────────────────────────────────────

  async function fetchStatus() {
    const res = await sendRequest({ type: "GetStatus" });
    if (res?.type === "Status") {
      status.value = res.payload;
      connected.value = true;
      if (res.payload.mode) mode.value = res.payload.mode as AudioMode;
    }
  }

  async function fetchAudio() {
    const res = await sendRequest({ type: "GetEq" });
    if (res?.type === "Eq") audio.value = res.payload;
  }

  async function fetchProfiles() {
    const res = await sendRequest({ type: "GetProfiles" });
    if (res?.type === "Profiles") profiles.value = res.payload;
  }

  async function fetchDevice() {
    const res = await sendRequest({ type: "GetDevice" });
    if (res?.type === "Device") device.value = res.payload as DeviceInfo | null;
  }

  async function fetchMode() {
    const res = await sendRequest({ type: "GetMode" });
    if (res?.type === "Mode") mode.value = res.payload as AudioMode;
  }

  async function setMode(newMode: AudioMode) {
    const res = await sendRequest({ type: "SetMode", payload: { mode: newMode } });
    if (res?.type === "Ok") {
      mode.value = newMode;
      if (status.value) status.value.mode = newMode;
    }
  }

  async function toggleMode() {
    const res = await sendRequest({ type: "ToggleMode" });
    if (res?.type === "Mode") {
      mode.value = res.payload as AudioMode;
      if (status.value) status.value.mode = res.payload as AudioMode;
    }
  }

  async function setSmartButton(action: string) {
    const res = await sendRequest({ type: "SetSmartButton", payload: { action } });
    if (res?.type === "Ok" && status.value) status.value.smart_button_action = action;
  }

  async function setEqBands(bands: EqBand[]) {
    if (!audio.value) return;
    audio.value.eq.bands = bands;
    audio.value.eq.enabled = true;
    await sendRequest({ type: "SetEq", payload: { eq: audio.value } });
  }

  async function setSidetone(enabled: boolean, level: number) {
    if (!audio.value) return;
    audio.value.sidetone = { enabled, level };
    await sendRequest({ type: "SetSidetone", payload: { enabled, level } });
  }

  async function setNoiseGate(enabled: boolean, thresholdDb: number) {
    if (!audio.value) return;
    audio.value.noise_gate = { enabled, threshold_db: thresholdDb };
    await sendRequest({ type: "SetNoiseGate", payload: { enabled, threshold_db: thresholdDb } });
  }

  async function setVoiceEnhancer(modeName: string, customBands?: EqBand[]) {
    if (!audio.value) return;
    audio.value.voice_enhancer = { mode: modeName, custom_bands: customBands || null };
    await sendRequest({
      type: "SetVoiceEnhancer",
      payload: { mode: modeName, custom_bands: customBands || null },
    });
  }

  async function setMicGain(gain: number) {
    if (!audio.value) return;
    audio.value.mic_gain = gain;
    await sendRequest({ type: "SetMicGain", payload: { gain } });
  }

  async function setActiveProfile(name: string) {
    const res = await sendRequest({ type: "SetActiveProfile", payload: { name } });
    if (res?.type === "Ok" && status.value) {
      status.value.active_profile = name;
      await fetchAudio();
    }
  }

  async function createProfile(name: string) {
    if (!audio.value) return;
    const res = await sendRequest({ type: "CreateProfile", payload: { name, audio: audio.value } });
    if (res?.type === "Ok") {
      await fetchProfiles();
      await fetchStatus();
    }
    return res?.type === "Ok";
  }

  async function deleteProfile(name: string) {
    const res = await sendRequest({ type: "DeleteProfile", payload: { name } });
    if (res?.type === "Ok") {
      await fetchProfiles();
      await fetchStatus();
    }
    return res?.type === "Ok";
  }

  /**
   * Auto-poll daemon status every 3s. Call once from App.vue setup.
   */
  let pollTimer: ReturnType<typeof setInterval> | null = null;

  function startPolling() {
    if (pollTimer) return;
    fetchStatus();
    fetchAudio();
    fetchProfiles();
    fetchDevice();
    pollTimer = setInterval(() => {
      fetchStatus();
      fetchAudio();
      fetchMode();
      fetchProfiles();
      fetchDevice();
    }, 3000);
  }

  function stopPolling() {
    if (pollTimer) {
      clearInterval(pollTimer);
      pollTimer = null;
    }
  }

  return {
    connected,
    status,
    audio,
    profiles,
    mode,
    device,
    fetchStatus,
    fetchAudio,
    fetchProfiles,
    fetchDevice,
    fetchMode,
    setMode,
    toggleMode,
    setEqBands,
    setSidetone,
    setNoiseGate,
    setVoiceEnhancer,
    setMicGain,
    setActiveProfile,
    createProfile,
    deleteProfile,
    setSmartButton,
    startPolling,
    stopPolling,
  };
});