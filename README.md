# EPOS GSX 300 Linux

Open-source Linux replacement for EPOS Gaming Suite.

EQ, sidetone, noise gate, voice enhancer, and audio control for the EPOS GSX 300 USB DAC.

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

## Install

```bash
# From source
git clone --recurse-submodules https://github.com/slimulv1/epos-gsx300-linux.git
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
