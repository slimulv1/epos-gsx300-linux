/* Drive the surround plugin the way a real stereo stream will, and measure
 * what actually comes out.
 *
 * The plugin is 2-in / 2-out and synthesises the 7.1 speaker set internally, so
 * "does each speaker work" is no longer a question about eight input ports. It is
 * a question about the image: drive one stereo channel at a time and measure the
 * energy at each ear, then check that the picture is where it should be.
 *
 * Every reading here is a difference against the plugin's own measurement of
 * itself, so the checks are ratios, not absolute levels - the monitor DAC on this
 * machine is not a trustworthy absolute reference.
 *
 * Build and run:
 *   cc -O2 -o harness harness.c -ldl -lm
 *   ./harness ~/.local/lib/ladspa/epos-surround.so
 */
#include <dlfcn.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>


/* The descriptor comes from PipeWire's own header, not from a transcription of
 * it. This file previously declared it by hand and was wrong about it in exactly
 * the two places the published LADSPA 1.1 spec differs from the header PipeWire
 * ships - `LADSPA_Properties` is an int and `LADSPA_Data` is a float - which
 * made it segfault on a descriptor that loads perfectly well. It agreed with
 * tools/abi.c only by accident. One declaration in the tree, from the source. */
#include "ladspa.h"

typedef LADSPA_PortRangeHint PortRange;
typedef LADSPA_PortDescriptor PortDescriptor;
typedef LADSPA_Descriptor Descriptor;

#define RATE 48000
#define NCTRL 4

enum { P_OUT_L, P_OUT_R, P_IN_L, P_IN_R, P_GAIN, P_SPREAD, P_FRONT_W, P_REAR_LVL, P_NPORTS };

static double now_s(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec + ts.tv_nsec * 1e-9;
}

static double energy(const float *b, int n) {
    double s = 0;
    for (int i = 0; i < n; i++) s += (double)b[i] * b[i];
    return s;
}

typedef struct {
    double el, er, peak_l, peak_r;
} Reading;

/* One measurement, on a plugin instance that has never run. deactivate/activate
 * does NOT clear the filter history, so a reused instance makes every reading a
 * sum of all the previous ones - which is exactly what the first version of this
 * harness did before it produced a table of nonsense. */
static Reading run_fresh(const Descriptor *d, float *in_l, float *in_r, int nsamp,
                         const float *ctrl) {
    float *out[2];
    out[0] = calloc(nsamp, sizeof(float));
    out[1] = calloc(nsamp, sizeof(float));

    void *inst = d->instantiate(d, RATE);
    d->connect_port(inst, P_OUT_L, out[0]);
    d->connect_port(inst, P_OUT_R, out[1]);
    d->connect_port(inst, P_IN_L, in_l);
    d->connect_port(inst, P_IN_R, in_r);
    for (int i = 0; i < NCTRL; i++) d->connect_port(inst, P_GAIN + i, (float *)ctrl + i);
    d->activate(inst);
    /* run(), never run_adding(). Port index 0 is a valid audio output here, so a
     * host that picks run_adding by mistake lands in the driver's function
     * pointer slot instead. filter-chain calls run(); this harness does the
     * same, and getting it wrong crashes in a way that looks like a plugin bug
     * for twenty minutes. */
    d->run(inst, nsamp);
    d->deactivate(inst);

    Reading r = { energy(out[0], nsamp), energy(out[1], nsamp), 0, 0 };
    for (int i = 0; i < nsamp; i++) {
        r.peak_l = fmax(r.peak_l, fabsf(out[0][i]));
        r.peak_r = fmax(r.peak_r, fabsf(out[1][i]));
    }
    d->cleanup(inst);
    free(out[0]); free(out[1]);
    return r;
}

