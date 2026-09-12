# EPOS GSX 300 Linux Daemon — Blueprint

> Design document for user review. Modeled after `sgtaziz/lian-li-linux` architecture.
> Generated: 2026-09-09

---

## 1. Reference Architecture: lian-li-linux

```
lianli-daemon          Rust daemon — fan control loop + LCD streaming
  lianli-devices       HID/USB device drivers
  lianli-transport     USB bulk transport (wireless protocol, display streaming)
  lianli-media         Image/video/GIF encoding, sensor gauge rendering
  lianli-shared        IPC types, config schema, device IDs

lianli-gui             Tauri desktop app (Rust + Vue) — connects via Unix socket
```

**Key patterns we borrow:**
- Daemon ↔ GUI via **Unix socket IPC**
- JSON config at `~/.config/<app>/config.json`
- udev rules for USB access (no root)
- systemd user service
- Tauri (Rust + Vue) for GUI

---

## 2. Proposed Architecture: `epos-gsx300d`

```
epos-gsx300d              Rust daemon — audio DSP pipeline + device control
  epos-devices            HID/USB device detection + enumeration
  epos-audio              PipeWire audio processing (EQ, sidetone, noise gate, voice enhancer)
  epos-hid                HID event handling (volume knob, smart button, LED)
  epos-config             Config schema, load/save, preset management
  epos-shared             IPC types, device IDs, shared constants

epos-gsx300-gui           Tauri desktop app (Rust + Vue/React) — config GUI
                          Connects to daemon via Unix socket
```

---

## 3. Component Details

### 3.1 `epos-gsx300d` (Daemon)

**Language**: Rust (stable)  
**Dependencies**: `tokio`, `serde`, `serde_json`, `rusb`, `libusb1-sys`, `pw` (PipeWire bindings), `notify` (file watcher)

#### `epos-devices`
```rust
// Device detection via udev
// Matches VID 0x1395, PID 0x0098
// Monitors connect/disconnect events
// Enumerates ALSA card number + PipeWire node names

pub struct DeviceInfo {
    pub usb_bus: u8,
    pub usb_addr: u8,
    pub alsa_card: u8,           // Card 4 in our case
    pub pipewire_sink: String,   // alsa_output.usb-Sennheiser_EPOS_GSX_300_*
    pub pipewire_source: String, // alsa_input.usb-Sennheiser_EPOS_GSX_300_*
    pub hidraw: PathBuf,         // /dev/hidraw3
    pub input_event: PathBuf,    // /dev/input/event4
}
```

#### `epos-audio`
PipeWire integration for audio DSP. No direct audio processing — delegates to PipeWire modules.

```rust
// EQ: 9-band parametric via PipeWire module-eq
pub struct EqConfig {
    pub bands: [EqBand; 9],  // 64, 125, 250, 500, 1k, 2k, 4k, 8k, 16k
}

pub struct EqBand {
    pub freq: u32,
    pub gain_db: f32,      // ±12 dB
    pub q: f32,            // Quality factor (default 1.0)
}

// Sidetone: PipeWire loopback with volume control
pub struct SidetoneConfig {
    pub enabled: bool,
    pub level: f32,        // 0.0 - 1.0
}

// Noise gate: PipeWire rnnoise or echo-cancel
pub struct NoiseGateConfig {
    pub enabled: bool,
    pub threshold_db: f32, // -60 to 0 dB
}

// Voice enhancer: PipeWire EQ on capture stream
pub struct VoiceEnhancerConfig {
    pub mode: VoiceMode,   // Off, Warm, Clear, Custom
    pub custom_bands: Option<Vec<EqBand>>,
}
```

#### `epos-hid`
HID event processing for volume knob and smart button.

```rust
// Volume knob: Consumer Control HID events
// Already works via Linux input subsystem
// Optional: sync with PipeWire volume

// Smart button: detect press, map to configured action
pub enum SmartButtonAction {
    ToggleEq,
    CyclePreset,
    ToggleSidetone,
    ToggleNoiseGate,
    // Future: Toggle71 (requires vendor protocol RE)
}
```

