"""An independent reimplementation of the surround plugin's DSP, for comparison.

Everything else in this crate's verification agrees with the plugin because it
was written by whoever wrote the plugin. This does not: it is a second reading
of the same written specification - the blob's layout, the upmix gains, the
ring, the direct-form convolution, the ear mapping - from the source text, in a
different language, with no shared code. If both agree on the output samples then
the agreement means something; if they agree because both are the same mistake,
it does not.

It is slow and it does not care. It is a reference, not a benchmark.

    python3 reference.py <hrir.bin> <plugin.so>
"""
import ctypes
import math
import struct
import subprocess
import sys

TAPS = 256
N_SPK = 7
NCTRL = 4
RATE = 48000

# The blob's format, as written by tools/extract_hrir.c and read back by
# `load_hrir` in src/lib.rs: an ASCII line "EPOSSURROUND2 <filters> <taps>",
# a newline, then binary. The first version of this file assumed a fixed-width
# binary header and failed twice on the real file, which is the point of writing
# the parser against the reader rather than against a guess.
MAGIC = "EPOSSURROUND2"

def read_blob(path):
    raw = open(path, "rb").read()
    nl = raw.index(b"\n")
    header = raw[:nl].decode("ascii")
    parts = header.split()
    assert parts[0] == MAGIC, header
    filters, taps = int(parts[1]), int(parts[2])
    assert taps == TAPS, taps
    assert filters == N_SPK * 2, filters

    body = raw[nl + 1:]
    delay_bytes = N_SPK * 2 * 4
    delays = struct.unpack_from("<%df" % (N_SPK * 2), body, 0)
    body = body[delay_bytes:]
    assert len(body) == N_SPK * 2 * TAPS * 4, len(body)
    data = struct.unpack_from("<%df" % (N_SPK * 2 * TAPS), body, 0)
    # [speaker][ear][tap], ear 0 = right, ear 1 = left
    hrir = [[[data[(s * 2 + e) * TAPS + k] for k in range(TAPS)]
             for e in range(2)] for s in range(N_SPK)]

    # Unity gain, one scalar per speaker so the interaural ratio survives.
    for s in range(N_SPK):
        pair = hrir[s][0] + hrir[s][1]
        e = sum(v * v for v in pair)
        if e > 0:
            g = 1.0 / math.sqrt(e)
            hrir[s] = [[v * g for v in hrir[s][0]], [v * g for v in hrir[s][1]]]
    return hrir, list(delays)


# The upmix, transcribed from `process` in src/lib.rs.
CH_FL, CH_FR, CH_FC, CH_SL, CH_SR, CH_BL, CH_BR = range(7)

def upmix(sp, l, r, spread, front_w, rear_lvl):
    if sp == CH_FL:
        return l + r * front_w
    if sp == CH_FR:
        return r + l * front_w
    if sp == CH_FC:
        return (l - r) * 0.5 * spread
    if sp == CH_SL:
        return l * (0.35 * spread + 0.5 * rear_lvl) + r * 0.3 * rear_lvl
    if sp == CH_SR:
        return r * (0.35 * spread + 0.5 * rear_lvl) + l * 0.3 * rear_lvl
    if sp == CH_BL:
        return l * (0.25 * spread + 0.7 * rear_lvl) + r * 0.35 * rear_lvl
    if sp == CH_BR:
        return r * (0.25 * spread + 0.7 * rear_lvl) + l * 0.35 * rear_lvl
    return 0.0


def reference(hrir, in_l, in_r, ctrl, nsamp):
    """The plugin's algorithm, written out longhand."""
    gain, spread, front_w, rear_lvl = ctrl
    hist = [[0.0] * TAPS for _ in range(N_SPK)]
    pos = 0
    out_l = [0.0] * nsamp
    out_r = [0.0] * nsamp
    for s in range(nsamp):
        l, r = in_l[s], in_r[s]
        left = right = 0.0
        for sp in range(N_SPK):
            x = upmix(sp, l, r, spread, front_w, rear_lvl)
            # The history index and the filter index run in OPPOSITE directions:
            # stepping the history back from the newest sample is stepping the
            # filter forward from tap 0.
            #
            # The first version of this file reversed the filter index as well,
            # which convolved with the time-reversed filter. The outputs had the
            # right magnitudes and the wrong sign, so a peak comparison would have
            # missed it entirely. It is worth keeping that in mind about this tool:
            # it caught its own author, and a coarser check would not have.
            acc_r = acc_l = 0.0
            for k in range(TAPS):
                h = hist[sp][(pos - k) % TAPS]
                acc_r += h * hrir[sp][0][k]
                acc_l += h * hrir[sp][1][k]
            left += acc_l
            right += acc_r
            hist[sp][pos] = x
        pos = (pos + 1) % TAPS
        out_l[s] = left * gain
        out_r[s] = right * gain
    return out_l, out_r


# ── the plugin, through the ABI ──
class PortRange(ctypes.Structure):
    _fields_ = [("hint", ctypes.c_int),
                ("lower", ctypes.c_float),
                ("upper", ctypes.c_float)]


