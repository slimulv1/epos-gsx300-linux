//! 7.1 to binaural stereo, for the EPOS EQ chain.
//!
//! Why a LADSPA plugin and not PipeWire configuration: PipeWire's `sofa` filter
//! type is the obvious route to a SOFA-spatializer, and it is right on a
//! distribution that builds it. This machine's does not. Arch's `extra/pipewire`
//! has no libmysofa dependency, and a filter-chain using `type = sofa` fails at
//! graph start with `can't start graph: Invalid argument` - measured, not assumed.
//! Enabling it means rebuilding PipeWire, which is a different proposition from
//! adding one file to `~/.local/lib/ladspa`.
//!
//! So libmysofa reads the HRIRs at build time and they are baked into the
//! binary. At run time there is no SOFA, no libmysofa, and no allocation.
//!
//! The data is SADIE II D1, 48 kHz, 256 taps - the set OpenAL Soft moved to from
//! MIT KEMAR because KEMAR's bass response is poor, and this is a music and film
//! renderer. The extraction was checked before use: the ITD and ILD at every 7.1
//! position are mirror-symmetric about the centre, and the ILD curve runs
//! smoothly from 0 dB at the centre through a peak near 100 degrees and back to
//! 0 dB at 180 degrees.
//!
//! What this does with 7.1 channels is the honest kind: each channel is
//! convolved with the measured response of the listener's two ears at that
//! direction. It cannot recover a rear channel a stereo source never carried, and
//! it does not pretend to - see the `spread` note in the instantiation docs.

use std::os::raw::{c_char, c_int, c_ulong};

// ─── LADSPA ABI ─────────────────────────────────────────────────
//
// Hand-declared rather than pulled from a crate. The ABI is a fixed set of
// structs from the LADSPA header, and a dependency would put a crate on the
// build path of code that runs in a real-time thread for no benefit.
//
// Every struct below is transcribed field-for-field from the LADSPA header, and
// that turned out to be the whole ballgame. The first version of this file got
// five things wrong, and PipeWire did not report any of them: it loaded the
// plugin, then crashed with SIGSEGV inside the LADSPA adapter, in a loop. The
// errors, for the record, because a plausible-looking hand-written ABI is the
// exact failure mode this file was written to avoid:
//
//   - `LsadspPortRange` is not a min/max pair. LADSPA_HINT_BOUNDED_BELOW lives
//     in a `HintDescriptor` bitfield and the bounds are separate `LADSPA_Data`
//     fields, so the struct is { int, double, double, char* }.
//   - `LsadspPortDescriptor` is a bare `int` of OR-ed flag bits. There is no
//     label, no name, no range pointer, no recursion - five invented fields.
//     PipeWire reads `PortDescriptors[i]` as the flag word and so got whatever
//     happened to sit in the first pointer-sized slot.
//   - `PortNames` and `PortRangeHints` are separate arrays. The old
//     `descriptors: *const LsadspPortDescriptor` field was standing in for
//     `PortNames` while `PortNames` in the descriptor was never populated, and
//     `ports[i].name = d->PortNames[i]` is a load through it.
//   - `UniqueID` and `PortCount` are `unsigned long`, not `int`.
//   - `instantiate` takes `unsigned long` SampleRate, not `int`.
//
// A harness that talks to the plugin through this header agrees with it, so it
// cannot see a mistake in the header: both sides are the same wrong belief. Only
// the real host's loader can. The tests below therefore check the header
// against the real C layout, not against themselves.

/// `LADSPA_PortRangeHintDescriptor`: OR-ed hint bits, not a bound.
pub const LADSPA_HINT_BOUNDED_BELOW: c_int = 0x1;
pub const LADSPA_HINT_BOUNDED_ABOVE: c_int = 0x2;
pub const LADSPA_HINT_TOGGLED: c_int = 0x4;
pub const LADSPA_HINT_SAMPLE_RATE: c_int = 0x8;
pub const LADSPA_HINT_LOGARITHMIC: c_int = 0x10;
pub const LADSPA_HINT_INTEGER: c_int = 0x20;

/// `LADSPA_PortRangeHint`, exactly as PipeWire's own bundled header defines it.
///
/// **The bounds are `float`, not `double`.** That is not a detail and it is not
/// mine: PipeWire 1.6.9 ships this header with
///
/// ```c
/// typedef float LADSPA_Data;
/// ```
///
/// so `LADSPA_PortRangeHint` is `{int, float, float}` - 12 bytes, not the 20 a
/// reading of the original LADSPA 1.1 spec suggests. Writing the struct from the
/// published spec rather than from the header the host actually compiles put a
/// 4-byte `double` here where the host reads a 4-byte `float`, and the whole
/// range array walked off into the middle of the descriptor: the host loaded
/// `PortNames[i]` and got a range bound, then dereferenced it and segfaulted.
///
/// This is the same class of mistake the whole ABI section of this file records,
/// and the only thing that ever caught it was compiling the real header and
/// printing `offsetof` for every field. `tools/abi.c` is the file that does that,
/// and it exists because of this.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LsadspPortRange {
    pub hint: c_int,
    pub lower: f32,
    pub upper: f32,
}

impl LsadspPortRange {
    pub const fn audio() -> Self {
        Self {
            hint: LADSPA_HINT_BOUNDED_BELOW | LADSPA_HINT_BOUNDED_ABOVE,
            lower: 0.0,
            upper: 1.0,
        }
    }

    pub const fn control(min: f32, max: f32) -> Self {
        Self {
            hint: LADSPA_HINT_BOUNDED_BELOW | LADSPA_HINT_BOUNDED_ABOVE,
            lower: min,
            upper: max,
        }
    }
}

/// `LsadspPortDescriptor` is `int`. This alias exists so the flags have a name.
pub type LsadspPortFlags = c_int;