int main(int argc, char **argv) {
    setvbuf(stdout, NULL, _IONBF, 0);
    if (argc < 2) { fprintf(stderr, "usage: %s <plugin.so>\n", argv[0]); return 2; }

    void *h = dlopen(argv[1], RTLD_NOW);
    if (!h) { fprintf(stderr, "  dlopen: %s\n", dlerror()); return 1; }

    /* PipeWire's door: dlsym("ladspa_descriptor"), walked by index. Not the 1.1
     * symbol - filter-graph never looks for that one. */
    unsigned long (*desc_fn)(unsigned long) =
        (unsigned long (*)(unsigned long))dlsym(h, "ladspa_descriptor");
    if (!desc_fn) { fprintf(stderr, "  no ladspa_descriptor: %s\n", dlerror()); return 1; }
    const Descriptor *d = (const Descriptor *)desc_fn(0);
    if (!d) { fprintf(stderr, "  ladspa_descriptor(0) is NULL\n"); return 1; }

    printf("  unique_id %lu  Label \"%s\"  Name \"%s\"  ports %lu\n",
           d->UniqueID, d->Label, d->Name, d->PortCount);
    printf("  ports:");
    for (unsigned long i = 0; i < d->PortCount; i++) {
        int f = d->PortDescriptors[i];
        const char *kind = (f & LADSPA_PORT_AUDIO)
                         ? ((f & LADSPA_PORT_INPUT) ? "audio-in " : "audio-out")
                         : "control  ";
        printf(" [%lu]%s:%-13s", i, kind, d->PortNames[i]);
    }
    printf("\n");

    int fails = 0;
    #define CHECK(cond, ...) do { if (!(cond)) { printf("  FAIL: "); printf(__VA_ARGS__); printf("\n"); fails++; } } while (0)

    int nsamp = RATE;
    float *in_l = calloc(nsamp, sizeof(float));
    float *in_r = calloc(nsamp, sizeof(float));
    /* gain 1.0, spread 0.6, front width 0.0, rear level 0.35 - the instantiate
     * defaults, written explicitly so the numbers below do not depend on them
     * being what the source says they are. */
    float ctrl[NCTRL] = { 1.0f, 0.6f, 0.0f, 0.35f };

    /* ── 1. a unit impulse on one stereo channel must reach both ears ── */
    printf("\n  impulse on one input channel:\n");
    in_l[0] = 1.0f;
    Reading only_l = run_fresh(d, in_l, in_r, nsamp, ctrl);
    in_l[0] = 0.0f;
    in_r[0] = 1.0f;
    Reading only_r = run_fresh(d, in_l, in_r, nsamp, ctrl);
    in_r[0] = 0.0f;
    printf("    L in:  E_left=%.3e E_right=%.3e   %.2fx\n", only_l.el, only_l.er, only_l.er > 0 ? only_l.el/only_l.er : 0);
    printf("    R in:  E_left=%.3e E_right=%.3e   %.2fx\n", only_r.el, only_r.er, only_r.el > 0 ? only_r.er/only_r.el : 0);
    CHECK(only_l.el > only_l.er * 3.0, "a left-channel impulse must be much louder in the left ear");
    CHECK(only_r.er > only_r.el * 3.0, "a right-channel impulse must be much louder in the right ear");
    CHECK(only_l.el > 0 && only_r.er > 0, "an impulse must produce sound at all");

    /* ── 2. the spread controls have to actually widen the image ── */
    printf("\n  does the rear level open the image? (sweep, R channel in)\n");
    printf("    %-10s %12s %12s   L/R ratio\n", "rear_level", "E_left", "E_right");
    for (float rl = 0.0f; rl <= 1.001f; rl += 0.25f) {
        float c[NCTRL] = { 1.0f, 0.6f, 0.0f, rl };
        in_r[0] = 1.0f;
        Reading r = run_fresh(d, in_l, in_r, nsamp, c);
        in_r[0] = 0.0f;
        printf("    %-10.2f %12.3e %12.3e   %6.2fx\n", rl, r.el, r.er, r.el > 0 ? r.er/r.el : 0);
    }
    /* An impulse is symmetric-ish in total energy, so the ratio is the measure
     * that changes; the raw energies barely move. */
    float c_lo[NCTRL] = { 1.0f, 0.6f, 0.0f, 0.0f };
    float c_hi[NCTRL] = { 1.0f, 0.6f, 0.0f, 1.0f };
    in_r[0] = 1.0f;
    Reading lo = run_fresh(d, in_l, in_r, nsamp, c_lo);
    Reading hi = run_fresh(d, in_l, in_r, nsamp, c_hi);
    in_r[0] = 0.0f;
    double lo_ratio = lo.el > 0 ? lo.er / lo.el : 0;
    double hi_ratio = hi.el > 0 ? hi.er / hi.el : 0;
    CHECK(lo_ratio > 0, "rear_level 0 must still produce sound");
    CHECK(hi_ratio < lo_ratio,
           "opening the rear level must move energy toward the right ear: %.2fx -> %.2fx", lo_ratio, hi_ratio);

    /* ── 3. gain is a gain ── */
    float c0[NCTRL]  = { 0.0f, 0.6f, 0.0f, 0.35f };
    float c1[NCTRL]  = { 1.0f, 0.6f, 0.0f, 0.35f };
    float c2[NCTRL]  = { 2.0f, 0.6f, 0.0f, 0.35f };
    in_l[0] = 1.0f;
    Reading g0 = run_fresh(d, in_l, in_r, nsamp, c0);
    Reading g1 = run_fresh(d, in_l, in_r, nsamp, c1);
    Reading g2 = run_fresh(d, in_l, in_r, nsamp, c2);
    in_l[0] = 0.0f;
    CHECK(g0.peak_l == 0.0 && g0.peak_r == 0.0, "gain 0 must be silence, got L=%g R=%g", g0.peak_l, g0.peak_r);
    CHECK(fabs(g2.peak_l - 2.0 * g1.peak_l) < 0.02 * g1.peak_l,
          "gain must scale: peak at gain 1 = %g, at gain 2 = %g", g1.peak_l, g2.peak_l);

    /* ── 4. output has to be bounded: a real-time filter that can blow up is a
     *        real-time filter that will eventually blow up on someone's music ── */
    static float white[RATE];
    unsigned seed = 12345;
    for (int i = 0; i < RATE; i++) {
        seed = seed * 1103515245u + 12345u;
        white[i] = ((float)((seed >> 9) & 0xFFFF) / 32768.0f - 1.0f) * 0.5f;
    }
    memcpy(in_l, white, RATE * sizeof(float));
    Reading w = run_fresh(d, in_l, in_r, RATE, ctrl);
    printf("\n  full-scale white noise, 1 s:  peak L=%.3f  peak R=%.3f\n", w.peak_l, w.peak_r);
    CHECK(w.peak_l < 8.0f && w.peak_r < 8.0f,
          "output ran away: peak L=%g R=%g", w.peak_l, w.peak_r);
    CHECK(w.peak_l > 0.0f, "white noise must produce output");

    /* ── 5. real-time cost, worst case: both channels hot ── */
    memcpy(in_r, white, RATE * sizeof(float));
    void *inst = d->instantiate(d, RATE);
    float *out[2];
    out[0] = calloc(RATE, sizeof(float));
    out[1] = calloc(RATE, sizeof(float));
    d->connect_port(inst, P_OUT_L, out[0]);
    d->connect_port(inst, P_OUT_R, out[1]);
    d->connect_port(inst, P_IN_L, in_l);
    d->connect_port(inst, P_IN_R, in_r);
    for (int i = 0; i < NCTRL; i++) d->connect_port(inst, P_GAIN + i, ctrl + i);
    d->activate(inst);
    double t0 = now_s();
    d->run(inst, (unsigned long)RATE);
    double dt = now_s() - t0;
    d->deactivate(inst); d->cleanup(inst);
    free(out[0]); free(out[1]);
    printf("\n  1.00 s of audio, both channels hot: %.4f s of CPU\n", dt);
    printf("  real-time factor: %.1fx  (%s)\n", 1.0 / dt, dt < 1.0 ? "fits" : "DOES NOT FIT");
    CHECK(dt < 0.5, "a 2-in/2-out filter taking more than half the period is a problem");

    printf("\n  %s (%d failure%s)\n", fails ? "FAILURES" : "all checks passed", fails, fails == 1 ? "" : "s");
    dlclose(h);
    return fails ? 1 : 0;
}
