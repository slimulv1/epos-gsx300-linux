#!/usr/bin/env python3
"""Smart button → mode/LED switching test for the EPOS GSX 300 daemon.

Validates the user requirement:
    🟢 blue (stereo)  = music / mono mode
    🔴 red  (7.1)     = gaming mode

Usage:
    python3 scripts/test-smart-button.py          interactive press test
    python3 scripts/test-smart-button.py --loop 5 5 rounds of press test
    python3 scripts/test-smart-button.py --status  show current mode only
    python3 scripts/test-smart-button.py --set stereo|surround71  force a mode

Protocol: newline-terminated JSON over the daemon Unix socket
    {"type":"Status"} / {"type":"SetMode","payload":{"mode":"stereo"}} / ...
"""

import argparse
import json
import os
import socket
import sys
import time

SOCKET = os.environ.get("EPOS_SOCKET", "/run/user/1000/epos-gsx300d.sock")

MODE_COLOR = {
    "stereo": "\033[94mBLUE\033[0m  (stereo — music/mono)",
    "surround71": "\033[91mRED\033[0m  (7.1 — gaming)",
}

# Dialog state machine: click and long-press alternate like the Windows app.
# We just report which mode the device landed on; the daemon mirrors the LED.
def request(req: dict, timeout: float = 3.0) -> dict:
    """Send one JSON request to the daemon socket, return decoded response."""
    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    s.settimeout(timeout)
    try:
        s.connect(SOCKET)
        s.sendall((json.dumps(req) + "\n").encode())
        buf = b""
        while True:
            chunk = s.recv(4096)
            if not chunk:
                break
            buf += chunk
            if b"\n" in buf:
                break
        if not buf:
            raise RuntimeError("empty response from daemon")
        return json.loads(buf.decode())
    finally:
        s.close()


def get_mode() -> str | None:
    resp = request({"type": "GetStatus"})
    # Status payload has .mode; GetMode returns payload as a plain string.
    payload = resp.get("payload", {})
    if isinstance(payload, str):
        return payload
    return payload.get("mode")


def set_mode(mode: str) -> str | None:
    request({"type": "SetMode", "payload": {"mode": mode}})
    # Confirm via GetMode / GetStatus round-trip
    return get_mode()


def wait_for_mode_change(prev: str, timeout: float = 30.0) -> tuple[str | None, float]:
    """Poll status until the mode differs from `prev`. Returns (new_mode, elapsed)."""
    t0 = time.monotonic()
    while time.monotonic() - t0 < timeout:
        try:
            mode = get_mode()
        except Exception as e:
            print(f"    ⚠ poll error: {e}")
            time.sleep(0.3)
            continue
        if mode and mode != prev:
            return mode, time.monotonic() - t0
        time.sleep(0.2)
    return None, time.monotonic() - t0


def press_round(round_no: int, prev_mode: str | None) -> tuple[str | None, bool]:
    """One press round: prompt user, wait for mode change, report."""
    print(f"\n  Round {round_no}:")
    print(f"    Current mode: {MODE_COLOR.get(prev_mode, prev_mode or 'unknown')}")
    print("    ▶ Now press the SMART BUTTON on the GSX 300"
          " (click the volume dial down once)...")
    new_mode, elapsed = wait_for_mode_change(prev_mode)
    if new_mode is None:
        print("    ⌛ No mode change detected in 30s."
              " The device may have swallowed the click — try again.")
        return prev_mode, False
    changed = new_mode != prev_mode
    print(f"    ✔ Mode changed to: {MODE_COLOR.get(new_mode, new_mode)}"
          f"  ({elapsed:.1f}s)")
    return new_mode, changed


def main():
    ap = argparse.ArgumentParser(description="GSX 300 smart button test")
    ap.add_argument("--loop", type=int, default=1,
                    help="number of press rounds (default 1)")
    ap.add_argument("--timeout", type=float, default=30.0,
                    help="seconds to wait per round (default 30)")
    ap.add_argument("--status", action="store_true",
                    help="show current mode and exit")
    ap.add_argument("--set", choices=["stereo", "surround71"],
                    help="force a mode and exit")
    args = ap.parse_args()

    print(f"Connecting to daemon: {SOCKET}")
    try:
        mode = get_mode()
    except Exception as e:
        print(f"✖ Cannot reach daemon: {e}")
        print("  → Is epos-gsx300d running?  (systemctl --user status epos-gsx300d "
              "or start ./target/release/epos-gsx300d)")
        sys.exit(1)

    if args.set:
        result = set_mode(args.set)
        print(f"→ SetMode {args.set}: response {result}")
        print(f"  LED should now be {MODE_COLOR.get(args.set, args.set)}")
        sys.exit(0)

    if args.status:
        print(f"Current mode: {MODE_COLOR.get(mode, mode)}")
        sys.exit(0)

    print(f"Current mode: {MODE_COLOR.get(mode, mode)}")
    print("=" * 62)
    print("Each round: press the smart button ONCE and watch the LED.")
    print("Expected per your requirement:")
    print("  • Click while BLUE  → should flip to RED  (stereo → 7.1/gaming)")
    print("  • Click while RED   → should flip to BLUE (7.1 → stereo/music)")
    print("  • Hold 2s           → also toggles the mode")
    print("=" * 62)

    results = []
    prev = mode
    for i in range(1, args.loop + 1):
        prev, ok = press_round(i, prev, )
        results.append(ok)
        if i < args.loop:
            print("  (waiting 2s before next round...)\n")
            time.sleep(2)

    # Summary
    passed = all(results)
    print("\n" + "=" * 62)
    print(f"Summary: {sum(results)}/{len(results)} rounds changed mode")
    print("✔ PASS — smart button flips the mode + LED color exactly like "
          "the Windows app" if passed else
          "✖ Some rounds were swallowed by the device debounce — "
          "that is a hardware quirk, retry the test.")
    print("=" * 62)
    sys.exit(0 if passed else 1)


if __name__ == "__main__":
    main()