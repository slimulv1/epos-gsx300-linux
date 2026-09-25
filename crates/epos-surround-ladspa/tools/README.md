# tools/

Measurement tools for the LADSPA plugin. None of them are part of the build; each
is compiled and run by hand, and each exists because something could not be
trusted without being measured.

## abi.c — is the descriptor laid out the way the host reads it?

    curl -o ladspa.h https://raw.githubusercontent.com/PipeWire/pipewire/1.6.9/spa/plugins/filter-graph/ladspa.h
    cc -O2 -o abi abi.c -ldl
    ./abi ~/.local/lib/ladspa/epos-surround.so

Nine separate ABI mistakes came out of this plugin, one at a time. Every one
passed the Rust unit tests, and every one passed a C harness that shared the same
hand-transcription of the LADSPA header — because a struct declared wrongly in
one place is declared wrongly in both, and the two agree with each other and with
nothing else. Three of them were found by PipeWire segfaulting, one of them only
on `cleanup`, after producing correct audio for the whole session.

This is the only tool that can see them, because it includes the real header
rather than a copy of it. Two of the differences from the published LADSPA 1.1
spec are worth more than everything else here put together:

    typedef float LADSPA_Data;         not double
    typedef int   LADSPA_Properties;   not a struct of two pointers and a count

`LADSPA_PortRangeHint` is therefore 12 bytes, not 20. Writing it as 20 put every
port's `UpperBound` eight bytes out of step, so the host read a bounds float
where a `PortNames` entry belonged and dereferenced it.

Do not vendor `ladspa.h`. It has to be the revision the host compiles, and a
copy in the tree would drift from it without anybody noticing.

## harness.c — does it render, and can it render in real time?

    cc -O2 -o harness harness.c -ldl -lm
    ./harness ~/.local/lib/ladspa/epos-surround.so

Drives a stereo signal through the plugin the way a real stream will, and checks
the image and the level rather than the presence of output. An impulse per
channel is what makes the failure modes visible: a silent output and a correct
one are both "no obvious noise", and three separate bugs in this DSP each made
every speaker output exact silence.

Every measurement is a ratio or a difference against the plugin's own
measurement of itself. Never an absolute level: the monitor DAC is not a
trustworthy absolute reference.

## level.c — how loud is it?

    cc -O2 -o level level.c -ldl -lm
    ./level ~/.local/lib/ladspa/epos-surround.so

Exists because the harness printed a peak 19 dB above the input and that was not
a real defect — the peak of noise through a 256-tap filter is high by
construction. RMS against a defined reference is the number that means
something, and reading it properly is what showed the genuine +9.3 dB that the
unit-peak HRIR normalisation was causing.

## extract_hrir.c — bake the HRIRs

Reads a SADIE II D1 SOFA through libmysofa and writes `src/hrir.bin`, checking
ITD and ILD symmetry before writing anything. Not a build step: the blob is
committed so the build needs neither the SOFA file nor libmysofa. The SOFA is
36 MB and is not in this repository.

## sweep_sofa.c — the data check

What showed the angles were good all the way round, after an earlier suspicion
that the rear ones were missing turned out to be a flaw in the peak-picking
rather than in the data.

## output_mode.c — does `run` write its output, or add to it?

    cc -O2 -o output_mode output_mode.c -ldl -lm
    ./output_mode ~/.local/lib/ladspa/epos-surround.so

The harness allocates its output buffers with `calloc`, so the question never
arises there. It arises with the real host. PipeWire,
`spa/plugins/filter-graph/filter-graph.c`:

```c
if (port->desc)
        port->desc->connect_port(*port->hndl, port->port, out[i]);
else
        memset(out[i], 0, n_samples * sizeof(float));
```

The buffer is cleared only when the graph's output port is *not* connected to a
node. This filter is the last node in its chain and the graph's output port is
its own output port, so the port is connected, so nothing clears it.

Adding to that buffer means adding to whatever the previous period left in it.
Measured: a silent input came out at exactly the value the buffer had been
pre-filled with, and the same signal produced different output depending on the
buffer's prior contents. A harness that zeroes its buffers cannot see this.

## reference.py — an independent implementation of the same DSP

    python3 reference.py ../src/hrir.bin ~/.local/lib/ladspa/epos-surround.so

A second reading of the written specification, in a different language, sharing
no code with the plugin. It exists because everything else here agrees with the
plugin because the same person wrote both. If the two agree sample-for-sample,
the agreement means something.

It also caught its own author: the first version reversed the filter index,
which convolves with the time-reversed filter. The output had the right
magnitudes and the wrong sign, so a peak comparison would have missed it. Worth
remembering about this tool, and about peak comparisons generally.

It is verified to fail: four deliberate mutations of the plugin - swapped ears,
a stray factor, a wrong rear gain, an off-by-one tap - all make it disagree.
