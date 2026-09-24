# EQ chain: root cause found, fixed, and verified by measurement

Date: 2026-09-24. Status: **working.** A 500 Hz band moves the headset
output by +14.8 dB / −7.8 dB as configured.

## Root cause

`node.dont-fallback = true` on the EQ's **capture** side.

It is the correct fail-closed property on a playback stream, and the
voice chain verifiably runs with it on both sides. But on a capture whose
target is the anchor's monitor, the filter-chain module abandons the
target lookup and fails:

```
mod.filter-chain | capture_state_changed(): error: defined target not found
mod.filter-chain | core_error() res:-2 (No such file or directory)
```

`epos-eq-capture` then never appears in MAIN and the EQ is a silent
no-op — the UI showed the EQ enabled and the audio was untouched.

Measured both directions on the same machine, same conf otherwise:

| EQ capture side | `epos-eq-capture` in MAIN |
| --- | --- |
| with `node.dont-fallback` | absent |
| without it | present, reaches `running` |

The playback side keeps `node.dont-fallback`, so EQ'd audio still cannot
spill to another sink when the headset disappears. Verified that this
combination runs.

## Two earlier defects, also required

1. **`remote.name` at module args level.** With it only inside
   `capture.props` / `playback.props`, the module's streams connect back
   to their own daemon:

   ```
   mod.protocol-native | try_connect() connecting to 'pipewire-epos-eq'
   ```

   The loopback-based sidetone has it at the module level, which is the
   only reason sidetone ever worked.

2. **`node.passive` removed.** `node.passive = true` on both sides means
   the chain is never linked, so it sat at `suspended` even while audio
   played into it.

## What was ruled out, by measurement

Several plausible theories were tested and are wrong:

- **Not a cross-daemon problem.** Adding a null-sink *inside* the eq
  instance and targeting it locally failed identically.
- **Not the filter graph.** A passthrough graph and a 9-band
  `bq_peaking` graph both publish once the capture side is fixed.
- **Not a startup race.** It reproduces with the anchor present, as the
  default sink, and with an active stream playing into it.
- **Not `media.class`.** Adding `Stream/Input/Audio` to the EQ capture
  changed nothing; the voice chain has it, but so does the working EQ.
- **Not the target spelling.** `epos-eq-input`, `epos-eq-input.monitor`
  and a local null-sink all failed identically before the real fix.

## Verification

Method: play a quiet 440 Hz tone into `epos-eq-input`, record the EPOS
hardware sink's monitor, and measure the 500 Hz component with a Goertzel
filter. The band under test is 500 Hz, so its effect is isolated.

| 500 Hz band | 500 Hz magnitude |
| --- | --- |
| 0 dB (dropped as flat → passthrough) | 0.904 |
| **+24 dB** | **4.943** (+14.8 dB) |
| **−24 dB** | **0.367** (−7.8 dB) |

1 kHz and 4 kHz stay low throughout, so the change is band-selective
rather than a level shift. Audio genuinely passes through the filter.

## Routing consequence

`desired_output_route()` only picks the `epos-eq-input` anchor when
`epos-eq-capture` is actually published in MAIN, and `expected_node("eq")`
judges the role by the chain node when the graph carries bands. That
guard was added while the chain was broken, to stop apps being routed
into a sink nobody drained. It is still correct — it is what keeps the
route honest — and it now simply succeeds.
