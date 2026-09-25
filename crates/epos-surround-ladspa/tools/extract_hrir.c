/* Extract the eight 7.1 speaker HRIRs from a SOFA file, and prove which ear is
 * which. The AES69 coordinate convention is not guessed: for a source on the
 * left, the left ear must receive the signal earlier (positive ITD) and louder
 * (positive ILD). This tool prints the measured peak of each ear so that can be
 * checked, and fails loudly if the two ears are identical - which would mean the
 * extraction silently produced two mono copies and every later measurement of
 * "the 7.1 works" would have been meaningless.
 *
 * Output: one little-endian f32 blob, 16 filters of N taps, in the order
 * FL_FR FL_LR FR_FR FR_LR FC_FR FC_LR ... (right ear first, as libmysofa
 * returns it), preceded by a fixed-size ASCII header.
 */
#include <mysofa.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

#define TAPS 256
#define RATE 48000.0f
#define RADIUS 1.0f

/* Azimuth in degrees, positive to be checked against the ITD. */
static const struct { const char *name; float az; int is_lfe; } LAYOUT[] = {
    { "FL",  30.0f, 0 }, { "FR", -30.0f, 0 },
    { "FC",   0.0f, 0 },
    { "SL", 110.0f, 0 }, { "SR",-110.0f, 0 },
    { "BL", 150.0f, 0 }, { "BR",-150.0f, 0 },
};

/* Cartesian for a spherical direction, listener at the origin facing -Z. */
static void dir(float az_deg, float c[3]) {
    float a = az_deg * (float)M_PI / 180.0f;
    c[0] = RADIUS * cosf(a);
    c[1] = RADIUS * sinf(a);
    c[2] = -RADIUS;
}

static int peak_index(const float *f, int n) {
    int best = 0;
    float bv = fabsf(f[0]);
    for (int i = 1; i < n; i++) {
        float v = fabsf(f[i]);
        if (v > bv) { bv = v; best = i; }
    }
    return best;
}

int main(int argc, char **argv) {
    if (argc < 3) {
        fprintf(stderr, "usage: %s <file.sofa> <out.f32>\n", argv[0]);
        return 2;
    }
    int len = 0, err = 0;
    struct MYSOFA_EASY *easy = mysofa_open(argv[1], RATE, &len, &err);
    if (!easy) {
        fprintf(stderr, "mysofa_open failed on %s: err=%d\n", argv[1], err);
        return 1;
    }
    if (len < TAPS) {
        fprintf(stderr, "file has only %d taps, need %d\n", len, TAPS);
        mysofa_close(easy);
        return 1;
    }

    static float out[7 * 2 * TAPS];
    static float delays[7 * 2];
    static float right[TAPS], left[TAPS];
    fprintf(stderr, "%-4s %8s %8s %8s %9s\n", "pos", "peakR", "peakL", "ITD_us", "ILD_dB");
    for (unsigned i = 0; i < sizeof(LAYOUT) / sizeof(LAYOUT[0]); i++) {
        float c[3];
        dir(LAYOUT[i].az, c);
        float dl = 0.0f, dr = 0.0f;
        mysofa_getfilter_float(easy, c[0], c[1], c[2], left, right, &dl, &dr);

        delays[i * 2 + 0] = dr;
        delays[i * 2 + 1] = dl;
        int pr = peak_index(right, len) + (int)lrintf(dr);
        int pl = peak_index(left, len) + (int)lrintf(dl);
        float ar = fabsf(right[pr]);
        float al = fabsf(left[pl]);
        float itd_us = (float)(pl - pr) * 1000000.0f / RATE;
        float ild_db = 20.0f * log10f((al > 1e-9f ? al : 1e-9f) /
                                      (ar > 1e-9f ? ar : 1e-9f));

        int same = 1;
        for (int k = 0; k < TAPS; k++) {
            if (fabsf(left[k] - right[k]) > 1e-7f) { same = 0; break; }
        }
        if (same) {
            fprintf(stderr, "  %s: the two ears are identical - extraction is wrong\n",
                    LAYOUT[i].name);
            mysofa_close(easy);
            return 1;
        }

        memcpy(out + (i * 2 + 0) * TAPS, right, sizeof(float) * TAPS);
        memcpy(out + (i * 2 + 1) * TAPS, left,  sizeof(float) * TAPS);
        fprintf(stderr, "%-4s %8.4f %8.4f %8.1f %9.2f\n",
                LAYOUT[i].name, ar, al, itd_us, ild_db);
    }
    mysofa_close(easy);

    FILE *f = fopen(argv[2], "wb");
    if (!f) { perror("fopen"); return 1; }
    fprintf(f, "EPOSSURROUND2 %d %d\n", 7 * 2, TAPS);
    fwrite(delays, sizeof(float), 14, f);
    fwrite(out, sizeof(float), (size_t)7 * 2 * TAPS, f);
    long n = ftell(f);
    fclose(f);
    fprintf(stderr, "wrote %ld bytes to %s\n", n, argv[2]);
    return 0;
}
