/* The descriptor layout, checked against the header PipeWire actually compiles.
 *
 * Nine separate ABI mistakes came out of this plugin, one at a time. Every one of
 * them passed the Rust unit tests, and every one of them passed a C harness that
 * shared the same hand-transcription of the LADSPA header - because a struct
 * declared wrongly in one place is declared wrongly in both, and the two
 * therefore agree with each other and with nothing else.
 *
 * So this includes the real header. Not a copy of it, and not a reading of the
 * published spec: `ladspa.h` as PipeWire 1.6.9 ships it, which differs from the
 * classic LADSPA 1.1 header in two ways that both segfault the host -
 *
 *   typedef float    LADSPA_Data;         not double
 *   typedef int      LADSPA_Properties;    not a struct of two pointers and a count
 *
 * and those two differences are worth more than everything else in this file
 * put together. `LADSPA_PortRangeHint` is therefore 12 bytes, not 20, and
 * writing it as 20 put every port's UpperBound 8 bytes out of step, so the host
 * read a bounds float where a name pointer belonged and dereferenced it.
 *
 * Fetch the header next to this file before building:
 *   curl -O https://raw.githubusercontent.com/PipeWire/pipewire/1.6.9/spa/plugins/filter-graph/ladspa.h
 *   cc -O2 -o abi abi.c -ldl
 *   ./abi ~/.local/lib/ladspa/epos-surround.so
 */
#include <dlfcn.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "ladspa.h"

static int fails;
#define CHECK(cond, ...) do { if (!(cond)) { printf("  FAIL  "); printf(__VA_ARGS__); printf("\n"); fails++; } } while (0)

int main(int argc, char **argv) {
    setvbuf(stdout, NULL, _IONBF, 0);
    if (argc < 2) { fprintf(stderr, "usage: %s <plugin.so>\n", argv[0]); return 2; }

    printf("  compiled against the header itself:\n");
    printf("    LADSPA_Data                 %zu bytes\n", sizeof(LADSPA_Data));
    printf("    LADSPA_Properties           %zu bytes\n", sizeof(LADSPA_Properties));
    printf("    LADSPA_PortDescriptor       %zu bytes\n", sizeof(LADSPA_PortDescriptor));
    printf("    LADSPA_PortRangeHint        %zu bytes\n", sizeof(LADSPA_PortRangeHint));
    printf("    LADSPA_Descriptor           %zu bytes\n", sizeof(LADSPA_Descriptor));
    CHECK(sizeof(LADSPA_Data) == sizeof(float),
          "this header must be the one PipeWire ships; if LADSPA_Data is not a "
          "float the header is the wrong revision and every offset below is moot");
    CHECK(offsetof(LADSPA_Descriptor, PortNames) == 0x40,
          "PortNames is at 0x%zx, not 0x40: wrong header revision",
          offsetof(LADSPA_Descriptor, PortNames));
    CHECK(offsetof(LADSPA_Descriptor, cleanup) == 0x90,
          "cleanup is at 0x%zx, not 0x90: wrong header revision",
          offsetof(LADSPA_Descriptor, cleanup));

    void *h = dlopen(argv[1], RTLD_NOW);
    if (!h) { fprintf(stderr, "  dlopen: %s\n", dlerror()); return 1; }
    LADSPA_Descriptor_Function fn =
        (LADSPA_Descriptor_Function)dlsym(h, "ladspa_descriptor");
    CHECK(fn != NULL,
          "PipeWire's filter-graph only dlsym()s \"ladspa_descriptor\"; a plugin "
          "exporting only the 1.1 \"lsap_entry\" is refused with -ENOSYS");
    if (!fn) { printf("\n  %d failure(s)\n", fails); return 1; }

    const LADSPA_Descriptor *d = fn(0);
    CHECK(d != NULL, "ladspa_descriptor(0) is NULL");
    if (!d) { printf("\n  %d failure(s)\n", fails); return 1; }
    CHECK(fn(1) == NULL, "ladspa_descriptor(1) must be NULL; a host walks until NULL");

    printf("\n  plugin: %lu ports, Label \"%s\"\n", d->PortCount, d->Label ? d->Label : "(null)");
    CHECK(d->Label && d->Name && d->Maker && d->Copyright,
          "Label/Name/Maker/Copyright must be non-NULL per the header");
    CHECK(d->PortDescriptors && d->PortNames && d->PortRangeHints,
          "the three parallel port arrays must all be populated");
    CHECK(d->PortCount > 0, "PortCount must be positive");

    for (unsigned long i = 0; i < d->PortCount; i++) {
        const char *n = d->PortNames[i];
        CHECK(n != NULL, "PortNames[%lu] is NULL", i);
        int f = d->PortDescriptors[i];
        CHECK((f & 0x1) ^ (f & 0x2),
              "port %lu ('%s') flags 0x%x: must be input XOR output", i, n ? n : "?", f);
        CHECK((f & 0x4) ^ (f & 0x8),
              "port %lu ('%s') flags 0x%x: must be control XOR audio", i, n ? n : "?", f);
        const LADSPA_PortRangeHint *r = &d->PortRangeHints[i];
        CHECK(r->LowerBound <= r->UpperBound,
              "port %lu ('%s') range %f..%f is not ordered",
              i, n ? n : "?", (double)r->LowerBound, (double)r->UpperBound);
        if (r->HintDescriptor & LADSPA_HINT_BOUNDED_BELOW)
            CHECK(r->LowerBound >= 0.0f,
                  "port %lu ('%s') claims a lower bound and gives a negative one",
                  i, n ? n : "?");
    }

    /* The functions the host calls on every graph build, every buffer, and on
     * teardown. A shifted pointer here is the failure that produced correct
     * audio and then a segfault on the way out. */
    CHECK(d->instantiate != NULL, "instantiate is NULL");
    CHECK(d->connect_port != NULL, "connect_port is NULL");
    CHECK(d->run != NULL, "run is NULL");
    CHECK(d->cleanup != NULL, "cleanup is NULL");

    printf("  ports:");
    for (unsigned long i = 0; i < d->PortCount; i++) printf(" %s", d->PortNames[i]);
    printf("\n\n  %s (%d failure%s)\n", fails ? "FAILURES" : "layout is correct",
           fails, fails == 1 ? "" : "s");
    return fails ? 1 : 0;
}
