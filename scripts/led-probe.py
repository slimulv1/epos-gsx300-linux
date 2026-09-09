#!/usr/bin/env python3
"""
EPOS GSX 300 LED Ring Probe
===========================
CONFIRMED PROTOCOL (2026-09-09, hardware tested):

  Vendor Report ID 0x02 output byte (1B):
    0x00 = LED off
    0x01 = BLUE  (bit0 = stereo mode)
    0x02 = RED   (bit1 = 7.1 surround mode)
    0x03 = PINK  (both bits = red+blue mix)

Usage:
  python3 led-probe.py            # probe vendor Report ID 0x02 (1-byte outputs)
  python3 led-probe.py --primary  # probe consumer Report ID 0x04 (38-byte outputs)
  python3 led-probe.py --auto     # step through all candidates with 2s delays

The GSX 300 LED ring states (from EPOS Gaming Suite behavior):
  Blue  = Stereo (2.0)
  Red   = Surround (7.1)
"""

import argparse
import os
import sys
import time

VID = "1395"
PID = "0098"
HIDRAW_SIZE = 64


def find_hidraw() -> str:
    hidraw_dir = "/sys/class/hidraw"
    for entry in sorted(os.listdir(hidraw_dir)):
        uevent = os.path.join(hidraw_dir, entry, "device", "uevent")
        try:
            with open(uevent) as f:
                content = f.read()
            if VID in content and PID in content:
                return f"/dev/{entry}"
        except OSError:
            continue
    sys.exit(f"ERROR: GSX 300 (USB {VID}:{PID}) not found in {hidraw_dir}")


def send_report(dev_path: str, report_id: int, payload: bytes):
    """Write a HID output report with the given report ID + payload."""
    packet = bytearray(HIDRAW_SIZE)
    packet[0] = report_id
    data = payload[: HIDRAW_SIZE - 1]
    packet[1 : 1 + len(data)] = data
    with open(dev_path, "wb", buffering=0) as f:
        f.write(packet)
    print(f"  >> Report ID 0x{report_id:02X} [{len(payload)}B] {payload.hex(' ')}")


def probe_vendor(dev: str):
    """Report ID 0x02 — 1-byte output, 2 LED bits (usages 0x05/0x06)."""
    print("\n=== Vendor Report ID 0x02 (1-byte, 2 LED bits) ===")
    print("Watch the LED ring. Hit Ctrl+C between tests to pause.")
    for value in range(0, 4):
        color = {0: "Off", 1: "Blue??", 2: "Red??", 3: "Both/Purple??"}[value]
        print(f"\n--- Try value 0x{value:02X} (bin {value:02b}) = {color} ---")
        send_report(dev, 0x02, bytes([value]))
        time.sleep(3)
    print("\nDone vendor probe. Note which value = Blue and which = Red.")


def probe_primary(dev: str):
    """Report ID 0x04 — 38-byte output, full device command."""
    print("\n=== Consumer Report ID 0x04 (38-byte) ===")
    print("Byte 0 = mode candidate. Trying mode values 0x00-0x03...")
    for mode_val in range(0, 4):
        payload = bytes([mode_val]) + bytes(37)  # 38 bytes
        print(f"\n--- Try byte0=0x{mode_val:02X} ---")
        send_report(dev, 0x04, payload)
        time.sleep(3)
    print("\nDone primary probe. Note any LED change.")


def probe_auto(dev: str):
    probe_vendor(dev)
    probe_primary(dev)


def main():
    parser = argparse.ArgumentParser(description="Probe EPOS GSX 300 LED reports")
    parser.add_argument(
        "--primary", action="store_true", help="probe the 38-byte Report ID 0x04"
    )
    parser.add_argument(
        "--auto", action="store_true", help="step through all candidates automatically"
    )
    args = parser.parse_args()

    dev = find_hidraw()
    print(f"Found GSX 300 at {dev}")
    if not os.access(dev, os.W_OK):
        sys.exit(f"ERROR: no write access to {dev}. Run: sudo udevadm trigger && log out/in (or use sudo).")

    if args.primary:
        probe_primary(dev)
    elif args.auto:
        probe_auto(dev)
    else:
        probe_vendor(dev)

    print("\nProbe complete. Update LedProbeConfig default bytes with results.")


if __name__ == "__main__":
    main()