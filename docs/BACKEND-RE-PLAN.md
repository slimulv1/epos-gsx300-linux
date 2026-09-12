# Backend Plan — Applying Firmware RE + Hardware Knowledge to epos-gsx-gui backend

Date: 2026-09-12
Status: PLAN (design + verification findings, no code changed yet)

> Every item below is tagged with its byte-evidence source (FIRMWARE-REPORT.md,
> HARDWARE-BOOK.md, QUICK-REFERENCE.md, AGENT-FINDINGS.md hardware tests, or
> direct disassembly). Nothing here is from memory alone.

---

## 1. Ground truth from firmware RE (used below)

| # | Fact | Evidence |
|---|------|----------|
| G1 | Device = CX21988 (Conexant) + W65C02S, firmware FREEMAN_V03.01.00.00 | version string @0xEC74, chip check LDA $1005 CMP #$08 |
| G2 | LED processing gated on mode state $137D ∈ {3,4}; device boots to state 0/1 | $CA5F-$CA68; mode write sites (1@BCFD,2@BD25,0@BB95,5@BD72) |
| G3 | Device readbacks mode via report 0x02 input: wire 0x01=stereo, 0x02=7.1, 0x04=long-press; debounce quirk drops reports on rapid clicks | AGENT-FINDINGS §3.2 hardware-confirmed |
| G4 | No absolute volume readback (incremental consumer detents only) | HID descriptor RE; hid.rs comments |
| G5 | No NVM persistence (volume/mode changes → EEPROM diff = 0 bytes) | NVM-PERSISTENCE-REPORT.md |
| G6 | Wire LED byte: 0=off, 1=blue, 2=red, 3=pink (physical LED observation) | AGENT-FINDINGS §3.1 |
| G7 | Firmware internal LED dispatch rejects value ≥3 (BCS @$CA93); 0→clear-pair, 2→CON/bit-serial, 5→clear-pair | disasm $CA6E-$CA97 |
| G8 | Memory bus = HID report 0x04 (OUT, interrupt) / 0x05 (IN) / 1KB pages; **bit6 = EEPROM write = never** | HARDWARE-BOOK.md |
| G9 | Reports 0x06/0x07/0x1A = firmware update protocol ("Download initiated/Upload completed" strings) = brick risk | FIRMWARE-REPORT.md, memory #11 |
| G10 | 5-band HW EQ exists ($0E00+ biquad init $BBD6) but NO USB control path; EQ index only via $12D0/$12D1 internal | FIRMWARE-REPORT.md |
| G11 | Capture = mono S16_LE@48k only; playback S16_LE@48k / S24_3LE@48k+96k | HARDWARE-BOOK.md / ALSA altsets |

---

## 2. Current backend flow (as-built)

```
~/.config/epos-gsx300/config.json
        │ (load at boot; mtime watch 2s)
        ▼
┌─────────────── epos-gsx300d ─────────────────────────────────┐
│ main()                                                        │
│  ├─ devices::detect() → DeviceInfo (bus/addr/alsa_card)       │
│  ├─ AudioPipeline::new + apply_full (EQ/voice/noise/sidetone) │
│  ├─ HidHandler::spawn_reader (hidraw, blocking thread)        │
│  │    → mpsc channel: ModeChanged/LongPress/VolumeChanged     │
│  ├─ LedController::new(config.led_probe) → set_mode(mode)     │
│  │    → report 0x02 packet[1]=vendor_blue(1)/vendor_red(2)    │
│  ├─ IpcState { config, audio, led, volume:AtomicI32(100) }    │
│  │                                                             │
│  ├─ smart-button task   (ModeChanged→Toggle/ToggleEq/Cycle/   │
│  │                       sidetone/noise-gate; save config;     │
│  │                       led.set_mode(new)  ← readback-trust)  │
│  ├─ VolumeChanged task  (volume ±5 clamp 0..100, host-side,    │
│  │                       display-only)                         │
│  ├─ device_hotplug_loop (5s: detect → apply_full + led.reopen  │
│  │                       + set_mode)                           │
│  ├─ config_watch_loop  (2s: audio→apply, mode→LED sync)        │
│  ├─ led_heartbeat_loop (2s: re-assert config.mode on ring)     │
│  ├─ signal handler (SIGTERM/INT → LED blue → exit)             │
│  └─ ipc::run_server (UNIX socket) + run_http_bridge (GUI)     │
└───────────────────────────────────────────────────────────────┘
        ▲ GetStatus (3s poll) / Set* requests
┌───────┴───────── epos-gsx300-gui (Tauri+Vue) ─────────────────┐
│ daemon.ts store │ DeviceView.vue (firmware="unknown")          │
│ ipc_client.rs   │ Settings/Profiles/EQ tabs                    │
└───────────────────────────────────────────────────────────────┘
```