class Descriptor(ctypes.Structure):
    _fields_ = [("UniqueID", ctypes.c_ulong),
                ("Label", ctypes.c_char_p),
                ("Properties", ctypes.c_int),
                ("Name", ctypes.c_char_p),
                ("Maker", ctypes.c_char_p),
                ("Copyright", ctypes.c_char_p),
                ("PortCount", ctypes.c_ulong),
                ("PortDescriptors", ctypes.POINTER(ctypes.c_int)),
                ("PortNames", ctypes.POINTER(ctypes.c_char_p)),
                ("PortRangeHints", ctypes.POINTER(PortRange)),
                ("ImplementationData", ctypes.c_void_p),
                ("instantiate", ctypes.c_void_p),
                ("connect_port", ctypes.c_void_p),
                ("activate", ctypes.c_void_p),
                ("run", ctypes.c_void_p),
                ("run_adding", ctypes.c_void_p),
                ("set_run_adding_gain", ctypes.c_void_p),
                ("deactivate", ctypes.c_void_p),
                ("cleanup", ctypes.c_void_p)]


def run_plugin(so, in_l, in_r, ctrl, nsamp):
    h = ctypes.CDLL(so)
    fn = h.ladspa_descriptor
    fn.restype = ctypes.POINTER(Descriptor)
    fn.argtypes = [ctypes.c_ulong]
    d = fn(0).contents

    inst_fn = ctypes.CFUNCTYPE(ctypes.c_void_p, ctypes.c_void_p, ctypes.c_ulong)(d.instantiate)
    cp_fn = ctypes.CFUNCTYPE(None, ctypes.c_void_p, ctypes.c_ulong, ctypes.c_void_p)(d.connect_port)
    act_fn = ctypes.CFUNCTYPE(ctypes.c_int, ctypes.c_void_p)(d.activate)
    run_fn = ctypes.CFUNCTYPE(None, ctypes.c_void_p, ctypes.c_ulong)(d.run)
    deact_fn = ctypes.CFUNCTYPE(None, ctypes.c_void_p)(d.deactivate)
    clean_fn = ctypes.CFUNCTYPE(None, ctypes.c_void_p)(d.cleanup)

    L = (ctypes.c_float * nsamp)(*in_l)
    R = (ctypes.c_float * nsamp)(*in_r)
    # Not zeroed: the real host does not clear an output buffer that belongs to a
    # node, and a reference that zeroes it would be comparing something else.
    OL = (ctypes.c_float * nsamp)(*[12345.0] * nsamp)
    OR = (ctypes.c_float * nsamp)(*[12345.0] * nsamp)
    C = (ctypes.c_float * NCTRL)(*ctrl)

    inst = inst_fn(None, RATE)
    for port, buf in ((0, OL), (1, OR), (2, L), (3, R)):
        cp_fn(inst, port, ctypes.cast(buf, ctypes.c_void_p))
    for i in range(NCTRL):
        cp_fn(inst, 4 + i, ctypes.cast(ctypes.byref(C, 4 * i), ctypes.c_void_p))
    act_fn(inst)
    run_fn(inst, nsamp)
    deact_fn(inst)
    clean_fn(inst)
    return list(OL), list(OR)


def main():
    if len(sys.argv) != 3:
        print(__doc__)
        return 2
    blob, so = sys.argv[1], sys.argv[2]
    hrir, delays = read_blob(blob)
    if any(abs(d) > 0.5 for d in delays):
        print("  blob has non-zero bulk delay, the reference does not model it")

    # A signal with content in every part of the spectrum, so a wrong filter
    # cannot hide in a gap.
    N = 3000
    in_l, in_r = [], []
    seed = 1
    for i in range(N):
        seed = (seed * 1103515245 + 12345) & 0xFFFFFFFF
        noise = ((seed >> 9) & 0xFFFF) / 32768.0 - 1.0
        in_l.append(0.4 * noise + 0.3 * math.sin(2 * math.pi * 440 * i / RATE)
                    + 0.1 * math.sin(2 * math.pi * 3000 * i / RATE))
        in_r.append(0.35 * noise - 0.25 * math.sin(2 * math.pi * 660 * i / RATE))

    worst = 0.0
    fails = 0
    for ctrl in [(1.0, 0.0, 0.0, 0.0),
                 (1.0, 0.6, 0.0, 0.35),
                 (0.5, 1.0, 1.0, 1.0),
                 (2.0, 0.25, 0.75, 0.5)]:
        rl, rr = run_plugin(so, in_l, in_r, ctrl, N)
        kl, kr = reference(hrir, in_l, in_r, ctrl, N)
        dl = max(abs(a - b) for a, b in zip(rl, kl))
        dr = max(abs(a - b) for a, b in zip(rr, kr))
        peak = max(max(abs(v) for v in kl), max(abs(v) for v in kr), 1e-9)
        rel = max(dl, dr) / peak
        worst = max(worst, rel)
        ok = rel < 2e-4
        fails += 0 if ok else 1
        print("  ctrl %-18s peak %.5f  max|diff| %.3e (%.4f%% of peak)  %s"
              % (str(ctrl), peak, max(dl, dr), 100 * rel, "ok" if ok else "MISMATCH"))

    print()
    print("  worst relative difference: %.3e" % worst)
    print("  %s" % ("the two implementations agree" if fails == 0
                    else "%d control setting(s) disagree" % fails))
    return 1 if fails else 0


if __name__ == "__main__":
    sys.exit(main())
