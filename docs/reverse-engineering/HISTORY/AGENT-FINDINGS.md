# EPOS GSX 300 — Research Archive (Agent Compilation)

> Compilation of all documentation gathered by background research agents
> (arise_background shadows) + experimental reverse-engineering results,
> centrally archived at `~/Projects/epos-gsx300-reverse-engineering/`.
>
> **Last updated:** 2026-09-10 (DSP milestone complete)

---

## 1. Sources & organization

| File | Content | Source |
|------|----------|-------|
| `README.md` | 13-chapter hardware analysis (chipset, USB, UAC, ALSA/PipeWire, 7.1 mystery, protocol RE) | 3 agents round 1: beru (local data), 2× tank (hardware + USB kernel) |
| `QUICK-REFERENCE.md` | Single-page lookup card (device identity, streams, mixer, workaround) | Compiled from round 1 |
| `FEASIBILITY-ANALYSIS.md` | 18-feature EPOS Gaming Suite → Linux feasibility analysis, updated with SOLVED status | 2 agents round 2 (UE UI/protocol analysis) + acceptance 2026-09-10 |
| `BLUEPRINT.md` | epos-gsx300-linux architecture design (daemon + Tauri GUI, 4 phases) | 3 agents round 3 (lian-li-linux architecture research) |
| `AGENT-FINDINGS.md` | **This file** — scoreboard + per-agent detail + latest findings | All rounds + hardware experiments |
| `_epos-tmp/` | Old workspace scaffold (Cargo.toml) — **removable** | Round 3 |

---

## 2. Agent findings summary

### Round 1 (2026-09-09) — Hardware identification & kernel RE

| Agent | Task | Key findings |
|-------|----------|----------------|
| **beru** | Collect local data (lsusb, /proc/asound, dmesg) | VID:PID `1395:0098`, serial `A003200202602692`, 4 USB interfaces, UAC1, Full Speed 12Mbps, bus-powered 100mA, CX20988 (initially mistaken for CX21988) |
| **tank A** | Chipset deep dive (Conexant CX20988) | No public datasheet; 7.1 is host-side DSP (no real 7.1 DAC); EPOS Gaming Suite = entirely software processing |
| **tank B** | USB Audio Class kernel RE | UAC1 classification, sample rate negotiation (48k/96k), feedback endpoint, 7.1 descriptor layout (UAC2), ALSA/PipeWire driver stack works natively |

### Round 2 (2026-09-09) — EPOS Gaming Suite UI / protocol feasibility

| Agent | Task | Key findings |
|-------|----------|----------------|
| **tank C** | Analyze EPOS Gaming Suite Windows UI (feature-by-feature) | All 18 features listed (volume, EQ, presets, 7.1 toggle, reverb, sound test, smart button, mic gain, voice enhancer, noise gate, sidetone, mic mute, mic test, LED, firmware, language, boot, 2nd device) |
| **tank D** | Assess per-feature Linux feasibility difficulty | Result: 7 fully possible, 8 partial, 3 impossible (LED, firmware, hw 7.1). **After 2026-09-10 acceptance: only firmware update + hw 7.1 audio processing remain unsolved** |

### Round 3 (2026-09-09) — Architecture reference (lian-li-linux)

| Agent | Task | Key findings |
|-------|----------|----------------|
| **bellion/tank E** | Analyze `sgtaziz/lian-li-linux` repo | Standard pattern: **Rust daemon + Tauri GUI + Unix socket IPC + JSON config (~/.config/app/) + udev rules (no root) + systemd user service** |
| **tank F** | Tech validation | Tauri (~12MB vs Electron 187MB), PipeWire `pw_filter` API (`pipewire` crate), `rusb` + `hidapi-rs` for USB/HID |
| **beru (bg)** | Scaffold support | 3 crates: epos-shared, epos-gsx300d, epos-gsx300-gui |

### Round 4 (2026-09-09) — LED HID protocol

| Agent | Task | Key findings |
|-------|----------|----------------|
| **tank G** | Find EPOS LED protocol documentation | NO public RE for GSX 300 LED; EPOS Gaming Suite controls LED via APO driver; Busylight BL20 (PID 0x0074) is a different device, not applicable |
| **beru (bg)** | Dump local HID descriptor | Decoded 120-byte HID report descriptor: Report 1 (consumer vol), 0x1A (absolute), 4/5 (primary cmd), 6/7 (secondary), **Report 2 vendor (0xFF13): 2 LED bits + 3 button bits** |

---

## 3. Experimental findings (hardware confirmed 2026-09-09 → 10)

> ⚠️ **Important:** these findings are NOT in the research agent texts —
> they were obtained by writing directly to `/dev/hidraw3` and observing the physical LED.

### 3.1 LED ring protocol — CONFIRMED

| Byte (Report ID 0x02 OUTPUT, vendor) | LED Effect |
|--------------------------------------|--------------|
| `0x00` | Off |
| `0x01` (bit0) | **BLUE** — stereo (2.0) |
| `0x02` (bit1) | **RED** — surround (7.1) |
| `0x03` (both bits) | **PINK** (mix) |

