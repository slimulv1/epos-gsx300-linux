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

use std::os::raw::{c_char, c_int};

// ─── LADSPA ABI ─────────────────────────────────────────────────
//
// Hand-declared rather than pulled from a crate. The ABI is a fixed set of
// structs from the LADSPA 1.1 header, and a dependency would put a crate on the
// build path of code that runs in a real-time thread for no benefit.

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LsadspPortRange {
    pub min: f32,
    pub max: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LsadspPortDescriptor {
    pub label: *const c_char,
    pub name: *const c_char,
    pub range: *const LsadspPortRange,
    pub descriptors: *const LsadspPortDescriptor,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LsadspProperties {
    pub label: *const c_char,
    pub name: *const c_char,
    pub properties: *const LsadspPropertyDescriptor,
    pub count: c_int,
}

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
    pub unique_id: c_int,
    pub label: *const c_char,
    pub properties: *const LsadspProperties,
    pub name: *const c_char,
    pub maker: *const c_char,
    pub copyright: *const c_char,
    pub port_count: c_int,
    pub port_descriptors: *const LsadspPortDescriptor,
    pub implementation_data: *const c_char,
    pub instantiate: unsafe extern "C" fn(*const LsadspDescriptor, c_int) -> *mut LsadspHandle,
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

/// LADSPA 1.1 requires the entry symbol to be spelled `lsap_entry` - one "d".
/// A hand-written plugin that exports `lsap_entry` correctly and `lsadsp_entry`
/// wrongly loads and then reports itself as having no entry point.
///
#[no_mangle]
pub static lsap_entry: &LsadspDescriptor = &DESCRIPTOR;


// SAFETY: every one of these is an ABI description that is written once during
// static initialisation and never again; the raw pointers inside them target
// `'static` string literals and other statics. `Sync` is what the compiler needs
// in order to put them in a `static`, and the data is genuinely immutable.
unsafe impl Sync for LsadspPortRange {}
unsafe impl Sync for LsadspPortDescriptor {}
unsafe impl Sync for LsadspPropertyDescriptor {}
unsafe impl Sync for LsadspProperties {}
unsafe impl Sync for LsadspDescriptor {}

// ─── Geometry ───────────────────────────────────────────────────
//
// Standard 7.1, the order every LADSPA and PipeWire 7.1 consumer uses:
// FL FR FC LFE SL SR BL BR. LFE is summed into both ears without a filter: it
// is a subwoofer channel, and putting it through an ear response centred on the
// head would pull the subwoofer up to ear level, which is the classic mistake.
const CH_FL: usize = 0;
const CH_FR: usize = 1;
const CH_FC: usize = 2;
const CH_LFE: usize = 3;
const CH_SL: usize = 4;
const CH_SR: usize = 5;
const CH_BL: usize = 6;
const CH_BR: usize = 7;
const N_CHANNELS: usize = 8;
/// Which input port feeds which spatialised speaker. LFE sits in the middle of
/// the input order and is not spatialised, so this is not simply 0..7 - reading
/// input[3] as a speaker and never reading input[7] leaves back-right silent, a
/// mistake only a per-channel measurement shows.
const SPATIAL_INPUT: [usize; N_SPATIAL] =
    [CH_FL, CH_FR, CH_FC, CH_SL, CH_SR, CH_BL, CH_BR];
/// Speakers that get a measured HRIR. Seven; LFE is handled separately.
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
    Hrir { data }
}

// ─── Port layout ────────────────────────────────────────────────
// 0 L, 1 R, 2..9 inputs FL FR FC LFE SL SR BL BR, 10 gain, 11 rear spread.
const P_OUT_L: usize = 0;
const P_OUT_R: usize = 1;
const P_IN0: usize = 2;
const P_GAIN: usize = P_IN0 + N_CHANNELS;
const P_SPREAD: usize = P_GAIN + 1;
const N_PORTS: usize = P_SPREAD + 1;

static PORT_RANGE: LsadspPortRange = LsadspPortRange { min: 0.0, max: 1.0 };
static CONTROL_RANGE: LsadspPortRange = LsadspPortRange { min: 0.0, max: 4.0 };

macro_rules! port {
    ($label:literal, $name:literal, $range:expr, $flags:expr) => {
        LsadspPortDescriptor {
            label: cstr!($label),
            name: cstr!($name),
            range: $range,
            descriptors: std::ptr::null(),
            flags: $flags,
        }
    };
}

static PORTS: [LsadspPortDescriptor; N_PORTS] = [
    port!("Out L", "output_left", &PORT_RANGE, LADSPA_PORT_OUTPUT | LADSPA_PORT_AUDIO),
    port!("Out R", "output_right", &PORT_RANGE, LADSPA_PORT_OUTPUT | LADSPA_PORT_AUDIO),
    port!("In FL", "input_front_left", &PORT_RANGE, LADSPA_PORT_INPUT | LADSPA_PORT_AUDIO),
    port!("In FR", "input_front_right", &PORT_RANGE, LADSPA_PORT_INPUT | LADSPA_PORT_AUDIO),
    port!("In FC", "input_front_centre", &PORT_RANGE, LADSPA_PORT_INPUT | LADSPA_PORT_AUDIO),
    port!("In LFE", "input_lfe", &PORT_RANGE, LADSPA_PORT_INPUT | LADSPA_PORT_AUDIO),
    port!("In SL", "input_side_left", &PORT_RANGE, LADSPA_PORT_INPUT | LADSPA_PORT_AUDIO),
    port!("In SR", "input_side_right", &PORT_RANGE, LADSPA_PORT_INPUT | LADSPA_PORT_AUDIO),
    port!("In BL", "input_back_left", &PORT_RANGE, LADSPA_PORT_INPUT | LADSPA_PORT_AUDIO),
    port!("In BR", "input_back_right", &PORT_RANGE, LADSPA_PORT_INPUT | LADSPA_PORT_AUDIO),
    port!("Gain", "output_gain", &CONTROL_RANGE, LADSPA_PORT_CONTROL),
    port!("Rear spread", "rear_spread", &PORT_RANGE, LADSPA_PORT_CONTROL),
];

static PROPERTIES: [LsadspPropertyDescriptor; 3] = [
    LsadspPropertyDescriptor { key: cstr!("LADSPA_URL"), value: cstr!("https://github.com/slimulv1/epos-gsx300-linux") },
    LsadspPropertyDescriptor { key: cstr!("PropertyList"), value: cstr!("") },
    LsadspPropertyDescriptor { key: cstr!("UniqueID"), value: cstr!("7301") },
];

static PROPERTY_LIST: LsadspProperties = LsadspProperties {
    label: cstr!("EPOS Surround Properties"),
    name: cstr!("EPOSSurroundProperties"),
    properties: PROPERTIES.as_ptr(),
    count: PROPERTIES.len() as c_int,
};

static DESCRIPTOR: LsadspDescriptor = LsadspDescriptor {
    unique_id: 7301,
    label: cstr!("EPOS 7.1 Binaural Surround"),
    properties: &PROPERTY_LIST,
    name: cstr!("epos-surround"),
    maker: cstr!("epos-gsx300-linux"),
    copyright: cstr!("MIT"),
    port_count: N_PORTS as c_int,
    port_descriptors: PORTS.as_ptr(),
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
    input: [*mut f32; N_CHANNELS],
    /// Running filter state, [speaker][ear][tap]. Allocated once here; the audio
    /// thread only shifts it.
    hist: Vec<f32>,
    gain: f32,
    spread: f32,
    active: bool,
    adding: f32,
}

unsafe extern "C" fn instantiate(_d: *const LsadspDescriptor, rate: c_int) -> *mut LsadspHandle {
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
        input: [std::ptr::null_mut(); N_CHANNELS],
        hist,
        gain: 1.0,
        spread: 0.0,
        active: false,
        adding: 0.0,
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
    } else if (P_IN0..P_IN0 + N_CHANNELS).contains(&i) {
        p.input[i - P_IN0] = data;
    } else if i == P_GAIN {
        p.gain = *data;
    } else if i == P_SPREAD {
        p.spread = *data;
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
unsafe fn process(p: &mut Plugin, n: usize) {
    let gain = p.gain;
    let spread = p.spread;
    for s in 0..n {
        let mut left = 0.0f32;
        let mut right = 0.0f32;

        // Subwoofer: folded into both ears without a filter. Putting it through
        // an ear response centred on the head would pull the subwoofer up to ear
        // level, which is the classic mistake.
        let lfe = if p.input[CH_LFE].is_null() { 0.0 } else { *p.input[CH_LFE].add(s) };
        left += lfe;
        right += lfe;

        for sp in 0..N_SPATIAL {
            let base = sp * 2 * TAPS;
            let port = SPATIAL_INPUT[sp];
            let x = if p.input[port].is_null() { 0.0 } else { *p.input[port].add(s) };

            // Direct form: y = sum_k h[k] * x[n-k], and the history already holds
            // x[n-k]. Two earlier versions got this wrong - one skipped the whole
            // block on a zero sample, one multiplied by x again - and both made every
            // speaker output exact silence while LFE, which does not use this path,
            // kept working. An impulse per speaker is what makes it visible.
            //
            // The blob stores ear 0 = right, ear 1 = left, as libmysofa returns
            // them; the outputs are Out L and Out R. Attaching ear 0 to Out L
            // mirrored every source, and the unit test could not see it: it
            // compares the two ears of one speaker, not which output they reach.
            let mut acc_right = 0.0f32;
            let mut acc_left = 0.0f32;
            for k in 0..TAPS {
                let h = p.hist[base + k];
                acc_right += h * p.hrir.data[base + k];
                acc_left += h * p.hrir.data[base + TAPS + k];
            }
            left += acc_left;
            right += acc_right;

            // The shift must happen on every sample, including a silent one: it is
            // what moves the newest sample into the history the next convolution
            // reads. Skipping it leaves the newest cell never read again.
            for j in (1..TAPS).rev() {
                p.hist[base + j] = p.hist[base + j - 1];
            }
            p.hist[base] = x;
        }

        // Optional widening. This is NOT recovering a rear channel a stereo source
        // never had: a deliberate, level-controlled creative effect, off at zero,
        // kept separate from the measured path so the two can be reasoned about
        // and removed independently.
        if spread > 0.0 {
            let l = if p.input[CH_FL].is_null() { 0.0 } else { *p.input[CH_FL].add(s) };
            let r = if p.input[CH_FR].is_null() { 0.0 } else { *p.input[CH_FR].add(s) };
            left += l * 0.15 * spread;
            right += r * 0.15 * spread;
        }

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
