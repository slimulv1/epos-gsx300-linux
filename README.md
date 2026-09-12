# EPOS GSX 300 Linux

Open-source Linux replacement for EPOS Gaming Suite.

EQ, sidetone, noise gate, voice enhancer, and audio control for the
EPOS GSX 300 USB DAC — plus a fully documented reverse-engineering
archive of the device (firmware, HID protocol, hardware internals).

## Features

- 9-band parametric EQ with draggable curve (real-time PipeWire filter-chain)
- Preset system (Flat, Music, Movie, eSport + custom)
- Sidetone (mic monitoring)
- Voice enhancer (Warm / Clear / **Custom** — custom bands built from current EQ)
- Noise gate (rnnoise, real-time neural noise suppression, threshold-tunable)
- Mic gain control
- **LED ring control** — blue = stereo / red = 7.1, synced with mode
- **Configurable smart button** — physical dial click (or long-press) dispatches one of 5 actions: toggle mode (default), toggle EQ, cycle presets, toggle sidetone, toggle noise gate — set via Settings tab
- Dark UI with gaming aesthetic

## Repository layout

```
.
├── crates/
│   ├── epos-gsx300d/       Rust daemon — audio DSP + device control
│   │   └── src/            (audio, hid, led, devices, ipc, mic_meter)
│   ├── epos-gsx300-gui/    Tauri 2.x + Vue 3 desktop app (Pinia + Naive UI)
│   └── epos-shared/        Shared crate — IPC types, device IDs, config schema
├── docs/
│   ├── GUI-REDESIGN-PLAN.md          GUI roadmap
│   └── reverse-engineering/          ↓ device RE archive (see below)
├── config/
│   └── default.json                   Default daemon config
├── packaging/
│   ├── archlinux/                    PKGBUILD
│   └── epos-gsx300-gui.desktop       Desktop entry
├── scripts/
│   ├── install.sh                    Install / uninstall helper
│   ├── hid-capture.py                Raw HID report capture (debug)
│   ├── led-probe.py                  LED ring probe (debug)
│   └── test-smart-button.py          Smart-button behavior test
├── systemd/
│   ├── epos-gsx300d.service          Daemon user service
│   └── pipewire-ladspa.conf          PipeWire rnnoise LADSPA config
├── udev/
│   └── 70-epos-gsx300.rules          Device access rule (audio group)
├── Cargo.toml                        Workspace manifest
└── README.md
```

## Install

```bash
# From source
git clone https://github.com/slimulv1/epos-gsx300-linux.git
cd epos-gsx300-linux
cargo build --release

# One-time udev rule (needs root, REQUIRED for device access):
sudo ./scripts/install.sh --udev

# Install daemon as a systemd user service:
./scripts/install.sh

# Install GUI (Tauri desktop app + desktop entry):
./scripts/install.sh --gui
```

<details>
<summary><code>scripts/install.sh</code> options</summary>

| Option        | What it does                                              |
| ------------- | --------------------------------------------------------- |
| (none)        | Install daemon to `~/.local/bin`, enable systemd user service |
| `--gui`       | Build + install Tauri desktop app + desktop entry + icon  |
| `--udev`      | Install udev rule (sudo) — grants `audio` group access    |
| `--system`    | Install to `/usr/local/bin` + systemd user service        |
| `--uninstall` | Remove daemon, GUI, desktop entry, config                 |

</details>

> The GSX 300 is a USB audio class device: no kernel module is needed.
> The udev rule gives members of the `audio` group read/write on the
> device's hidraw interface, which the daemon uses to control the LED
> ring and read the smart button / volume dial.

## Usage

```bash
# Start daemon (systemd user service)
systemctl --user enable --now epos-gsx300d

# Launch GUI (desktop app)
epos-gsx300-gui
```

The GUI is a responsive native desktop app (Tauri 2.x) that scales from
640×400 to fullscreen — works well with dwm tiling. Bottom-nav has 4 tabs:
**Playback** (EQ + presets + surround + sidetone + sound test),
**Microphone** (voice enhancer + gain + noise gate),
**Device** (USB info + smart button config),
**Settings** (autostart + status + about).

## Architecture