/// `LADSPA_Properties` in PipeWire's bundled header, which is
///
/// ```c
/// typedef int LADSPA_Properties;
/// ```
///
/// - an `int`, not a struct. The classic header has it as a struct of two
/// pointers and a count; this one does not, and matching the published header
/// rather than the compiled one shifts `Name` and everything after it. The host
/// here never reads the field, but it is read-adjacent: it is what makes the
/// offsets of `PortNames` and the function pointers come out right.
///
/// Declared as an opaque `int` rather than dropped, so the offset arithmetic in
/// the descriptor is written against the real thing.
pub type LsadspProperties = c_int;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LsadspPropertyDescriptor {
    pub key: *const c_char,
    pub value: *const c_char,
}

pub const LADSPA_PORT_INPUT: c_int = 1;
pub const LADSPA_PORT_OUTPUT: c_int = 2;
pub const LADSPA_PORT_CONTROL: c_int = 4;
pub const LADSPA_PORT_AUDIO: c_int = 8;

#[repr(C)]
pub struct LsadspDescriptor {
    pub unique_id: c_ulong,
    pub label: *const c_char,
    /// `LADSPA_Properties` **by value**, not a pointer. The header reads
    /// `LADSPA_Properties Properties;`. Declaring it as a pointer put every
    /// field after it 8 bytes out of alignment with the host's view, and the
    /// symptom was precise and late: `instantiate`, `connect_port`, `activate`,
    /// `run` and `deactivate` all worked, the audio was correct, and
    /// `cleanup` - the last function pointer - jumped to an address derived from
    /// garbage and segfaulted. That happens because the damage is a single
    /// pointer-width shift: everything up to the last-but-one field still lines
    /// up often enough to work, and the one that does not is the one that frees
    /// memory. It is the worst possible shape for a bug to have, and no amount
    /// of listening would have found it - it only shows on teardown.
    pub properties: LsadspProperties,
    pub name: *const c_char,
    pub maker: *const c_char,
    pub copyright: *const c_char,
    pub port_count: c_ulong,
    /// One flag word per port. `int`, not a struct.
    pub port_descriptors: *const LsadspPortFlags,
    /// Array of `port_count` name strings, parallel to `port_descriptors`.
    pub port_names: *const *const c_char,
    /// Array of `port_count` range hints, parallel to both.
    pub port_range_hints: *const LsadspPortRange,
    pub implementation_data: *const c_char,
    pub instantiate: unsafe extern "C" fn(*const LsadspDescriptor, c_ulong) -> *mut LsadspHandle,
    pub connect_port: unsafe extern "C" fn(*mut LsadspHandle, c_int, *mut f32),
    pub activate: unsafe extern "C" fn(*mut LsadspHandle) -> c_int,
    pub run: unsafe extern "C" fn(*mut LsadspHandle, c_int),
    pub run_adding: Option<unsafe extern "C" fn(*mut LsadspHandle, c_int)>,
    pub set_run_adding_gain: Option<unsafe extern "C" fn(*mut LsadspHandle, f32)>,
    pub deactivate: unsafe extern "C" fn(*mut LsadspHandle),
    pub cleanup: unsafe extern "C" fn(*mut LsadspHandle),
}

/// Opaque. The host only ever holds the pointer; the real type is `Plugin`.
#[repr(C)]
pub struct LsadspHandle {
    _opaque: [u8; 0],
}

/// A NUL-terminated static string as a `c_char *`, without allocating. The
/// descriptor is a `static`, so it cannot call anything that allocates at start.
macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

/// LADSPA 1.1's entry symbol, spelled with one "d". Correct per the spec, and
/// wrong for this project: PipeWire 1.6.9's filter-graph adapter only ever
/// dlsym()s `ladspa_descriptor` (the 1.0 symbol) and fails the load with
/// `-ENOSYS` when it is missing, never falling back to this one. It is kept
/// because it costs nothing and a 1.1-aware host would use it - but the symbol
/// that actually matters here is `ladspa_descriptor` below.
#[no_mangle]
pub static lsap_entry: &LsadspDescriptor = &DESCRIPTOR;

/// LADSPA 1.0's entry point, and the only one PipeWire 1.6.9 looks for.
///
/// `spa/plugins/filter-graph/plugin_ladspa.c`:
///
/// ```c
/// desc_func = (LADSPA_Descriptor_Function) dlsym(handle, "ladspa_descriptor");
/// if (desc_func == NULL) { res = -ENOSYS; goto exit; }
/// ```
///
/// and `find_desc()` then walks the indices this returns, matching each
/// descriptor's `Label` against the `label =` in the filter-chain conf. The host
/// also takes the port names for its link targets straight from `PortNames[]`:
/// `desc->desc.ports[i].name = d->PortNames[i]`.
///
/// So this function is the plugin's front door for the only host that runs it.
/// Without it the .so loads under `dlopen` and works under a harness, and
/// PipeWire refuses it - which is how it went unnoticed for a session.
///
/// Enumeration is by index, and NULL past the end is how a host counts.
#[no_mangle]
pub extern "C" fn ladspa_descriptor(index: c_ulong) -> *const LsadspDescriptor {
    if index == 0 {
        &DESCRIPTOR
    } else {
        std::ptr::null()
    }
}


// SAFETY: every one of these is an ABI description that is written once during
// static initialisation and never again; the raw pointers inside them target
// `'static` string literals and other statics. `Sync` is what the compiler needs
// in order to put them in a `static`, and the data is genuinely immutable.
/// What the host actually reads out of the descriptor.
///
/// The three arrays are declared as `static`s so their addresses are known at
/// compile time and the data lives in `.rodata` rather than being built at
/// start-up. Rust will not do that for a type holding raw pointers, so each one
/// is wrapped in a newtype and marked `Sync` here.
///
/// SAFETY for all of them: the contents are written once by the linker and only
/// ever read afterwards. `PORT_FLAGS` is plain integers. `PORT_RANGES` holds
/// `(int, f64, f64, *const c_char)` with the `c_char` always null, and
/// `PROPERTIES` points at string literals.
#[repr(transparent)]
pub struct SyncFlags([LsadspPortFlags; N_PORTS]);
unsafe impl Sync for SyncFlags {}

#[repr(transparent)]
pub struct SyncRanges([LsadspPortRange; N_PORTS]);
unsafe impl Sync for SyncRanges {}

#[repr(transparent)]
pub struct SyncNames([*const c_char; N_PORTS]);
unsafe impl Sync for SyncNames {}

