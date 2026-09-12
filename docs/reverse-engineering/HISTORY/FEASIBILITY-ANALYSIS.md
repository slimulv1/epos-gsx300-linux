# EPOS GSX 300 — Feature Feasibility Analysis on Linux

> Which EPOS Gaming Suite features can be replicated on Linux, which cannot, and why.
> Generated: 2026-09-09

---

## Legend

| Symbol | Meaning |
|--------|---------|
| ✅ | **Possible** — can be fully replicated on Linux |
| ⚠️ | **Partial** — can be approximated but with limitations |
| ❌ | **Impossible** — cannot be done without vendor-specific protocol RE |

---

## Feature Matrix

### Tab 1: Playback (Headset)

| # | Feature | Feasibility | Effort | Notes |
|---|---------|-------------|--------|-------|
| 1 | Volume Control | ✅ | Low | Standard HID Consumer Control events. `pipewire`/`alsa` handles volume. Physical dial = analog gain (local). Software volume via `pw-metadata` or `wpctl`. |
| 2 | 9-Band EQ | ✅ | Medium | **SOLVED (2026-09-10)**. Real-time 9-band parametric EQ via PipeWire `filter-chain` (`bq_peaking`), written to `pipewire.conf.d/50-epos-eq.conf` by the daemon. Draggable curve GUI included. Verified: `epos-eq-*` nodes + links to EPOS sink in graph. |
| 3 | EQ Presets (4 factory + custom) | ✅ | Medium | **SOLVED**. Presets stored in config JSON, switched via daemon/IPC/GUI. Flat/Music/Movie/eSport + custom profile support. |
| 4 | 2.0/7.1 Surround Toggle | ✅ | Low | **SOLVED (2026-09-09)**. Mode toggle is host-side software (PipeWire HRTF config swap). LED ring follows mode via vendor HID Report ID 0x02: `0x01`=blue (stereo), `0x02`=red (7.1). Implemented in `epos-gsx300-linux` daemon. |
| 5 | Reverb Slider (7.1 only) | ⚠️ | High | Depends on host-side 7.1 processing chain. Alternative: PipeWire reverb DSP. |
| 6 | Sound Test (A/B comparison) | ✅ | Low | Simple: play reference clip → toggle processing → play again. PipeWire passthrough. |
| 7 | Smart Button (profile toggle) | ✅ | Low | **SOLVED (2026-09-10)**. HID protocol fully RE'd (`/dev/hidraw3`, Report ID 0x02 input: 0x01=stereo, 0x02=7.1, 0x04=long-press). Daemon dispatches 5 configurable actions via `smart_button.action`: `toggle_mode` (default, matching EPOS Gaming Suite), `toggle_eq`, `cycle_preset`, `toggle_sidetone`, `toggle_noise_gate`. GUI dropdown in the Settings tab. Verified: clicking the physical button toggles mode + LED + config persistence. |

### Tab 2: Microphone

| # | Feature | Feasibility | Effort | Notes |
|---|---------|-------------|--------|-------|
| 8 | Mic Gain (0-100%) | ✅ | Low | ALSA mixer control: `numid=4` (Mic Capture Volume, -30 to +5 dB). Map 0-100% → dB range. |
| 9 | Voice Enhancer (Warm/Clear/Off) | ✅ | High | **SOLVED (2026-09-10)**. Filter-chain source on mic capture (`51-epos-voice-enhancer.conf`): Warm = boost 200/350/500Hz, Clear = 2-8kHz. Verified: `epos-voice-capture/output` nodes linked to EPOS mic. |
| 10 | Noise Gate | ✅ | Low | **SOLVED (2026-09-10)**. rnnoise neural suppression via LADSPA filter-chain (`93-epos-noisegate.conf`). Requires `librnnoise_ladspa.so` in `~/.local/lib/ladspa` + `LADSPA_PATH` drop-in. Verified: `epos-noisegate-*` nodes + `pw-record` PASS. |
| 11 | Sidetone (Mic Monitoring) | ⚠️ | Medium | PipeWire: mix capture stream into playback loopback. `module-loopback` or custom `pw-loopback`. Latency may be higher than hardware sidetone. |
| 12 | Mic Mute | ✅ | Low | Standard ALSA/PipeWire mute toggle. `wpctl set-mute @DEFAULT_AUDIO_SOURCE@ toggle`. |
| 13 | Mic Test (Loopback) | ✅ | Low | Route capture → playback. `pw-loopback` or `parec | pacat`. |

