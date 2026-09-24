# EPOS GSX 300 — Quick Reference Card

> One-page cheat sheet for hardware/software engineers

---

## Device Identity

| Field | Value |
|-------|-------|
| **USB VID:PID** | `1395:0098` |
| **Manufacturer** | `DSEA A/S` (EPOS Group A/S) |
| **Product** | `EPOS GSX 300` |
| **Serial** | `A003200202602692` (per-device) |
| **USB Class** | UAC1 (USB Audio Class 1.0) |
| **USB Speed** | Full Speed 12 Mbps (despite USB 2.0 bus) |
| **Power** | Bus-powered, 100mA max |

---

## Chipset

| Parameter | Value |
|-----------|-------|
| **DAC** | Conexant CX20988-10Z (Synaptics AudioSmart CX21988-THX) |
| **Package** | 46-pin WLCSP, 3mm × 3.3mm |
| **THX Certified** | Yes (first USB-C audio codec, 2017) |
| **Crystal** | Internal (crystal-less) |
| **DAC** | 24-bit stereo playback |
| **ADC** | 16-bit mono capture |
| **Max Rate** | 48kHz (datasheet) / 96kHz (marketing) |
| **EQ** | 5-band parametric (playback), 2-band (recording) |

---

## USB Interfaces

| IF | Class | Description | EP | Format |
|----|-------|-------------|-----|--------|
| 0 | Audio/AC | Audio Control (UAC1) | — | — |
| 1 | Audio/AS | Capture (mic) IN | 0x81 Async | S16_LE 1ch 48kHz |
| 2 | Audio/AS | Playback (HP) OUT | 0x01 Adaptive | S16/S24_3LE 2ch 48/96kHz |
| 3 | HID | Consumer Control | 0x84 Int | Volume knob |

---

## Linux Identifiers

| System | Value |
|--------|-------|
| **ALSA Card** | Card 4 (`E300`) |
| **PipeWire Node** | `alsa_output.usb-Sennheiser_EPOS_GSX_300_A003200202602692-00.analog-stereo` |
| **HID Device** | `/dev/hidraw3`, `/dev/input/event4` (kbd) |
| **Kernel Module** | `snd_usb_audio` (generic) |
| **Quirks** | NONE |
| **UCM Profiles** | NONE |

---

## Key Facts

1. **Stereo only** — 7.1 is software-only (EPOS Gaming Suite, Windows only)
2. **No 44.1kHz** — only 48kHz and 96kHz in USB descriptors
3. **Volume dial** — local analog gain, no software sync on Linux
4. **LED ring** — blue = stereo, red = surround (state persists from Windows)
5. **Low power** — CX20988 is a headset codec, not a standalone DAC
6. **Recommended impedance** — 25-75 ohm headphones

---

## WirePlumber Config

```conf
# /etc/wireplumber.conf.d/ or ~/.config/wireplumber/wireplumber.conf.d/
# 50-epos-gsx300-gaming.conf

monitor.alsa.rules = [
  {
    matches = [{ node.name = "~alsa_output.usb-Sennheiser_EPOS_GSX_300.*" }]
    actions = {
      update-props = {
        session.suspend-timeout-seconds = 0
        node.latency = "256/48000"
        audio.rate = 48000
        audio.format = S24_3LE
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

## Useful Commands

```bash
# USB info
lsusb -v -d 1395:0098

# ALSA info
cat /proc/asound/card4/stream0
amixer -c 4 contents
arecord -l && aplay -l

# PipeWire info
wpctl status
pw-dump | grep -A20 "GSX"

# HID test
evtest /dev/input/event4

# Kernel log
dmesg | grep -i "epos\|sennheiser\|1395"
journalctl -b | grep -i "1395:0098"
```

---

## Confirmed HID Protocol (2026-09-10 — hardware verified)

Device: `/dev/hidraw3` (VID `1395` PID `0098`), ep-gsx300 daemon.

### LED Ring (Report ID `0x02`, Output, vendor page 0xFF13)

Firmware-decode (handler entry $CA47 `LDX #$02`, 2026-09-12): value = byte[6] & 0x1F

**Wire protocol (hardware-confirmed, AGENT-FINDINGS §3.1)** — report 0x02 byte[1]:

