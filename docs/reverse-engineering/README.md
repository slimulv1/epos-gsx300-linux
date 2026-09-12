# EPOS GSX 300 — Reverse Engineering Archive

Full reverse-engineering work product of the EPOS GSX 300 USB DAC
(2026-09). Read-only investigation only — **no firmware modification,
no memory-bus write, no brick risk** was ever performed.

## Safety rules (read before touching the device)

1. **Never** set bit 6 (write-enable) on the memory bus (report 0x04/0x05).
2. **Never** probe reports `0x06` (OUT 36B) / `0x07` (IN 32B) / `0x1A`
   with write intent — suspected EPOS Gaming Suite profile-write bus /
   firmware update protocol ("Download initiated.." string at `0xEBEB`).
3. All dump/analysis scripts here are read-only by design.

## Document map

| File | What it is |
| --- | --- |
| [`HARDWARE-BOOK.md`](HARDWARE-BOOK.md) | **Community-facing device book** — identity, USB/HID protocol, firmware architecture, registers, EQ verdict, tooling. Start here. |
| [`QUICK-REFERENCE.md`](QUICK-REFERENCE.md) | At-a-glance reference — HID reports, LED map, registers, persistence matrix. |
| [`FIRMWARE-REPORT.md`](FIRMWARE-REPORT.md) | Deep firmware RE log (Phase 7a–7i): boot chain, vector table, mode state machine, LED shift-register, EQ chain, misc register map. |
| [`NVM-PERSISTENCE-REPORT.md`](NVM-PERSISTENCE-REPORT.md) | Persistence test matrix — volume knob / mode / LED across EEPROM + RAM + unplug/replug. |
| [`HISTORY/`](HISTORY/) | Earlier-stage research docs (blueprint, feasibility, agent findings, original research archive). |
| [`analysis/`](analysis/) | Disassembly output (`full-disasm.asm` 10k+ lines, handler extracts). |
| [`tools/`](tools/) | `recdis.py` — W65C02S recursive disassembler (canonical). `dis_w65.py` — legacy, deprecated. |
| [`dumps/`](dumps/) | Raw firmware/EEPROM/RAM dumps. **Git-ignored** (copyrighted firmware) — kept local only. |

## Key conclusions (verify with the docs above)

- **Chip**: Conexant/Synaptics **CX21988** (chip-ID register `$1005` = 0x08).
- **CPU**: **W65C02S** — undefined opcodes 0xC2–0xF7 are NOPs.
- **Firmware**: FREEMAN_V03.01.00.00 @ `0xEC74`, patch 44.05.62, 48 kHz.
- **Memory**: 128 KB EEPROM (7× firmware copies, wear-leveled) + 64 KB RAM.
  Boot chain self-patches its own vector table (`$1238`, 10 slots).
- **Mode state machine** (`$137D`, 0–5): cooperative linked-list scheduler;
  LED processed only in states 3/4 → **daemon must re-assert LED after
  replug** (it does).
- **HW EQ verdict**: chip's 5-band EQ is **preset-only** (4 ROM coefficient
  banks, `$F663`/`$F673`/`$F693`/`$F6A3`) and **not exposed over USB** —
  the daemon's host-side 9-band LADSPA EQ is the correct design.
- **Persistence**: volume + mode are fully **volatile** (0 bytes EEPROM
  diff, even after unplug) → host-side volume tracking is the only
  correct design (daemon does this).

## How to regenerate dumps / disassembly

See [`HARDWARE-BOOK.md`](HARDWARE-BOOK.md) § Tooling. TL;DR:
`tools/recdis.py <ram-dump.bin> <start-addr...>` produces disassembly;
dump scripts live in the eeprom-dumps workflow described there.