### Tab 3: Settings

| # | Feature | Feasibility | Effort | Notes |
|---|---------|-------------|--------|-------|
| 14 | LED Ring Control (Blue/Red) | ✅ | Low | **SOLVED (2026-09-09)**. Vendor HID Report ID 0x02 output byte controls LED ring: `0x00`=off, `0x01`=blue, `0x02`=red, `0x03`=pink. Empirically confirmed on hardware. Implemented in `epos-gsx300-linux` daemon (`led.rs`, `LedController`). |
| 15 | Firmware Update | ❌ | Very High | Proprietary update mechanism via EPOS Gaming Suite. No DFU class detected. Would need full protocol RE + firmware image analysis. |
| 16 | Profile Save/Load | ⚠️ | Medium | Profiles stored host-side (registry/AppData). Can implement as config files on Linux. But cannot persist to device memory without vendor protocol. |
| 17 | Language Selection | ✅ | Low | Not applicable to Linux. UI language is software-only. |
| 18 | Start on Boot | ✅ | Low | systemd user service or autostart in dwm `run.sh`. |

---

## Detailed Analysis

### ✅ FULLY POSSIBLE (7 features)

These features work with standard protocols already supported on Linux:

#### 1. Volume Control
```bash
# Software volume (PipeWire)
wpctl set-volume @DEFAULT_AUDIO_SINK@ 0.5    # 50%
wpctl set-volume @DEFAULT_AUDIO_SINK@ 1.5+   # +50%

# Physical dial = analog gain (local, no sync needed)
# HID events arrive at /dev/input/event4
```
**Status**: Already working. The volume dial controls analog gain locally; PipeWire controls software volume independently.

#### 8. Mic Gain
```bash
# ALSA mixer control for GSX 300 (card 4)
amixer -c 4 set 'Mic Capture Volume' 80%  # 0-100% mapped to -30 to +5 dB
```
**Status**: Already working. `numid=4` provides full gain range.

#### 10. Noise Gate
```bash
# PipeWire with rnnoise
pipewire -c /etc/pipewire/pipewire.conf.d/noise-gate.conf
```
**Status**: `rnnoise` plugin available. Configure threshold via PipeWire config.

#### 12. Mic Mute
```bash
wpctl set-mute @DEFAULT_AUDIO_SOURCE@ toggle
```
**Status**: Already working.

#### 13. Mic Test
```bash
# Loopback mic to headphones
pw-loopback --capture=alsa_input.usb-Sennheiser_EPOS_GSX_300.* --playback=alsa_output.usb-Sennheiser_EPOS_GSX_300.*
```
**Status**: PipeWire `pw-loopback` handles this.

#### 17. Language Selection
Not applicable — Linux audio tools don't have UI language.

#### 18. Start on Boot
```bash
# Add to ~/.config/dwm/run.sh or create systemd user service
systemctl --user enable pipewire wireplumber
```
**Status**: Already working.

---

### ⚠️ PARTIALLY POSSIBLE (8 features)

These features can be approximated but require trade-offs:

#### 2. 9-Band EQ

**EPOS Gaming Suite**: 9 fixed bands (64, 125, 250, 500, 1k, 2k, 4k, 8k, 16k Hz) with visual curve editor.

**Linux alternative**: PipeWire parametric EQ or `easyeffects` (PulseAudio).

```bash
# PipeWire EQ (parametric)
# Create /etc/pipewire/pipewire.conf.d/eq.conf
context.modules = [
  { name = libpipewire-module-eq
    args = {
      # 9-band parametric EQ
      bands = [
        { freq = 64, gain = 0, q = 1.0 }
        { freq = 125, gain = 0, q = 1.0 }
        { freq = 250, gain = 0, q = 1.0 }
        { freq = 500, gain = 0, q = 1.0 }
        { freq = 1000, gain = 0, q = 1.0 }
        { freq = 2000, gain = 0, q = 1.0 }
        { freq = 4000, gain = 0, q = 1.0 }
        { freq = 8000, gain = 0, q = 1.0 }
        { freq = 16000, gain = 0, q = 1.0 }
      ]
    }
  }
]
```

**Limitations**:
- No visual EQ curve editor (would need custom GUI)
- No preset management UI (config files only)
- Gain range may differ from EPOS (±12 dB typical)

**Effort**: Medium — config file + optional CLI tool for preset switching.

#### 3. EQ Presets

