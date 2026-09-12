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
- Boot EEPROM 8KB image @ file 0x805: `JMP $2805` stub → chip reg $0894 control → `JMP ($A875)` indirect into RAM firmware

## Vectors
- RAM @0xFFFA: NMI=$A0A3, RESET=$A000, IRQ=$A003
- Reset entry: `SEI / LDX #$FF / TXS` @0xA008; early `LDA $1005 CMP #$08 BEQ` = CX21988 chip-ID check in firmware
- $A090: stack dump routine (LDA $0100,X → JSR output → INX → BNE) = debug console dump

## Console / CLI discovery
- **$A5C9** = hex nybble output (LSR×4 → JSR $A5D4 → AND#$0F → CMP#$0A → ADC#$06 → out) — ASCII hex converter
- **$A5A5** = char output; **$A5BE/$A5C2** = CR/LF out; **$B687** = space out
- **$B5CA** = hex BYTE reader (2 hex chars → ASL×4 → ORA combine)
- **$B500-$B560** = command line parser: reads char, CMP #$0D (CR=end), CMP #$20 (SP=token sep), JSR $B5CA/$B617 = token dispatch
- $B5F1 / $B611: `JMP ($xxxx)` indirect through RAM pointer table = dispatch table

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

### LED HID handler hoàn chỉnh (bảng dispatch 3 tầng)
1. **$EC89** (LDX#0) → 16 entry chính
2. **$EC91** (LDX#1) → 26 entry chi tiết  
3. **$ECAB** (LED value ASL→TAX) → 16 LED mode: [0]=$CA9A off, [1]=$CAAC, [2]=$CAAE(mode2 handler dài), [3]=$CCA4, [4]=$CCD9, [5]=$CD1B

### LED bit-serial write (PHÁT HIỆN CHÍNH)
- **$1388/$1389 = LED shift-register pair** — CLC/BBR→SEC + ROL $1388 (giống SPI bit-bang)
- **$137D ∈ {3,4} = LED processing gate** (chỉ 2 mode này xử lý LED!)
- $A1 = LED pattern flag byte (BBR3 đọc, RMB2 clear)
- $9F = LED state flag (SMB0 set khi mode)
- Blue path: value==5 → clear $1388/$1389 → JMP $ECAB
- kết thúc: LDA ($88) + RTI (pointer hardware write)

### Mode state machine ($137D + callback $124B/$124C)
- $BCFD: mode=1, handler ptr=$BD25
- $BD25: mode=2, handler ptr=$BB95
- $BB95: mode=0, + JSR $BBD6 (init) + thunk set
- $BD72: mode=5
- $BBD6 = **INIT ROUTINE**: khởi tạo 5-band EQ coefficients:
  - Bands tại $0E00/$0E10/$0E20/$0E30/$0E40 (3-byte biquad coeff mỗi band: $xx00/$xx01/$xx02)
  - State pairs tại $xx0C/$xx0D (biquad state), enable $xx0E
  - $0D00/$0D02-$0D0F/$0D2E/$0D2F = EQ config/state
  - $1384-$1387 = additional state (clear tại $BCE4)
  - $12D0/#12D1 = last-command (init 7/2)
- $BCF9: STA $12D1 = write last-command reg

### Register map cập nhật (bổ sung)
| Reg | Role |
|-----|------|
| $1388/$1389 | LED shift pair |
| $137D | mode gate (LED only 3/4) |
| $124B/$124C | mode callback ptr |
| $1239/$123A | handler ptr (alt) |
| $0E00-0E4E | **5-band EQ: coeff+state+enable** |
| $0D00-$0D2F | EQ config/state frame |
| $1347 | chip-ID store ($1005 copy) |

### KẾT LUẬN KỸ THUẬT QUAN TRỌNG
**CX21988 CÓ 5-band HW EQ trong firmware** (biquad coefficient stores tại $0E00-$0E4E) — nhưng EQ host-side 9-band của daemon là cái khác (LADSPA bq_peaking). Chip HW EQ không exposed qua USB (daemon không thể truy cập) — xác nhận memory #11: "chip 5-band HW EQ không exposed qua USB".

## Phase 7c (2026-09-12): EQ config frame + control register map chuẩn

### (a) EQ/state frame — $1384-$1387 + $0Dxx
- $1386/$1387 = **EQ GPU enable flags** (set 0 tại $BE61/$BE64 khi mode-bypass, đọc tại $D5AC/$D573 trong EQ apply routine $D564)
- $1384 (STA tại $CEB6, HID handler), $1385 (STA $1385,Y tại $CE6A — indexed write từ report byte!)
- $12D0/$12D1 = last-command regs → dispatch LDX → JSR $DA8C/$DA6F (EQ command table!)
- EQ apply chain: $D564 (EQ0 check) → $D573 (EQ1: mode==5 gate) → $D5A1 (EQ2: $1386)
- $BE5F: bypass EQ → clear $1386/$1387 → JSR $D564 → mode0 ($BCFD path)

### (b) Control registers
- **$0F70/$0F71 = control pair** — set bit 0-1 (AND #$FC clear, ORA #$03 enable tại $D897/$D89C)
- **$0F44 = control register** (5 refs: $E0D2/$E3CF/$E407/$E599) — AND #$3F clear / ORA pattern (LED-control khu vực)
- **$0F1B = DAC/IO control** — ORA #$88 (enable) / ORA #$33 (enable) — $E167/$E174 helpers
- **$0F2A/$0F2B/$0F20 = mode alias triple** (đồng bộ cùng giá trị từ $1016)
- **$1016 = USB config register** (low nibble = mode ID, đọc tại $E001)
- **$E001 = mode-alias syncer**: $1016 nibble → copy vào $0F2A/$0F2B/$0F20 (shift 2)
- **$E06E = value clamp helper** (5..0xB6) — volume/dB clamp
- **$E082**: dùng $A3 bit1 → clamp 0xB1

### (c) Register map hoàn chỉnh (merged)
| Reg | Role |
|-----|------|
| $1016 | USB config mode ID |
| $0F2A/$0F2B | mode nibble mirrors |
| $0F20 | mode bits 2-5 shifted |
| $0F70/$0F71 | control pair (bits 0-1 enable) |
| $0F1B | DAC/IO enable ($E167/$E174) |
| $0F44 | LED-area control AND #$3F |
| $1384-$1387 | EQ enable flags (GPU-side) |
| $12D0/$12D1 | EQ command dispatch |

## Phase 7d (2026-09-12): Boot dump — self-patching vector table $1238

### Thí nghiệm
Stop daemon → pkexec unbind 1-7 → bind → 3 RAM dumps liên tiếp (0 errors) → so sánh với runtime (ram-64k-vol0.bin).

### Kết quả stability
- **IDENTICAL 100%**: 0x2000-0x2FFF, 0x6000-0xFFFF (boot vs runtime + giữa 3 boots)
- **Dynamic**: 0x0000-0x1FFF (38-167B), 0x3000-0x5FFF (32-43B)
- Vector table @1400, $1A50 runtime, $EC89 LED dispatch, $BBD6 EQ init, $A000 code: **SAME** mọi dumps

### 🔑 PHÁT HIỆN — Bảng thunk $1238 = 10-slot command handler vector, SELF-PATCHING

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

- **Caller duy nhất dạng JSR vào slot**: $A124→slot2, $A150→slot0, $A35A→slot3, $A415→slot4, $A8BF→slot1, $A8CD→slot5, $A8E4→slot9, $A907/$A946→slot6, $A910/$A94F→slot7
- **Handler tự patch bảng**: $A3ED/$A3F8/$A419 ghi pointer vào $1242/$1243 (slot3 target) và $1245/$1246 (slot4 target) — `LDA #lo/STA $1242/LDA #hi/STA $1243/RTS`
- Slots 2/3/4 hội tụ về $A321/$A3ED/$A4A4 = **2-level dispatch: ROM code patch RAM vector → gọi handler sâu**

### Handler decoded
- **$A321** (slot2 final): LDA #$3F/STA $123F/LDA #$A3/STA $1240 = TỰ patch chính nó (target $A33F); quản lý down-counters $C5/$C6 (DEC, SMB2 $96 khi đạt 0)
- **$A33F**: patch $123F/$1240 → target $A1D2 (BBS1 $99 chain)
- **$A3ED** (slot3 final): SED/STA $1242/LDA #$A3/STA $1243 → patch slot3 → $A3xx
- **$A3F8**: patch slot3 → $A405
- **$A419**: patch slot3 → $A426
- **$A431**: patch slot4 target ($1245/$1246) → $A44F; BBS0 $95 → đếm down-counters $CD/$CE
- **$A415**: JSR $1244 (slot4) → RTS = call-through wrapper

### Kết luận
Firmware dùng **self-modifying dispatch table** trong RAM ($1238-$1255): các routine ở vùng $A3xx cài đặt lẫn nhau bằng cách ghi trực tiếp JMP target vào bảng. Boot chạy multi-stage: mỗi lần dump bắt được stage khác nhau (A13F→A203→A2C7→A321). Đây là pattern dispatcher-table của Conexant — handler install qua pointer patch chứ không phải vector cứng.

### Phụ lục: raw bytes slot region (boot3)
`1238: 4c be b3 4c 58 b4 4c c7 a2 4c 84 a3 4c 8e a4 4c fd e4 4c 72 bd 4c 8a c9 4c 34 d3 4c ba e9`

## Phase 7e — EQ 5-band firmware: KHÔNG thể dùng qua USB (verdict cuối)

### EQ apply chain decode (from boot-dump-3)

```
$D564: JSR $1486→JMP $166D (gate) / BBR5 $93 → RTS   ; flag $93 bit5
$D56B: LDA $137D / CMP #$05 / RTS                     ; CHỈ apply khi mode==5
$D573: LDA $1387 → BNE → EQ2 path                     ; $1387 = EQ2 enable
       else JMP $D5A1
$D57E: JSR $143B / JSR $D8B4 (EQ2 setup) / LDX $12D0 → JSR $DA8C
$D5A1: BBR1 $A2 → JSR $DAB2 (EQ2 disable) / LDX #$20 loop
$D5AC: LDA $1386 → BNE → EQ1 path                     ; $1386 = EQ1 enable
$D5B4: JSR $143E / JSR $D956 (EQ1 setup) / LDX $12D1 → JSR $DA6F
$D5DA: LDA $0F1D AND #$07 CMP $12D1                   ; mode-nibble compare
```

### Cấu trúc 2 EQ paths (EQ1/EQ2 = stereo L/R!):
- **EQ2** ($D8B4/$DA8C): `LDA ,X$F76E` (ROM preset table) → $D3 → JSR $D894 (mask $0F71)
- **EQ1** ($D956/$DA6F): `LDA ,X$F779` (ROM preset table) → $D5 → JSR $D8A4 (mask $0F70)
- **$12D0 = EQ2 index, $12D1 = EQ1 index** (từ HID report bytes)
- **$0F13 low-nibble = EQ2 state** ($DA8C: AND#$F0|ORA), **$0F1D low-3 = EQ1 state** ($DA6F: AND#$F8|ORA)

### Bảng preset ROM:
```
$F76E (EQ2): 0c 00 06 00 00 06 04 04 02 02 02 0c 00 03 00 00 03 02 02 00 40...
$F779 (EQ1): 0c 00 03 00 00 03 02 02 00 40 00 00 00 00 00 00 00 03 00 04 03 0b 02...
```
- Giá trị giống nhau pattern (0x00-0x40 range) = **mức gain cố định theo preset**

### RAM translation (runtime-built):
```
$16A8 (RAM thunk) → $16BD: LDX $12D1 / LDA $16C6,X / JMP $D964
$16C6 (RAM table): 12 00 0a 00 00 09 03 03 33 6b 00 28 bb 03 17 c2 ...
   = bảng index-translation được tạo lúc boot (tương ứng $0E00-$0E40 banks)
```

### VERDICT — Không thể dùng 5-band HW EQ qua USB:
1. EQ index chỉ qua **$12D0/$12D1** — được set từ HID report nội bộ, không expose qua report 0x04/0x05 (read-only bus)
2. Coeffs giới hạn preset ROM ($F76E/$F779), **KHÔNG phải biquad tùy ý** — daemon 9-band host-side LADSPA linh hoạt hơn
3. Muốn điều khiển cần write memory bus (bit6) = **nguy cơ brick, chưa từng test**
4. **Kết luận: giữ thiết kế hiện tại (LADSPA 9-band host-side) — đúng đắn**

### Bonus findings:
- **$1000-$100F = firmware-writable regs**: $1005 chip-ID (STA tại E9D8-EA80), $1001/$1003/$1004/$1006 init (B7DB-B7F2), $100A/$100B watchdog counters (INC đến 5 / DEC đến 0xB7 tại E780-E7A6), $1009/$100E/$100F event reads
- **$1015-$1017 = READ-ONLY hardware state** (0 firmware write) — $1016 = mode/state từ USB config
- **$E001 syncer**: $1016 low-nibble → mirror vào $0F2A (nibble), $0F2B (nibble), $0F20 (<<2 shift) — hardware state mirror, chạy qua vector (không caller trực tiếp)
- **$A321 = 16-bit soft timer** ($C5/$C6): tick qua slot2 dispatch, gate flag $93 bit2, expired → SMB2 $96; $17BD→$A121 chain nạp timer callback từ RAM runtime

## Phase 7f — EQ index source + mode dispatch + DSP config commit

### $12D0/$12D1 (EQ index) source:
- **$12D0** (EQ2 target): đọc tại $C040/$C045 (compare + load), $D589 (LDX → $DA8C). So sánh với $0F13 low-nibble (active) + input $40
- **$12D1** (EQ1 target): **CHỈ MỘT write site: $BCF9 (STA $12D1 / RTS)** — set qua thunk vector $1238 slot0($B3BE?) hoặc pointer $124B/$124C chain. Đọc tại $16BD (RAM trans), $C1FC, $C2DC, $D5BD, $D5DA, $D95E

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
- $C07F: $0FBE AND #$C0 CMP #$C0 → nếu = : ASL $44/ROL $45 (scale ×2)
- $C08C: LDA $0FC7/$0FC6 hoặc $C09A: $0FC5/$0FC4 → 2 cặp position candidates
- $C0A6: SEC / SBC 16-bit delta (new - old) → $C0B3 BPL / $C0B5 ADC bù (negative wrap)
- $C0C1: JSR $1462 (= RTS noop — hook trống)
- $C0D1: LDA $0F71 / ORA #$03 / STA $0F71  ← **EQ ENABLE commit**
- **$C0EA: LDA #$04 / STA $0D08** ← DSP config direct write!

### DSP config region $0D00-$0D08:
| Addr | Refs | Ý nghĩa |
|------|------|---------|
| $0D00 | CMP/STA @BD67/$BD6C, CMP @C14B | config mirror (mode commit) |
| $0D08 | STA @1AA2(#01),A0BF,BF5D(#01),BFA7(#02),C0EC(#04); LDA @BF4D | **state flag 1/2/4** (mode-class) |

- $0D08 = **DSP state class**: 01 = basic, 02 = mode2, 04 = EQ-applied — firmware viết TRỰC TIẾP qua memory (không phải USB report) → **đây là bằng chứng DSP config có thể viết qua memory bus, NHƯNG nó là nội bộ firmware-only**
- $BD62-$BD6F = mode-config commit: LDA $137F → CMP $0D00 → STA $0D00 → RMB1 $93
- $BD72 = **MODE-5 SET**: LDA #$05 / STA $137D / SMB2 $A3 (chuyển vào EQ-active mode; caller gián tiếp qua mode table — không JSR trực tiếp)
- $BF80-$BF9B = mode dispatch: BBS1 $A9 chọn handler ptr $C3xx/$C9xx vào $124E/$124F → SMB3 $9F
- $A0BF/$A0C2-$A0CD = firmware mode-2 internal skip chain → JMP $A178

### Tiến trình EQ hardware:
→ I/O buffer (encoder/HID) → $12D0/$12D1 target → commit $0F13/$0F1D → EQ setup $D8B4/$D956 (ROM preset $F76E/$F779) → enable $0F71 ORA #$03 → DSP state $0D08 → coefficients $0E00-$0E40 (5 bands). **Toàn bộ internal — không có đường USB để daemon set EQ. Verdict 7e giữ nguyên.**

## Phase 7g — Boot chain + config loader + console handler (hoàn chỉnh)

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
- **$A5F4 = 4-nibble hex reader**: `LDA ($40),X` → JSR $A5E3 (ASCII→hex) → ASL×4 → STA $42 → next char → ORA $42 → RTS
- **$A5E3 = ASCII→hex convert**: CMP#$61 lowercase → SBC#$28; CMP#$41 uppercase → SBC#$08; SBC#$2F digit
- **$A84A = 16-bit pointer increment**: INC $40/BNE/INC $41 (stream advance)
- **$A84A caller-only path**: $A5F4 chỉ 3 caller — TẤT CẢ trong $B7D0 block
- **$B790 = end-of-stream check**: LDA $46 / CMP #$38 (56 = config length) / BCC loop / JMP $B47D
- **$E06E = clamp helper**: BMI→RTS, CMP #$06, BCC→RTS, LDA #$05 (clamp max 5)

### Boot completion ($B47D-$B48C):
```
b47d: STZ $20B4       ; data
b47e: RMB2 $B4        ; clear boot flag
b480: RMB0 $93        ; clear state bit0
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
- Đọc qua hex stream (từ config buffer $40/$41 hoặc NVM)
- Mirror vào state $126A-$127B (double-slot: $126A+$126C, $126E+$1270, etc.)
- Đọc lại tại $DE03/$DE2B: LDY $126f/$1271 → JSR $E082 (I/O config writer)
- $E082 write vào I/O context $0F1E/$0F1F với mask BIT #$4F

### Ý nghĩa kiến trúc:
**Firmware là máy trạng thái khởi động nhiều giai đoạn:** ROM stub → hardware init → config stream load (registers $1001-$1006 + NVM state) → progressive vector patching ($1238 slots) → console handler install → runtime mode. Điều này giải thích hiện tượng "3 slots hội tụ" ở Phase 7c: mỗi stage boot patch vector khác nhau cho tới khi ổn định.

## Phase 7h — Mode state machine COMPLETE ($137D, 6 states 0-5)

**Kiến trúc: cooperative state machine** — mỗi mode routine tự pre-install handler kế tiếp vào $124B/$124C + flag $93 (RMB1 đóng/SMB2 mở). Dispatcher đọc $137D, nếu mode 0 → install $BCFD (auto-advance).

### Mode table:
| Mode | Set at | Hành vi |
|------|--------|---------|
| **0** | $BE5F-$BE6C | **EQ OFF**: LDA #$00 → STA $1386/$1387 (clear BOTH EQ flags) → JSR $D564 (apply no-op) → mode=0 → install handler $BB95. Dispatcher $BE41: mode==0 → install $BCFD (advance mode 1) |
| **1** | $BCFD | mode=1, RMB1 $93, install $124B/$124C = $BD25 (mode 2) hoặc $BB95 (alt) |
| **2** | $BD25 | mode=2, RMB1 $93, tiếp tục → mode 3 |
| **3** | $BD35 | mode=3, RMB1 $93 — **LED/gate state** (LED handler chỉ xử lý khi state ∈ {3,4}) |
| **4** | $CC5C | Report byte < $80 → STA $137F (config); = 0 → install $BD35 (về mode 3); ≠ 0 → mode=4 |
| **5** | $BD72 | **EQ-active**: mode=5 + SMB2 $A3 — gate cho EQ chain $D564 (mode==5 + $93 bit5) |

### Mode 4 processor ($CC32-$CC5C):
```
cc32: LDA $137d / CMP #$03 / BEQ / CMP #$04 / BNE exit  ; gate {3,4}
cc3d: LDX #$02 / LDA ($40),Y / CMP #$80 / BCS exit       ; threshold 0x80
cc45: STA $137f                                         ; config store
cc48: BNE $10 → $CC5A (mode=4)
cc4a: RMB1 $93 / ptr=$BD35 / SMB2 $93                   ; zero → mode-3 handler
cc5a: LDA #$04 / STA $137d                              ; nonzero → mode 4
```

### Ý nghĩa:
- **$137D = device state machine 0-5**: 0=idle/EQ-off, 1=setup, 2=intermediate, 3/4=LED+config active, 5=EQ-active
- **$137F = config byte** (từ report < $80) — dùng tại $BD62 (mirror $0D00 commit)
- **$1386/$1387 = EQ1/EQ2 flags** — mode 0 clears cả 2, mode 5 + $93 bit5 gates apply
- $124B/$124C = linked-list handler pointer — **mỗi mode pre-install mode kế tiếp** (cooperative scheduler)
- Giải thích HID LED handler gate {3,4}: chỉ khi device ở mode LED-active mới chấp nhận report 0x02 — **daemon LED re-assert sau replug là cần thiết vì device khởi động về mode 0/1**

### Mode transitions:
```
0 →(dispatcher)→ 1 → 2 → 3 ⇄ 4 (qua $137F config byte) / → 5 (EQ-active) → ... 
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
bbe3: RMB1 $A3
bbe5: LDA $0D03 / AND #$02 / STA $0D03   ; mask $0D03 bit1
```

### $C1EC-$C258 = EQ1 coefficient load (RUNTIME):
```
c1f4: LDA $0F70 / ORA #$41 / STA $0F70   ; EQ1 ENABLE (bit 0 + 6)
c1fc: LDA $12D1 / ASL A / TAX / CMP #$0C ; index×2, idx 12 special
c205: DEC $13B2 → =0 → JSR $C2DA         ; delay countdown reload
c20f: LDA $D7 / CMP #$01 → chọn bảng     ; $D7==1 → $F663 else $F693
c215: LDA ,X$F693/x$F694 → $40/$41       ; 16-bit table value
c249: BBR1 $4D → CLC / LSR $41 / ROR $40 ; ÷2 scale
c251: STA $0E1C / STA $0E1D              ; → DSP COEFFICIENT REGISTERS!
```
Second pair: $D7 test → $F6A3/$F673 → cũng ÷2 → $0E1C/$0E1D.

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

### 4 EQ coefficient banks ($12D1×2 index) — QUAN HỆ TOÁN HỌC:
| Bank | Addr | Values (8 đầu) | Hệ thức |
|------|------|----------------|---------|
| A    | $F663 | 20 2C 40 58 60 80 B0 C0 | base |
| B    | $F673 | 24 30 44 5C 64 84 B4 C4 | **A + 4** |
| C    | $F693 | 30 42 60 84 90 C0 108 120 | **A × 1.5** |
| D    | $F6A3 | 36 48 66 8A 96 C6 10E 126 | **C + 6** |

→ 4 envelope cố định (base / +4 / ×1.5 / ×1.5+6) — DSP chọn theo index, KHÔNG phải biquad tùy ý.
→ VERDICT CUỐI: HW EQ preset-only, không điều khiển qua USB được → daemon LADSPA 9-band đúng.

### DSP coefficient registers xác nhận:
- $0E1C/$0E1D = EQ1 coeff write target (÷2 từ ROM banks)
- $0E00-$0E40 = 5-band area (band stride 0x10, confirm trước đó)
- $0D00 = DSP config mirror (từ $137F commit)
- $0D08 = DSP state class (01/02/04)
