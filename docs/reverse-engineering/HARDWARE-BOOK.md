# EPOS GSX 300 — Hardware Book

**Reverse engineering documentation — 100% read-only methodology**
Phase 6 (P6) findings, compiled 2026-09-12.
Owner: Magnus (slimulv1). Repo: [epos-gsx300-linux](https://github.com/slimulv1/epos-gsx300-linux)

> ⚠️ **SAFETY WARNING**: All findings here were obtained via READ-ONLY probes.
> Never set bit 6 of the memory-bus command word (EEPROM write enable) and
> never probe HID reports 0x06/0x07 with write intent — they carry a
> firmware-flash protocol ("Download initiated..", "Upload completed - power
> cycle device", "Upload error" strings confirmed in ROM). A wrong write can
> brick the device. The EEPROM contains 7x redundant firmware copies with
> wear-leveling — a partial write there is NOT recoverable from userspace.

---

## 1. Device Identity

| Property | Value |
|----------|-------|
| Product | EPOS GSX 300 (gaming DAC/amp) |
| Chip | Conexant **CX21988** (ERP ROM) |
| CPU | **W65C02S** (WDC, 65C02 with RMB/SMB/BBR/BBS bit ops, WAI/STP) |
| Chip ID register | $1005 = 0x08 (quirk base 0x21980 + 8) |
| Firmware build | **FREEMAN_V03.01.00.00** @ $EC74 |
| Firmware patch | `Sennheiser_EntryDAC_Patch_44-05-62_Rev_0062_CX21988_NVM-` |
| Patch version | 44.05.62 (@ $0022: `44 05 62`) |
| Memory bus | HID interrupt reports 0x04 (OUT) / 0x05 (IN), 1 KB pages |
| EEPROM | 128 KB (0x0000–0x1FFFF), read via bus, 7x FW copies (wear-leveling) |
| RAM | 64 KB (0x0000–0xFFFF, wrap-mirror ≥0x10000), read via bus |
| USB VID:PID | 1395:0098 (vendor class, HID) |

## 2. USB Audio Profile (host-visible)

| Direction | Format | Altsets | Notes |
|-----------|--------|---------|-------|
| Playback | S16_LE @ 48 kHz | 2 | + S24_3LE @ 48/96 kHz |
| Capture | S16_LE mono @ 48 kHz | 1 | Hardware mono-fallback (no stereo mic) |

- **44.1 kHz is NOT native** — keep 48 kHz as global clock. rnnoise also requires 48k.
- 96 kHz exists but global clock is 48k (A2+ config); leaving 48k is correct.
- **Sidetone** is device-resident (REVERBERATION), not host-processed.
- **EQ is host-side.** The chip HAS a 5-band hardware EQ internally (see §6),
  but it is preset-only and NOT exposed via USB. 9-band LADSPA host EQ is the
  correct design for Linux.

## 3. HID Protocol (decoded)

### Report 0x01 — Volume dial (input, incremental)
- 3-bit dial: `0x01` = up detent, `0x02` = down detent, `0x00` = release.
- Consumer codes 0x09E9/0x09EA/0x09CF.
- **NO absolute readback, NO volume register anywhere** (proven by exhaustive
  EEPROM+RAM diff tests, Phase D2/D2c/P6-retest — 0 bytes ever changed).
  → Host-side tracking ±5/detent, init 100, clamp 0–100 is the ONLY correct design.

### Report 0x02 — LED / mode (vendor 0xFF13, output)
Firmware handler decoded @ $CA47 (entry `LDX #$02`)–$CA97. LED value byte = byte[6] & 0x1F.
Two distinct layers must not be confused:

**Wire protocol (hardware-confirmed, AGENT-FINDINGS §3.1):** report 0x02, byte[1] raw value:

| Wire value | LED (physical) |
|------------|----------------|
| 0x00 | Off |
| 0x01 | Blue (stereo) |
| 0x02 | Red (7.1) |
| 0x03 | Pink (both bits) |

> Values >0x03 are clamped by the daemon (& 0x03). The LED field is 2 bits wide in the
> report descriptor; usage-ids 0x05/0x06 in some earlier docs are HID *usage numbers*
> (usage-min offset +4), NOT wire values — the wire byte is 0x01/0x02.

**Firmware internal LED dispatch value (RE decode, handler entry $CA47):** the firmware
re-reads its own copy of the report (format gate byte[2]=0/[3]=0/[6]=2/[7]=0, mode gate
$137D∈{3,4}) and dispatches on its internal value:

| Int. value | Firmware path |
|------------|---------------|
| 0 | $CA85: clear $1388/$1389 pair (off) |
| 2 | CON path (byte[4]&0x0F) — red-ish/surround |
| 5 | $CA85: clear $1388/$1389 pair (blue path, mode≠4) |
| ≥3 (dispatch) | rejected — `BCS` @$CA93, SMB0 $9F flag only |

> Open question: hardware test confirms wire 0x03 = pink works, yet the internal
> dispatch rejects values ≥3 at $CA93 — the wire→internal mapping (and the pink path)
> is not fully closed; do NOT change the daemon clamps on this basis.

**LED is a shift register, not direct GPIO:**
- $1388/$1389 = 2-bit LED shift-register pair, built bit-serial:
  `CLC → BBR4 $A1 → SEC → ROL $1388` (bit 0), then `CLC → BBS5 $4F → SEC → ROL $1388` (bit 1).
- Terminated by `LDA ($88) → RTI` — hardware write through ZP pointer $88/$89.
- Complete chain: $CA87–$CAEC, self-contained.
- LED dispatch table $ECAB: 16 entries via `ASL A → TAX → JMP (,X)`.
  [0]=$CA9A off, [1]=$CAAC, [2]=$CAAE, [3]=$CCA4, [4]=$CCD9, [5]=$CD1B.

### Report 0x04/0x05 — Memory bus (vendor)
- OUT 0x04 / IN 0x05, interrupt path only (control path = Broken pipe).
- 1 KB page reads. 0 read errors over 128 KB EEPROM + 64 KB RAM.
- **bit 6 = EEPROM write enable — NEVER SET in this project.**

### Report 0x06/0x07/0x1A — Undecoded (do NOT probe with write intent)
- 0x06 = OUT 36 B, 0x07 = IN 32 B, 0x1A = unknown.
- ROM strings confirm a firmware-update bus here:
  "Download initiated..", "Upload completed - power cycle device", "Upload error" @ $EBEB–$EC6B.
- **Treat as flash-write risk. Avoid.**

## 4. Firmware Architecture

### 4.1 Memory map (64 KB RAM view)
| Region | Contents |
|--------|----------|
| 0x0000–0x1FFF | Data/vectors, RAM runtime handlers ($1400–$1A00), config |
| 0x2000–0x3FFF | Zeros (RAM data area), unused |
| 0x4000–0x5FFF | Strings |
| 0x6000–0x9FFF | Unprogrammed 0xFF / BRK (not code) |
| 0xA000–0xFFFF | Main firmware code (~24 KB) + ROM tables |

Code stability (verified across 3 boot + 1 runtime dumps): 0x2000–0x2FFF and
0x6000–0xFFFF are IDENTICAL between boot and runtime. 0x0000–0x1FFF and
0x3000–0x5FFF hold dynamic data.

### 4.2 Boot chain
```
ROM stub → HW init → config stream (56 bytes hex, @$B7D0):
  write registers $1001/$1003/$1004/$1006
  copy NVM config pairs → $126A–$127B
  patch handler vector $1238 → JMP $B3BE (console char parser)
→ runtime
```
- Reset entries: NMI=$A0A3, RESET=$A000, IRQ=$A003; classic SEI/LDX#$FF/TXS.
- Console/CLI exists on-chip: hex output $A5C9/$A5D4, char output $A5A5,
  CR/LF $A5BE/$A5C2, line parser $B500–$B560, hex reader $B5CA/$B64B.

### 4.3 Command handler vector table ($1238, 10 slots)
`JSR $1238` dispatches into handler slots. **3 slots are SELF-PATCHING during
boot** (multi-stage init converges: $A13F→$A203→$A2C7→$A321):

| Slot | Addr | Handler | Type |
|------|------|---------|------|
| 0 | $1238 | $B3BE console char parser | patched at boot end |
| 1 | $123B | $B458 | static |
| 2 | $123E | **$A321 → $A33F → $A1D2** | self-patched (timer chain) |
| 3 | $1241 | **$A3ED** (self-patches $1242/$1243) | self-patched |
| 4 | $1244 | **$A4A4** | self-patched |
| 5 | $1247 | $E4FD | static |
| 6 | $124A | $BD72 (mode-5 / EQ-active) | static |
| 7 | $124D | $C98A | static |
| 8 | $1250 | $D34B | static |
| 9 | $1253 | $E9BA | static |

- $A321 = **16-bit soft timer** ($C5/$C6 down-counter; expired → set flag $96 bit2).
- $A33F self-patches slot 2 → $A1D2 (lifecycle continuation).
- $B47E = boot-end: RMB0 $93 → patch $1239/$123A → $B3BE → RTS.

### 4.4 Mode state machine ($137D, 6 states) — cooperative scheduler
Each mode routine pre-installs the next handler into $124B/$124C
(linked list) + flag $93 (RMB2 close / SMB2 arm — opcode 0x27/0xA7, 2026-09-12 verified).

| Mode | Set at | Meaning |
|------|--------|---------|
| 0 | $BE5F | **Idle / EQ-off**: clear $1386/$1387, JSR $D564, install $BB95; dispatcher $BE41 auto-advances → mode 1 |
| 1 | $BCFD | Setup — pushes $BD25 (mode 2) or $BB95 (reset path) |
| 2 | $BD25 | Intermediate |
| 3 | $BD35 | **LED-active gate** — report 0x02 processed only in {3,4} |
| 4 | $CC5C | Config-high: byte <$80 → $137F; =0 → back to $BD35; ≠0 → mode 4 |
| 5 | $BD72 | **EQ-active**: mode=5 + SMB2 $A3, gates EQ chain $D564 |

⇒ Device boots into mode 0/1 and only reaches {3,4} later. **LED reports sent
too early are dropped** — the daemon's LED re-assert after hotplug is required.

### 4.5 Register map (firmware-visible)
| Addr | Role |
|------|------|
| $1001/$1003/$1004/$1006 | HW config registers (boot loader) |
| $1005 | Chip ID (0x08 = CX21988) |
| $1016 | USB-side config state — read-only, mirrored by syncer $E001 → $0F2A/$0F2B/$0F20 |
| $0D00 | DSP config mirror (committed from $137F @ $BD62) |
| $0D03 | DSP config bitmask (bit1 EQ related) |
| $0D08 | DSP state class (01 basic / 02 mode2 / 04 EQ-applied) |
| $0E00–$0E40 | 5-band coefficient area, stride 0x10 (band + 3-byte coeffs + state + enable) |
| $0E1C/$0E1D | EQ coefficient write target (÷2 from ROM banks) |
| $0F13 | Low nibble = EQ2 ACTIVE index (committed from $12D0 @ $C04C) |
| $0F1D | bits 0–2 = EQ1 index (committed from $12D1 @ $DA6F) |
| $0F44 | Control register (AND #$3F clear / ORA #$C0) |
| $0F70/$0F71 | EQ control pair: AND #$BE (disable), ORA #$41 (EQ1 enable), AND #$FC / ORA #$03 (EQ2 enable/disable) |
| $0FB8–$0FC7 | 16-byte I/O buffer (encoder/HID) |
| $12D0/$12D1 | EQ target index (from HID) — $12D1 single write site $BCF9 |
| $124B/$124C | Mode handler pointer (linked list) |
| $124E/$124F | Second handler pointer (mode dispatch) |
| $126A–$127B | NVM config mirrors |
| $137D | Mode state (0–5) |
| $137F | Config byte from report (<0x80) |
| $1384/$1385 | EQ data from HID report |
| $1386/$1387 | EQ1/EQ2 enable flags (mode 0 clears both) |
| $1388/$1389 | **LED shift-register pair** |
| $13B2 | Delay countdown (10 ticks, index 8 → 5) |

### 4.6 EQ apply chain (host → DSP)
```
I/O buffer $0FC7/$0FC6 → 16-bit delta (SEC/SBC, BPL/ADC wrap)
→ $12D0/$12D1 target
→ commit: $0F13 low-nibble (EQ2) / $0F1D bits0-2 (EQ1)
→ ROM preset lookup: $F76E (EQ2, 1-byte) / $F779 (EQ1, 1-byte)
→ coefficient banks: $12D1×2 → $F663/$F673/$F693/$F6A3 → ÷2 → $0E1C/$0E1D
→ enable: $0F70 ORA #$41 / $0F71 ORA #$03
→ DSP state $0D08 (01/02/04)
```

#### 4.6.1 EQ coefficient banks — mathematically fixed presets
| Bank | Addr | Values (first 8) | Relation |
|------|------|------------------|----------|
| A | $F663 | 20 2C 40 58 60 80 B0 C0 | base |
| B | $F673 | 24 30 44 5C 64 84 B4 C4 | A + 4 |
| C | $F693 | 30 42 60 84 90 C0 108 120 | A × 1.5 |
| D | $F6A3 | 36 48 66 8A 96 C6 10E 126 | C + 6 |

⇒ Preset-only envelopes. **Not modifiable via USB. Host-side EQ is the only
path — confirmed by firmware-level decoding, not just inference.**

### 4.7 $BBD6 — EQ DSP init ($BB95 reset path)
```
LDA #$00 → STA $08B7/$0F50
LDA #$0F → STA $08B8
RMB2 $A3
LDA $0D03 / AND #$02 / STA $0D03
```

## 5. Persistence (proven by diff-tests)

| Test | Action | EEPROM bytes changed | Verdict |
|------|--------|---------------------|---------|
| D2 | Volume knob 7 detents (100→45→55) | 0 | Volume does NOT persist |
| D2c | Smart button 3× (Stereo→7.1→Stereo) | 0 | Mode/LED does NOT persist |
| Retest B | Volume knob, full 64 KB RAM diff | 152 (noise) | No usable volume register |
| Retest C | Unplug 3 s + replug after knob | 0 | Nothing persists even at power loss |
| Retest D | Mode toggle + unplug/replug | 0 | Nothing persists |

⇒ All device state is **volatile**. The daemon's host-side volume tracking (±5/detent)
and LED re-assert heartbeat are the correct and only designs.

## 6. What This Means for epos-gsx300d

| Daemon feature | Firmware truth | Verdict |
|----------------|----------------|---------|
| Volume ±5/detent tracking | No readback exists anywhere | ✅ Correct |
| LED re-assert heartbeat | Mode {3,4} gate + volatile state | ✅ Required |
| 9-band LADSPA EQ | HW EQ preset-only, USB-locked | ✅ Correct |
| Noise gate (mono, rnnoise) | Captures is genuinely mono | ✅ Correct |
| 48 kHz global | Chip native (no 44.1k) | ✅ Correct |
| Sidetone device-resident | REVERBERATION in DSP | ✅ Correct |
| LED clamp ≤0x03 | Firmware ignores >0x03 | ✅ Correct |

## 7. Tooling

- `firmware-re/recdis.py` — recursive W65C02S disassembler (206 opcodes,
  full ZP/abs/indirect/bit-op tables, 10,447-line output).
- Dump scripts: memory bus via HID 0x04/0x05 (report files + page loop),
  auto-detect EPOS hidraw via `/sys/class/hidraw` uevent (replug-safe).
- All dumps stored in `eeprom-dumps/` (boot/runtime/RAM/EEPROM variants).

## 8. Open Questions / Not Decoded

- Reports 0x06 (OUT 36 B) / 0x07 (IN 32 B) / 0x1A — likely EPOS Gaming Suite
  profile-write bus (firmware flash protocol). **Deliberately not probed.**
- String region 0x4000–0x5FFF contents (partial decode only).
- Full console/CLI command set (parser found, command table not enumerated).