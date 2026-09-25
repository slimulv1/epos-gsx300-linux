/* Load the LADSPA plugin on its own and prove two things: that it renders, and
 * whether it can render in real time.
 *
 * The EPOS EQ chain is a 24/7 system the user listens to. Wiring an untested
 * DSP block into it to find out whether it works is not an acceptable order of
 * operations, so the plugin is exercised here instead: dlopen, connect, activate,
 * run a known signal, and time it.
 *
 * The signal is an impulse on one speaker at a time. The answer is then checkable
 * without taste - the ear response for that direction has to dominate that ear.
 */
/* Run with:
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

#define LADSPA_PORT_INPUT 1
#define LADSPA_PORT_OUTPUT 2
#define LADSPA_PORT_CONTROL 4
#define LADSPA_PORT_AUDIO 8

typedef struct { float min; float max; } PortRange;

typedef struct {
    const char *label, *name;
    int port_range;
    const PortRange *ranges;
    int flags;
} PortDescriptor;

typedef struct {
    const char *label, *name;
    const void *properties;
    int property_count;
} Properties;

typedef struct {
    int unique_id;
    const char *label;
    const Properties *properties;
    const char *name, *maker, *copyright;
    int port_count;
    const PortDescriptor *port_descriptors;
    const char *implementation_data;
    void *(*instantiate)(const void *, long);
    void (*connect_port)(void *, long, float *);
    int (*activate)(void *);
    void (*run)(void *, long);
    void (*run_adding)(void *, long);
    void (*set_run_adding_gain)(void *, float);
    void (*deactivate)(void *);
    void (*cleanup)(void *);
} Descriptor;

#define RATE 48000
#define TAPS 256
#define NCH 8

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

int main(int argc, char **argv) {
    if (argc < 2) { fprintf(stderr, "usage: %s <plugin.so>\n", argv[0]); return 2; }
    setvbuf(stdout, NULL, _IONBF, 0);
    void *h = dlopen(argv[1], RTLD_NOW);
    if (!h) { fprintf(stderr, "dlopen: %s\n", dlerror()); return 1; }
    /* LADSPA 1.1: lsap_entry is a POINTER to the descriptor, so dlsym yields
     * the address of a pointer and the descriptor is one dereference further. */
    const Descriptor **entry = (const Descriptor **)dlsym(h, "lsap_entry");
    if (!entry) { fprintf(stderr, "no lsap_entry: %s\n", dlerror()); return 1; }
    const Descriptor *d = *entry;
    if (!d) { fprintf(stderr, "lsap_entry is null\n"); return 1; }

    printf("  unique_id %d  '%s'  maker '%s'  ports %d\n",
           d->unique_id, d->label, d->maker, d->port_count);
    printf("  ports:");
    for (int i = 0; i < d->port_count; i++) {
        const PortDescriptor *p = &d->port_descriptors[i];
        const char *kind = (p->flags & LADSPA_PORT_AUDIO)
                         ? ((p->flags & LADSPA_PORT_INPUT) ? "audio-in" : "audio-out")
                         : "control";
        printf(" [%d]%s:'%s'", i, kind, p->name);
    }
    printf("\n");

    int nsamp = RATE;  /* one second */
    float *out[2], *in[NCH], ctrl[2] = { 1.0f, 0.0f };
    for (int i = 0; i < 2; i++) out[i] = calloc(nsamp, sizeof(float));
    for (int i = 0; i < NCH; i++) in[i] = calloc(nsamp, sizeof(float));

    void *inst = d->instantiate(d, RATE);
    if (!inst) { fprintf(stderr, "instantiate failed (wrong sample rate?)\n"); return 1; }
    d->activate(inst);

    const char *names[NCH] = { "FL", "FR", "FC", "LFE", "SL", "SR", "BL", "BR" };
    printf("\n  impulse on one speaker at a time:\n");
    printf("  %-4s %12s %12s   left/right\n", "ch", "E_left", "E_right");
    for (int ch = 0; ch < NCH; ch++) {
        memset(out[0], 0, nsamp * sizeof(float));
        memset(out[1], 0, nsamp * sizeof(float));
        for (int i = 0; i < NCH; i++) memset(in[i], 0, nsamp * sizeof(float));
        in[ch][0] = 1.0f;     /* unit impulse */
        /* A fresh instance per channel. deactivate/activate does NOT clear the
         * filter history, so without this every reading is the tail of all the
         * previous ones and a per-channel measurement silently becomes
         * cumulative - which is exactly what the first run of this harness did. */
        d->deactivate(inst); d->cleanup(inst);
        inst = d->instantiate(d, RATE);
        d->connect_port(inst, 0, out[0]);
        d->connect_port(inst, 1, out[1]);
        for (int i = 0; i < NCH; i++) d->connect_port(inst, 2 + i, in[i]);
        d->connect_port(inst, 10, &ctrl[0]);
        d->connect_port(inst, 11, &ctrl[1]);
        d->activate(inst);
        d->run(inst, nsamp);
        double el = energy(out[0], nsamp), er = energy(out[1], nsamp);
        printf("  %-4s %12.3e %12.3e   %.2fx\n", names[ch], el, er,
               er > 1e-30 ? el / er : 0.0);
    }

    /* Real-time cost, on the worst case: every speaker active at once. */
    for (int i = 0; i < NCH; i++) for (int k = 0; k < nsamp; k++) in[i][k] = sinf(2.0f * 3.14159265f * 440.0f * k / RATE);
    memset(out[0], 0, nsamp * sizeof(float));
    memset(out[1], 0, nsamp * sizeof(float));
    d->deactivate(inst); d->activate(inst);
    double t0 = now_s();
    d->run(inst, nsamp);
    double dt = now_s() - t0;
    double xrt = dt;  /* 1 s of audio took dt seconds */
    printf("\n  1.00 s of audio, all 8 speakers active: %.3f s of CPU\n", dt);
    printf("  real-time factor: %.2fx  (%s)\n", 1.0 / xrt, xrt < 1.0 ? "fits" : "DOES NOT FIT");

    d->deactivate(inst);
    d->cleanup(inst);
    dlclose(h);
    return 0;
}
