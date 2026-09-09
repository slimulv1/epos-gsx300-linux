# EPOS GSX 300 Linux

Open-source Linux replacement for EPOS Gaming Suite.

EQ, sidetone, noise gate, voice enhancer, and audio control for the EPOS GSX 300 USB DAC.

## Features

- 9-band parametric EQ with draggable curve
- Preset system (Flat, Music, Movie, eSport + custom)
- Sidetone (mic monitoring)
- Voice enhancer (Warm / Clear)
- Noise gate
- Mic gain control
- Smart button remapping
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
# Arch Linux (AUR)
yay -S epos-gsx300-linux-git

# From source
git clone --recurse-submodules https://github.com/slimulv1/epos-gsx300-linux.git
cd epos-gsx300-linux
cargo build --release
```

## Usage

```bash
# Start daemon
systemctl --user enable --now epos-gsx300d

# Launch GUI
epos-gsx300-gui
```

## Tech Stack

- **Daemon**: Rust + Tokio
- **GUI**: Tauri 2.x + Vue 3
- **Audio**: PipeWire integration
- **USB**: udev rules (no root needed)
- **Config**: JSON at `~/.config/epos-gsx300/config.json`

## License

MIT
