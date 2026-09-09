import { ref } from "vue";

const SOCKET_PATH = "/run/user/1000/epos-gsx300d.sock"; // TODO: connect to daemon
void SOCKET_PATH;

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

export interface DeviceStatus {
  daemon_version: string;
  device_connected: boolean;
  eq_active: boolean;
  active_profile: string;
}

const connected = ref(false);
const status = ref<DeviceStatus | null>(null);
const audio = ref<AudioConfig | null>(null);
const profiles = ref<Profile[]>([]);

async function sendRequest(request: any): Promise<any> {
  // For now, return mock data since daemon may not be running
  // In production, this would use Tauri IPC or Unix socket
  try {
    // Tauri invoke would go here
    // For development, return mock data
    return mockResponse(request);
  } catch (e) {
    console.error("Daemon connection failed:", e);
    return null;
  }
}

function mockResponse(request: any): any {
  switch (request.type) {
    case "GetStatus":
      return {
        type: "Status",
        payload: {
          daemon_version: "0.1.0",
          device_connected: true,
          eq_active: false,
          active_profile: "Flat",
        },
      };
    case "GetEq":
      return {
        type: "Eq",
        payload: {
          eq: {
            enabled: false,
            bands: defaultBands(),
          },
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

export function useDaemon() {
  async function fetchStatus() {
    const res = await sendRequest({ type: "GetStatus" });
    if (res?.type === "Status") {
      status.value = res.payload;
      connected.value = true;
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

  async function setMicGain(gain: number) {
    if (!audio.value) return;
    audio.value.mic_gain = gain;
    await sendRequest({
      type: "SetMicGain",
      payload: { gain },
    });
  }

  async function setActiveProfile(name: string) {
    await sendRequest({
      type: "SetActiveProfile",
      payload: { name },
    });
    status.value!.active_profile = name;
  }

  return {
    connected,
    status,
    audio,
    profiles,
    fetchStatus,
    fetchAudio,
    fetchProfiles,
    setEqBands,
    setSidetone,
    setNoiseGate,
    setMicGain,
    setActiveProfile,
  };
}
