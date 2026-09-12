# EPOS GSX 300 Firmware RE — Phase 1 Report

Date: 2026-09-12
Method: 100% READ-ONLY — memory bus interrupt path, never set bit6/write. No flash writes ever.

## Dumps used (in eeprom-dumps/)
- ram-64k-vol0.bin / vol50 / vol100 — runtime RAM 64KB
- ram-boot-1/2/3.bin — boot-time dumps (~2s after replug)
- eeprom-baseline-20260912.bin — 128KB EEPROM NVM
- eeprom-baseline-A.bin — pre-volume-turn baseline (md5 6b1d7ea8)

## Firmware identity
- **CPU: W65C02S** (CMOS 65C02; undefined opcodes 0xC2/0xE2/0x0F etc. = NOP, extensions: BRA/STZ/TSB/TRB/INA/DEA/PHX/PHY/BIT#imm/SBC(zp))
- **Firmware build string @0xEC74: `FREEMAN_V03.01.00.00`** ← version string (project codename FREEMAN)
- Bootloader strings @0xEBEB–0xEC6B:
  - `(C) CONEXANT`   @0xEBD5 (handshake/copyright)
  - `Download initiated ..` @0xEBEB
  - `Upload completed - power cycle device` @0xEC35
  - `Upload error - Try again` @0xEC5B
  → Device HAS a firmware update (Download/Upload) protocol = the 0x06/0x07 vendor bus we avoided probing.

## Memory map (verified via 4-dump stability analysis)
- Code (stable, dense): **0x1400–0x1A00** (vector/jump tables) + **0xA000–0xFFFF** (main firmware ~24KB)
- 0x6000–0x9FFF: BRK(0x00)/FF fill — unused
- Data (dynamic): zero-page, 0x400–0x6FF, 0x900–0xBFF, 0x3200–0x5FFF (strings)
- Boot EEPROM 8KB image @ file 0x805: `JMP $2805` stub → chip reg $0894 control → `JMP $A875` ABSOLUTE (opcode 4C `4c 75 a8`, NOT $6C indirect — byte-verified) into RAM firmware

## Vectors
- RAM @0xFFFA: NMI=$A0A3, RESET=$A000, IRQ=$A003
- Reset entry: `SEI / LDX #$FF / TXS` @0xA008; early `LDA $1005 CMP #$08 BEQ` = CX21988 chip-ID check in firmware
- $A090: stack dump routine (LDA $0100,X → JSR output → INX → BNE) = debug console dump

## Console / CLI discovery
- **$A5C9** = hex nybble output (LSR×4 → JSR $A5D4 → AND#$0F → CMP#$0A → ADC#$06 → out) — ASCII hex converter
- **$A5A5** = char output; **$A5BE/$A5C2** = CR/LF out; **$B687** = space out
- **$B5CA** = hex BYTE reader (2 hex chars → ASL×4 → ORA combine)
- **$B500-$B560** = command line parser: reads char, CMP #$0D (CR=end), CMP #$20 (SP=token sep), JSR $B5CA/$B617 = token dispatch
- $B5F1: `4c e5 ea` = `JMP $EAE5` ABSOLUTE (not indirect); $B611: `20 17 b6 4c 7a b6` = `JSR $B617` + `JMP $B67A` absolute — no 0x6C indirect at either address (byte-verified)

## Chip registers seen in code
- **$0894** — control register (boot stub AND #$C3 / ORA mask; LED-adjacent: STA $4b + ORA $4b pattern)
- **$1005** — chip-ID (8 = CX21988 quirk)
- **$04E2–$04E6** — I/O context block (used by EB22 output routine)
- **$1282 / $12D7 / $12DB / $12D8** — block copy control (D3A0 upload/download code region)
- **$40/$41/$42/$43/$46/$47** — zero-page pointer pairs (string ptr, hex acc, block ptr)

## Key routines
- $A9D6 — string compare handshake: loops 13 chars (CPY #$0D) vs "(C) CONEXANT", SEC=pass / CLC=fail
- $D3A0 — block-copy / buffer manager (uses $12D7 status, $1282 buffer, size $12D8)
- $EB22 — writes to $04E4,X context + delay loop (LDX #$3A DEX BNE) = busy-wait

## Next steps (Phase 2)
1. Proper recursive disassembler with W65C02S opcode table → resolve all JSR/JMP targets from A000 + vectors (no linear misalignment)
2. Locate command dispatch table by cross-referencing the 0x1400 vector entries ($C8E2, $6B33, $A21F, $D5E8, $E7F6, $D9E5...) with CLI parser
3. Find HID report handler (0x01 volume / 0x02 LED) in firmware → compare with daemon protocol decode
4. Map uplink protocol strings to 0x06/0x07 report handlers (read-only mapping only — NEVER probe write)
5. Search for DSP/audio register region (EQ/sidetone hardware control)

## Phase 7b (2026-09-12): LED handler chain + mode state machine + 5-band HW EQ init

### Full LED HID handler (3-tier dispatch table)
1. **$EC89** (LDX#0) → 16 primary entries
2. **$EC91** (LDX#1) → 13 entries (26 bytes of 16-bit pointers, 2026-09-12 byte-count corrected from "26 entries")
3. **$ECAB** (LED value ASL→TAX) → 16 LED modes: [0]=$CA9A off, [1]=$CAAC, [2]=$CAAE (long mode-2 handler), [3]=$CCA4, [4]=$CCD9, [5]=$CD1B

### LED bit-serial write (KEY FINDING)
- **$1388/$1389 = LED shift-register pair** — CLC/BBR→SEC + ROL $1388 (SPI-like bit-banging)
- **$137D ∈ {3,4} = LED processing gate** (only these 2 modes process LED!)
- $A1 = LED pattern flag byte (BBR4 reads @CA9B `4f a1 01`, RMB2 clears @CB2D)
- $9F = LED state flag (SMB0 set on mode change)
- Blue path: value==5 → clear $1388/$1389 → JMP $ECAB
- Termination: LDA ($88) + RTI (pointer hardware write)

### Mode state machine ($137D + callback $124B/$124C)
- $BCFD: mode=1, handler ptr=$BD25
- $BD25: mode=2, handler ptr=$BB95
- $BB95: mode=0, + JSR $BBD6 (init) + thunk set
- $BD72: mode=5
- $BBD6 = **INIT ROUTINE**: initializes 5-band EQ coefficients:
  - Bands at $0E00/$0E10/$0E20/$0E30/$0E40 (3-byte biquad coeff per band: $xx00/$xx01/$xx02)
  - State pairs at $xx0C/$xx0D (biquad state), enable $xx0E
  - $0D00/$0D02-$0D0F/$0D2E/$0D2F = EQ config/state
  - $1384-$1387 = additional state (cleared at $BCE4)
  - $12D0/#12D1 = last-command (init 7/2)
- $BCF9: STA $12D1 = write last-command reg

### Updated register map (additions)
| Reg | Role |
|-----|------|
| $1388/$1389 | LED shift pair |
| $137D | mode gate (LED only 3/4) |
| $124B/$124C | mode callback ptr |
| $1239/$123A | handler ptr (alt) |
| $0E00-0E4E | **5-band EQ: coeff+state+enable** |
| $0D00-$0D2F | EQ config/state frame |
| $1347 | chip-ID store ($1005 copy) |

### KEY TECHNICAL CONCLUSION
**The CX21988 HAS a 5-band hardware EQ in firmware** (biquad coefficient stores at $0E00-$0E4E) — but the daemon's host-side 9-band EQ is a different thing (LADSPA bq_peaking). The chip HW EQ is NOT exposed via USB (daemon cannot reach it) — confirms memory #11: "chip 5-band HW EQ not exposed via USB".

## Phase 7c (2026-09-12): EQ config frame + control register map

### (a) EQ/state frame — $1384-$1387 + $0Dxx
- $1386/$1387 = **EQ enable flags** (set to 0 at $BE61/$BE64 on mode-bypass, read at $D5AC/$D573 in the EQ apply routine $D564)
- $1384 (STA at $CEB6, HID handler), $1385 (STA $1385,Y at $CE6A — indexed write from the report byte!)
- $12D0/$12D1 = last-command regs → dispatch LDX → JSR $DA8C/$DA6F (EQ command table!)
- EQ apply chain: $D564 (EQ0 check) → $D573 (EQ1: mode==5 gate) → $D5A1 (EQ2: $1386)
- $BE5F: bypass EQ → clear $1386/$1387 → JSR $D564 → mode0 ($BCFD path)

### (b) Control registers
- **$0F70/$0F71 = control pair** — set bits 0-1 (AND #$FC clear, ORA #$03 enable at $D897/$D89C)
- **$0F44 = control register** (5 refs: $E0D2/$E3CF/$E407/$E599) — AND #$3F clear / ORA pattern (LED-control region)
- **$0F1B = DAC/IO control** — ORA #$88 (enable) / ORA #$33 (enable) — $E167/$E174 helpers
- **$0F2A/$0F2B/$0F20 = mode alias triple** (synced to the same value from $1016)
- **$1016 = USB config register** (low nibble = mode ID, read at $E001)
- **$E001 = mode-alias syncer**: $1016 nibble → copied into $0F2A/$0F2B/$0F20 (shift 2)
- **$E06E = value clamp helper** (5..0xB6) — volume/dB clamp
- **$E082**: uses $A3 bit1 → clamp 0xB1

### (c) Complete register map (merged)
| Reg | Role |
|-----|------|
| $1016 | USB config mode ID |
| $0F2A/$0F2B | mode nibble mirrors |
| $0F20 | mode bits 2-5 shifted |
| $0F70/$0F71 | control pair (bits 0-1 enable) |
| $0F1B | DAC/IO enable ($E167/$E174) |
| $0F44 | LED-area control AND #$3F |
| $1384-$1387 | EQ enable flags |
| $12D0/$12D1 | EQ command dispatch |

## Phase 7d (2026-09-12): Boot dump — self-patching vector table $1238

### Experiment
Stop daemon → pkexec unbind 1-7 → bind → 3 consecutive RAM dumps (0 errors) → compare against runtime (ram-64k-vol0.bin).

### Stability results
- **100% IDENTICAL**: 0x2000-0x2FFF, 0x6000-0xFFFF (boot vs runtime + across the 3 boots)
- **Dynamic**: 0x0000-0x1FFF (38-167B), 0x3000-0x5FFF (32-43B)
- Vector table @1400, $1A50 runtime, $EC89 LED dispatch, $BBD6 EQ init, $A000 code: **SAME** in every dump

### 🔑 DISCOVERY — Thunk table $1238 = 10-slot command handler vector, SELF-PATCHING

```
slot0 @1238: JMP $B3BE   (static)
slot1 @123b: JMP $B458   (static)
slot2 @123e: →boot1 $A13F →boot2 $A203 →boot3 $A2C7 →runtime $A321  (DYNAMIC!)
slot3 @1241: →$A405 →$A3E0 →$A384 →$A3ED  (DYNAMIC!)
slot4 @1244: →$A4A4 →$A44F →$A48E →$A4A4  (DYNAMIC!)
slot5 @1247: JMP $E4FD   (static)
slot6 @124a: JMP $BD72   (static)
slot7 @124d: JMP $C98A   (static)
slot8 @1250: JMP $D34B   (static)
slot9 @1253: JMP $E9BA   (static)
```

- **Only JSR entry points into slots**: $A124→slot2, $A150→slot0, $A35A→slot3, $A415→slot4, $A8BF→slot1, $A8CD→slot5, $A8E4→slot9, $A907/$A946→slot6, $A910/$A94F→slot7
- **Handlers patch the table themselves**: $A3ED/$A3F8/$A419 write pointers into $1242/$1243 (slot3 target) and $1245/$1246 (slot4 target) — `LDA #lo/STA $1242/LDA #hi/STA $1243/RTS`
- Slots 2/3/4 converge to $A321/$A3ED/$A4A4 = **2-level dispatch: ROM code patches RAM vector → calls deeper handler**

### Decoded handlers
- **$A321** (final slot2): LDA #$3F/STA $123F/LDA #$A3/STA $1240 = patches ITSELF (target $A33F); manages down-counters $C5/$C6 (DEC, SMB2 $96 on reaching 0)
- **$A33F**: patches $123F/$1240 → target $A1D2 (BBS1 $99 chain)
- **$A3ED** (final slot3): `LDA #$F8 / STA $1242 / LDA #$A3 / STA $1243 / RTS` — the `F8` byte is the LDA #immediate OPERAND, not a SED opcode (2026-09-12 disasm-verified); patches slot3 → $A3xx
- **$A3F8**: patches slot3 → $A405
- **$A419**: patches slot3 → $A426
- **$A431**: patches slot4 target ($1245/$1246) → $A44F; BBS0 $95 → counts down $CD/$CE
- **$A415**: JSR $1244 (slot4) → RTS = call-through wrapper

### Conclusion
The firmware uses a **self-modifying dispatch table** in RAM ($1238-$1255): routines in the $A3xx region install each other by writing JMP targets directly into the table. Boot runs multi-stage: each dump captured a different stage (A13F→A203→A2C7→A321). This is a Conexant dispatcher-table pattern — handlers are installed via pointer patching, not hard vectors.

### Appendix: raw slot-region bytes (boot3)
`1238: 4c be b3 4c 58 b4 4c c7 a2 4c 84 a3 4c 8e a4 4c fd e4 4c 72 bd 4c 8a c9 4c 34 d3 4c ba e9`

## Phase 7e — 5-band EQ firmware: NOT usable via USB (final verdict)

### EQ apply chain decode (from boot-dump-3)

```
$D564: JSR $1486→JMP $166D (gate) / BBR6 $93 → RTS   ; flag $93 bit6 (0x6F=BBR6)
$D56B: LDA $137D / CMP #$05 / RTS                     ; ONLY applies when mode==5
$D573: LDA $1387 → BNE → EQ2 path                     ; $1387 = EQ2 enable
       else JMP $D5A1
$D57E: JSR $143B / JSR $D8B4 (EQ2 setup) / LDX $12D0 → JSR $DA8C
$D5A1: BBR1 $A2 → JSR $DAB2 (EQ2 disable) / LDX #$20 loop
$D5AC: LDA $1386 → BNE → EQ1 path                     ; $1386 = EQ1 enable
$D5B4: JSR $143E / JSR $D956 (EQ1 setup) / LDX $12D1 → JSR $DA6F
$D5DA: LDA $0F1D AND #$07 CMP $12D1                   ; mode-nibble compare
```

### The 2 EQ path structure (EQ1/EQ2 = stereo L/R!):
- **EQ2** ($D8B4/$DA8C): `LDA ,X$F76E` (ROM preset table) → $D3 → JSR $D894 (mask $0F71)
- **EQ1** ($D956/$DA6F): `LDA ,X$F779` (ROM preset table) → $D5 → JSR $D8A4 (mask $0F70)
- **$12D0 = EQ2 index, $12D1 = EQ1 index** (from HID report bytes)
- **$0F13 low-nibble = EQ2 state** ($DA8C: AND#$F0|ORA), **$0F1D low-3 = EQ1 state** ($DA6F: AND#$F8|ORA)

### ROM preset tables:
```
$F76E (EQ2): 0c 00 06 00 00 06 04 04 02 02 02 0c 00 03 00 00 03 02 02 00 40...
$F779 (EQ1): 0c 00 03 00 00 03 02 02 00 40 00 00 00 00 00 00 00 03 00 04 03 0b 02...
```
- Values share the same pattern (0x00-0x40 range) = **fixed per-preset gain levels**

### RAM translation (runtime-built):
```
$16A8 (RAM thunk) → $16BD: LDX $12D1 / LDA $16C6,X / JMP $D964
$16C6 (RAM table): 12 00 0a 00 00 09 03 03 33 6b 00 28 bb 03 17 c2 ...
   = index-translation table created at boot (corresponds to $0E00-$0E40 banks)
```

### VERDICT — 5-band HW EQ cannot be driven over USB:
1. EQ index goes only through **$12D0/$12D1** — set from internal HID reports, not exposed via report 0x04/0x05 (the read-only bus)
2. Coeffs are limited to ROM presets ($F76E/$F779), **NOT arbitrary biquads** — the daemon's host-side 9-band LADSPA is more flexible
3. Driving it would require writing the memory bus (bit6) = **brick risk, never tested**
4. **Conclusion: keep the current design (host-side 9-band LADSPA) — it is correct**

### Bonus findings:
- **$1000-$100F = firmware-writable regs**: $1005 chip-ID (STA at E9D8-EA80), $1001/$1003/$1004/$1006 init (B7DB-B7F2), $100A/$100B watchdog counters (INC to 5 / DEC to 0xB7 at E780-E7A6), $1009/$100E/$100F event reads
- **$1015-$1017 = READ-ONLY hardware state** (0 firmware writes) — $1016 = mode/state from USB config
- **$E001 syncer**: $1016 low-nibble → mirrored to $0F2A (nibble), $0F2B (nibble), $0F20 (<<2 shift) — hardware state mirror, invoked through a vector (no direct caller)
- **$A321 = 16-bit soft timer** ($C5/$C6): ticks via slot2 dispatch, gated by flag $93 bit2, expired → SMB2 $96; $17BD→$A121 chain loads the timer callback from RAM runtime

## Phase 7f — EQ index source + mode dispatch + DSP config commit

### $12D0/$12D1 (EQ index) source:
- **$12D0** (EQ2 target): read at $C040/$C045 (compare + load), $D589 (LDX → $DA8C). Compared against $0F13 low-nibble (active) + input $40
- **$12D1** (EQ1 target): **ONLY ONE write site: $BCF9 (STA $12D1 / RTS)** — set via thunk vector $1238 slot0($B3BE?) or the $124B/$124C pointer chain. Read at $16BD (RAM trans), $C1FC, $C2DC, $D5BD, $D5DA, $D95E

### EQ2 apply tick ($C03B-C058):
```
c03b: LDA $0f13 / AND #$0f        ; active EQ2 index (low nibble $0F13)
c040: CMP $12d0 / BEQ skip        ; target == active → no-op
c045: LDA $12d0 / CMP $40         ; verify vs input buffer byte
c04c: STA $4b / LDA $0f13 / AND #$f0 / ORA $4b / STA $0f13  ; COMMIT index
c058: JSR $d8b4                   ; EQ2 setup (ROM preset $F76E)
```
→ **$0F13 low-nibble = EQ2 ACTIVE index, $12D0 = pending target**

### Encoder/volume position processor ($C067-C0BF):
- $0FBC/$0FBB/$0FB8/$0FBE = I/O buffer reads (mask $07/$C0) — **encoder + button report data**
- $C07F: $0FBE AND #$C0 CMP #$C0 → if equal: ASL $44/ROL $45 (scale ×2)
- $C08C: LDA $0FC7/$0FC6 or $C09A: $0FC5/$0FC4 → 2 position-candidate pairs
- $C0A6: SEC / SBC 16-bit delta (new - old) → $C0B3 BPL / $C0B5 ADC wrap (negative wrap)
- $C0C1: JSR $1462 (= RTS noop — empty hook)
- $C0D1: LDA $0F71 / ORA #$03 / STA $0F71  ← **EQ ENABLE commit**
- **$C0EA: LDA #$04 / STA $0D08** ← DSP config direct write!

### DSP config region $0D00-$0D08:
| Addr | Refs | Meaning |
|------|------|---------|
| $0D00 | CMP/STA @BD67/$BD6C, CMP @C14B | config mirror (mode commit) |
| $0D08 | STA @1AA2(#01),A0BF,BF5D(#01),BFA7(#02),C0EC(#04); LDA @BF4D | **state flag 1/2/4** (mode-class) |

- $0D08 = **DSP state class**: 01 = basic, 02 = mode2, 04 = EQ-applied — firmware writes it DIRECTLY via memory (not USB report) → **this proves DSP config CAN be written via the memory bus, BUT it is firmware-internal only**
- $BD62-$BD6F = mode-config commit: LDA $137F → CMP $0D00 → STA $0D00 → RMB2 $93 (`27 93` @BD6F — 2026-09-12 byte-verified)
- $BD72 = **MODE-5 SET**: LDA #$05 / STA $137D / SMB2 $A3 (transitions into EQ-active mode; called indirectly through the mode table — no direct JSR)
- $BF80-$BF9B = mode dispatch: BBS1 $A9 selects handler ptr $C3xx/$C9xx into $124E/$124F → SMB3 $9F
- $A0BF/$A0C2-$A0CD = firmware mode-2 internal skip chain → JMP $A178

### EQ hardware flow:
→ I/O buffer (encoder/HID) → $12D0/$12D1 target → commit $0F13/$0F1D → EQ setup $D8B4/$D956 (ROM preset $F76E/$F779) → enable $0F71 ORA #$03 → DSP state $0D08 → coefficients $0E00-$0E40 (5 bands). **Entirely internal — no USB path for the daemon to set the EQ. Verdict 7e stands.**

## Phase 7g — Boot chain + config loader + console handler (complete)

### Boot config loader block $B7D0-$B831:
```
b7d1: LDA $8d → JSR $a5f4(hex reader) → STA $1001   ; WRITE HW register $1001
b7de: JSR $a84a (ptr inc $40/$41)
b7e1: JSR $a5f4 → STA $1004                          ; $1004
b7ea: JSR $a5f4 → STA $1003                          ; $1003
b7f0: LDA #$0f → STA $1006                           ; $1006 = 0x0F
b7f5: LDA $66 → STA $126a/$126c  (16-bit pair $66/$67)
b805: LDA $6e → STA $126e/$1270, $6f → $126f/$1271
b815: LDA $76 → STA $1276/$1278, $77 → $1277/$1279
b825: LDA $7e → STA $127a, $7f → $127b
b82f: LDA $86 → STA $1272 ...
```

### Helpers:
- **$A5F4 = 2-hex-char (1 byte) reader**: `LDA ($40),X` → JSR $A5E3 (ASCII→hex) → ASL×4 → STA $42 → next char → ORA $42 → RTS (reads 2 hex chars = 1 byte; NOT 4-nibble — corrected 2026-09-12)
- **$A5E3 = ASCII→hex convert**: CMP#$61 lowercase → SBC#$28; CMP#$41 uppercase → SBC#$08; SBC#$2F digit
- **$A84A = 16-bit pointer increment**: INC $40/BNE/INC $41 (stream advance)
- **$A84A caller-only path**: $A5F4 has 4 callers — all inside the $B7D0 block: @B7CF→STA $1002, @B7D8, @B7E1, @B7EA (corrected from "3" on 2026-09-12)
- **$B790 = end-of-stream check**: LDA $46 / CMP #$38 (56 = config length) / BCC loop / JMP $B47D
- **$E06E = clamp helper**: BMI→RTS, CMP #$06, BCC→RTS, LDA #$05 (clamp max 5)

### Boot completion ($B47D-$B48C):
```
b47d: JSR $B447       ; `20 47 b4` — dispatch through helper (no STZ $20B4 — corrected 2026-09-12)
b480: RMB0 $93        ; clear state bit0 (`07 93`)
b482: LDA #$BE / STA $1239   ; PATCH vector slot0 operand hi
b487: LDA #$B3 / STA $123A   ; PATCH vector slot0 operand lo
b48c: RTS
```
→ **$1238 = `4C BE B3` = JMP $B3BE** — boot patches the *operand bytes* of the JMP at $1238 to install the **console handler**.

### $B3BE = console line editor:
```
b3be: AND #$7f        ; strip 7-bit
b3c0: CMP #$0d → CR execute command
b3c4: CMP #$7f → DEL
b3c8: CMP #$08 → backspace
b3cc: LDX $a7 (buffer len) / BEQ skip
b3d0: DEC $a7 / LDA #$08 / JSR $A5A5 (echo backspace)
b3d7: LDA #$20 / JSR $A5A5 (echo space)
```

### Config source ($66/$67/$6E/$6F/$76/$77/$7E/$7F/$86/$8D) = 16-bit NVM/config values:
- Read via the hex stream (from config buffer $40/$41 or NVM)
- Mirrored into state $126A-$127B (double-slot: $126A+$126C, $126E+$1270, etc.)
- Re-read at $DE03/$DE2B: LDY $126f/$1271 → JSR $E082 (I/O config writer)
- $E082 writes into I/O context $0F1E/$0F1F with mask BIT #$4F

### Architectural meaning:
**The firmware is a multi-stage boot state machine:** ROM stub → hardware init → config stream load (registers $1001-$1006 + NVM state) → progressive vector patching ($1238 slots) → console handler install → runtime mode. This explains the "3 slots converging" phenomenon from Phase 7c: each boot stage patches different vectors until stable.

## Phase 7h — Mode state machine COMPLETE ($137D, 6 states 0-5)

**Architecture: cooperative state machine** — each mode routine pre-installs the next handler into $124B/$124C + flag $93 (RMB2 `27 93` disarms / SMB2 `a7 93` arms — all 5 sites byte-verified 2026-09-12). The dispatcher reads $137D; if mode 0 → installs $BCFD (auto-advance).

### Mode table:
| Mode | Set at | Behavior |
|------|--------|---------|
| **0** | $BE5F-$BE6C | **EQ OFF**: LDA #$00 → STA $1386/$1387 (clears BOTH EQ flags) → JSR $D564 (apply no-op) → mode=0 → installs handler $BB95. Dispatcher $BE41: mode==0 → installs $BCFD (advance to mode 1) |
| **1** | $BCFD | mode=1, RMB2 $93 (`27 93` @BD06), installs $124B/$124C = $BD25 (mode 2) or $BB95 (alt) |
| **2** | $BD25 | mode=2, RMB2 $93, continues → mode 3 |
| **3** | $BD35 | mode=3, RMB2 $93 — **LED/gate state** (LED handler only processes when state ∈ {3,4}) |
| **4** | $BD58 + $CC5C | Report byte < $80 → STA $137F (config); = 0 → installs $BD35 (back to mode 3); ≠ 0 → mode=4 (TWO write sites: $BD58 `a9 04` + $CC5C — 2026-09-12) |
| **5** | $BD72 | **EQ-active**: mode=5 + SMB2 $A3 — gate for EQ chain $D564 (mode==5 + flag $93 bit6, BBR6 `6f 93 01` @D567) |

### Mode-4 processor ($CC32-$CC5C):
```
cc32: LDA $137d / CMP #$03 / BEQ / CMP #$04 / BNE exit  ; gate {3,4}
cc3d: LDX #$02 / LDA ($40),Y / CMP #$80 / BCS exit       ; threshold 0x80
cc45: STA $137f                                         ; config store
cc48: BNE $10 → $CC5A (mode=4)
cc4a: RMB2 $93 / ptr=$BD35 / SMB2 $93                   ; zero → mode-3 handler (`27 93` — 2026-09-12 byte-verified)
cc5a: LDA #$04 / STA $137d                              ; nonzero → mode 4
```

### Meaning:
- **$137D = device state machine 0-5**: 0=idle/EQ-off, 1=setup, 2=intermediate, 3/4=LED+config active, 5=EQ-active
- **$137F = config byte** (from report < $80) — used at $BD62 (mirrors $0D00 commit)
- **$1386/$1387 = EQ1/EQ2 flags** — mode 0 clears both; mode 5 + $93 bit5 gates the apply
- $124B/$124C = linked-list handler pointer — **each mode pre-installs the next** (cooperative scheduler)
- Explains the HID LED handler gate {3,4}: the device accepts report 0x02 only while in LED-active mode — **the daemon's LED re-assert after replug is necessary because the device boots into mode 0/1**

### Mode transitions:
```
0 →(dispatcher)→ 1 → 2 → 3 ⇄ 4 (via $137F config byte) / → 5 (EQ-active) → ...
```

## Phase 7i — EQ coefficient banks + reset path closure

### $BB95 = mode-1 ALT = RESET/CLEAR path:
```
bb95: LDA #$00 / STA $137d        ; mode 0
bba2: JSR $bbd6                   ; EQ DSP init!
bbab: ptr $124E/$124F = $C2E9     ; reset target
bbb7: patch vector slot0 → $B3BE  ; console handler re-assert
bbc7: LDA #$fd → install $BCFD    ; mode 1 (advance)
bbd1: SMB2 $93 / RMB2 $A2 / RTS
```

### $BBD6 = EQ DSP init:
```
bbd6: LDA #$00 → STA $08B7 / STA $0F50
bbde: LDA #$0F → STA $08B8
bbe3: RMB2 $A3
bbe5: LDA $0D03 / AND #$02 / STA $0D03   ; mask $0D03 bit1
```

### $C1EC-$C258 = EQ1 coefficient load (RUNTIME):
```
c1f4: LDA $0F70 / ORA #$41 / STA $0F70   ; EQ1 ENABLE (bit 0 + 6)
c1fc: LDA $12D1 / ASL A / TAX / CMP #$0C ; index×2, idx 12 special
c205: DEC $13B2 → =0 → JSR $C2DA         ; delay countdown reload
c20f: LDA $D7 / CMP #$01 → table select  ; $D7==1 → $F663 else $F693
c215: LDA ,X$F693/x$F694 → $40/$41       ; 16-bit table value
c249: BBR1 $4D → CLC / LSR $41 / ROR $40 ; ÷2 scale
c251: STA $0E1C / STA $0E1D              ; → DSP COEFFICIENT REGISTERS!
```
Second pair: $D7 test → $F6A3/$F673 → also ÷2 → $0E1C/$0E1D.

### $C2DA = countdown loader:
```
a9 0a        LDA #$0A          ; 10 ticks default
ac d1 12     LDY $12D1
c0 08        CPY #$08
d0 02        BNE +2
a9 05        LDA #$05          ; index 8 → 5 ticks
8d b2 13     STA $13B2         ; $13B2 = delay counter
60           RTS
```

### The 4 EQ coefficient banks ($12D1×2 index) — MATHEMATICAL RELATIONSHIP:
| Bank | Addr | Values (first 8) | Relation |
|------|------|----------------|---------|
| A    | $F663 | 20 2C 40 58 60 80 B0 C0 | base |
| B    | $F673 | 24 30 44 5C 64 84 B4 C4 | **A + 4** |
| C    | $F693 | 30 42 60 84 90 C0 108 120 | **A × 1.5** |
| D    | $F6A3 | 36 48 66 8A 96 C6 10E 126 | **C + 6** |

→ 4 fixed envelopes (base / +4 / ×1.5 / ×1.5+6) — DSP picks by index, NOT arbitrary biquads.
→ FINAL VERDICT: HW EQ is preset-only, not USB-controllable → daemon's LADSPA 9-band is correct.

### DSP coefficient registers confirmed:
- $0E1C/$0E1D = EQ1 coeff write target (÷2 from ROM banks)
- $0E00-$0E40 = 5-band area (band stride 0x10, confirmed earlier)
- $0D00 = DSP config mirror (from $137F commit)
- $0D08 = DSP state class (01/02/04)