```
epos-gsx300d              Rust daemon — audio DSP + device control
  epos-devices            USB/udev detection
  epos-audio              PipeWire audio processing
  epos-hid                HID event handling
  epos-config             Config + preset management
  epos-shared             IPC types, device IDs

epos-gsx300-gui           Tauri 2.x + Vue 3 — native desktop app (Pinia + Naive UI)
                          Connects to daemon via Unix socket IPC
                          Responsive: scales from 640×400 to fullscreen (dwm-friendly)
```

## Hardware compatibility notes (CX21988)

The GSX 300 is built on the **Conexant/Synaptics CX21988** (AudioSmart)
codec. Key facts that shape the daemon design:

- **Playback**: USB altsets advertise `S16_LE`@48k and `S24_3LE`@48k/**96k**.
  The PipeWire drop-in (`50-epos-gsx300-gaming.conf`) pins the sink to
  `S24_3LE`@48k — 96k is skipped deliberately: the noise gate (rnnoise)
  is fixed at 48k, 44.1kHz is *not* natively supported by the chip, and
  48k keeps A2+/EPOS sharing one clock without resampling.
- **Capture**: hardware is **mono** `S16_LE`@48k only (one altset). The
  `mono-fallback` PipeWire source is the real hardware path — the noise
  gate therefore runs in mono (`noise_suppressor_mono`).
- **EQ 9-band is host-side** (PipeWire filter-chain), because the chip's
  built-in 5-band hardware EQ is not exposed over USB by the firmware.
- **Sidetone** (REVERBERATION in the GUI) is the one device-resident
  DSP feature and is controlled via vendor HID commands.
- **HID protocol** (fully decoded, see `src/led.rs` header): Report 0x01
  = volume dial (incremental detents only, no absolute readback), 0x02 =
  LED / button state (2-bit output, 3-bit input), 0x04/0x05/0x06/0x07/
  0x1A = undisclosed vendor commands. Because the dial exposes only
  relative detents, the daemon tracks the volume level host-side and
  reports it via `GetStatus` for the GUI.

## Reverse engineering

This repository contains the full reverse-engineering archive of the
EPOS GSX 300 (2026-09): firmware dump + disassembly (W65C02S CPU),
HID/memory-bus protocol decode, register map, persistence tests.

**Start here:**

- [`docs/reverse-engineering/README.md`](docs/reverse-engineering/README.md) — index + safety rules
- [`docs/reverse-engineering/HARDWARE-BOOK.md`](docs/reverse-engineering/HARDWARE-BOOK.md) — community-facing device book
- [`docs/reverse-engineering/QUICK-REFERENCE.md`](docs/reverse-engineering/QUICK-REFERENCE.md) — at-a-glance protocols & registers

Raw firmware dumps are kept local (copyrighted Sennheiser/EPOS firmware).

## DSP / Noise Gate setup

EQ and Voice Enhancer use PipeWire's built-in `filter-chain` module —
nothing extra to install.

Noise gate needs the rnnoise LADSPA plugin (neural noise suppression):

```bash
# 1. Grab the prebuilt LADSPA wrapper (no root needed):
#    werman/noise-suppression-for-voice → releases → linux-rnnoise.zip
mkdir -p ~/.local/lib/ladspa
unzip linux-rnnoise.zip -d /tmp/rnnoise
cp /tmp/rnnoise/linux-rnnoise/ladspa/librnnoise_ladspa.so ~/.local/lib/ladspa/

# 2. Tell PipeWire where to find it (user-level, no root):
mkdir -p ~/.config/systemd/user/pipewire.service.d
cp systemd/pipewire-ladspa.conf ~/.config/systemd/user/pipewire.service.d/ladspa.conf
# (edit the path inside if your username differs)

# 3. Reload
systemctl --user daemon-reload
systemctl --user restart pipewire
```

Verify: `pw-cli ls Node | grep epos-` should show `epos-noisegate-*`
alongside `epos-eq-*` and `epos-voice-*` filter nodes.

## Tech Stack

- **Daemon**: Rust + Tokio + signal-hook
- **GUI**: Tauri 2.x + Vue 3 + Pinia + Naive UI + Lucide icons
- **Audio**: PipeWire filter-chain (EQ / voice enhancer / noise gate)
- **USB**: udev rules (no root needed) + rusb + hidapi
- **IPC**: Unix socket (daemon ↔ GUI) + HTTP bridge (127.0.0.1:9898)
- **Config**: JSON at `~/.config/epos-gsx300/config.json` (auto-reload on save)

## License

MIT