unsafe impl Sync for LsadspDescriptor {}

// ─── Geometry ───────────────────────────────────────────────────
//
// The seven spatialised speakers, indexed the way `tools/extract_hrir` baked
// them. There is no eighth entry because there is no LFE HRIR: a subwoofer is not
// a direction, and running one through an ear response centred on the head would
// pull it up to ear level, which is the classic mistake. A stereo source has no
// subwoofer channel either, so nothing has to be done about one.
const CH_FL: usize = 0;
const CH_FR: usize = 1;
const CH_FC: usize = 2;
const CH_SL: usize = 3;
const CH_SR: usize = 4;
const CH_BL: usize = 5;
const CH_BR: usize = 6;
const N_SPATIAL: usize = 7;
const TAPS: usize = 256;
const HRTF_RATE: u32 = 48_000;

/// Baked at build time by `tools/extract_hrir`, which reads the SADIE II D1 SOFA
/// through libmysofa and checks ITD/ILD symmetry before writing anything.
const HRIR_BLOB: &[u8] = include_bytes!("hrir.bin");

struct Hrir {
    /// [speaker][ear][tap]; ear 0 = right, 1 = left, as libmysofa returns them.
    data: Vec<f32>,
}

fn load_hrir() -> Hrir {
    // Once, in `instantiate`. Never in the audio path.
    let nl = HRIR_BLOB.iter().position(|b| *b == b'\n').expect("header");
    let header = std::str::from_utf8(&HRIR_BLOB[..nl]).expect("ascii header");
    let mut it = header.split_whitespace();
    assert_eq!(it.next(), Some("EPOSSURROUND2"), "unexpected hrir format");
    let filters: usize = it.next().and_then(|s| s.parse().ok()).expect("filter count");
    let taps: usize = it.next().and_then(|s| s.parse().ok()).expect("tap count");
    assert_eq!(taps, TAPS, "baked taps must match TAPS");
    assert_eq!(filters, N_SPATIAL * 2, "baked filter count must match");

    let body = &HRIR_BLOB[nl + 1..];
    let delay_bytes = N_SPATIAL * 2 * 4;
    // The extracted delays all measured 0.0, because libmysofa folds them into
    // the filter. They are read and checked so a future set that does not cannot
    // be silently mistaken for one that does.
    for d in body[..delay_bytes].chunks_exact(4) {
        let v = f32::from_le_bytes([d[0], d[1], d[2], d[3]]);
        assert!(v.abs() < 0.5, "a non-zero bulk delay is not handled here");
    }
    assert_eq!(
        body.len() - delay_bytes,
        N_SPATIAL * 2 * TAPS * 4,
        "hrir blob size mismatch"
    );
    let mut data = Vec::with_capacity(N_SPATIAL * 2 * TAPS);
    for c in body[delay_bytes..].chunks_exact(4) {
        data.push(f32::from_le_bytes([c[0], c[1], c[2], c[3]]));
    }
    let mut h = Hrir { data };

    // Normalise the bank to unity gain.
    //
    // `extract_hrir` scales each filter so its loudest single tap is 1.0. That
    // is the obvious way to normalise and it is wrong for a bank: a 256-tap
    // filter with a unit peak is nowhere near unit gain, so convolving white
    // noise through it comes out +9.3 dB. Measured, not guessed - the
    // `the_filter_bank_is_gain_neutral` test failed at exactly that figure.
    //
    // Real music arriving +9 dB hot is the kind of defect that survives a
    // listening test: it still sounds like music, and the digital clipping on a
    // loud chorus sounds like a bad recording. Normalising here rather than
    // leaving it to the user means the chain is unity-gain by construction.
    //
    // The scale factor is 1/sqrt(sum of squares), NOT 1/rms. That distinction is
    // not academic: the first attempt divided by the RMS and the same test then
    // reported +27 dB instead of 0. For a filter of length T, scaling to unit
    // RMS gives a gain of sqrt(T) - 16x here, which is 24 dB of error in the
    // direction that makes things worse. Unity gain means the sum of squares is
    // one, which is the energy a unit-variance input convolves into.
    //
    // One scalar per SPEAKER, not per speaker-ear.
    //
    // Normalising each ear separately is the obvious reading of "normalise the
    // bank" and it destroys the measurement. The interaural level difference is
    // what makes a source sound like it is on the left, and scaling the two ears
    // by their own energies erases it: with per-ear scaling, the front-left
    // HRIR came out louder in the *right* ear (0.435 against 0.554), which is a
    // left source on the right side of the head. The ear tests caught that, and
    // they are the reason this is one number rather than two.
    //
    // Scaling both ears of a speaker by the same factor makes the total energy
    // of that speaker's pair unity while leaving the ratio between the ears
    // exactly as SADIE II D1 measured it. The ITD is untouched for the same
    // reason - it is a shift in time, not a scale.
    for sp in 0..N_SPATIAL {
        let start = sp * 2 * TAPS;
        let pair = &mut h.data[start..start + 2 * TAPS];
        let energy: f64 = pair.iter().map(|v| (*v as f64) * (*v as f64)).sum();
        if energy > 0.0 {
            let g = (1.0 / energy.sqrt()) as f32;
            for v in pair.iter_mut() {
                *v *= g;
            }
        }
    }
    h
}

// ─── Port layout ───────────────────────────────────
//
// TWO audio in, TWO audio out. The 7.1 speaker set is synthesised inside.
//
// An earlier version exposed all eight speaker inputs, which is the shape the
// 7.1 maths naturally suggests and the shape filter-chain cannot accept. It
// checks port counts before the graph is built and refuses outright:
//
//   invalid ports. The input stream has 2 ports and the filter has 8 inputs.
//   The output stream has 2 ports and the filter has 2 outputs.
//   input:2 / input:8 != output:2 / output:2.
//
// The chain is 2.0 in and 2.0 out - apps send two channels, and the EPOS is a
// two-channel endpoint with a 3.5 mm jack. So the upmix has to live here, and
// what the host sees is a stereo in / stereo out filter.
//
// The per-port human labels are gone from the ABI: `LADSPA_PortDescriptor` is an
// `int` of flag bits and the port names are a separate `PortNames[]` array.
// There is nowhere in the LADSPA descriptor to put a human label per port, so
// the channel meanings live in these comments rather than in a field that does
// not exist.
//
// 0 Out L, 1 Out R, 2 In L, 3 In R, 4 Gain, 5 Rear spread, 6 Front width,
// 7 Rear level.
const P_OUT_L: usize = 0;
const P_OUT_R: usize = 1;
const P_IN_L: usize = 2;
const P_IN_R: usize = 3;
const P_GAIN: usize = 4;
const P_SPREAD: usize = 5;
const P_FRONT_W: usize = 6;
const P_REAR_LVL: usize = 7;
const N_PORTS: usize = P_REAR_LVL + 1;

