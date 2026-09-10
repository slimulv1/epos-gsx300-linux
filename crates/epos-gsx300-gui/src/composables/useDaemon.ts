import { ref } from "vue";

// Tauri invoke — loaded dynamically so we can fall back to mock in browser dev mode
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
}

// ─── State ──────────────────────────────────────────────────

const connected = ref(false);
const status = ref<DeviceStatus | null>(null);
const audio = ref<AudioConfig | null>(null);
const profiles = ref<Profile[]>([]);
const mode = ref<AudioMode>("stereo");
const device = ref<DeviceInfo | null>(null);

// ─── IPC Layer ──────────────────────────────────────────────

/**
 * Response shape from daemon: { type: "Status", payload: {...} }
 * Matches Rust serde(tag = "type", content = "payload")
 */
interface DaemonResponse {
  type: string;
  payload: any;
}

let useMock = false;

/**
 * Try connecting to the daemon via a small HTTP bridge,
 * or fall back to Tauri invoke, or mock.
 *
 * In production (Tauri app): use invoke("daemon_request", { request })
 * In dev (npm run dev): try HTTP bridge on 127.0.0.1:9898, fall back to mock
 *
 * The HTTP bridge is retried on every call (not sticky) so the GUI picks
 * the daemon up as soon as it becomes reachable.
 */
async function sendRequest(
  request: Record<string, unknown>
): Promise<DaemonResponse | null> {
  if (useMock) return mockResponse(request);

  // Try Tauri invoke first (works in Tauri dev & production)
  if (tauriInvoke) {
    try {
      const jsonRequest = JSON.stringify(request);
      const jsonResponse: string = await tauriInvoke("daemon_request", {
        request: jsonRequest,
      });
      return JSON.parse(jsonResponse) as DaemonResponse;
    } catch {
      // Tauri invoke failed → fall through to HTTP/mock
    }
  }

  // Not in Tauri runtime → try HTTP bridge
  try {
    const resp = await fetch("http://127.0.0.1:9898/ipc", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(request),
    });
    // A 400 from the bridge still carries a parsed Error response
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

// ─── Mock Data (dev mode fallback) ─────────────────────────

// Module-level mock state so ToggleMode flips the value (mirrors daemon behavior)
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
  const type = request.type as string;
  switch (type) {
    case "GetStatus":
      return {
        type: "Status",
        payload: { ...mockStatus, mode: mockMode },
      };
    case "GetMode":
      return { type: "Mode", payload: mockMode };
    case "ToggleMode":
      mockMode = mockMode === "stereo" ? "surround71" : "stereo";
      return { type: "Mode", payload: mockMode };
    case "SetMode": {
      const req = request as { type: string; payload: { mode: AudioMode } };
      if (req.payload?.mode) mockMode = req.payload.mode;
      return { type: "Ok", payload: null };
    }
    case "SetSmartButton": {
      const req = request as { type: string; payload: { action: string } };
      if (mockStatus.smart_button_action) {
        mockStatus.smart_button_action = req.payload?.action || "toggle_mode";
      }
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

function defaultBands(): EqBand[] {
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

// ─── Composable ─────────────────────────────────────────────

export function useDaemon() {
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
    if (res?.type === "Eq") {
      audio.value = res.payload;
    }
  }

  async function fetchProfiles() {
    const res = await sendRequest({ type: "GetProfiles" });
    if (res?.type === "Profiles") {
      profiles.value = res.payload;
    }
  }

  async function fetchDevice() {
    const res = await sendRequest({ type: "GetDevice" });
    if (res?.type === "Device") {
      device.value = res.payload as DeviceInfo | null;
    }
  }

  async function setEqBands(bands: EqBand[]) {
    if (!audio.value) return;
    audio.value.eq.bands = bands;
    audio.value.eq.enabled = true;
    await sendRequest({
      type: "SetEq",
      payload: { eq: audio.value },
    });
  }

  async function setSidetone(enabled: boolean, level: number) {
    if (!audio.value) return;
    audio.value.sidetone = { enabled, level };
    await sendRequest({
      type: "SetSidetone",
      payload: { enabled, level },
    });
  }

  async function setNoiseGate(enabled: boolean, thresholdDb: number) {
    if (!audio.value) return;
    audio.value.noise_gate = { enabled, threshold_db: thresholdDb };
    await sendRequest({
      type: "SetNoiseGate",
      payload: { enabled, threshold_db: thresholdDb },
    });
  }

  async function setVoiceEnhancer(mode: string, customBands?: EqBand[]) {
    if (!audio.value) return;
    audio.value.voice_enhancer = { mode, custom_bands: customBands || null };
    await sendRequest({
      type: "SetVoiceEnhancer",
      payload: { mode, custom_bands: customBands || null },
    });
  }

  async function setMicGain(gain: number) {
    if (!audio.value) return;
    audio.value.mic_gain = gain;
    await sendRequest({
      type: "SetMicGain",
      payload: { gain },
    });
  }

  async function setActiveProfile(name: string) {
    const res = await sendRequest({
      type: "SetActiveProfile",
      payload: { name },
    });
    if (res?.type === "Ok" && status.value) {
      status.value.active_profile = name;
      await fetchAudio();
    }
  }

  async function createProfile(name: string) {
    if (!audio.value) return;
    const res = await sendRequest({
      type: "CreateProfile",
      payload: { name, audio: audio.value },
    });
    if (res?.type === "Ok") {
      await fetchProfiles();
      await fetchStatus();
    }
    return res?.type === "Ok";
  }

  async function deleteProfile(name: string) {
    const res = await sendRequest({
      type: "DeleteProfile",
      payload: { name },
    });
    if (res?.type === "Ok") {
      await fetchProfiles();
      await fetchStatus();
    }
    return res?.type === "Ok";
  }

  async function fetchMode() {
    const res = await sendRequest({ type: "GetMode" });
    if (res?.type === "Mode") {
      mode.value = res.payload as AudioMode;
    }
  }

  async function setMode(newMode: AudioMode) {
    const res = await sendRequest({
      type: "SetMode",
      payload: { mode: newMode },
    });
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
    const res = await sendRequest({
      type: "SetSmartButton",
      payload: { action },
    });
    if (res?.type === "Ok" && status.value) {
      status.value.smart_button_action = action;
    }
  }

  /**
   * Auto-poll daemon status every 3 seconds.
   * Call once from App.vue setup.
   */
  function startPolling() {
    fetchStatus();
    fetchAudio();
    fetchProfiles();
    fetchDevice();
    setInterval(() => {
      fetchStatus();
      fetchAudio();
      fetchMode();
    }, 3000);
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
  };
}
