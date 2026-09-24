# Microphone signal watchdog

Date: 2026-09-24. Added because `device_connected` was answering a different
question than the one people were asking it.

## The gap

Every other liveness check in this daemon asks whether a node **exists**. That
cannot tell a working microphone from a connected but dead one. A muted capture
element, a source that went stale after ALSA re-enumerated, or a converter that
stopped working all leave:

- the USB device present, so `device_connected: true`
- every PipeWire node published, so the `voice` watchdog sees nothing wrong
- the default source pointed at `epos-voice-output`

…while applications record silence. The status had no way to say so.

## Why not a level threshold

The obvious implementation — "the level is too low" — is unusable, and not
because it is inaccurate. It is because **a quiet room is not a broken
microphone**. A gate strong enough to catch a dead capture also fires whenever
nobody is talking, so it would report a working headset as broken constantly,
and the obvious response to that noise is to ignore it. A check that cries wolf
is worse than no check, because it teaches the reader to stop reading.

## What is safe to measure: digital silence

Measured on this machine, 122 windows of 150 ms on the raw capture in a quiet
room:

| | min | median | max |
| --- | --- | --- | --- |
| exact-zero sample fraction | 0.03 % | 0.15 % | **0.31 %** |
| window peak | 562 | 787 | 1086 |

A healthy ADC is never digitally silent. It always carries its preamp noise
floor, whether or not anyone is speaking. So the discriminator is **exact zeros
from a stream that is otherwise running**, with the threshold set at 50 % —
about 160x above the worst healthy window ever observed.

That margin is the whole reason this can be automatic. It is not a clever
threshold; it is a threshold with enough room that no plausible quiet room
reaches it.

## The raw source, never the processed one

The probe reads `pipewire_source` (the hardware microphone), **not**
`epos-voice-output`. The processed node cannot answer the question: rnnoise
legitimately outputs zeros while it suppresses, which measured 34 % exact-zero
samples on quiet room tone. Judged against a 50 % threshold, a perfectly healthy
microphone behind the voice chain would read as intermittently dead.

## The rules

- **One silent probe never concludes anything.** It reports `Unknown`, not
  `Silent`. This is the rule that makes the check safe, and it is why a single
  threshold edge, a late-starting stream or a USB re-enumeration cannot produce
  a false alarm.
- **Three consecutive silent probes conclude "no signal"** — roughly three
  minutes at one probe a minute.
- **The verdict is sticky.** Recovery takes two *consecutive* good readings, and
  a good reading in between silences does not retract the verdict. Otherwise a
  flapping capture flips the report between "no signal" and "unknown" while
  still delivering nothing — the same class of mistake in the other direction.
- **An inconclusive probe moves nothing.** A probe that could not run is not
  evidence, so it must not push either counter — otherwise a failing capture tool
  slowly convicts a working microphone.
- **Recovery clears the accumulator on any good reading**, so two separate
  hiccups minutes apart cannot add up to a verdict.

## It reports, it does not repair

There is deliberately **no restart**. A muted capture element, a stale ALSA
source and a dead ADC all belong to the capture path, not to the DSP instances
this daemon owns, so restarting `pipewire-epos@voice` would not fix any of them
— it would only add log noise and false confidence. What the daemon can honestly
do is stop describing a microphone that is not delivering audio as a healthy one,
which is what `Status.mic_input` now does.

## Why it is slow and gated

The probe opens the capture device, so it engages the microphone's recording
indicator. It therefore runs at most once a minute, for about a second, and only
while a voice feature is actually engaged (enhancer or noise gate on). With both
off, a silent microphone is a mute the user chose, and there is no reason to keep
waking the capture device to observe it.

The start of every capture is discarded (`MIC_SETTLE`, 240 ms, from the measured
ADC-settling behaviour already documented in `mic_meter.rs`). A capture hands over
zeros while the converter is still waking up; judging those would call every
probe dead.

## Measured

Healthy microphone, 70 s spanning a probe boundary: `Signal` throughout, no
false flip, no stray capture processes.

Capture element muted via ALSA, then restored:

| elapsed | `mic_input` | why |
| --- | --- | --- |
| 0–40 s | `Signal` | probe interval not yet elapsed |
| 60 s | `Unknown` | first silent probe — suspicion only |
| 80–160 s | `Unknown` | waiting for the required evidence |
| 180 s | `Silent` | three consecutive silent probes |
| +120 s after restore | `Signal` | two consecutive good readings |

The `Unknown` step is the safety property, and it is visible in the measurement
rather than only asserted in a test.

## The GUI is deliberately not involved

`MicrophoneView.vue` already shows a real-time, fail-closed signal readout driven
by the backend's own `mic-level` stream — with an explicit standing rule in that
file that the text never animates, never self-generates and never fabricates.
Adding a second indicator there would duplicate a deliberately fail-closed
display. `Status.mic_input` exists for IPC consumers that have no meter.

## Not covered

- It measures the capture path, not audio *quality*. A microphone that is
  delivering hiss, clipping or distortion reads as `Signal`.
- It says nothing when no voice feature is engaged, by design.
- The probe adds a capture stream once a minute while a voice feature is on,
  which is visible as a brief recording indicator.