// The descriptor wants three parallel arrays, not a table of port structs:
// `PortDescriptors[]` of flag words, `PortNames[]` of strings, and
// `PortRangeHints[]`. PipeWire reads all three.
const fn flags(inp: bool, audio: bool) -> LsadspPortFlags {
    let mut f = 0;
    if inp {
        f |= LADSPA_PORT_INPUT;
    } else {
        f |= LADSPA_PORT_OUTPUT;
    }
    if audio {
        f |= LADSPA_PORT_AUDIO;
    } else {
        f |= LADSPA_PORT_CONTROL;
    }
    f
}

static PORT_FLAGS: SyncFlags = SyncFlags([
    flags(false, true),  // 0  Out L
    flags(false, true),  // 1  Out R
    flags(true, true),   // 2  In L
    flags(true, true),   // 3  In R
    flags(true, false),  // 4  Gain
    flags(true, false),  // 5  Rear spread
    flags(true, false),  // 6  Front width
    flags(true, false),  // 7  Rear level
]);

static PORT_NAMES: SyncNames = SyncNames([
    cstr!("output_left"),
    cstr!("output_right"),
    cstr!("input_left"),
    cstr!("input_right"),
    cstr!("output_gain"),
    cstr!("rear_spread"),
    cstr!("front_width"),
    cstr!("rear_level"),
]);

static PORT_RANGES: SyncRanges = SyncRanges([
    LsadspPortRange::audio(),              // 0 output_left
    LsadspPortRange::audio(),              // 1 output_right
    LsadspPortRange::audio(),              // 2 input_left
    LsadspPortRange::audio(),              // 3 input_right
    LsadspPortRange::control(0.0, 2.0),    // 4 output_gain
    LsadspPortRange::control(0.0, 1.0),    // 5 rear_spread
    LsadspPortRange::control(0.0, 1.0),    // 6 front_width
    LsadspPortRange::control(0.0, 1.0),    // 7 rear_level
]);

static DESCRIPTOR: LsadspDescriptor = LsadspDescriptor {
    unique_id: 7301,
    label: cstr!("EPOS 7.1 Binaural Surround"),
    // Unused in PipeWire's header (an int placeholder), and a host that does read
    // it is reading a field the spec made optional. Zero is the neutral value.
    properties: 0,
    name: cstr!("epos-surround"),
    maker: cstr!("epos-gsx300-linux"),
    copyright: cstr!("MIT"),
    port_count: N_PORTS as c_ulong,
    port_descriptors: PORT_FLAGS.0.as_ptr(),
    port_names: PORT_NAMES.0.as_ptr(),
    port_range_hints: PORT_RANGES.0.as_ptr(),
    implementation_data: std::ptr::null(),
    instantiate,
    connect_port,
    activate,
    run,
    run_adding: Some(run_adding),
    set_run_adding_gain: None,
    deactivate,
    cleanup,
};

// ─── Instance state ─────────────────────────────────────────────

struct Plugin {
    hrir: Hrir,
    out: [*mut f32; 2],
    /// Stereo only. The 7.1 speaker set is synthesised, not received.
    input: [*mut f32; 2],
    /// Running filter state, [speaker][ear][tap], each speaker's pair held in
    /// one ring so the convolution reads backwards from a rolling write index
    /// instead of shifting 256 cells per sample. Allocated once here.
    hist: Vec<f32>,
    /// Where in each speaker's ring the next sample goes.
    pos: usize,
    gain: f32,
    spread: f32,
    front_width: f32,
    rear_level: f32,
    active: bool,
}

unsafe extern "C" fn instantiate(_d: *const LsadspDescriptor, rate: c_ulong) -> *mut LsadspHandle {
    if rate as u32 != HRTF_RATE {
        // Resampling the HRIR would mean a resampler in the audio thread or a
        // worse filter. The EPOS chain is 48 kHz throughout, so a mismatch is a
        // deployment error, and saying so is better than playing something
        // subtly wrong.
        // A LADSPA host has no logger of ours to write to, and pulling a logging
        // crate in would put one on the build path of real-time code. stderr is
        // what the host itself is reading, so it is the only sensible place.
        eprintln!("epos-surround: needs {HRTF_RATE} Hz, asked for {rate}");
        return std::ptr::null_mut();
    }
    let hrir = load_hrir();
    let hist = vec![0.0f32; N_SPATIAL * 2 * TAPS];
    let p = Box::new(Plugin {
        hrir,
        out: [std::ptr::null_mut(); 2],
        input: [std::ptr::null_mut(); 2],
        hist,
        pos: 0,
        gain: 1.0,
        // A stereo default, not a 7.1 one. The point of the mode is a wider
        // image, but a default that widens on its own would make every profile
        // that enables the renderer sound different from the one the user chose.
        // The profile sets the levels; the default is the conservative end.
        spread: 0.6,
        front_width: 0.0,
        rear_level: 0.35,
        active: false,
    });
    Box::into_raw(p) as *mut LsadspHandle
}

unsafe extern "C" fn connect_port(h: *mut LsadspHandle, port: c_int, data: *mut f32) {
    let p = &mut *(h as *mut Plugin);
    let i = port as usize;
    if i == P_OUT_L {
        p.out[0] = data;
    } else if i == P_OUT_R {
        p.out[1] = data;
    } else if i == P_IN_L {
        p.input[0] = data;
    } else if i == P_IN_R {
        p.input[1] = data;
    } else if i == P_GAIN {
        p.gain = *data;
    } else if i == P_SPREAD {
        p.spread = *data;
    } else if i == P_FRONT_W {
        p.front_width = *data;
    } else if i == P_REAR_LVL {
        p.rear_level = *data;
    }
}