#### `epos-config`
```rust
pub struct Config {
    pub device: DeviceConfig,
    pub audio: AudioConfig,
    pub profiles: Vec<Profile>,
    pub active_profile: String,
    pub smart_button: SmartButtonConfig,
}

pub struct AudioConfig {
    pub eq: EqConfig,
    pub sidetone: SidetoneConfig,
    pub noise_gate: NoiseGateConfig,
    pub voice_enhancer: VoiceEnhancerConfig,
}

pub struct Profile {
    pub name: String,        // "Gaming", "Music", "Movie", "Custom"
    pub audio: AudioConfig,
    pub created_at: String,
}
```

#### IPC Protocol (Unix Socket)
```json
// GUI → Daemon: request
{
  "type": "set_eq",
  "payload": {
    "bands": [
      {"freq": 64, "gain_db": 2.5},
      {"freq": 125, "gain_db": 1.0},
      ...
    ]
  }
}

// Daemon → GUI: response
{
  "type": "ok",
  "payload": {
    "eq_active": true,
    "current_profile": "Music"
  }
}

// Daemon → GUI: event (push)
{
  "type": "device_connected",
  "payload": {
    "alsa_card": 4,
    "name": "EPOS GSX 300"
  }
}
```

### 3.2 `epos-gsx300-gui` (GUI)

**Framework**: Tauri 2.x (Rust + Vue 3) or React  
**Design**: Dark theme, gaming aesthetic, responsive

#### UI Layout (3 tabs, matching EPOS Gaming Suite)

```
┌─────────────────────────────────────────────────────────┐
│  EPOS GSX 300 Linux                          ─ □ ×     │
├──────┬──────────────────────────────────────────────────┤
│      │                                                  │
│  🔊  │  ┌──────────────────────────────────────────┐   │
│  Tab │  │          9-BAND EQUALIZER                 │   │
│      │  │                                          │   │
│  🎤  │  │   dB                                     │   │
│  Tab │  │   +12 ┤    ╭──╮                          │   │
│      │  │    0 ┤───╯    ╰────────╮                 │   │
│  ⚙   │  │  -12 ┤                  ╰──             │   │
│  Tab │  │      └──┬──┬──┬──┬──┬──┬──┬──┬──        │   │
│      │  │        64 125 250 500 1k 2k 4k 8k 16k   │   │
│      │  │         Hz                               │   │
│      │  └──────────────────────────────────────────┘   │
│      │                                                  │
│      │  Preset: [Flat] [Music] [Movie] [eSport] [+]   │
│      │                                                  │
│      │  ┌─────────┐  ┌─────────┐  ┌────────────────┐  │
│      │  │Sidetone │  │Voice    │  │  Smart Button  │  │
│      │  │  30%    │  │  Warm   │  │  → Cycle Preset│  │
│      │  │ ────○── │  │  ○ Clear│  │                │  │
│      │  └─────────┘  └─────────┘  └────────────────┘  │
│      │                                                  │
│      │  Status: ● Connected (Card 4) | Firmware: v0.62│
└──────┴──────────────────────────────────────────────────┘
```

#### Tab 1: Playback (🔊)
- **9-Band EQ**: Draggable curve (like EPOS Gaming Suite)
- **Preset selector**: Factory + custom presets
- **Sidetone slider**: 0-100%
- **Voice enhancer**: Off / Warm / Clear / Custom
- **Smart button assignment**: Dropdown

#### Tab 2: Microphone (🎤)
- **Mic gain slider**: 0-100%
- **Noise gate threshold**: Slider
- **Mic mute toggle**: Button
- **Mic test**: "Listen" button (loopback)
- **Voice enhancer**: Same as Tab 1 (shared setting)

