# EPOS GSX 300 Linux

Open-source Linux replacement for EPOS Gaming Suite.

EQ, sidetone, noise gate, voice enhancer, and audio control for the EPOS GSX 300 USB DAC.

## Features

- 9-band parametric EQ with draggable curve (real-time PipeWire filter-chain)
- Preset system (Flat, Music, Movie, eSport + custom)
- Sidetone (mic monitoring)
- Voice enhancer (Warm / Clear)
- Noise gate (rnnoise, real-time neural noise suppression)
- Mic gain control
- **LED ring control** — blue = stereo / red = 7.1, synced with mode
- **Smart button** — physical dial click toggles mode + LED (long-press too)
- Dark UI with gaming aesthetic

## Architecture

```
epos-gsx300d              Rust daemon — audio DSP + device control
  epos-devices            USB/udev detection
  epos-audio              PipeWire audio processing
  epos-hid                HID event handling
  epos-config             Config + preset management
  epos-shared             IPC types, device IDs

epos-gsx300-gui           Tauri 2.x (Rust + Vue 3) — config GUI
                          Connects via Unix socket
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
```

<details>
<summary><code>scripts/install.sh</code> options</summary>

| Option        | What it does                                              |
| ------------- | --------------------------------------------------------- |
| (none)        | Install binary to `~/.local/bin`, enable systemd user service |
| `--udev`      | Install udev rule (sudo) — grants `audio` group access    |
| `--system`    | Install to `/usr/local/bin` + systemd user service        |
| `--uninstall` | Remove binary, service, config                            |

</details>

> The GSX 300 is a USB audio class device: no kernel module is needed.
> The udev rule gives members of the `audio` group read/write on the
> device's hidraw interface, which the daemon uses to control the LED
> ring and read the smart button / volume dial.

## Usage

```bash
# Start daemon
systemctl --user enable --now epos-gsx300d

# Launch GUI
epos-gsx300-gui
```

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

- **Daemon**: Rust + Tokio
- **GUI**: Tauri 2.x + Vue 3
- **Audio**: PipeWire integration
- **USB**: udev rules (no root needed)
- **Config**: JSON at `~/.config/epos-gsx300/config.json`

## License

MIT
