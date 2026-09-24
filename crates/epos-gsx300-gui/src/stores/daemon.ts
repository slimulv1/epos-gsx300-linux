import { defineStore } from "pinia";
import { ref } from "vue";
import { notifySmartButton } from "../lib/notify";

// Tauri invoke — loaded dynamically so browser dev mode can fall back
let tauriInvoke: ((cmd: string, args?: Record<string, unknown>) => Promise<any>) | null = null;
try {
  // Use Tauri invoke only inside the real Tauri runtime. The module also
  // resolves under plain `vite dev` (node_modules), so gate on the runtime
  // marker — otherwise browser dev mode would take the invoke() path,
  // fail against the missing backend, and never reach the HTTP bridge/mock.
  if (typeof window !== "undefined" && "__TAURI_INTERNALS__" in window) {
    const mod = await import("@tauri-apps/api/core");
    tauriInvoke = mod.invoke;
  }
} catch {
  // Running outside Tauri (e.g. plain `vite dev`)
}

// ─── Types (mirror Rust IPC structs) ────────────────────────

export interface EqBand {
  freq: number;
  gain_db: number;
  q: number;
}

/**
 * Wire names for `epos_shared::config::VoiceMode` (`#[serde(rename_all =
 * "lowercase")]`). Keeping this a union instead of `string` means a typo in a
 * mode name is a compile error rather than a request the daemon silently
 * rejects at runtime.
 */
export type VoiceModeName = "off" | "warm" | "clear" | "custom";

export const VOICE_MODE_NAMES: readonly VoiceModeName[] = [
  "off",
  "warm",
  "clear",
  "custom",
];

/**
 * Canonical neutral-profile name, mirroring
 * `epos_shared::config::FLAT_PROFILE_NAME`. The daemon writes "FLAT" while
 * older configs and user-created profiles used "Flat", so this is the single
 * spelling the GUI should generate. Comparisons against existing profile
 * names stay case-insensitive.
 */
export const FLAT_PROFILE_NAME = "FLAT";

export interface AudioConfig {
  eq: {
    enabled: boolean;
    bands: EqBand[];
  };
  sidetone: { enabled: boolean; level: number };
  noise_gate: { enabled: boolean; threshold_db: number };
  voice_enhancer: { mode: VoiceModeName; custom_bands: EqBand[] | null };
  mic_gain: number;
}

export type AudioMode = "stereo" | "surround71";

export interface Profile {
  name: string;
  /** Serialised by Rust (`Profile::mode`), previously missing here. */
  mode: AudioMode;
  audio: AudioConfig;
  created_at: string;
}

export interface DeviceInfo {
  usb_bus: number;
  usb_addr: number;
  /** null while ALSA has not enumerated the device yet. Card 0 is a real index. */
  alsa_card: number | null;
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
  /**
   * How many bands are actually in the EQ filter graph right now. Optional
   * because it was added after the first Status shape: a GUI talking to an
   * older daemon does not have it, and must not conclude the EQ is
   * transparent when it is only missing the number.
   */
  eq_active_bands?: number;
  active_profile: string;
  mode: AudioMode;
  smart_button_action?: string;
  volume?: number;
  sidetone_enabled: boolean;
  noise_gate_enabled: boolean;
  voice_enhancer_enabled: boolean;
  smart_button_seq: number;
}

// ─── Response shape from daemon ─────────────────────────────
/**
 * Discriminated union mirroring `epos_shared::ipc::Response`
 * (`#[serde(tag = "type", content = "payload")]`).
 *
 * This used to be `{ type: string; payload: any }`, which meant the compiler
 * could not catch a Rust/TypeScript field drift (e.g. the `Profile` shape
 * losing `mode`) and silently accepted any response body. Callers narrow on
 * `res.type` and the payload type follows.
 */