**Confirmed-correct design choices (with RE backing):**
- LED heartbeat 2s re-assert → REQUIRED by G2 + G3 (device boots to state 0/1 where LED is ignored; readbacks get dropped). Not a workaround — a hardware requirement.
- LED clamp &0x03 → matches G7/descriptor 2-bit field.
- Readback-trust ToggleMode (never double-toggle) → correct per G3 (device toggles itself + reports new state).
- Host-side volume tracking → only option per G4.
- 9-band host EQ → only option per G10 (HW EQ not USB-controllable).
- Mono noise gate → correct per G11.
- No-suspend + 48k WirePlumber config → correct per G11.

---

## 3. Change list (each: evidence → action → risk)

### P1 — Fix LED docs ambiguity: wire value vs HID usage id
- **Evidence:** G6 (hardware: wire 1=blue/2=red/3=pink) vs HARDWARE-BOOK.md:63 + QUICK-REFERENCE.md:139 + memory #11 ("0x05=blue/0x06=red" = HID usage ids, not wire bytes). The two numbering systems are mixed in the docs.
- **Action:** Edit HARDWARE-BOOK.md + QUICK-REFERENCE.md to state both explicitly: "wire byte (report 0x02 field) 0-off / 1-blue / 2-red / 3-pink; HID usage ids are 0x05=blue / 0x06=red (usage = wire+4 offset)". Do NOT change daemon.
- **Risk:** 0 (doc-only). **Files:** docs/reverse-engineering/*.md

### P2 — Open question: pink (wire 3) reachability
- **Evidence:** G6 says pink works; G7 says internal dispatch rejects ≥3 (@$CA93 BCS). Likely resolution: pink goes through a different internal path or the value is transformed before this handler; OR the pink observation used a different report mode (probe script supports vendor + consumer path).
- **Action:** Flag in QUICK-REFERENCE as open question. Do NOT change clamp (0x03 needed for pink). Do NOT attempt to "fix" firmware. Optional targeted verify: repeat pink test isolating vendor report + capture disasm of the consumer report path.
- **Risk:** 0. **Files:** QUICK-REFERENCE.md (one note line)

### P3 — Populate real firmware version + chip-ID in DeviceInfo (read-only)
- **Evidence:** G1 + G8 (memory bus read-only via report 0x04/0x05, 1KB pages). The full read path was proven safe across the entire RE campaign (thousands of reads, bit6 never set).
- **Action:** Add small read-only HID module in daemon: request memory page containing $EC74 via report 0x04 (never set bit6) → parse response report 0x05 → extract FREEMAN version string; also read $1005 chip-ID (expect 0x08). Populate `DeviceInfo.firmware_version` + new `chip_id` field. GUI DeviceView shows real values instead of "unknown". Failure → fall back to None (graceful).
- **Risk:** LOW-MED (must reproduce exact request layout from RE dump scripts; add guard: refuse any write command variant in 0x04 payload — read pages only). **Tests:** verify against known string bytes @0xEC74; ensure request never contains bit6.
- **Files:** daemon new `hwinfo.rs`, `epos-shared/src/device.rs`, GUI DeviceView.vue.

### P4 — Volume tracker: init from persisted config instead of hardcoded 100
- **Evidence:** G4 (no readback) + G5 (device persists nothing) + code: `AtomicI32::new(100)`.
- **Action:** Add `volume: i32` (default 100) to config; on daemon start init tracker from config; persist last-known value (debounced) on each detent; on device reconnect keep host value. GUI then shows the last known position across restarts.
- **Risk:** LOW (config field add, atomic init read). **Files:** shared config.rs, daemon config.rs, main.rs, ipc.rs.

### P5 — Write-guard: hard-block firmware-update reports (0x06/0x07/0x1A)
- **Evidence:** G9 — these reports reach the firmware update path; written incorrectly = flash = brick.
- **Action:** In LedController/write path, reject any request targeting report id 0x06/0x07/0x1A (log + return error). Also explicitly validate that 0x04 payloads never carry bit6 (P3 integration).
- **Risk:** 0 (defensive). **Files:** daemon led.rs (or new write_guard helper).

### P6 — Reduce hotplug→LED convergence delay (optional)
- **Evidence:** G2 — device boots to state 0/1, LED ignored until {3,4}; hotplug loop is 5s, heartbeat 2s → after a replug, LED can lag up to ~5s.
- **Action (optional):** on detected connect, immediately re-assert LED (already done in hotplug handler) + optionally run a 500ms×6 bootstrap burst of LED writes to catch the boot window. Existing heartbeat already converges; burst only shortens the window.
- **Risk:** LOW. **Files:** main.rs hotplug loop.

### P7 — Push events over IPC (optional, not RE-driven)
- **Evidence:** shared/ipc.rs already defines `Event` enum but daemon never emits; GUI polls 3s.
- **Action (optional):** emit Event::ModeChanged/VolumeChanged/StatusChanged on the socket; GUI subscribes → real-time updates, removes 3s poll lag.
- **Risk:** LOW-MED (protocol add). **Files:** shared ipc.rs, daemon ipc.rs, GUI ipc_client.rs + daemon.ts.

---

## 4. Self-critique rounds (findings from re-verification)

### R1 — What I got wrong in my own first draft, then corrected:
1. Draft said "read $137D to learn true hardware mode for LED sync." **Rejected:** byte evidence shows $137D==3 only sets flag $9f and $137D==4 runs full dispatch — the register doesn't cleanly encode "LED state". Heartbeat remains the correct mechanism. → dropped from plan.
2. Draft said "tighten LED clamp to 0x02 because firmware rejects ≥3." **Rejected:** hardware test G6 shows pink (3) works on the wire; clamp 0x03 stays. P2 documented instead.
3. Draft treated IpcState.volume as audio volume. **Corrected:** it is device-gain *display* tracking only (device applies gain locally). P4 therefore only changes init/persistence, not audio mixing.

### R2 — Things the plan does NOT touch (and why):
- 9-band EQ structure (G10 forbids HW EQ attempts; host EQ is the only lever).
- Noise gate mono (G11).
- Smart-button readback trust (G3).
- WirePlumber/PipeWire no-suspend configs (already correct for G11).
- Any firmware write path / bit6 (brick risk, G8/G9).

### R3 — Residual risks:
- P3 is the only change with any hardware interaction; mitigated by read-only page requests + fallback + full RE precedent (safe). No change is made until the exact request layout is re-confirmed from dump scripts.
- P2 unresolved → documented, no action.

---

## 5. Status — IMPLEMENTED (2026-09-12, same day)

P5 ✅ (led.rs: doc header wire-values + bit6 guard in write_primary_report + write_vendor_report comment)
P4 ✅ (config.rs `DeviceConfig.volume: Option<i32>`; main.rs init from config + 2s debounce-persist; verified: knob → config.json, GetStatus `volume:90` after restart)
P3 ✅ (new hwinfo.rs: read-only memory-bus probe RAM $EC74 version + $1005 chip-ID; O_NONBLOCK + 2s timeout; merged into GetDevice; verified live: `Firmware version: FREEMAN_V03.01.00.00`, `Chip ID: 0x08`, IPC `"firmware_version":"FREEMAN_V03.01.00.00"`)
P1 ✅ (HARDWARE-BOOK + QUICK-REFERENCE: split wire-protocol table (0x00-0x03) vs firmware-internal dispatch table (0/2/5/≥3), usage-id note, open-question pink)
P2 ✅ (open-question documented in both docs)
P6 ❌ skipped (not requested; heartbeat converges fine)
P7 ❌ skipped (not requested, protocol change)

Verified: cargo build --release PASS; smoke test daemon manual run (LED red/Surround OK, clean shutdown); systemd service restored active; **all hardware ops read-only — zero bit6, zero brick risk.**

## 6. Original next steps (pre-implementation)
1. User reviews plan (this document).
2. Approve scope: P1+P2 (docs) / P3 (hwinfo) / P4 (volume persist) / P5 (guard) / P6+P7 (optional).
3. Implement in order P5→P4→P3→P1/P2 (safety first, then behavior, then docs).
4. Verify each with functional smoke tests (daemon log, GUI display, physical LED) — no firmware writes ever.
5. Commit + push after explicit user confirmation (rule #24).