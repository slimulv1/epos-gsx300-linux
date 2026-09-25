/* Does `run` write its output or add to it?
 *
 * The obvious harness allocates the output buffer with calloc, so the plugin's
 * output is always added to zeroes and the question never arises. That is not
 * what the host does.
 *
 * PipeWire, spa/plugins/filter-graph/filter-graph.c:
 *
 *     for (i = 0; i < graph->n_outputs; i++) {
 *             if (out[i] == NULL) continue;
 *             port = &graph->output[i];
 *             if (port->desc)
 *                     port->desc->connect_port(*port->hndl, port->port, out[i]);
 *             else
 *                     memset(out[i], 0, n_samples * sizeof(float));
 *     }
 *
 * The output buffer is only zeroed when the graph's output port is NOT connected
 * to a node. This plugin's graph output IS connected - it is the renderer's own
 * output port, `surround:output_left` - so nothing zeroes it, and whatever the
 * buffer held when the period started is still in it.
 *
 * So the buffer is pre-filled here with a known value instead of zeroes, and the
 * two implementations are compared. A `run` that adds shows up as the input value
 * leaking straight through.
 */
#include "common.h"

#define NOISE_LEN 480

int main(int argc, char **argv) {
    setvbuf(stdout, NULL, _IONBF, 0);
    const Descriptor *d = load(argv[1]);
    int fails = 0;
    #define CHECK(c, ...) do { if (!(c)) { printf("  FAIL  "); printf(__VA_ARGS__); printf("\n"); fails++; } } while (0)

    static float noise[NOISE_LEN];
    white(noise, NOISE_LEN);

    /* 1. What does the host actually hand us? */
    for (int poisoned = 0; poisoned < 2; poisoned++) {
        float *ol = malloc(NOISE_LEN * sizeof(float));
        float *or_ = malloc(NOISE_LEN * sizeof(float));
        // Zeroed, as a calloc-based harness always does.
        if (!poisoned) { memset(ol, 0, NOISE_LEN * sizeof(float)); memset(or_, 0, NOISE_LEN * sizeof(float)); }
        // Or whatever the previous period left behind, which is the real case.
        else { memcpy(ol, noise, NOISE_LEN * sizeof(float)); memcpy(or_, noise, NOISE_LEN * sizeof(float)); }

        void *inst = d->instantiate(d, RATE);
        float *il = calloc(NOISE_LEN, sizeof(float));
        float *ir = calloc(NOISE_LEN, sizeof(float));
        for (int i = 0; i < NOISE_LEN; i++) { il[i] = noise[i]; ir[i] = noise[i]; }
        d->connect_port(inst, P_OUT_L, ol);
        d->connect_port(inst, P_OUT_R, or_);
        d->connect_port(inst, P_IN_L, il);
        d->connect_port(inst, P_IN_R, ir);
        float ctrl[NCTRL] = { 1.0f, 0.0f, 0.0f, 0.0f };
        for (int i = 0; i < NCTRL; i++) d->connect_port(inst, P_GAIN + i, ctrl + i);
        d->activate(inst);
        d->run(inst, NOISE_LEN);
        d->deactivate(inst);
        d->cleanup(inst);

        double el = 0, er = 0;
        for (int i = 0; i < NOISE_LEN; i++) { el += (double)ol[i]*ol[i]; er += (double)or_[i]*or_[i]; }
        printf("  output buffer starts %-8s  ->  rms L %.4f  rms R %.4f\n",
               poisoned ? "NON-ZERO" : "zeroed", sqrt(el/NOISE_LEN), sqrt(er/NOISE_LEN));
        if (poisoned) {
            // The same signal in, with and without a dirty buffer. If run() adds
            // instead of writing, the dirty case comes out strictly louder by the
            // leftover, and that leftover is whatever the previous period held.
            printf("  (the two rows must agree; a difference is the plugin "
                   "accumulating onto a buffer the host never cleared)\n");
        }
        free(ol); free(or_); free(il); free(ir);
    }

    /* 2. The direct test: a buffer full of a known value must not appear in the
     *    output at all. */
    float *ol = malloc(NOISE_LEN * sizeof(float));
    float *or_ = malloc(NOISE_LEN * sizeof(float));
    for (int i = 0; i < NOISE_LEN; i++) { ol[i] = 1000.0f; or_[i] = 1000.0f; }
    float *il = calloc(NOISE_LEN, sizeof(float));
    float *ir = calloc(NOISE_LEN, sizeof(float));
    void *inst = d->instantiate(d, RATE);
    d->connect_port(inst, P_OUT_L, ol);
    d->connect_port(inst, P_OUT_R, or_);
    d->connect_port(inst, P_IN_L, il);
    d->connect_port(inst, P_IN_R, ir);
    float ctrl[NCTRL] = { 1.0f, 0.0f, 0.0f, 0.0f };
    for (int i = 0; i < NCTRL; i++) d->connect_port(inst, P_GAIN + i, ctrl + i);
    d->activate(inst);
    d->run(inst, NOISE_LEN);
    d->deactivate(inst);
    d->cleanup(inst);
    double maxabs = 0;
    for (int i = 0; i < NOISE_LEN; i++) {
        if (fabs(ol[i]) > maxabs) maxabs = fabs(ol[i]);
        if (fabs(or_[i]) > maxabs) maxabs = fabs(or_[i]);
    }
    printf("\n  input silent, output buffer pre-filled with 1000.0:\n");
    printf("    largest value in the output: %.4f\n", maxabs);
    CHECK(maxabs < 1e-6,
          "a silent input must produce a silent output, but the buffer's previous "
          "contents (%g) are still in it: run() is adding to the output instead of "
          "writing it. PipeWire only zeroes an output buffer when its graph output "
          "port is not connected to a node, and this one is.", maxabs);

    printf("\n  %s (%d failure%s)\n", fails ? "FAILURES" : "run() writes its output",
           fails, fails == 1 ? "" : "s");
    return fails ? 1 : 0;
}