type DaemonResponse =
  | { type: "Ok" }
  | { type: "Error"; payload: { message: string } }
  | { type: "Status"; payload: DeviceStatus }
  | { type: "Device"; payload: DeviceInfo | null }
  | { type: "Eq"; payload: AudioConfig }
  | { type: "Mode"; payload: AudioMode }
  | { type: "Profiles"; payload: Profile[] };

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
  const lastSmartSeq = ref<number>(0);
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
  let mockSmartSeq = 0;
  const mockStatus = {
    daemon_version: "0.1.0",
    device_connected: true,
    eq_active: false,
    eq_active_bands: 0,
    active_profile: FLAT_PROFILE_NAME,
    mode: mockMode as AudioMode,
    smart_button_action: "toggle_mode",
    sidetone_enabled: false,
    noise_gate_enabled: false,
    voice_enhancer_enabled: false,
  };

  function mockAudioConfig(): AudioConfig {
    return {
      eq: { enabled: false, bands: defaultBands() },
      sidetone: { enabled: false, level: 0 },
      noise_gate: { enabled: false, threshold_db: -30 },
      voice_enhancer: { mode: "off", custom_bands: null },
      mic_gain: 80,
    };
  }

  function mockResponse(request: Record<string, unknown>): DaemonResponse {
    switch (request.type) {
      case "GetStatus":
        return { type: "Status", payload: { ...mockStatus, mode: mockMode, smart_button_seq: mockSmartSeq } };
      case "GetMode":
        return { type: "Mode", payload: mockMode };
      case "ToggleMode":
        mockMode = mockMode === "stereo" ? "surround71" : "stereo";
        mockSmartSeq += 1;
        return { type: "Mode", payload: mockMode };
      case "SetMode": {
        const m = (request as { payload?: { mode?: AudioMode } }).payload?.mode;
        if (m) mockMode = m;
        return { type: "Ok" };
      }
      case "SetSmartButton": {
        const a = (request as { payload?: { action?: string } }).payload?.action;
        if (a) mockStatus.smart_button_action = a;
        return { type: "Ok" };
      }
      case "GetEq":
        return { type: "Eq", payload: mockAudioConfig() };
      case "GetProfiles":
        return {
          type: "Profiles",
          payload: [
            { name: FLAT_PROFILE_NAME, mode: "stereo", audio: mockAudioConfig(), created_at: "2026-09-09" },
            { name: "Music", mode: "surround71", audio: mockAudioConfig(), created_at: "2026-09-09" },
            { name: "Movie", mode: "surround71", audio: mockAudioConfig(), created_at: "2026-09-09" },
            { name: "eSport", mode: "stereo", audio: mockAudioConfig(), created_at: "2026-09-09" },
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
        return { type: "Ok" };
    }
  }

  // ─── Actions ──────────────────────────────────────────────

  async function fetchStatus() {
    const res = await sendRequest({ type: "GetStatus" });
    if (res?.type === "Status") {
      status.value = res.payload;
      connected.value = true;
      if (res.payload.mode) mode.value = res.payload.mode as AudioMode;
      const seq = (res.payload as { smart_button_seq?: number }).smart_button_seq;
      if (typeof seq === "number" && seq !== lastSmartSeq.value) {
        lastSmartSeq.value = seq;
        const profile = (res.payload as { active_profile?: string }).active_profile;
        void notifySmartButton(profile ?? "Flat", res.payload.mode as AudioMode | undefined);
      }
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

  /**
   * Select a voice-enhancer mode.
   *
   * Returns true only when the daemon actually accepted the change. The
   * previous implementation updated the UI first and then discarded the
   * response, so a rejected or dropped request left the button highlighted
   * for a mode that was never applied. Now the optimistic value is rolled
   * back on any failure (transport error or Response::Error).
   */
  async function setVoiceEnhancer(modeName: VoiceModeName, customBands?: EqBand[]) {
    if (!audio.value) return false;
    const previous = audio.value.voice_enhancer;
    const bands = customBands || null;
    audio.value.voice_enhancer = { mode: modeName, custom_bands: bands };
    const res = await sendRequest({
      type: "SetVoiceEnhancer",
      payload: { mode: modeName, custom_bands: bands },
    });
    if (res?.type === "Error") {
      audio.value.voice_enhancer = previous;
      return false;
    }
    if (!res) {
      audio.value.voice_enhancer = previous;
      return false;
    }
    return true;
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