#### Tab 3: Settings (⚙)
- **Device info**: USB bus, ALSA card, firmware version
- **Profile management**: Create / rename / delete / export
- **Auto-start**: Toggle systemd service
- **LED ring**: Status display (blue/red, read-only for now)
- **About**: Version, links, credits

---

## 4. Tech Stack Decision

| Option | Bundle Size | USB/HID | GUI Quality | Dev Speed | Recommendation |
|--------|-------------|---------|-------------|-----------|----------------|
| **Tauri (Rust + Vue)** | ~5-10 MB | `rusb`, `hidapi` | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ✅ **Best choice** |
| Electron (Node.js) | ~150 MB | `usb`, `node-hid` | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ❌ Too heavy |
| GTK4 + Python | ~20 MB | `hidapi`, `pyusb` | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⚠️ Good but less polished |
| Wails (Go + Vue) | ~10 MB | `go-hid`, `go-usb` | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⚠️ Go HID ecosystem smaller |

### **Recommendation: Tauri 2.x + Vue 3**

**Why:**
- Same stack as lian-li-linux (proven pattern)
- Rust daemon = same language as GUI backend
- Tiny bundle (~5-10 MB vs Electron's ~150 MB)
- Native Linux look with web-based UI flexibility
- `rusb` + `hidapi` Rust crates are mature
- PipeWire Rust bindings (`pipewire-rs`) available
- System tray support built into Tauri

---

## 5. Project Structure

```
epos-gsx300-linux/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── epos-gsx300d/             # Daemon
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs           # Entry point, signal handling
│   │       ├── devices.rs        # USB/udev detection
│   │       ├── audio.rs          # PipeWire integration
│   │       ├── hid.rs            # HID event handling
│   │       ├── config.rs         # Config load/save
│   │       └── ipc.rs            # Unix socket server
│   ├── epos-shared/              # Shared types
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── ipc.rs            # IPC message types
│   │       ├── config.rs         # Config schema
│   │       └── device.rs         # Device IDs
│   └── epos-gsx300-gui/          # Tauri GUI
│       ├── Cargo.toml
│       ├── tauri.conf.json
│       └── src/
│           ├── main.rs           # Tauri entry point
│           └── frontend/         # Vue 3 app
│               ├── package.json
│               ├── src/
│               │   ├── App.vue
│               │   ├── components/
│               │   │   ├── EqCurve.vue      # Draggable EQ curve
│               │   │   ├── EqBand.vue        # Individual band slider
│               │   │   ├── PresetList.vue    # Preset selector
│               │   │   ├── SidetoneSlider.vue
│               │   │   ├── VoiceEnhancer.vue
│               │   │   ├── SmartButton.vue
│               │   │   ├── MicSettings.vue
│               │   │   ├── DeviceInfo.vue
│               │   │   └── ProfileManager.vue
│               │   ├── composables/
│               │   │   └── useDaemon.ts     # IPC client
│               │   └── assets/
│               │       └── styles.css
│               └── public/
├── packaging/
│   ├── archlinux/
│   │   ├── PKGBUILD
│   │   └── epos-gsx300d.service
│   ├── fedora/
│   │   └── epos-gsx300d.spec
│   └── debian/
│       └── debian/
├── udev/
│   └── 70-epos-gsx300.rules
├── config/
│   └── default.json              # Default config template
└── README.md
```

---

## 6. Implementation Phases

### Phase 1: Daemon Core (MVP)
**Time**: 2-3 weeks  
**Goal**: Daemon detects device, applies EQ, runs as service

- [ ] Project scaffolding (Cargo workspace)
- [ ] USB device detection via udev (`epos-devices`)
- [ ] PipeWire EQ integration (`epos-audio`)
- [ ] JSON config load/save (`epos-config`)
- [ ] Unix socket IPC server (`epos-ipc`)
- [ ] systemd user service
- [ ] udev rules for USB access

**Deliverable**: `epos-gsx300d` binary that:
- Detects GSX 300 on USB connect
- Applies EQ settings from `~/.config/epos-gsx300/config.json`
- Runs as systemd service
- Responds to IPC commands

### Phase 2: Audio Features
**Time**: 2-3 weeks  
**Goal**: All audio DSP features working

- [ ] 9-band EQ with parametric filter
- [ ] Preset system (factory + custom)
- [ ] Sidetone loopback
- [ ] Noise gate (rnnoise integration)
- [ ] Voice enhancer (capture EQ)
- [ ] Mic gain control
- [ ] HID event handling (volume knob sync)
- [ ] Smart button action mapping

**Deliverable**: Feature-complete daemon

### Phase 3: GUI
**Time**: 3-4 weeks  
**Goal**: Full Tauri GUI matching EPOS Gaming Suite

- [ ] Tauri + Vue 3 scaffolding
- [ ] EQ curve editor (draggable)
- [ ] Preset manager
- [ ] Sidetone / voice enhancer controls
- [ ] Mic settings tab
- [ ] Device info / settings tab
- [ ] System tray integration
- [ ] Dark theme (gaming aesthetic)

**Deliverable**: Complete desktop app

### Phase 4: Packaging & Polish
**Time**: 1-2 weeks  
**Goal**: Installable on major distros

- [ ] AUR PKGBUILD
- [ ] Fedora COPR spec
- [ ] Debian/Ubuntu .deb
- [ ] Flatpak manifest
- [ ] README + documentation
- [ ] Icon + desktop entry

**Deliverable**: Publishable release

---

## 7. Config File Format

```json
{
  "version": 1,
  "device": {
    "auto_detect": true,
    "usb_vid": "1395",
    "usb_pid": "0098"
  },
  "audio": {
    "eq": {
      "enabled": true,
      "bands": [
        {"freq": 64, "gain_db": 0.0, "q": 1.0},
        {"freq": 125, "gain_db": 0.0, "q": 1.0},
        {"freq": 250, "gain_db": 0.0, "q": 1.0},
        {"freq": 500, "gain_db": 0.0, "q": 1.0},
        {"freq": 1000, "gain_db": 0.0, "q": 1.0},
        {"freq": 2000, "gain_db": 0.0, "q": 1.0},
        {"freq": 4000, "gain_db": 0.0, "q": 1.0},
        {"freq": 8000, "gain_db": 0.0, "q": 1.0},
        {"freq": 16000, "gain_db": 0.0, "q": 1.0}
      ]
    },
    "sidetone": {
      "enabled": false,
      "level": 0.0
    },
    "noise_gate": {
      "enabled": false,
      "threshold_db": -30.0
    },
    "voice_enhancer": {
      "mode": "off",
      "custom_bands": null
    },
    "mic_gain": 80
  },
  "profiles": [
    {
      "name": "Flat",
      "eq_bands": [
        {"freq": 64, "gain_db": 0.0},
        {"freq": 125, "gain_db": 0.0},
        {"freq": 250, "gain_db": 0.0},
        {"freq": 500, "gain_db": 0.0},
        {"freq": 1000, "gain_db": 0.0},
        {"freq": 2000, "gain_db": 0.0},
        {"freq": 4000, "gain_db": 0.0},
        {"freq": 8000, "gain_db": 0.0},
        {"freq": 16000, "gain_db": 0.0}
      ]
    },
    {
      "name": "Music",
      "eq_bands": [
        {"freq": 64, "gain_db": 4.0},
        {"freq": 125, "gain_db": 2.0},
        {"freq": 250, "gain_db": -1.0},
        {"freq": 500, "gain_db": -2.0},
        {"freq": 1000, "gain_db": 0.0},
        {"freq": 2000, "gain_db": 2.0},
        {"freq": 4000, "gain_db": 3.0},
        {"freq": 8000, "gain_db": 2.0},
        {"freq": 16000, "gain_db": 1.0}
      ]
    }
  ],
  "active_profile": "Flat",
  "smart_button": {
    "action": "toggle_mode"
  }
}
```

---

## 8. udev Rules

```udev
# /etc/udev/rules.d/70-epos-gsx300.rules
# EPOS GSX 300 — grant access to audio group
SUBSYSTEM=="usb", ATTR{idVendor}=="1395", ATTR{idProduct}=="0098", MODE="0664", GROUP="audio"
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1395", ATTRS{idProduct}=="0098", MODE="0664", GROUP="audio"
SUBSYSTEM=="input", ATTRS{idVendor}=="1395", ATTRS{idProduct}=="0098", MODE="0664", GROUP="audio"
```

---

## 9. systemd Service

```ini
# ~/.config/systemd/user/epos-gsx300d.service
[Unit]
Description=EPOS GSX 300 Audio Daemon
After=pipewire.service wireplumber.service
Requires=wireplumber.service

[Service]
Type=simple
ExecStart=/usr/bin/epos-gsx300d
Restart=on-failure
RestartSec=5

[Install]
WantedBy=default.target
```

---

## 10. Future: USB Protocol RE (Phase 5+)

If vendor protocol is reverse-engineered:

| Feature           | Impact                                                  |
| ----------------- | ------------------------------------------------------- |
| LED Ring Control  | Daemon can set blue/red color via USB vendor command    |
| 7.1 HW Toggle     | Daemon can toggle device's internal 7.1 mode            |
| Firmware Update   | Daemon can flash firmware updates                       |
| Device Profiles   | Store EQ/surround in device memory (persist without PC) |

**How to RE**: USBPcap capture in Windows VM → decode → implement in `epos-devices`.

---

## 11. Comparison: EPOS Gaming Suite vs Our Daemon

| Feature           | EPOS Gaming Suite (Windows)          | epos-gsx300d (Linux)                    |
| ----------------- | ------------------------------------ | --------------------------------------- |
| **EQ**                | 9-band, visual curve, 4+ presets     | ✅ 9-band, visual curve, unlimited presets |
| **7.1 Surround**      | Software HRTF, toggle button         | ⚠️ PipeWire HRTF (no HW toggle)         |
| **Sidetone**          | 0-100% slider                        | ✅ 0-100% via pw-loopback               |
| **Voice Enhancer**    | Warm/Clear/Off                       | ✅ Approximate via capture EQ            |
| **Noise Gate**        | Threshold slider                     | ✅ rnnoise plugin                        |
| **Mic Gain**          | 0-100%                               | ✅ ALSA mixer control                    |
| **Smart Button**      | 2 options                            | ✅ Configurable actions                  |
| **LED Ring**          | Blue/Red auto                        | ❌ Requires vendor protocol RE           |
| **Firmware Update**   | Built-in                             | ❌ Requires vendor protocol RE           |
| **Bit Depth**         | Capped at 16-bit                     | ✅ Full 24-bit (no software cap)        |
| **Open Source**       | No                                   | ✅ GPL-2.0                               |
| **Linux Support**     | None                                 | ✅ Native                                |
| **Bundle Size**       | ~18.7 MB (MSI)                       | ~5-10 MB (Tauri)                        |

---

## 12. Open Questions for User

1. **GUI Framework**: Tauri (Rust + Vue) — agree? Or prefer GTK4/Python?
2. **Feature Priority**: Start with EQ only, or full daemon?
3. **Distribution**: AUR first? Flatpak? Both?
4. **Naming**: `epos-gsx300-linux`? `gsx300d`? Something else?
5. **License**: GPL-2.0 (like lian-li-linux)? Or MIT?
6. **7.1 Surround**: Implement software HRTF, or skip until vendor RE?
7. **USB RE**: Want to do the USBPcap capture in Windows VM now?

---

*Blueprint v1.0 — Ready for review.*