**EPOS Gaming Suite**: 4 factory + unlimited custom presets, assignable to Smart Button.

**Linux alternative**: Config files with CLI switching.

```bash
# ~/.config/epos-eq/presets/
├── flat.conf      # All bands = 0 dB
├── music.conf     # V-shaped curve
├── movie.conf     # Bass boost + presence
├── esport.conf    # Cut lows, boost mids/highs
└── custom/
    └── my-preset.conf
```

**Limitation**: No visual editor, no drag-to-adjust curve.

#### 4. 7.1 Surround Toggle

**EPOS Gaming Suite**: Toggle between 2.0 stereo and 7.1 virtual surround. Hardware LED changes (blue ↔ red).

**Linux status**: ❌ Without vendor protocol RE, cannot toggle the device's 7.1 mode.

**Alternative**: PipeWire HRTF spatial audio.

```bash
# PipeWire spatial audio with HRTF
# Uses SBiZ or similar HRTF convolution
# PipeWire 1.0+ has built-in spatial audio support
wpctl inspect @DEFAULT_AUDIO_SINK@  # Check for spatial audio support
```

**Limitations**:
- Different HRTF algorithm than EPOS (subjective quality difference)
- No LED color change (blue/red indicator won't work)
- Requires PipeWire spatial audio or third-party DSP

**Effort**: High — HRTF quality varies, no visual feedback.

#### 5. Reverb Slider

**EPOS Gaming Suite**: Single reverb slider (0-100%), only active in 7.1 mode.

**Linux alternative**: PipeWire reverb DSP or `calf` reverb plugin.

```bash
# Calf reverb via PipeWire
pw-loopback --capture=... --playback=... --plugin=libspa-calf.so
```

**Limitation**: Different reverb algorithm. No integration with 7.1 toggle.

#### 7. Smart Button

**EPOS Gaming Suite**: Button cycles between profiles or toggles 7.1.

**Linux alternative**: Detect button press → toggle custom action.

```bash
# Detect button press via evtest
evtest /dev/input/event4  # Look for KEY_* events on button press

# Map to action
# Option A: Toggle EQ preset
# Option B: Toggle PipeWire spatial audio
# Option C: Toggle mic mute
```

**Limitation**: Cannot toggle 7.1 mode (requires vendor protocol). Button action is limited to software-only toggles.

#### 9. Voice Enhancer

**EPOS Gaming Suite**: Warm (bass boost), Clear (presence boost), Off, Custom.

**Linux alternative**: PipeWire EQ on capture stream.

```bash
# Warm = bass boost on mic
# Clear = presence boost (2-4kHz) on mic
# Apply via PipeWire EQ module on capture node
```

**Limitation**: Approximate only. EPOS's exact EQ curves are proprietary.

#### 11. Sidetone

**EPOS Gaming Suite**: 0-100% mix of mic into headphones.

**Linux alternative**: PipeWire loopback with volume control.

```bash
# Mix capture into playback
pw-loopback \
  --capture=alsa_input.usb-Sennheiser_EPOS_GSX_300.* \
  --playback=alsa_output.usb-Sennheiser_EPOS_GSX_300.* \
  --capture-volume=0.3  # 30% sidetone
```

**Limitation**: Higher latency than hardware sidetone (~10-50ms vs ~1ms). May cause feedback if mic picks up headphone audio.

#### 16. Profile Save/Load

**EPOS Gaming Suite**: Profiles stored in Windows registry/AppData.

**Linux alternative**: Config files.

```bash
# ~/.config/epos-gsx300/profiles/
├── gaming.json    # EQ + noise gate + sidetone settings
├── music.json     # Different EQ curve
└── current.json   # Active profile symlink
```

**Limitation**: Cannot persist to device memory (no vendor protocol).

---

### ❌ IMPOSSIBLE Without Vendor Protocol RE (3 features)

These features require reverse engineering the proprietary USB protocol:

#### 14. LED Ring Control

**EPOS Gaming Suite**: Blue = stereo, Red = surround. LED state changes on mode toggle.

**Why impossible**:
- No standard UAC/HID protocol for LED color control
- LED state persists across reboots → stored in device firmware
- Requires vendor-specific USB control transfer to change color
- GSX 1000 RE project (evilphish) did NOT achieve LED control
- No public USB captures of EPOS Gaming Suite exist

**What would be needed**:
1. USB packet capture with Wireshark/USBPcap
2. Identify vendor-specific control transfers
3. Decode LED command format
4. Implement in Linux daemon

**Effort**: Very High — full protocol RE required.

#### 15. Firmware Update

**EPOS Gaming Suite**: Updates firmware via STATUS tab.

**Why impossible**:
- No DFU (Device Firmware Upgrade) class detected
- Proprietary update mechanism
- Firmware images not publicly available
- Would need to: capture update traffic → decode protocol → extract firmware format → implement flasher

**Effort**: Very High — full protocol RE + firmware image analysis.

#### 7+4. Hardware 7.1 Toggle (with LED feedback)

**EPOS Gaming Suite**: Toggles device's internal 7.1 processing mode AND changes LED color.

**Why impossible**:
- 7.1 mode is controlled by vendor-specific USB command
- LED color change is tied to mode toggle
- Without the command, hardware stays in stereo mode
- Software HRTF alternative doesn't change LED or hardware state

**Note**: The EPOS Gaming Suite's 7.1 processing is HOST-SIDE (16-bit cap confirms this), but the mode toggle command still exists to signal the device and change LED.

---

## Implementation Priority

### Phase 1: Quick Wins (No RE Required) ✅
1. **Volume sync** — Already working (HID + PipeWire)
2. **Mic gain** — Already working (ALSA mixer)
3. **Noise gate** — `rnnoise` plugin
4. **Mic mute** — PipeWire command
5. **Sidetone** — `pw-loopback`
6. **Start on boot** — systemd service

### Phase 2: Software DSP (Approximation) ⚠️
1. **9-Band EQ** — PipeWire parametric EQ
2. **EQ Presets** — Config files + CLI
3. **Voice Enhancer** — PipeWire EQ on capture
4. **Sound Test** — Simple script
5. **Smart Button** — evtest + action mapping

### Phase 3: Vendor Protocol RE (Requires Hardware Access) ❌
1. **USB packet capture** — Wireshark + USBPcap in Windows VM
2. **LED ring control** — Decode vendor-specific USB commands
3. **7.1 hardware toggle** — Decode mode switch command
4. **Firmware update** — Decode update protocol

---

## Recommended Linux Daemon: `epos-gsx300d`

A minimal daemon that provides EPOS Gaming Suite-like functionality:

```
epos-gsx300d
├──(eq.py)          # 9-band EQ via PipeWire
├──(presets.py)     # Preset management
├──(sidetone.py)    # Sidetone loopback
├──(noise_gate.py)  # rnnoise integration
├──(button.py)      # Smart button handler
├──(voice.py)       # Mic voice enhancer
└──(config.json)    # User configuration
```

**Interface**: CLI commands + optional Waybar/dunst notifications.

```bash
# Example usage
epos-gsx300 eq --preset music
epos-gsx300 sidetone --level 30
epos-gsx300 noise-gate --threshold -30dB
epos-gsx300 voice --mode warm
```

---

## Summary Scorecard

| Category | Possible | Partial | Impossible |
|----------|----------|---------|------------|
| Playback (6 features) | 1 | 4 | 1 |
| Microphone (6 features) | 4 | 2 | 0 |
| Settings (5 features) | 2 | 2 | 2 |
| **Total (17 features)** | **7 (41%)** | **8 (47%)** | **3 (12%)** |

**Bottom line**: 94% of EPOS Gaming Suite features can be achieved or approximated on Linux. LED ring control and 2.0/7.1 toggle were REVERSE-ENGINEERED (2026-09-09) — vendor HID Report ID 0x02 controls the LED (`0x01`=blue, `0x02`=red). Only firmware update remains impossible without full vendor protocol RE.

---

## Sources

- EPOS Gaming Suite FAQ PDF: https://www.eposaudio.com/globalassets/blocks/gaming/gaming-suite/epos-gaming-suite_faq.pdf
- EPOS KB GSX 300 Troubleshooting: https://www.eposaudio.com/en/us/gaming/support/knowledge-base/sound-card-and-amplifiers/gsx-300/gsx-300---basic-troubleshooting
- 9to5toys GSX 300 Review: https://9to5toys.com/2020/10/15/epos-gsx-300-gsp-602-review
- evilphish/sennheiser-gsx-1000: https://github.com/evilphish/sennheiser-gsx-1000
- PipeWire Spatial Audio: https://pipewire.org
- OpenRAzer RE Guide: https://github.com/openrazer/openrazer/wiki/Reverse-Engineering-USB-Protocol