| Wire value | LED (physical) |
|------------|----------------|
| `0x00` | Off |
| `0x01` | **Blue** — stereo (2.0) |
| `0x02` | **Red** — 7.1 |
| `0x03` | **Pink** (both bits) |

> Usage-ids 0x05/0x06 in older docs are HID *usage numbers*, NOT wire values.
> Daemon clamps & 0x03 (2-bit field).

**Firmware internal dispatch value (RE decode):**

| Int. value | Firmware path |
|------------|---------------|
| `0` | Off — $CA85 clear $1388/$1389 |
| `2` | CON path (byte[4]&0x0F), red-ish |
| `5` | Blue path (mode≠4) — $CA85 clear pair |
| `≥3` | Rejected at dispatch (`BCS` @$CA93) — SMB0 $9F flag only |

> Open question: wire 0x03 = pink verified working on hardware, but internal dispatch
> rejects ≥3 — wire→internal mapping not fully closed; daemon clamps left as-is.

LED = 2-bit shift register $1388/$1389 built via CLC/BBR4/SEC/ROL chain.

### Smart Button (Report ID `0x02`, Input — state readback)

| Value | Meaning |
|-------|---------|
| `0x01` | Mode = stereo (blue) |
| `0x02` | Mode = 7.1 (red) |
| `0x04` | Long-press (>2s) |

Vol knob: Report `0x01` Input — `0x01`=up, `0x02`=down, `0x00`=release.
No absolute readback; all state volatile (EEPROM diff = 0 bytes; see NVM-PERSISTENCE-REPORT.md).
Host-side ±5/detent tracking (daemon init 100) is the only design.

### Firmware (RE Phase 6+ — see HARDWARE-BOOK.md for full detail)

| Item | Value |
|------|-------|
| Chip | Conexant **CX21988** (chip ID $1005 = 0x08) |
| CPU | **W65C02S** (bit ops RMB/SMB/BBR/BBS, WAI/STP, undef→NOP) |
| FW build | `FREEMAN_V03.01.00.00` / patch 44.05.62 |
| Memory bus | HID 0x04/0x05 interrupt, 1KB pages; **bit6 = write, NEVER set** |
| EEPROM | 128KB, 7x FW copies (wear-leveling) |
| RAM | 64KB (0x0000–0xFFFF) |
| Vector table | $1238 = 10 handler slots, 3 self-patched at boot |
| Mode machine | $137D 0–5: idle/EQ-off, setup, intermediate, LED-active{3,4}, EQ-active(5) |
| LED regs | $1388/$1389 shift pair, gate $137D∈{3,4} |
| HW EQ | 5-band preset-only (4 ROM banks, matematical envelopes) — NOT USB-controllable |
| Persistence | **NONE** — volume/mode fully volatile, daemon re-assert is correct |

⚠️ Reports 0x06/0x07/0x1A = firmware-update bus ("Download initiated.." strings in
ROM) — **do NOT probe with write intent** (brick risk).

### DSP (PipeWire filter-chain, all SOLVED 2026-09-10)

| Feature | File in `~/.config/pipewire/pipewire.conf.d/` |
|---------|-----------------------------------------------|
| 9-band EQ (`bq_peaking`) | `50-epos-eq.conf` |
| Voice Enhancer (Warm/Clear) | `~/.config/pipewire-epos/voice/pipewire.conf` (instance `pipewire-epos@voice`; the old main-instance `51-epos-voice-enhancer.conf` was removed to avoid double-processing) |
| Noise Gate (rnnoise LADSPA) | `93-epos-noisegate.conf` |

Noise gate needs `librnnoise_ladspa.so` → `~/.local/lib/ladspa/` +
`LADSPA_PATH` drop-in `~/.config/systemd/user/pipewire.service.d/ladspa.conf`.

### Daemon

- Config: `~/.config/epos-gsx300/config.json` (atomic write)
- IPC: Unix socket + **HTTP bridge `127.0.0.1:9898`** (CORS) — web GUI
- Service: `epos-gsx300d` (systemd user, `Wants=pipewire`)
- Repo: `github.com/slimulv1/epos-gsx300-linux`
