# GSX 300 LED ring — EPROTO is a wedged USB endpoint, not an unsupported device

Date: 2026-09-24. Status: **root-caused, recovered, fix verified on hardware.**

An earlier version of this file concluded that HID output was
"non-functional on this unit". **That conclusion was wrong** and has been
withdrawn. The same report, on the same unit, with the same descriptor,
succeeds — the failure was a wedged USB endpoint, and a physical replug
cleared it.

## What the symptoms look like

Every HID output write to the GSX 300's `/dev/hidrawN` returns:

```
errno 71 EPROTO  (Protocol error)
```

The device remains fully present the whole time: USB interfaces, the ALSA
card, the hidraw node and the input reports (volume dial, smart button) all
keep working. Only output is refused. There is no code change that reliably
reproduces or clears it.

## Recovery

**Unplug the GSX 300 from USB for ~10 seconds, then plug it back in.** The
daemon's hotplug loop reopens the hidraw node and re-asserts the LED within
its 5 s poll, so no manual restart is needed.

Audio recovers on its own the same way — it is the same USB device.

## Verified recovery, by measurement

After a physical replug on the same boot, same unit, same descriptor:

| Write to `/dev/hidraw3` | Result |
| --- | --- |
| `[0x02, 0x01] + 62 zero bytes` (the historical 64-byte buffer) | `OK (64)` |
| `[0x02, 0x01]` (descriptor-exact, 1-byte payload) | `OK (2)` |
| `0x01` / `0x02` / `0x03` / `0x00` in sequence | all `OK` |

Then, with the daemon started normally:

```
INFO epos_gsx300d::led: Found GSX 300 at /dev/hidraw3
INFO epos_gsx300d::hid: HID listener active on /dev/hidraw3
INFO epos_gsx300d: LED controller initialized
```

and over a 45 s observation window (the heartbeat re-asserts every 2 s):

```
LED error lines in last 45s: 0
any WARN/ERROR in last 45s:   -- No entries --
```

Two sizes are accepted because the kernel pads/truncates an oversized HID
report rather than rejecting it. The descriptor-exact 1-byte payload is what
the code now sends, because it is what the descriptor specifies — not
because the previous 64-byte buffer was failing.

## Timeline of the 2026-09-24 investigation

1. **11:37:38** — boot. USB enumerates the GSX 300 at port `1-7`, device
   number `5`, hidraw3, ALSA card 4. The daemon initialises the LED ring and
   **every daemon from boot until 13:13 runs with zero LED errors**, i.e. the
   ring is genuinely being driven for ~100 minutes.
2. **13:15:00** — first `LED heartbeat: failed to re-assert Stereo`. Daemon
   `142017` had started at 13:13:14 and its first write succeeded, so this is
   a transition, not a boot-time misconfiguration. Nothing in the day's commits
   touched `led.rs` or `hid.rs`, and the only activity in that window was a
   rebuild. The trigger was not established.
3. **13:15–13:50** — a long diagnosis wrongly pursued a protocol mismatch:
   every payload size `1..=64` for report ids `0x01,0x02,0x04,0x05,0x06,0x07,0x1A`
   (448 combinations) was tried as root, plus a port reset. All returned
   EPROTO. That is consistent with a dead endpoint, and none of it was
   conclusive evidence about the descriptor.
4. **13:50** — escalating the diagnosis made it materially worse. Toggling
   `/sys/bus/usb/devices/1-7/authorized` to `0` and back to `1` dropped all
   four USB interfaces and the ALSA card, and the device then stopped
   accepting its USB address:

   ```
   usb 1-7: can't set config #1, error -71
   usb 1-7: authorized to connect
   usb 1-7: Device not responding to setup address
   usb 1-7: device not accepting address 17, error -71
   ```

   This step should not have been taken on a still-functional input path, and
   it is what turned a recoverable condition into one that needed hands on the
   cable.
5. **13:57** — physical replug. Device returns with all four interfaces, the
   ALSA card and hidraw3. All writes succeed, as tabulated above.

Two separate lessons, both worth keeping:

- **A still-enumerated device with working input reports is not "broken".**
  Before touching USB power, `authorized` or a port reset, confirm the actual
  failure is at the USB layer. Here it was not.
- **Never power-cycle USB from the daemon or a probe script.** A close/open of
  the hidraw fd is the only automatic recovery implemented, by design.

## What `errno 71` does and does not tell you here

In `hidraw_write()`, EPROTO means the submitted report could not be matched /
sent against the device's report table. It is a transport-layer symptom. On
this device it does **not** imply:

- that the report descriptor is misparsed (the kernel's own
  `/sys/kernel/debug/hid/0003:1395:0098.0004/rdesc` matches the raw descriptor,
  and report `0x02` is present in it);
- that the unit lacks an output path (the identical write succeeds after a
  replug);
- that report ids collide between input and output.

For contrast, a different hidraw node on the same machine (the ASUS AURA LED
controller, `hidraw1`) answers the same call with `EPIPE` (endpoint stall) —
a different errno for a different reason, on a different device.

## Descriptor facts (120 bytes, verified)

| Report | Direction | Bits | Payload |
| --- | --- | --- | --- |
| `0x01` consumer | IN | 3 + 5 | 1 B |
| `0x1A` | IN | 16 | 2 B |
| `0x04` memory bus | OUT | 38 x 8 | **38 B** |
| `0x05` | IN | 34 | 5 B |
| `0x06` | OUT | 36 x 8 | **36 B** |
| `0x07` | IN | 32 | 4 B |
| `0x02` LED (vendor `0xFF13`) | IN + OUT | 8 + 8 | **1 B** |

The two LED usages are `ff13.0005` = bit0 = blue, `ff13.0006` = bit1 = red, so
the wire byte is `0x00` off / `0x01` blue / `0x02` red / `0x03` pink. Note that
usage-id and wire-value are two different numbering systems; do not mix them.

The device is **full-speed (USB 1.1, 12 Mbps)**, `bcdDevice 0.62`, serial
`A003200202602692`, and has no `hid-generic` hwdb quirk for
`0003:1395:0098`.

## Safety guard that must stay

Reports `0x04` and `0x06` are the **firmware flash / memory-bus** interfaces.
`write_primary_report` hard-refuses any `0x04` payload with bit 6 (`0x40`) set,
because that is EEPROM write = firmware flash = a bricked device. This guard
is unrelated to the wedge above and is not to be relaxed. Do not brute-force
payloads against this device: the LED needs exactly one report id and one
byte, both known.

## Code changes

- `led.rs`: descriptor-exact 1-byte LED payload (from the 64-byte buffer),
  likewise 38 bytes for report `0x04` and in the `Drop` reset path.
- `led.rs`: `last_write_failed` so a failure is reported once per transition
  instead of once per 2 s heartbeat; `write_failing()` so callers can report
  LED state honestly.
- `led.rs`: `recover_if_needed()` — bounded (3x) close/open of the hidraw fd
  for the "still enumerated but refusing writes" case, which the hotplug loop
  does not cover. Explicitly does not touch USB power/reset. `reopen()` (real
  re-enumeration) resets the budget.
- `main.rs`: startup warning says the ring is not being driven and names the
  replug remedy; heartbeat failure logging drops to `debug` and delegates the
  retry to `recover_if_needed()`.
- Live config `~/.config/epos-gsx300/config.json` had `vendor_blue: 2,
  vendor_red: 1` — the reverse of the descriptor and of the crate default.
  Corrected to `1, 2`. The repository defaults were always right.
