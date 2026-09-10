#!/usr/bin/env python3
"""Capture HID input reports from the EPOS GSX 300 to learn button/knob mappings.

Shows raw report bytes as they arrive. Press the smart button / rotate the dial
and observe which (Report ID, bits) fire.

Usage: sudo python3 hid-capture.py [hidraw_dev]
"""
import os
import sys
import time
import select

HIDRAW = sys.argv[1] if len(sys.argv) > 1 else "/dev/hidraw3"


def find_hidraw():
    """Find GSX 300 hidraw via sysfs uevent (VID 1395 / PID 0098)."""
    base = "/sys/class/hidraw"
    for entry in sorted(os.listdir(base)):
        uevent = os.path.join(base, entry, "device", "uevent")
        try:
            with open(uevent) as f:
                content = f.read()
            if "00001395" in content and "00000098" in content:
                return f"/dev/{entry}"
        except OSError:
            continue
    return None


def main():
    path = HIDRAW
    if not os.path.exists(path):
        found = find_hidraw()
        if found:
            print(f"{path} not found, using {found}")
            path = found
        else:
            print("GSX 300 hidraw device not found!")
            sys.exit(1)

    fd = os.open(path, os.O_RDWR)
    print(f"Listening on {path} — press the smart button / rotate the dial now. Ctrl+C to stop.\n")

    last_report = b""
    while True:
        r, _, _ = select.select([fd], [], [], 1.0)
        if not r:
            continue
        data = os.read(fd, 64)
        if data != last_report:
            rid = data[0] if data else 0
            print(f"[{time.strftime('%H:%M:%S')}] Report ID {rid:#04x}: {data.hex(' ')}")
            last_report = data


if __name__ == "__main__":
    main()