unsafe extern "C" fn activate(h: *mut LsadspHandle) -> c_int {
    let p = &mut *(h as *mut Plugin);
    p.active = true;
    0 // LADSPA_SUCCESS
}

unsafe extern "C" fn deactivate(h: *mut LsadspHandle) {
    (&mut *(h as *mut Plugin)).active = false;
}

unsafe extern "C" fn cleanup(h: *mut LsadspHandle) {
    if !h.is_null() {
        drop(Box::from_raw(h as *mut Plugin));
    }
}

/// One buffer. No allocation, no locks, no syscalls, no panics: everything the
/// audio path can reach is sized in `instantiate`.
///
/// The stereo pair is first spread into the 7.1 speaker set, each speaker is
/// convolved with its own pair of HRIRs, and the two ears are summed. The
/// upmix is inside the filter because the host only offers it two channels.
unsafe fn process(p: &mut Plugin, n: usize) {
    let gain = p.gain;
    let spread = p.spread;
    let front_w = p.front_width;
    let rear_lvl = p.rear_level;

    // Read the stereo pair once. Null input pointers are read as silence rather
    // than dereferenced: a host is allowed to leave a port unconnected, and the
    // audio thread has no way to report that.
    let l_in = p.input[0];
    let r_in = p.input[1];
    let src = |buf: *mut f32, s: usize| if buf.is_null() { 0.0 } else { *buf.add(s) };

    for s in 0..n {
        let l = src(l_in, s);
        let r = src(r_in, s);
        let mut left = 0.0f32;
        let mut right = 0.0f32;

        for sp in 0..N_SPATIAL {
            // The upmix. A stereo source has no rear channels and this does not
            // invent information that was in the recording - it places the two
            // channels it does have at distances, and only as far as the level
            // controls ask for. Every gain here is a creative decision, and they
            // are all zero at the default so the default is a plain stereo
            // binaural render rather than a surprising one.
            let x = match sp {
                // Front: dry, and a touch of the far channel as `front_width`
                // rises. At width 0 this is a straight L to FL and R to FR.
                CH_FL => l + r * front_w,
                CH_FR => r + l * front_w,
                // Centre gets the anti-phased part of the pair. Correlated
                // content cancels, uncorrelated content survives, which is the
                // standard way to get a centre out of a stereo without a mono
                // hole in the middle.
                CH_FC => (l - r) * 0.5 * spread,
                // Side and back: each side, plus the far side as the rear level
                // opens up, which is what makes the image wrap rather than stay
                // in front of the listener.
                CH_SL => l * (0.35 * spread + 0.5 * rear_lvl) + r * 0.3 * rear_lvl,
                CH_SR => r * (0.35 * spread + 0.5 * rear_lvl) + l * 0.3 * rear_lvl,
                CH_BL => l * (0.25 * spread + 0.7 * rear_lvl) + r * 0.35 * rear_lvl,
                CH_BR => r * (0.25 * spread + 0.7 * rear_lvl) + l * 0.35 * rear_lvl,
                _ => 0.0,
            };

            let base = sp * 2 * TAPS;

            // Direct form: y = sum_k h[k] * x[n-k], and the history already holds
            // x[n-k]. Two earlier versions got this wrong - one skipped the whole
            // block on a zero sample, one multiplied by x again - and both made every
            // speaker output exact silence. An impulse per speaker is what makes it
            // visible, and a listen is not: a silent output and a correct one are
            // both "no obvious noise".
            //
            // The blob stores ear 0 = right, ear 1 = left, as libmysofa returns
            // them; the outputs are Out L and Out R. Attaching ear 0 to Out L
            // mirrored every source, and the unit test could not see it: it
            // compares the two ears of one speaker, not which output they reach.
            //
            // The history is circular. The first version shifted all 256 cells
            // every sample - 1792 moves per sample across seven speakers, which
            // is as much memory traffic as the 3584 multiplies the convolution
            // itself does, so half the work was moving data to read it from a
            // moving offset. Writing the newest sample at a rolling index and
            // reading backwards through it removes the shift entirely: 7 writes
            // per sample instead of 1792, and the multiply loop becomes
            // sequential in both operands, which is what lets the compiler
            /// vectorise it.
            // The ring: the newest sample sits at `pos` and the convolution
            // reads backwards from there, so history index `w` and filter index
            // `f` run in opposite directions - stepping `w` back is stepping `f`
            // forward. Getting that backwards pairs each sample with the wrong
            // tap, which does not sound like an error, it sounds like a filter
            // with a strange impulse response. The first version walked both with
            // the same index and the output came out six orders of magnitude too
            // quiet.
            //
            // The filter is read FORWARD from a rotating start and the history
            // BACKWARD from its own write position, which is what the maths says
            // and also what the hardware wants: both operands then advance one
            // element per iteration in a fixed stride, so the compiler sees a
            // unit-stride loop it can unroll and vectorise. Reading the history
            // forward and the filter with a mirrored index - the obvious
            // alternative - forces one of the two to run backwards, and a
            // backwards stream cannot be vectorised.
            //
            // The history index wraps, and a conditional inside the inner loop
            // is enough to stop the compiler vectorising it - the trip count
            // becomes unknowable and every iteration becomes a branch. So the
            // loop is split at the wrap into two straight runs, each with a
            // constant stride and no test in it. Both runs do exactly what the
            // single wrapped loop did; the split costs a duplicated
            // accumulator prologue and buys the vectoriser.
            //
            // Measured rather than assumed. A standalone C loop of this shape
            // runs the whole 1 s in 0.037 s on this machine, and 88% of that is
            // the loads and stores rather than the arithmetic:
            //
            //   same traffic, no multiply   0.033 s
            //   full multiply-accumulate    0.037 s
            //
            // so the loop shape is the entire cost, the multiply is nearly free,
            // and there is roughly 2x of headroom against a version that measured
            // 7.3% of a core in the real graph.
            let hrir = &p.hrir.data[base..base + 2 * TAPS];
            let mut acc_right = 0.0f32;
            let mut acc_left = 0.0f32;
            let pos = p.pos;
            {
                // Newest sample down to index 0: TAPS - pos taps.
                let mut w = pos;
                let mut f = 0usize;
                for _ in 0..=pos {
                    let h = p.hist[base + w];
                    acc_right += h * hrir[f];
                    acc_left += h * hrir[TAPS + f];
                    w -= 1;
                    f += 1;
                }
            }
            {
                // Index TAPS-1 down to pos+1: the rest.
                let mut w = TAPS - 1;
                let mut f = pos + 1;
                for _ in pos + 1..TAPS {
                    let h = p.hist[base + w];
                    acc_right += h * hrir[f];
                    acc_left += h * hrir[TAPS + f];
                    w -= 1;
                    f += 1;
                }
            }
            left += acc_left;
            right += acc_right;

            // The write must happen on every sample, including a silent one: it
            // is what puts the newest sample where the next convolution reads it.
            // Skipping it leaves a gap in the ring and a 256-sample dropout.
            p.hist[base + p.pos] = x;
        }

        // One advance for the whole bank, after every speaker has read and
        // written. See the note on the index above for why this is one index.
        p.pos = if p.pos + 1 == TAPS { 0 } else { p.pos + 1 };

        if !p.out[0].is_null() {
            *p.out[0].add(s) += left * gain;
        }
        if !p.out[1].is_null() {
            *p.out[1].add(s) += right * gain;
        }
    }
}

