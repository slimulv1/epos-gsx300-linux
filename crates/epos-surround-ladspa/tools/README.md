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
