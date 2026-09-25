#!/usr/bin/env python3
"""Does audio actually reach the speakers? Measure it, do not infer it.

This exists because two rounds of acceptance testing passed on routing alone.
The EQ path was reported as "restored in 11 seconds" while the EQ had been silent
the whole time: every check read a sink name, a log line or a stream index, and
the test tone used along the way was at 0.92% amplitude - deliberately
inaudible, so that it could not disturb anyone. Nothing in that pipeline can
distinguish "the graph looks right" from "sound comes out".

So this measures sound. It plays a tone of known level, records the monitor of
the device the audio is supposed to reach, and reports the level actually
delivered. A monitor is exactly what is fed to the DAC, so it is the last
observable point before the speakers.

It also records the control path first. Without it a threshold means nothing: a
recording setup that is quietly broken looks exactly like an EQ that is quietly
broken, and that is not a distinction worth guessing about twice.

    scripts/eq-audible-check.py            # check the EPOS EQ path
    scripts/eq-audible-check.py --control  # only calibrate against the speakers

Exit status is 0 when the EQ path carries the tone and 1 when it does not, so it
can be run before and after a change and the result compared.

KNOWN DISCREPANCY, 2026-09-25. Do not trust a PASS from this script yet.

With the stock configuration this reported PASS -24.6 dBFS, within 1.2 dB of the
control - while `pw-dump` showed the EQ chain's capture fed by
`epos-voice-output`, the voice chain's output, and not by the anchor's monitor.
The same script on the same configuration earlier the same day measured -56.5
dBFS, which is the noise floor. Two runs, opposite verdicts, same files on disk,
and the mechanism behind the difference is not known.

A measurement that contradicts the graph it is supposed to reflect has not been
explained, and an unexplained PASS is the exact failure that produced the original
problem: a green check that meant nothing. Until the two agree, read this as a
hypothesis generator, not a gate.
"""

from __future__ import annotations

import argparse
import math
import shutil
import struct
import subprocess
import sys
import tempfile
import wave
from pathlib import Path

SAMPLE_RATE = 48000
CHANNELS = 2
RECORD_SAMPLES = 480_000  # 10 s
TONE_HZ = 1000
# -21 dBFS peak. ffmpeg's sine is not full scale and the level has to be known to
# compare anything against, so it is measured from the file rather than assumed.
EXPECTED_TONE_DBFS = -21.1
# The tone enters the anchor and leaves through the device. What arrives must be
# within this of the calibrated control, not merely "not zero": a chain that
# passes a tenth of the signal is broken audio, not working audio.
MAX_FALLBACK_DB = 12.0

DEFAULT_SPEAKERS = "alsa_output.usb-Generic_USB_Audio-00.HiFi_7_1__Speaker__sink"


def run(cmd: list[str], timeout: int = 30) -> subprocess.CompletedProcess:
    return subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)


def require(cmd: str) -> str:
    if shutil.which(cmd) is None:
        sys.exit(f"{cmd} is not installed; this check cannot be trusted without it")
    return cmd


def make_tone(path: Path) -> float:
    """Write a tone and return the level actually in the file, in dBFS."""
    require("ffmpeg")
    run([
        "ffmpeg", "-hide_banner", "-loglevel", "error", "-f", "lavfi",
        "-i", f"sine=frequency={TONE_HZ}:sample_rate={SAMPLE_RATE}:duration=20",
        "-ac", str(CHANNELS), "-y", str(path),
    ])
    return peak_dbfs(path)


def peak_dbfs(path: Path) -> float:
    with wave.open(str(path), "rb") as w:
        raw = w.readframes(w.getnframes())
        width = w.getsampwidth()
    if width != 2:
        sys.exit(f"expected a 16-bit wav, got {width * 8}-bit")
    samples = struct.unpack(f"<{len(raw) // 2}h", raw)
    if not samples:
        sys.exit("the wav is empty")
    peak = max(abs(s) for s in samples)
    return 20 * math.log10(peak / 32768) if peak else -99.0


