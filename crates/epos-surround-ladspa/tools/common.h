#pragma once
#include <dlfcn.h>
#include <math.h>
/* a helper the standard already provides; the earlier shadow of it here was a
 * redefinition that stopped the file compiling at all. */
static inline double db(double ratio) { return 20.0 * log10(ratio); }
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define RATE 48000
#define TAPS 256
#define NCTRL 4
enum { P_OUT_L, P_OUT_R, P_IN_L, P_IN_R, P_GAIN, P_SPREAD, P_FRONT_W, P_REAR_LVL, P_NPORTS };

/* The descriptor comes from PipeWire's own header, not from a transcription of
 * it. This file previously declared it by hand, and a hand-written struct that
 * disagrees with the host is precisely the class of bug this crate spent a
 * session fixing: `LADSPA_Properties` and `LADSPA_Data` differ from the
 * published LADSPA 1.1 spec in PipeWire's copy, and a copy made here drifted
 * from both. `tools/abi.c` is the tool that exists to catch that, and it works
 * because it includes the header rather than restating it. This file now does
 * the same thing, so there is one declaration in the tree instead of two.
 *
 * Fetch the header next to this file before building; see tools/README.md. */
#include "ladspa.h"

/* PipeWire's header names these differently; aliases so the call sites below
 * read the same as the plugin's own port constants. */
typedef LADSPA_PortRangeHint PortRange;
typedef LADSPA_PortDescriptor PortDescriptor;
typedef LADSPA_Descriptor Descriptor;

typedef struct { double el, er, peak_l, peak_r; } Reading;

static inline const Descriptor *load(const char *path) {
    void *h = dlopen(path, RTLD_NOW);
    if (!h) { fprintf(stderr, "dlopen: %s\n", dlerror()); exit(1); }
    unsigned long (*f)(unsigned long) = dlsym(h, "ladspa_descriptor");
    if (!f) { fprintf(stderr, "no ladspa_descriptor\n"); exit(1); }
    return (const Descriptor *)f(0);
}

static inline void white(float *b, int n) {
    unsigned seed = 12345;
    for (int i = 0; i < n; i++) {
        seed = seed * 1103515245u + 12345u;
        b[i] = ((float)((seed >> 9) & 0xFFFF) / 32768.0f - 1.0f) * 0.5f;
    }
}

static inline Reading run_side(const Descriptor *d, float *il, float *ir,
                               int nsamp, const float *ctrl) {
    float *ol = calloc(nsamp, sizeof(float)), *or_ = calloc(nsamp, sizeof(float));
    void *inst = d->instantiate(d, RATE);
    d->connect_port(inst, P_OUT_L, ol);
    d->connect_port(inst, P_OUT_R, or_);
    if (il) d->connect_port(inst, P_IN_L, il);
    if (ir) d->connect_port(inst, P_IN_R, ir);
    for (int i = 0; i < NCTRL; i++) d->connect_port(inst, P_GAIN + i, (float *)ctrl + i);
    d->activate(inst);
    d->run(inst, nsamp);
    d->deactivate(inst);
    Reading r = {0, 0, 0, 0};
    for (int i = 0; i < nsamp; i++) {
        r.el += (double)ol[i]*ol[i];
        r.er += (double)or_[i]*or_[i];
        if (fabsf(ol[i]) > r.peak_l) r.peak_l = fabsf(ol[i]);
        if (fabsf(or_[i]) > r.peak_r) r.peak_r = fabsf(or_[i]);
    }
    d->cleanup(inst);
    free(ol); free(or_);
    return r;
}
