# NVM / Persistence Report — EPOS GSX 300

**Phase D2 / D2c / P6-retest — Volatile state, empirically proven**
Dates: 2026-09-11 → 2026-09-12. All tests READ-ONLY (memory bus, bit6 never set).

## Baseline

| Dump | md5 | Read errors |
|------|-----|-------------|
| A — EEPROM (128KB), fresh replug | `6b1d7ea8…` | 0 |

## Test Matrix

### D2 — Volume persistence (EEPROM)
| Step | Action | Diff vs A |
|------|--------|-----------|
| 1 | Baseline A | — |
| 2 | Knob 100→45 (7 detents, daemon-log-verified) | 0 bytes |
| 3 | Knob 45→55 | 0 bytes |

**Verdict: volume does NOT persist to EEPROM.**

### D2c — Mode/LED persistence (EEPROM)
| Step | Action | Diff vs A |
|------|--------|-----------|
| 1 | Baseline A | — |
| 2 | Smart button 3× (Stereo→7.1→Stereo, 12:32:00/02/05 log) | 0 bytes |
| 3 | After unplug 3s + replug | 0 bytes |

**Verdict: mode/LED does NOT persist — even at power loss.**

### D2-RAM — Volume (RAM candidates)
| Candidate | 100→50→100 | Scale match |
|-----------|------------|-------------|
| $0F7E | 3C→34→3C (60→52→60) | 80% volume → 8 units — encoder counter, not volume |
| $0D07 | 2→3→6 | small counter — no |
| $0FCC | 112→116→117 | FIFO-scale — no |
| $01245 | 164→153→131 | non-monotonic vs 0→50→100 — no |
| $014A1 | 0→188→189 | FIFO-scale — no |

Cascade ring buffer $001D0-$01F5 = encoder FIFO. Full 64KB diff = 152 bytes
of firmware state-machine noise (0x20↔0x60 toggles at scattered positions).

**Verdict: no readable volume register in RAM either.**

### Passive HID listener (30s, daemon stopped)
127 reports observed: only 0x01 (detent direction + release) and 0x02
(smart button; 0x04 long-press auto-repeat ~3.3s when no host reads).
Zero 0x07/0x1A self-emitted → they are host-query command/response ONLY.

## Conclusion

| Question | Answer |
|----------|--------|
| Does volume persist? | **No** — host-side tracking required |
| Does mode/LED persist? | **No** — daemon re-assert heartbeat required |
| Readable volume register? | **Does not exist** (EEPROM, RAM, HID) |
| Power-loss write? | **No** (0 bytes after unplug/replug) |
| Is host-side ±5/detent correct? | **Yes — only correct design** |

## Extra finding (bonus)

The unplug/replug test double-verified daemon Bug#3 self-heal end-to-end:
hidraw3 vanished on unplug → device reprobed as hidraw11 → daemon I/O-error
→ re-scan → revived on hidraw11 → active LED re-asserted.

## Files

- Baselines/diffs: `eeprom-dumps/` (md5-indexed binaries + diff files)
- Summary: this report