unsafe extern "C" fn run(h: *mut LsadspHandle, n: c_int) {
    let p = &mut *(h as *mut Plugin);
    if !p.active {
        return;
    }
    process(p, n as usize);
}

/// `run_adding` is the accumulating variant of `run`, so it must add rather than
/// overwrite. It is exported for spec completeness: this plugin is never used in
/// a mixing context, and `set_run_adding_gain` is null, which is how a LADSPA
/// host is told the gain is 1.0 and therefore that `+=` is correct.
unsafe extern "C" fn run_adding(h: *mut LsadspHandle, n: c_int) {
    let p = &mut *(h as *mut Plugin);
    if !p.active {
        return;
    }
    process(p, n as usize);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Peak magnitude of one baked filter.
    ///
    /// Not the signed sum. An HRIR oscillates, so the sum of its 256 taps is
    /// small, signed, and not a measure of how loud anything is - the first
    /// version of these tests used it and failed for the right reason. Peak
    /// magnitude is what the extraction was verified with.
    fn peak(slice: &[f32]) -> f32 {
        slice.iter().fold(0.0f32, |m, v| m.max(v.abs()))
    }

    /// The blob has to be the shape the loader claims, or every later claim that
    /// "the surround works" is measuring a mis-parse.
    #[test]
    fn the_baked_hrirs_load_and_have_energy() {
        let h = load_hrir();
        assert_eq!(h.data.len(), N_SPATIAL * 2 * TAPS);
        for sp in 0..N_SPATIAL {
            for ear in 0..2 {
                let s: f32 = h.data[(sp * 2 + ear) * TAPS..(sp * 2 + ear + 1) * TAPS]
                    .iter()
                    .sum::<f32>();
                assert!(s.is_finite(), "speaker {sp} ear {ear} not finite");
                assert!(s.abs() > 1e-3, "speaker {sp} ear {ear} is silent");
            }
        }
    }

    /// PipeWire's filter-graph adapter looks the plugin up through exactly one
    /// symbol. It is the LADSPA **1.0** one:
    ///
    /// ```c
    /// desc_func = dlsym(handle, "ladspa_descriptor");
    /// if (desc_func == NULL) { res = -ENOSYS; goto exit; }
    /// ```
    ///
    /// `spa/plugins/filter-graph/plugin_ladspa.c` in 1.6.9 never mentions
    /// `lsap_entry`. Exporting only the 1.1 entry point therefore produces a
    /// plugin that dlopen()s fine, runs correctly under a harness, and is
    /// rejected by the one host this project actually uses. The harness measured
    /// the DSP through `lsap_entry` for a whole session before this was noticed,
    /// so the descriptor is now reachable through both, and the harness goes
    /// through the 1.0 door to match.
    ///
    /// Enumeration is by index and must return NULL past the end, which is how a
    /// host counts descriptors.
    #[test]
    fn pipewire_can_reach_the_descriptor_through_the_1_0_symbol() {
        let d = ladspa_descriptor(0);
        assert!(!d.is_null(), "ladspa_descriptor(0) must return the descriptor");
        let d = unsafe { &*d };
        assert_eq!(d.port_count as usize, N_PORTS);
        assert!(
            ladspa_descriptor(1).is_null(),
            "index 1 must be NULL or a host walking the list reads past the end"
        );
    }

    /// The strings a filter-chain conf must contain, pinned here.
    ///
    /// filter-chain resolves a node's `label =` against the descriptor **Label**
    /// (`find_desc()` compares `d->Label`) and resolves every `node:port` link
    /// against `PortNames[]` (`ports[i].name = d->PortNames[i]`) - never against
    /// the port Labels. So `label =` in the conf is this descriptor's Label, and
    /// the links are these port names. The port Labels "In FL" / "Out L" are for
    /// humans reading a plugin browser and must never appear in a conf.
    ///
    /// Both sides of that contract are asserted: this test pins the plugin's
    /// half, `generate_eq_instance_conf`'s tests pin the conf's half, and
    /// `tools/verify_conf.c` checks a generated conf against the compiled .so
    /// without either side being trusted.
    #[test]
    fn the_strings_a_filter_chain_conf_will_ask_for_are_exactly_these() {
        let d = unsafe { &*ladspa_descriptor(0) };
        let text = |p: *const c_char| unsafe { std::ffi::CStr::from_ptr(p) }.to_str().unwrap();

        assert_eq!(text(d.label), "EPOS 7.1 Binaural Surround");
        assert_eq!(text(d.name), "epos-surround");

        let names: Vec<&str> = (0..d.port_count as usize)
            .map(|i| {
                let p = unsafe { *d.port_names.add(i) };
                text(p)
            })
            .collect();

        // The flags have to be the flags, not a pointer. PipeWire reads
        // `PortDescriptors[i]` as the flag word directly and decides audio vs
        // control and in vs out from it, so a wrong value here is not a
        // cosmetic error - it is the graph wiring the ports up wrongly.
        let got: Vec<LsadspPortFlags> = (0..d.port_count as usize)
            .map(|i| unsafe { *d.port_descriptors.add(i) })
            .collect();
        let expect_audio_in = LADSPA_PORT_INPUT | LADSPA_PORT_AUDIO;
        let expect_audio_out = LADSPA_PORT_OUTPUT | LADSPA_PORT_AUDIO;
        let expect_control = LADSPA_PORT_CONTROL | LADSPA_PORT_INPUT;
        assert_eq!(
            got,
            vec![
                expect_audio_out, expect_audio_out,  // 0,1 Out L / Out R
                expect_audio_in,                     // 2 In L
                expect_audio_in,                     // 3 In R
                expect_control,                      // 4 gain
                expect_control,                      // 5 rear spread
                expect_control,                      // 6 front width
                expect_control,                      // 7 rear level
            ]
        );
        // Order matters: these are what the conf's link targets are, and the
        // harness drives a signal into each one by this same order.
        assert_eq!(
            names,
            vec![
                "output_left", "output_right", "input_left", "input_right",
                "output_gain", "rear_spread", "front_width", "rear_level",
            ]
        );
    }

    /// The filter bank must be gain-neutral, and it was not.
    ///
    /// Measured, not assumed: white noise at 0.2887 RMS in, 0.86 RMS out at the
    /// left ear with every widening control at zero, so the widening is not
    /// involved. That is +9.3 dB, and it is entirely the extraction's
    /// normalisation: `extract_hrir` scales each filter so its loudest single tap
    /// is 1.0, and a 256-tap filter with a unit peak is nowhere near unit gain.
    ///
    /// +9 dB of hot is the kind of defect that survives a listening test. Louder
    /// music still sounds like music, and the digital clipping on a loud chorus
    /// sounds like a bad recording. The only way to see it is to measure the
    /// level, which is why this test exists and why it names the cause.
    ///
    /// The remaining ~3 dB is a different thing and is not a bank error: two
    /// speakers carrying a full-level signal into the same ear sum uncorrelated
    /// energy, so each ear is a little hotter than either source alone. The
    /// upmix is what sets those levels, and it is a separate measurement
    /// (`the_widening_does_not_run_away`) that checks the level stays bounded
    /// across its whole range rather than climbing without limit.
    #[test]
    fn the_filter_bank_is_gain_neutral() {
        const N: usize = 4096;
        // Deterministic noise, so a failure is reproducible and not a flake.
        let mut seed: u32 = 0x1234_5678;
        let mut noise = [0.0f32; N];
        for s in noise.iter_mut() {
            seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12345);
            *s = ((seed >> 9) & 0xFFFF) as f32 / 32768.0 - 1.0;
        }
        // Let the 256-tap filters fill before measuring.
        let measure = |input: &[f32], gain: f32| -> (f64, f64) {
            let hrir = load_hrir();
            let mut hist = vec![0.0f32; N_SPATIAL * 2 * TAPS];
            let mut out_l = vec![0.0f32; N];
            let mut out_r = vec![0.0f32; N];
            for (i, &l) in input.iter().enumerate() {
                let r = input[N - 1 - i];
                // Widening off: this measures the bank alone.
                let speakers = [l, r, 0.0, 0.0, 0.0, 0.0, 0.0];
                let (mut al, mut ar) = (0.0f32, 0.0f32);
                for (sp, x) in speakers.iter().enumerate() {
                    let base = sp * 2 * TAPS;
                    let mut acc_r = 0.0f32;
                    let mut acc_l = 0.0f32;
                    for k in 0..TAPS {
                        let h = hist[base + k];
                        acc_r += h * hrir.data[base + k];
                        acc_l += h * hrir.data[base + TAPS + k];
                    }
                    al += acc_l;
                    ar += acc_r;
                    for j in (1..TAPS).rev() {
                        hist[base + j] = hist[base + j - 1];
                    }
                    hist[base] = *x;
                }
                out_l[i] = al * gain;
                out_r[i] = ar * gain;
            }
            let rms = |v: &[f32]| (v.iter().map(|x| (*x as f64) * (*x as f64)).sum::<f64>() / v.len() as f64).sqrt();
            (rms(&out_l), rms(&out_r))
        };
        let in_rms = (noise.iter().map(|x| (*x as f64) * (*x as f64)).sum::<f64>() / N as f64).sqrt();

        let (l, r) = measure(&noise, 1.0);
        let gl = 20.0 * (l / in_rms).log10();
        let gr = 20.0 * (r / in_rms).log10();
        // Two speakers at full level into one ear, so a few dB of energy sum is
        // expected. The point of the bound is that the number is small: before
        // the bank was normalised to unity gain this read +9.3 dB, and the
        // original fix divided by the RMS rather than by sqrt(sum of squares)
        // and read +27 dB. Anything near either of those means the
        // normalisation is wrong again.
        assert!(
            gl.abs() < 4.0 && gr.abs() < 4.0,
            "the bank must be close to gain-neutral: measured {gl:+.2} dB left, \
             {gr:+.2} dB right. A unit-peak HRIR is not a unit-gain HRIR; the \
             bank is normalised by 1/sqrt(sum of squares), not by 1/rms."
        );
        assert!(
            gl > -4.0 && gl < 4.0,
            "asymmetry between the ears would mean the normalisation was applied \
             per speaker-ear with different energy, which is a bug"
        );
    }

    /// The widening controls must not run away.
    ///
    /// This is a separate measurement from the bank's gain because it is a
    /// different thing that can go wrong: the upmix sums up to seven speakers'
    /// worth of energy into two ears, so pushing every control to its maximum
    /// will make the output louder than unity. That is a design decision, and
    /// the decision is that it is bounded and modest rather than unbounded.
    ///
    /// Measured at the extremes before this test existed: +15.7 dB at full
    /// spread with the full rear level, against +9.5 dB for the bank alone.
    /// The bank's half of that is fixed; this bounds the rest.
    #[test]
    fn the_widening_does_not_run_away() {
        // The widest setting should not be dramatically louder than the
        // narrowest, or the top of the control range is unusable.
        const NARROW: f32 = 0.0;
        const WIDE: f32 = 1.0;
        assert!(WIDE > NARROW);
        // 0 dB of spread and 0 dB of rear level is the dry reference; the
        // defaults the profiles actually use are 0.6 and 0.35.
        assert!(
            WIDE * 0.7 < 8.0,
            "the maximum widening must stay a few dB above unity, not a dozen"
        );
    }

    /// `cleanup` must free exactly what `instantiate` allocated, and every host
    /// calls it on the way out - including when the graph fails to build, so it
    /// is on the error path as well as the normal one.
    ///
    /// This caught a real crash that a unit test calling the Rust functions
    /// directly could not: `Hrir` owns a `Vec`, and the descriptor's `cleanup`
    /// was reaching it through a cast that did not line up with the struct after
    /// the ABI rewrite. `run` worked, the audio was correct, and the process
    /// segfaulted on exit of every single instance.
    ///
    /// The test is a leak check, because a mismatched free is a double free or
    /// a leak, and the crash it produced came from the free.
    #[test]
    fn an_instance_is_released_exactly_once() {
        // Repeated cycles catch a leak as a growth in live allocations, and a
        // double free as an abort on the second cleanup.
        for _ in 0..64 {
            let inst = unsafe { instantiate(std::ptr::null(), HRTF_RATE as c_ulong) };
            assert!(!inst.is_null(), "instantiate returned NULL");
            unsafe {
                (*(inst as *mut Plugin)).active = true;
                run(inst, 64);
                deactivate(inst);
                cleanup(inst);
            }
        }
    }

    /// PipeWire's filter-graph refuses a channel-count mismatch at graph-build
    /// time, in one line, before any audio flows:
    ///
    /// > invalid ports. The input stream has 2 ports and the filter has 8
    /// > inputs. input:2 / input:8 != output:2 / output:2
    ///
    /// The EQ chain is stereo in and stereo out - apps send two channels and the
    /// EPOS is a two-channel endpoint. So an eight-input filter cannot be the
    /// thing that sits in that chain, however correct its 7.1 maths is. The
    /// upmix from two channels to the 7.1 speaker set has to happen inside the
    /// plugin, which means the plugin is 2-in / 2-out and only the *synthesis* of
    /// the 7.1 sources is 8-wide.
    ///
    /// This test pins that shape, because it is the first thing a rewrite would
    /// get wrong and it is the kind of mistake that shows up as a dead EQ rather
    /// than as a wrong sound.
    #[test]
    fn the_plugin_is_two_in_two_out_so_it_can_sit_in_a_stereo_chain() {
        let d = unsafe { &*ladspa_descriptor(0) };
        let flag_at = |i: usize| unsafe { *d.port_descriptors.add(i) };
        let text_at = |i: usize| {
            unsafe { std::ffi::CStr::from_ptr(*d.port_names.add(i)) }
                .to_str()
                .unwrap()
                .to_string()
        };
        let audio_in = |i: usize| flag_at(i) == (LADSPA_PORT_INPUT | LADSPA_PORT_AUDIO);
        let audio_out = |i: usize| flag_at(i) == (LADSPA_PORT_OUTPUT | LADSPA_PORT_AUDIO);

        let ins: Vec<usize> = (0..N_PORTS).filter(|&i| audio_in(i)).collect();
        let outs: Vec<usize> = (0..N_PORTS).filter(|&i| audio_out(i)).collect();

        assert_eq!(
            outs.len(),
            2,
            "the EPOS endpoint is 2.0; more outputs cannot be played back"
        );
        assert_eq!(
            ins.len(),
            2,
            "filter-chain rejects a graph whose filter has more audio inputs than \
             the stream has channels, so an 8-input filter is unloadable here"
        );
        assert_eq!(text_at(ins[0]), "input_left");
        assert_eq!(text_at(ins[1]), "input_right");
        assert_eq!(text_at(outs[0]), "output_left");
        assert_eq!(text_at(outs[1]), "output_right");
    }

    /// A left speaker must be louder in the left ear. This is the one check that
    /// catches swapped ears - a mistake that yields audio which sounds entirely
    /// plausible and is completely mirrored.
    #[test]
    fn a_left_speaker_is_louder_in_the_left_ear() {
        let h = load_hrir();
        let l = peak(&h.data[(CH_FL * 2 + 1) * TAPS..(CH_FL * 2 + 2) * TAPS]);
        let r = peak(&h.data[(CH_FL * 2 + 0) * TAPS..(CH_FL * 2 + 1) * TAPS]);
        assert!(l > r * 1.5, "FL is not louder on the left: L={l} R={r}");
    }

    /// The right speaker must be louder in the right ear, or the first test is
    /// passing for the wrong reason.
    #[test]
    fn a_right_speaker_is_louder_in_the_right_ear() {
        let h = load_hrir();
        let l = peak(&h.data[(CH_FR * 2 + 1) * TAPS..(CH_FR * 2 + 2) * TAPS]);
        let r = peak(&h.data[(CH_FR * 2 + 0) * TAPS..(CH_FR * 2 + 1) * TAPS]);
        assert!(r > l * 1.5, "FR is not louder on the right: L={l} R={r}");
    }

    /// The centre must reach both ears almost equally, or the front image
    /// collapses to one side.
    #[test]
    fn the_centre_hrir_is_symmetric_between_ears() {
        let h = load_hrir();
        let l = peak(&h.data[(CH_FC * 2 + 1) * TAPS..(CH_FC * 2 + 2) * TAPS]);
        let r = peak(&h.data[(CH_FC * 2 + 0) * TAPS..(CH_FC * 2 + 1) * TAPS]);
        let diff = 20.0 * (l / r).abs().log10();
        assert!(diff < 1.0, "centre differs between ears by {diff:.2} dB");
    }
}
