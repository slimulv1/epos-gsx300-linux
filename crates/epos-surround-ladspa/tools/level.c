/* How loud does this plugin make things?
 *
 * The harness printed a peak of 4.39 from an input whose peak was 0.5, which
 * looks like 19 dB of gain. Peak is the wrong number for a question about
 * loudness: the output of noise through a 256-tap filter has a peak far above
 * its RMS by construction, and the ratio says more about the filter's
 * length than about its level.
 *
 * RMS against a defined reference is the number that matters here, and the
 * reference is the plugin's own output at zero widening - so this measures what
 * the widening adds, not what the HRIR bank does. Both are then compared to the
 * input, because "the plugin is unity gain" is a claim someone will make and it
 * should be checked rather than assumed.
 */
#include "common.h"

int main(int argc, char **argv) {
    setvbuf(stdout, NULL, _IONBF, 0);
    const Descriptor *d = load(argv[1]);
    int N = RATE;
    static float buf[RATE];
    white(buf, N);

    double in_rms = 0;
    for (int i = 0; i < N; i++) in_rms += (double)buf[i] * buf[i];
    in_rms = sqrt(in_rms / N);
    double in_peak = 0;
    for (int i = 0; i < N; i++)
        if (fabsf(buf[i]) > in_peak) in_peak = fabsf(buf[i]);
    printf("  input: peak %.3f  rms %.4f\n\n", in_peak, in_rms);

    printf("  %-8s %-6s %9s %9s %9s %9s\n",
           "spread", "rear", "rms L", "rms R", "gain L", "gain R");
    for (float sp = 0.0f; sp <= 1.001f; sp += 0.25f) {
        for (float rl = 0.0f; rl <= 1.001f; rl += 0.5f) {
            float c[4] = {1.0f, sp, 0.0f, rl};
            float same[RATE];
            memcpy(same, buf, N * sizeof(float));
            /* Both channels carry the same noise, so a symmetric chain gives a
             * symmetric result and any drift is the chain's, not the source's. */
            Reading r = run_side(d, buf, same, N, c);
            double rl_ = sqrt(r.el / N), rr_ = sqrt(r.er / N);
            printf("  %-8.2f %-6.2f %9.4f %9.4f %+8.2f %+8.2f\n",
                   sp, rl, rl_, rr_,
                   20 * log10(rl_ / in_rms), 20 * log10(rr_ / in_rms));
        }
    }

    printf("\n  for reference, what the widening costs in level, relative to the\n"
           "  same signal with every control at zero:\n");
    float off[4] = {1.0f, 0.0f, 0.0f, 0.0f};
    float same[RATE];
    memcpy(same, buf, N * sizeof(float));
    Reading base = run_side(d, buf, same, N, off);
    double b = sqrt((base.el + base.er) / 2.0 / N);
    for (float sp = 0.0f; sp <= 1.001f; sp += 0.5f) {
        for (float rl = 0.0f; rl <= 1.001f; rl += 1.0f) {
            float c[4] = {1.0f, sp, 0.0f, rl};
            Reading r = run_side(d, buf, same, N, c);
            double t = sqrt((r.el + r.er) / 2.0 / N);
            printf("    spread=%.2f rear=%.2f  %+.2f dB vs widening off\n",
                   sp, rl, 20 * log10(t / b));
        }
    }
    return 0;
}