- Initial guesses in config defaults (`vendor_blue: 0x01`, `vendor_red: 0x02`) were 100% correct.
- Report ID 0x02 OUTPUT path: vendor (or consumer Report 0x04 payload — probe script supports both).

### 3.2 Smart button (dial click) protocol — CONFIRMED

| Report ID 0x02 INPUT value | Meaning |
|----------------------------|---------|
| `0x01` | Mode = stereo (blue) |
| `0x02` | Mode = 7.1 (red) |
| `0x04` | Long-press (>2s) |

- Device sends **state readback**, not a momentary pulse — pressing the button toggles 2.0⇄7.1 and the device returns the new state.
- Debounce quirk: rapid successive clicks only register 3/7 reports (confirmed via test).
- Volume dial: Report ID 0x01 (0x01=up, 0x02=down, 0x00=release).

### 3.3 DSP — PipeWire filter-chain (SOLVED 2026-09-10)

| Feature | Mechanism | Config path |
|-----------|--------|-------------|
| **9-band EQ** | `filter-chain` with `bq_peaking` (Freq/Q/Gain — PipeWire computes coefficients itself) | `~/.config/pipewire/pipewire.conf.d/50-epos-eq.conf` |
| **Voice Enhancer** (Warm/Clear) | Source filter-chain on the EPOS mic: Warm = boost 200/350/500Hz, Clear = 2-8kHz | `51-epos-voice-enhancer.conf` |
| **Noise Gate** | rnnoise neural suppression via LADSPA (`noise_suppressor_stereo`) | `93-epos-noisegate.conf` |

**Noise gate requirements:**
- `librnnoise_ladspa.so` (prebuilt from werman/noise-suppression-for-voice v1.21) → `~/.local/lib/ladspa/`
- LADSPA_PATH drop-in: `~/.config/systemd/user/pipewire.service.d/ladspa.conf`

**VERIFIED end-to-end:** real nodes in `pw-dump` + correct links (EQ→EPOS sink FL/FR, voice/noisegate from EPOS mic) + `pw-record` PASS + GUI band drag → config + conf file + nodes sync.

### 3.4 Daemon architecture (final)

```
epos-gsx300d (Rust, Tokio)
├── epos-hid      HID listener: smart button + volume dial (/dev/hidraw3)
├── epos-led      LedController: Report 0x02 output, hotplug reopen, Drop reset
├── epos-audio    AudioPipeline: EQ/voice/noise-gate filter-chain configs + reload_pipewire + pw-dump node resolution
├── epos-devices  pw-dump JSON parser → real node names (replaces wildcards)
├── epos-config   Config ~/.config/epos-gsx300/config.json (atomic write tmp+rename)
└── epos-ipc      Unix socket + HTTP bridge 127.0.0.1:9898 (CORS) sharing handle_request
```

**GUI:** `epos-gsx300-gui` (Tauri 2 + Vue 3) — 3 tabs Playback/Microphone/Settings, draggable EQ canvas, 3s polling (fetchStatus+fetchAudio+fetchMode), mock tracking to prevent desync.

**systemd unit:** `epos-gsx300d.service` `Wants=pipewire` (NOT `Requires=wireplumber` — avoids crash loop when pipewire restarts).

---

## 4. Acceptance status (2026-09-10)

| Feature | Status | Verify |
|-----------|-----------|--------|
| Volume control (dial, local analog) | ✅ Native | — |
| 9-band EQ + presets | ✅ SOLVED | GUI drag → live nodes |
| 2.0/7.1 toggle + LED sync | ✅ SOLVED | curl + smart button + GUI |
| Smart button (click + long-press) | ✅ SOLVED | 46 events in one test, PASS |
| Voice Enhancer Warm/Clear/Off | ✅ SOLVED | conf file changes mode correctly |
| Noise Gate (rnnoise) | ✅ SOLVED | node create/remove + pw-record PASS |
| Sidetone | ⚠️ pw-loopback | final hardware listen test pending |
| Mic gain | ✅ amixer | GUI slider sync |
| Mic mute / mic test | ✅ Trivial | no UI yet |
| LED ring (blue/red) | ✅ SOLVED | protocol confirmed |
| Firmware update | ❌ | not feasible — proprietary |
| **Real 7.1 audio processing** | ❌ | host-side EPOS APO, no Linux alternative (only HRTF/headtracking spatial as replacement) |

**Overall feasibility:** 88% → **94%** (only 2 items impossible).

---

## 5. Code repositories

| Repo | Path | Status |
|------|------|-----------|
| **slimulv1/epos-gsx300-linux** | `~/Projects/epos-gsx300-linux` | GitHub public, commits `4dd6909` → `5148adc` → `aef7301` → `70170e9` (DSP milestone) |
| PipeWire configs (A2+ + EPOS) | `~/pipewire-audio-config` | GitHub public `slimulv1/pipewire-audio-config` |

---

## 6. Cleanup

`_epos-tmp/Cargo.toml` = discarded workspace scaffold from round 3, no longer used. Recommended for removal.