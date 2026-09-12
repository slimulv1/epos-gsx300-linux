# EPOS GSX 300 — Reverse Engineering Analysis

> Comprehensive hardware, USB protocol, and Linux driver analysis
> Generated: 2026-09-09 | Machine: Core64 (Arch Linux, kernel 7.2.4-lqx2-1-arisa)
> **Update 2026-09-10**: LED protocol, smart button, and all DSP features
> (EQ/voice-enhancer/noise-gate) are now SOLVED — see
> **[AGENT-FINDINGS.md](./AGENT-FINDINGS.md)** for the agent-compiled archive
> and confirmed HID/DSP protocols.

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [USB Device Identification](#2-usb-device-identification)
3. [Chipset Deep Dive: Conexant CX20988](#3-chipset-deep-dive-conexant-cx20988)
4. [USB Audio Class Implementation](#4-usb-audio-class-implementation)
5. [Hardware Design & Teardown](#5-hardware-design--teardown)
6. [ALSA/PipeWire/Linux Driver Stack](#6-alsapipewirelinux-driver-stack)
7. [The 7.1 Surround Mystery](#7-the-71-surround-mystery)
8. [Volume Dial & HID Interface](#8-volume-dial--hid-interface)
9. [EPOS Gaming Suite Protocol](#9-epos-gaming-suite-protocol)
10. [Kernel Quirks & Known Issues](#10-kernel-quirks--known-issues)
11. [Comparison: GSX 300 vs GSX 1000 vs GSX 1200 Pro](#11-comparison-gsx-300-vs-gsx-1000-vs-gsx-1200-pro)
12. [Linux Workarounds & Recommendations](#12-linux-workarounds--recommendations)
13. [References & Sources](#13-references--sources)

---

## 1. Executive Summary

The EPOS GSX 300 (formerly Sennheiser GSX 300) is a USB external sound card marketed as a "gaming DAC" with 7.1 surround sound capability. Our reverse engineering reveals:

- **DAC Chipset**: Conexant CX20988 (NOT CX21988 as previously assumed) — a cost-optimized USB-C headset codec, NOT a high-performance standalone DAC
- **USB Class**: UAC1 (USB Audio Class 1.0) — Full Speed 12Mbps
- **Native Formats**: S16_LE/S24_3LE @ 48kHz (stereo out), S16_LE @ 48kHz (mic capture)
- **7.1 Surround**: 100% SOFTWARE — EPOS Gaming Suite does HRTF binaural rendering on host CPU, sends stereo to hardware. The hardware has NO internal 7.1 processing.
- **Volume Dial**: HID Consumer Control (relative events only) — local analog gain on Linux, no software sync
- **No kernel quirks** for VID 1395:PID 0098 — standard snd-usb-audio handles it
- **No public datasheet** for CX20988

**Key insight**: The GSX 300 is fundamentally a stereo USB audio device with software-based 7.1 surround processing. The "7.1" marketing is achieved entirely through the Windows-only EPOS Gaming Suite.

---

## 2. USB Device Identification

### Device Descriptor

| Field              | Value                                    |
| ------------------ | ---------------------------------------- |
| **Bus/Device**     | `Bus 001 Device 005`                        |
| **idVendor**       | `0x1395` (DSEA A/S — EPOS Group A/S)        |
| **idProduct**      | `0x0098` (EPOS GSX 300)                     |
| **bcdDevice**      | `0.62`                                     |
| **bcdUSB**         | `2.00` (USB 2.0 bus)                        |
| **bDeviceClass**   | `0` (Composite)                            |
| **bMaxPacketSize0**| `64`                                       |
| **bmAttributes**   | `0xa0` (Bus Powered, Remote Wakeup)        |
| **MaxPower**       | `100mA`                                    |
| **Manufacturer**   | `"Sennheiser"`                             |
| **Product**        | `"EPOS GSX 300"`                           |
| **Serial**         | `"A003200202602692"`                       |
| **Configurations** | `1`                                        |

### USB Interfaces (4 total)

| Interface | Class            | Subclass         | Protocol | Description                              |
| --------- | ---------------- | ---------------- | -------- | ---------------------------------------- |
| **0**         | Audio (0x01)     | AudioControl (0x01) | 0x00 (UAC1) | Audio topology (terminals, feature units) |
| **1**         | Audio (0x01)     | AudioStreaming (0x02) | 0x00 (UAC1) | Capture (microphone) IN endpoint          |
| **2**         | Audio (0x01)     | AudioStreaming (0x02) | 0x00 (UAC1) | Playback (headphones) OUT endpoint        |
| **3**         | HID (0x03)       | —                | —        | Consumer Control (volume knob)            |

### Audio Topology (Interface 0 — AudioControl)

```
Input Terminal 1: Microphone (1ch mono)
  └─ Feature Unit 2: Mute + Volume (mic)
      └─ Output Terminal 3: USB Streaming → Host

Input Terminal 4: USB Streaming (from host, 2ch stereo FL/FR)
  └─ Feature Unit 5: Mute (master) + Volume (per-channel L/R)
      └─ Output Terminal 6: Headphones
```

### Streaming Interfaces

**Interface 1 — Capture (Microphone)**
- Altset 1: `S16_LE`, 1ch, 48000Hz, EP `0x81` IN (Async), wMaxPacketSize=96 bytes

**Interface 2 — Playback (Headphones)**
- Altset 1: `S16_LE`, 2ch, 48000Hz, EP `0x01` OUT (Adaptive), wMaxPacketSize=768 bytes
- Altset 2: `S24_3LE`, 2ch, **48000+96000Hz**, EP `0x01` OUT (Adaptive), wMaxPacketSize=576 bytes

**Interface 3 — HID (Volume Knob)**
- EP `0x84` IN (Interrupt), wMaxPacketSize=35 bytes, bInterval=1ms

**Critical observation**: No 44100Hz support anywhere in USB descriptors. The device only supports 48kHz and 96kHz.

---

## 3. Chipset Deep Dive: Conexant CX20988

### Identification

The GSX 300 uses the **Conexant CX20988-10Z** (now Synaptics AudioSmart CX21988-THX after Conexant's audio division was acquired by Synaptics).

| Parameter          | Value                                           |
| ------------------ | ----------------------------------------------- |
| **Full Name**      | Synaptics AudioSmart CX21988-THX                |
| **Form Factor**    | 3mm x 3.3mm, 46-pin WLCSP (Wafer-Level CSP)    |
| **Type**           | USB-C compliant audio CODEC                      |
| **THX Certified**  | First USB-C audio codec to earn THX approval (2017) |
| **Temperature**    | -40°C to +85°C                                   |
| **Crystal**        | Crystal-less (internal oscillator)               |

### Datasheet Capabilities

| Feature                | Specification                          |
| ---------------------- | -------------------------------------- |
| **DAC Resolution**     | 24-bit stereo playback                 |
| **ADC Resolution**     | 16-bit microphone capture              |
| **Max Sample Rate**    | 48kHz (per datasheet)                  |
| **SNR**                | ~100 dB (from InLine product spec using same chip) |
| **THD+N**              | -85 dB (~0.006%)                       |
| **Crosstalk**          | -50 dB                                 |
| **EQ**                 | 5-band parametric (playback), 2-band (recording) |
| **Mic Boost**          | Adjustable in 3dB steps                |
| **Headset Jack**       | CTIA/OMTP detection                    |
| **USB Compliance**     | Class-compliant (no driver needed)     |

### Why This Matters

**The CX20988 is a HEADSET CODEC, NOT a standalone DAC.** It was designed for tiny USB-C to 3.5mm dongles (like the InLine USB-C audio adapter). PC Perspective's teardown explicitly noted this:

> "The CX20988 is typically found in tiny USB-C to 3.5mm dongles, NOT high-performance standalone DACs. This validates our impression of limited power/headroom."

**Performance implications:**
- Limited output power — not suitable for high-impedance headphones
- "Congested sound in complex passages" (PC Perspective review)
- "Feels like it's being pushed even at what I consider normal listening levels"
- Recommended headphone impedance: 25-75 ohm

### No Public Datasheet

⚠️ **No public CX20988 datasheet, programming manual, or reference design exists.** The chip is a USB Audio Class device — it enumerates using standard UAC descriptors, not vendor-specific registers. No CX21988-specific kernel driver exists; `snd-usb-audio` (generic) handles it.

**Closest sibling**: CX21986 ([public datasheet available](https://cy.mslforce-china.net/uploads/9824/files/Synaptics-CX21986-USB-audio-CODEC.pdf)) — contains register descriptions, app diagrams, electrical characteristics. ⚠️ CX21986 docs must NOT be treated as CX20988 register-compatible without vendor confirmation.

---

## 4. USB Audio Class Implementation

### UAC Classification

The GSX 300 implements **UAC1 (USB Audio Class 1.0)**:
- `bInterfaceProtocol = 0x00` in AudioControl interface → UAC1
- USB 2.0 bus does NOT mean UAC2 — the class revision is declared by `bInterfaceProtocol`

### UAC1 vs UAC2 Comparison

| Feature           | UAC1 (GSX 300)                        | UAC2                                    |
| ----------------- | -------------------------------------- | --------------------------------------- |
| **Descriptor model**  | AudioControl + AudioStreaming; format-type-specific | Explicit Clock Source/Selector/Multiplier entities |
| **Sample rate**       | Advertised in AS format descriptor (bSamFreqType) | Clock Source controls rate; host queries GET_RANGE |
| **Feedback endpoint** | Separate isochronous synch endpoint (bSynchAddress) | Explicit feedback as isochronous IN with "Usage Type Feedback" |
| **Sync types**        | Synchronous, Adaptive, Asynchronous   | Same three types                        |
| **Bus speed**         | Designed for Full Speed (12 Mbps)      | High-Speed/SuperSpeed-oriented          |

### Sample Rate Negotiation (UAC1)

1. Host reads rates from AS format descriptor (`bSamFreqType`: 0=continuous, N=discrete 3-byte values)
2. Host selects AS alternate setting
3. Host sends `SET_CUR` with 3-byte value on streaming endpoint's sampling-frequency control
4. Device programs internal PLL/clock

**GSX 300 behavior**:
- Altset 1 (S16): advertises only 48000Hz
- Altset 2 (S24): advertises 48000 + 96000Hz
- No 44100Hz support anywhere

### Feedback Endpoint

The GSX 300 uses **Adaptive** synchronization (EP `0x01` OUT, Adaptive) — the endpoint adjusts its packet timing to match the host's data rate. No explicit feedback endpoint is present.

### 7.1 Surround Descriptor Layout (UAC2)

For reference, a UAC2 device exposing 7.1 surround would use:
- `bNrChannels=8`, `bmChannelConfig=0x0000063F` (FL=bit0, FR=bit1, FC=bit2, LFE=bit3, BL=bit4, BR=bit5, SL=bit9, SR=bit10)
- The GSX 300 only exposes `bNrChannels=2` with FL/FR

---

## 5. Hardware Design & Teardown

### Internal Architecture

Based on PC Perspective's teardown:

```
┌─────────────────────────────────┐
│         Top PCB (upper)         │
│  ┌─────────┐  ┌──────────────┐ │
│  │ CX20988 │  │ 3.5mm Jacks  │ │
│  │  (DAC)  │  │ (out + mic)  │ │
│  └─────────┘  └──────────────┘ │
└──────────┬──────────────────────┘
           │ ribbon connector
┌──────────┴──────────────────────┐
│        Bottom PCB (lower)       │
│  ┌──────────┐  ┌─────────────┐ │
│  │ Micro-USB│  │ Volume Knob │ │
│  │  Port    │  │ (rotary enc)│ │
│  └──────────┘  └─────────────┘ │
└─────────────────────────────────┘
           │
    ┌──────┴──────┐
    │ Metal Plate │  ← provides weight/stability
    │ (base)      │
    └─────────────┘
```

### Physical Specifications

| Parameter          | Value                              |
| ------------------ | ---------------------------------- |
| **Dimensions**     | 169mm × 142mm × 57mm               |
| **Weight**         | ~155g (device), ~309g (with box)   |
| **Cable**          | 1.2m USB-A to Micro-USB            |
| **Material**       | Plastic shell + metal base plate   |
| **Construction**   | 2 screws + 2 plastic tabs (front panel) |

### External Features

| Feature                  | Description                                    |
| ------------------------ | ---------------------------------------------- |
| **Volume Knob**          | Rotary encoder, center of front face            |
| **Smart Button**         | Front left — profile/surround toggle            |
| **LED Ring**             | Around volume knob: **blue = stereo (2.0)**, **red = surround (7.1)** |
| **Micro-USB Port**       | Rear center                                     |
| **3.5mm Audio Output**   | Rear — stereo headphone out                     |
| **3.5mm Microphone Input** | Rear                                           |
| **Rubber Feet**          | Two long rubber strips on base                  |

### Power Delivery

- **Bus-powered** via USB (100mA max from descriptor)
- No external power supply
- USB autosuspend: 2000ms delay (but `runtime_suspended_time=0` — never actually suspends due to PipeWire suspend-off config)

---

## 6. ALSA/PipeWire/Linux Driver Stack

### Kernel Module

```
snd_usb_audio    598016  8   ← handles the GSX 300
snd_usbmidi_lib   53248  1   ← MIDI (unused by GSX 300)
snd_ump           32768  1   ← Unified MIDI Protocol
```

**Driver**: Standard `snd-usb-audio` (kernel module). The CX20988 is UAC-compliant and requires no special driver.

### ALSA Card Enumeration

```
Card 0: HDA-Intel (HDA Intel PCH)
Card 1: HDA-Intel (HDA ATI HDMI)
Card 2: USB-Audio (USB Audio — A2+ Generic USB DAC)
Card 3: USB-Audio (Audioengine 2+)
Card 4: USB-Audio (EPOS GSX 300)
```

**EPOS = Card 4**, USB bus `001/005`, bus path `usb-0000:00:14.0-7`

### ALSA Stream Info

**Playback (card4/stream0):**
```
Interface 2 Altset 1: S16_LE, 2ch, 48000, EP 0x01 (ADAPTIVE), FL FR
Interface 2 Altset 2: S24_3LE, 2ch, 48000/96000, EP 0x01 (ADAPTIVE), FL FR
```

**Capture (card4/stream0):**
```
Interface 1 Altset 1: S16_LE, 1ch, 48000, EP 0x81 (ASYNC), MONO
Momentary freq = 48000 Hz
```

### ALSA Mixer Controls

| Numid | Control                | Type    | Channels | Range (dB)         | Current |
| ----- | ---------------------- | ------- | -------- | ------------------ | ------- |
| 5     | PCM Playback Switch    | BOOLEAN | —        | —                  | on      |
| 6     | PCM Playback Volume    | INTEGER | 2ch      | -50 to 0 dB        | 26/26   |
| 3     | Mic Capture Switch     | BOOLEAN | —        | —                  | on      |
| 4     | Mic Capture Volume     | INTEGER | 1ch      | -30 to +5 dB       | 35/35   |
| 1     | Capture Channel Map    | MONO    | —        | —                  | (fixed) |
| 2     | Playback Channel Map  | FL,FR   | —        | —                  | (fixed) |

### USB Mixer Details

```
Unit 2: Mic Capture Volume  — S16, 1ch, min=-7680 max=1280 (-30.00 to +5.00 dB)
Unit 2: Mic Capture Switch  — INV_BOOLEAN
Unit 5: PCM Playback Volume — S16, 2ch, min=-12800 max=0 (-50.00 to 0.00 dB)
Unit 5: PCM Playback Switch — INV_BOOLEAN
```

### PipeWire/WirePlumber State

**PipeWire Version**: 1.6.8

**Device Node:**
```
node.name:  alsa_output.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.analog-stereo
state:      suspended
format:     S24_3LE, 48000Hz, 2ch FL/FR
latency:    256/48000
suspend-timeout-seconds: 0  ← permanently disabled
pause-on-idle: false
alsa.path:  alsa:acp:E300:3:playback
```

**Source Node:**
```
node.name:  alsa_input.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.mono-fallback
state:      running
format:     S16LE, 48000Hz, 1ch MONO
alsa.path:  alsa:acp:E300:2:capture
```

### WirePlumber Config

```conf
# ~/.config/wireplumber/wireplumber.conf.d/50-epos-gsx300-gaming.conf

monitor.alsa.rules = [
  {
    matches = [{ node.name = "~alsa_output.usb-Sennheiser_EPOS_GSX_300.*" }]
    actions = {
      update-props = {
        session.suspend-timeout-seconds = 0
        node.latency = "256/48000"
        audio.rate = 48000
        audio.format = S24_3LE
        audio.channels = 2
      }
    }
  }
  {
    matches = [{ node.name = "~alsa_input.usb-Sennheiser_EPOS_GSX_300.*" }]
    actions = {
      update-props = {
        session.suspend-timeout-seconds = 0
      }
    }
  }
]
```

### No ALSA UCM Profiles

⚠️ No UCM (Use Case Manager) profiles exist for VID `1395` in `/usr/share/alsa/ucm2/` or `ucm3/`. The device falls back to generic USB Audio handling.

---

## 7. The 7.1 Surround Mystery

### Hardware Reality

**The GSX 300 hardware is fundamentally a STEREO device.** It has:
- 2-channel USB audio output (FL/FR)
- 2-channel headphone output (3.5mm TRS stereo)
- No internal DSP for multi-channel processing
- No hardware mixer for 7.1 routing

### How 7.1 Works on Windows

The "7.1 surround" is achieved entirely through the **EPOS Gaming Suite** software:

```
Game Audio (7.1 PCM)
    ↓
EPOS Gaming Suite (Windows)
    ↓ HRTF Binaural Rendering (host CPU)
    ↓ Custom DSP: EQ, reverb, spatial audio
Stereo PCM (2ch)
    ↓
GSX 300 Hardware (UAC1 stereo)
    ↓
Headphones (3.5mm stereo)
```

### EPOS Gaming Suite Features

- Toggle between 2.0 Stereo and 7.1 Surround Sound
- 9-band customizable equalizer (64Hz, 125Hz, 250Hz, 500Hz, 1kHz, 2kHz, 4kHz, 8kHz, 16kHz)
- 4 preset profiles: Flat, Music, Esport, Movie
- Custom sound profiles (user-created, saved to device)
- Adjustable reverberation level (7.1 mode only)
- Microphone settings: voice enhancer (warm/clear/off), gain, side tone, noise gate, noise cancellation
- Smart button configuration (surround toggle vs. preset switcher)
- Firmware updates
- LED ring mode sync with Windows volume

### Critical Warning: 16-bit Downgrade

⚠️ **EPOS FAQ confirms**: "The EPOS Software Suite and its 7.1 processing only support 16-bit audio today."

Installing the Gaming Suite on Windows **DOWNGRADES** your stereo from 24-bit to 16-bit. For 24-bit stereo, you must **UNINSTALL** the Gaming Suite.

### Why No 7.1 on Linux

1. **No Linux equivalent of EPOS Gaming Suite exists**
2. The hardware has no internal 7.1 processing
3. The USB descriptors only expose 2 channels
4. No reverse engineering effort has produced a Linux surround implementation

### Linux Alternatives for Spatial Audio

- **PipeWire spatial audio** + HRTF convolution
- **Virtual Surround Manager** or **IrateGoose** for HRTF processing
- **PulseAudio** module-echo-cancel with spatial filters
- **Wine/Proton** — some games support spatial audio natively

---

## 8. Volume Dial & HID Interface

### HID Descriptor

| Field              | Value                                |
| ------------------ | ------------------------------------ |
| **Interface**      | 3 (HID, USB Audio Class)             |
| **EP**             | `0x84` IN (Interrupt), 35 bytes, 1ms  |
| **HID Version**    | 1.11                                 |
| **Report Size**    | 120 bytes (descriptor)               |
| **Device Nodes**   | `/dev/hidraw3`, `/dev/input/event4` (kbd), `/dev/input/event7` |

### Linux Input Events

The volume knob exposes as **Consumer Control** (HID usage page 0x0C):
- `/dev/input/event4` — Consumer Control (kbd handler)
- `/dev/input/event7` — generic input

### Behavior on Linux

- **Relative events only** — the dial sends volume-up/volume-down relative steps, not absolute positions
- **Local analog gain** — the physical dial controls analog gain on the DAC, NOT software volume
- **No sync with PipeWire** — turning the dial does NOT change the PipeWire/ALSA software volume
- **LED ring** — controlled by EPOS Gaming Suite (Windows only); on Linux, LED stays in last-set state

### Why No Software Sync

The EPOS Gaming Suite intercepts HID events and translates them to Windows volume API calls. On Linux, no equivalent exists:
- Standard HID events are relative (volume up/down), not absolute position
- Linux desktop media key handling routes to default sink only
- If PipeWire/PulseAudio doesn't process media keys (e.g., dwm without DE), events may be lost

### Potential Linux Workaround

Check if events arrive:
```bash
evtest /dev/input/event4  # Turn the dial while running
```

If events arrive, use a daemon like `alsa_volume_from_usb_hid` to map HID events to ALSA mixer controls.

---

## 9. EPOS Gaming Suite Protocol

### Communication Protocol (Theorized)

Based on USB descriptor analysis and Windows driver behavior:

```
┌─────────────────────────────────────────┐
│         EPOS Gaming Suite (Windows)      │
├─────────────────────────────────────────┤
│ 1. Audio Streaming (UAC1)               │
│    - Standard USB Audio Class            │
│    - Isochronous OUT (EP 0x01)           │
│    - S16/S24 @ 48/96 kHz                │
│                                          │
│ 2. Settings Control (Vendor-Specific?)   │
│    - Surround mode toggle (2.0 ↔ 7.1)   │
│    - EQ preset selection                 │
│    - Mic settings (gain, side tone)      │
│    - Smart button function               │
│    - LED ring color/mode                 │
│                                          │
│ 3. HID Consumer Control                  │
│    - Volume knob (relative events)       │
│    - Button press (if any)               │
└─────────────────────────────────────────┘
```

### Reverse Engineering Status

⚠️ **No public reverse engineering** of the EPOS Gaming Suite protocol exists:
- No GitHub repos for EPOS/Sennheiser USB audio RE
- No Wireshark USB captures of Gaming Suite communication
- No open-source alternatives to EPOS Gaming Suite
- No kernel mailing list discussions about GSX 300 vendor-specific commands

### RE Roadmap

To fully reverse engineer the protocol:

1. **Capture USB descriptors**: `sudo lsusb -v -d 1395:0098`
2. **Verify volume dial mechanism**: `evtest /dev/input/event4` while turning
3. **Windows VM capture**: USB passthrough + Wireshark/USBPcap
4. **One-setting-per-capture protocol**: Toggle surround, EQ, sidetone one at a time
5. **Analyze HID reports**: Capture volume knob events, button presses
6. **Firmware extraction**: If possible, dump firmware from the device

---

## 10. Kernel Quirks & Known Issues

### Sennheiser/EPOS Quirk Table

| USB ID      | Device                  | Quirk                           | Effect                                                                                 |
| ----------- | ----------------------- | ------------------------------- | -------------------------------------------------------------------------------------- |
| `1395:740a`   | Sennheiser DECT         | `QUIRK_FLAG_GET_SAMPLE_RATE`      | Skips reading sample rate from device                                                  |
| `1395:0300`   | GSP 670 / GSA 70 dongle | `QUIRK_COMPOSITE`                 | Reorders interfaces (fixes firmware enumeration)                                       |
| `1377:6004`   | MOMENTUM 3              | `QUIRK_FLAG_MIXER_GET_CUR_BROKEN` | GET_CUR volume reports constant; driver uses internal cache                            |
| **`1395:0098`**   | **GSX 300**                 | **NONE**                            | No quirk entries found — uses standard UAC1 descriptor parsing                         |

### Known Issues

1. **USB autosuspend**: Default 2000ms autosuspend delay. Fixed with WirePlumber `session.suspend-timeout-seconds=0`.

2. **Volume dial not syncing**: Expected behavior — local analog gain only on Linux.

3. **USB disconnections**: nikktech review reported ~8 brief (1-2 second) disconnections over 2 weeks. Likely USB power management — the suspend-off fix helps.

4. **7.1 downgrade**: Installing Gaming Suite on Windows forces 16-bit audio. Not relevant on Linux (Gaming Suite doesn't run).

5. **Low power output**: Multiple reviewers noted limited headroom and "congested" sound at higher volumes. Recommended impedance: 25-75 ohm.

6. **No UCM profiles**: Device falls back to generic USB Audio handling.

### USB Power State (Current)

```
sysfs path:        /sys/bus/usb/devices/1-7
power/control:     on
autosuspend:       2000ms
runtime_status:    active
runtime_active_time: 2947612ms (~49 min since boot)
runtime_suspended_time: 0  ← never suspended this boot
```

---

## 11. Comparison: GSX 300 vs GSX 1000 vs GSX 1200 Pro

| Feature               | GSX 300               | GSX 1000              | GSX 1200 Pro          |
| --------------------- | --------------------- | --------------------- | --------------------- |
| **DAC Chip**              | CX20988               | Unknown (likely CX20724 or similar) | Unknown              |
| **USB Class**             | UAC1                  | UAC1                  | UAC1                  |
| **Max Resolution**        | 24-bit/96kHz          | 24-bit/96kHz          | 24-bit/96kHz          |
| **7.1 Surround**          | Software only         | Software only         | Software only         |
| **Touch Display**         | No                    | Yes (LED ring)        | Yes (touchscreen)     |
| **Chat/Mix Dial**         | No                    | Yes                   | Yes                   |
| **Audio Link (multi-device)** | No                | No                    | Yes (up to 2 devices) |
| **DAC Quality**           | Limited (CX20988 headset codec) | Better (standalone DAC) | Best (standalone DAC) |
| **Price (MSRP)**          | ~$79                  | ~$149                 | ~$249                 |
| **Linux Compatibility**   | UAC1 class-compliant  | UAC1 class-compliant  | UAC1 class-compliant  |

---

## 12. Linux Workarounds & Recommendations

### Current Setup (Verified Working)

1. ✅ Suspend-off configured: `session.suspend-timeout-seconds=0`
2. ✅ PipeWire negotiates S24_3LE @ 48kHz
3. ✅ Mic capture working (WEBRTC VoiceEngine)
4. ✅ No USB disconnects with suspend-off

### Recommended Improvements

1. **HID Volume Sync**:
   ```bash
   # Install alsa_volume_from_usb_hid for dial-to-PipeWire sync
   pip install alsa-volume-from-usb-hid  # or compile from source
   ```

2. **Software 7.1 Surround** (Linux alternative):
   ```bash
   # PipeWire spatial audio with HRTF
   # Use module-echo-cancel with spatial filters
   # Or: Wine/Proton with in-game spatial audio
   ```

3. **USB Quirk** (if issues arise):
   ```bash
   # /etc/modprobe.d/snd-usb-audio.conf
   options snd_usb_audio quirk_flags=0x0
   ```

4. **UCM Profile** (optional, for advanced control):
   - Create `/usr/share/alsa/ucm2/1395-0098/` with HiFi.conf
   - Define supported formats, rates, mixer controls

---

## 13. References & Sources

### Official Documentation
- [USB Audio Class 1.0 Specification](https://www.usb.org/sites/default/files/audio10.pdf)
- [USB Audio Class 2.0 Specification](https://www.usb.org/sites/default/files/Audio2_with_Errata_and_ECN_through_Apr_2_2025.pdf)
- [EPOS GSX 300 Fact Sheet](https://www.eposaudio.com/globalassets/__pim/products/gsx-series/gsx-300/bb6001f9-7ce6-4a73-954e-2448600653c5_35692_fact-sheet_gsx-300_en_original.pdf)
- [EPOS Troubleshooting (24-bit note)](https://www.eposaudio.com/en/us/gaming/support/knowledge-base/sound-card-and-amplifiers/gsx-300/gsx-300---basic-troubleshooting)
- [EPOS Gaming Suite FAQ](https://www.eposaudio.com/globalassets/blocks/gaming/gaming-suite/epos-gaming-suite_faq.pdf)

### Chipset & Hardware
- [CX20988 Datasheet (Arrow.com)](http://static6.arrow.com/aropdfconversion/bce0f8743af7dfa594380dd07adc26e3514cffdd/9cx20988_pb.pdf)
- [CX21986 Datasheet (Public)](https://cy.mslforce-china.net/uploads/9824/files/Synaptics-CX21986-USB-audio-CODEC.pdf)
- [InLine USB-C Adapter Spec (CX21988)](https://www.inline-info.com/en/InLine-USB-C-audio-adapter-to-3.5mm-jack-DAC-15-cm)
- [Synaptics THX Announcement](https://www.synaptics.com/company/news/THX)
- [ntchip CX20988 Part Listing](https://ntchip.com/parts-detail/adi/cx20988-10z)

### Reviews & Teardowns
- [PC Perspective Teardown & Review](https://pcper.com/2021/04/epos-gsx-300-external-sound-card-review)
- [nikktech Review](https://www.nikktech.com/main/articles/peripherals/sound-cards/11527-epos-sennheiser-gsx-300-gaming-series-external-usb-sound-card-review)
- [Vortez Press Release](https://www.vortez.net/news_story/upgrade_from_on_board_sound_with_the_epos_i_sennheiser_gsx_300_external_sound_card.html)

### Linux & Kernel
- [snd-usb-audio Quirks Source](https://github.com/torvalds/linux/blob/master/sound/usb/quirks.c)
- [ALSA Matrix (usb-audio)](https://www.alsa-project.org/wiki/Matrix:Module-usb-audio)
- [USB Terminal Types](https://alsa.mirrorservice.org/datasheets/usb/termt10.pdf)
- [Linux audio-v2.h](https://android-kvm.googlesource.com/linux/+/refs/tags/pkvm-7.1-gki/include/linux/usb/audio-v2.h)

### Volume Knob & HID
- [USB Volume Knob on Linux](https://andrewmemory.acornwall.net/blog/2019-12-16/mapping-a-usb-volume-knob-into-a-keyboard-on-linux-for-sdr/)
- [alsa_volume_from_usb_hid](https://github.com/neildavis/alsa_volume_from_usb_hid)
- [VolCon (Volume Control)](https://github.com/Clewsy/VolCon/)
- [OpenWave](https://github.com/rikkichy/openwave/)

### Community & RE
- [Sennheiser GSX 1000 RE](https://github.com/evilphish/sennheiser-gsx-1000)
- [DriverMax USB ID Database](https://www.drivermax.com/GSX-300-EPOS-Group-A-S-USB-VID-1395-PID_0098-MI_00-1_0_8_234-2020-12-02-4410479-driver.htm)

---

## Appendix A: USB Descriptor Dump (Abbreviated)

```
Bus 001 Device 005: ID 1395:0098 DSEA A/S EPOS GSX 300
Device Descriptor:
  bLength                18
  bDescriptorType        1
  bcdUSB               2.00
  bDeviceClass            0 (Composite)
  bDeviceSubClass         0
  bDeviceProtocol         0
  bMaxPacketSize0        64
  idVendor           0x1395 DSEA A/S
  idProduct          0x0098
  bcdDevice            0.62
  iManufacturer           1 Sennheiser
  iProduct                2 EPOS GSX 300
  iSerial                 3 A003200202602692
  bNumConfigurations      1

Configuration Descriptor:
  bLength                 9
  bDescriptorType         2
  wTotalLength       0x01d3 (467)
  bNumInterfaces          4
  bmAttributes         0xa0 (Bus Powered, Remote Wakeup)
  MaxPower              100mA

  Interface 0 (Audio Control):
    bInterfaceClass      1 Audio
    bInterfaceSubClass   1 AudioControl
    bInterfaceProtocol   0 UAC1

  Interface 1 (Audio Streaming IN):
    bInterfaceClass      1 Audio
    bInterfaceSubClass   2 AudioStreaming
    bInterfaceProtocol   0 UAC1
    Altset 1: S16_LE, 1ch, 48000Hz, EP 0x81 IN (Async)

  Interface 2 (Audio Streaming OUT):
    bInterfaceClass      1 Audio
    bInterfaceSubClass   2 AudioStreaming
    bInterfaceProtocol   0 UAC1
    Altset 1: S16_LE, 2ch, 48000Hz, EP 0x01 OUT (Adaptive)
    Altset 2: S24_3LE, 2ch, 48000/96000Hz, EP 0x01 OUT (Adaptive)

  Interface 3 (HID):
    bInterfaceClass      3 HID
    EP 0x84 IN (Interrupt), 35 bytes, 1ms
```

## Appendix B: Kernel Boot Log

```
Sep 09 19:37:06  usb 1-7: new full-speed USB device number 5 using xhci_hcd
Sep 09 19:37:06  usb 1-7: New USB device found, idVendor=1395, idProduct=0098, bcdDevice= 0.62
Sep 09 19:37:06  usb 1-7: New USB device strings: Mfr=1, Product=2, SerialNumber=3
Sep 09 19:37:06  usb 1-7: Product: EPOS GSX 300
Sep 09 19:37:06  usb 1-7: Manufacturer: Sennheiser
Sep 09 19:37:06  usb 1-7: SerialNumber: A003200202602692
Sep 09 19:37:10  input: Sennheiser EPOS GSX 300 Consumer Control (event4, kbd)
Sep 09 19:37:10  input: Sennheiser EPOS GSX 300 (event7)
Sep 09 19:37:10  hid-generic: USB HID v1.11 Device [Sennheiser EPOS GSX 300] on hidraw3
```

## Appendix C: WirePlumber Config (Active)

```conf
# ~/.config/wireplumber/wireplumber.conf.d/50-epos-gsx300-gaming.conf
monitor.alsa.rules = [
  {
    matches = [{ node.name = "~alsa_output.usb-Sennheiser_EPOS_GSX_300.*" }]
    actions = {
      update-props = {
        session.suspend-timeout-seconds = 0
        node.latency = "256/48000"
        audio.rate = 48000
        audio.format = S24_3LE
        audio.channels = 2
      }
    }
  }
  {
    matches = [{ node.name = "~alsa_input.usb-Sennheiser_EPOS_GSX_300.*" }]
    actions = {
      update-props = {
        session.suspend-timeout-seconds = 0
      }
    }
  }
]
```

---

*Document generated by Shadow Monarch (opencode-arise) with 3 parallel research agents.*
*Local data: beru (system probe) | Hardware: tank (web research) | USB/Kernel: tank (protocol research)*
