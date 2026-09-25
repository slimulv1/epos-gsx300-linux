/* Sweep the SOFA file across azimuth and report where it stops being usable.
 *
 * The concern from the first extraction: BL (150 deg) and BR (-150 deg) came back
 * almost mirror images of each other, which should not happen for a symmetric
 * measurement set. Either the dataset has no rear measurements and libmysofa is
 * interpolating from somewhere odd, or the convention is different at the rear.
 *
 * This walks azimuth from -180 to +180 in steps and prints ITD and ILD. A dataset
 * that is sound over some range is symmetric about zero there: ILD(a) == -ILD(-a)
 * and ITD(a) == -ITD(-a). Where that stops holding is where the data is not
 * measured, and no amount of interpolation turns an unmeasured angle into a real
 * HRIR.
 */
#include <mysofa.h>
#include <math.h>
#include <stdio.h>

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

#define TAPS 512
#define RATE 48000.0f

static void dir(float az_deg, float c[3]) {
    float a = az_deg * (float)M_PI / 180.0f;
    c[0] = cosf(a);
    c[1] = sinf(a);
    c[2] = -1.0f;
}

static int peak(const float *f, int n) {
    int best = 0; float bv = fabsf(f[0]);
    for (int i = 1; i < n; i++) if (fabsf(f[i]) > bv) { bv = fabsf(f[i]); best = i; }
    return best;
}

int main(int argc, char **argv) {
    if (argc < 2) { fprintf(stderr, "usage: %s <file.sofa>\n", argv[0]); return 2; }
    int len = 0, err = 0;
    struct MYSOFA_EASY *e = mysofa_open(argv[1], RATE, &len, &err);
    if (!e) { fprintf(stderr, "open failed err=%d\n", err); return 1; }

    static float L[TAPS], R[TAPS];
    float prev[361];  /* ITD by 1-degree bin, -180..180 */

    printf("  az     ITD_us   ILD_dB   peakL\n");
    for (int az = -180; az <= 180; az += 10) {
        float c[3];
        dir((float)az, c);
        float dl = 0, dr = 0;
        mysofa_getfilter_float(e, c[0], c[1], c[2], L, R, &dl, &dr);
        int pl = peak(L, len) + (int)lrintf(dl);
        int pr = peak(R, len) + (int)lrintf(dr);
        float itd = (float)(pl - pr) * 1000000.0f / RATE;
        float al = fabsf(L[peak(L, len)]), ar = fabsf(R[peak(R, len)]);
        float ild = 20.0f * log10f((al > 1e-9f ? al : 1e-9f) / (ar > 1e-9f ? ar : 1e-9f));
        prev[az + 180] = itd;
        printf("  %4d  %8.1f  %8.2f  %7.4f\n", az, itd, ild, al);
    }

    printf("\n  symmetry about 0 (|ITD(a) + ITD(-a)| should be small):\n");
    int first_bad = 999, worst = 0;
    for (int az = 10; az <= 180; az += 10) {
        float d = fabsf(prev[az + 180] + prev[-az + 180]);
        if (d > 120.0f) {   /* 120 us is well past any real ITD */
            if (az < first_bad) first_bad = az;
            if (d > worst) worst = (int)d;
        }
    }
    if (first_bad == 999) {
        printf("    symmetric at every 10 deg step tested\n");
    } else {
        printf("    first break at |az| = %d deg (worst mismatch %d us)\n", first_bad, worst);
    }
    mysofa_close(e);
    return 0;
}