def sink_index(name: str) -> int | None:
    out = run(["pactl", "list", "short", "sinks"]).stdout
    for line in out.splitlines():
        f = line.split()
        if len(f) > 1 and f[1] == name:
            return int(f[0])
    return None


def epos_sink() -> str | None:
    """The EPOS hardware sink, by USB device rather than by a hardcoded name."""
    out = run(["pactl", "list", "short", "sinks"]).stdout
    for line in out.splitlines():
        f = line.split()
        if len(f) > 1 and "EPOS_GSX_300" in f[1] and f[1].endswith("analog-stereo"):
            return f[1]
    return None


def record(source: str, path: Path) -> None:
    """Record 10 s of a source monitor. A monitor is what reaches the DAC."""
    require("pw-record")
    # A fixed sample count, not a timeout: a timeout that fires leaves a
    # half-written wav whose length says nothing about what was captured.
    run([
        "pw-record", "--target", source, "--rate", str(SAMPLE_RATE),
        "--channels", str(CHANNELS), "--format", "s16", "--container", "wav",
        "-n", str(RECORD_SAMPLES), str(path),
    ], timeout=40)
    if not path.exists() or path.stat().st_size < 4096:
        sys.exit(f"recording {source} produced nothing; the measurement is void")


def play_tone(tone: Path) -> subprocess.Popen:
    require("paplay")
    return subprocess.Popen(
        ["paplay", str(tone)], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL
    )


def route(sink: str) -> None:
    """Put the default on `sink` and let the daemon assert its own route."""
    run(["pactl", "set-default-sink", sink])
    # The daemon re-asserts the EQ anchor within one 5 s poll, and the tone has to
    # be opened after that or it lands on whichever sink was default at the time.
    import time
    time.sleep(9)


def measure(label: str, sink: str, tone: Path) -> float:
    index = sink_index(sink)
    if index is None:
        sys.exit(f"no sink named {sink}")
    route(sink)
    player = play_tone(tone)
    import time
    time.sleep(2)
    if player.poll() is not None:
        sys.exit("the test tone exited immediately; nothing was played")
    with tempfile.TemporaryDirectory() as td:
        out = Path(td) / "capture.wav"
        try:
            record(f"{sink}.monitor", out)
        finally:
            player.terminate()
            player.wait(timeout=10)
        level = peak_dbfs(out)
    print(f"  {label:<22} {level:>6.1f} dBFS peak at {sink}.monitor")
    return level


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--control", default="", help="sink to calibrate against")
    ap.add_argument("--expect-at-least", type=float, default=None,
                    help="absolute dBFS floor for the EQ path")
    args = ap.parse_args()

    with tempfile.TemporaryDirectory() as td:
        tone = Path(td) / "tone.wav"
        tone_db = make_tone(tone)
        print(f"  tone                  {tone_db:>6.1f} dBFS peak ({TONE_HZ} Hz)")

        control_sink = args.control or DEFAULT_SPEAKERS
        control = measure("control", control_sink, tone)
        if control < -60:
            print("  the control path is silent, so this run proves nothing.")
            print("  Fix the recording setup before drawing any conclusion.")
            return 2

        epos = epos_sink()
        if epos is None:
            print("  no EPOS sink is published; cannot check the EQ path")
            return 2

        level = measure("EPOS EQ path", epos, tone)

    floor = args.expect_at_least
    if floor is None:
        floor = control - MAX_FALLBACK_DB
    print(f"  threshold             {floor:>6.1f} dBFS "
          f"(control {control:.1f} - {MAX_FALLBACK_DB:.0f})")
    if level >= floor:
        print("  PASS: the EQ path carries the tone to the device")
        return 0
    print("  FAIL: the EQ path does not carry the tone. Routing is not sound;")
    print("        check what feeds the chain, not what the default sink says.")
    return 1


if __name__ == "__main__":
    sys.exit(main())
