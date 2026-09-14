//! Read-only memory-bus probe for hardware identification and live runtime
//! state snapshotting.
//!
//! SAFETY: This module only ever issues REPORT 0x04 read requests (flags bit6
//! clear). It NEVER sets bit6 (0x40 = EEPROM write enable) or touches the
//! firmware flash protocol (reports 0x06/0x07/0x1A). All operations are
//! read-only — see HARDWARE-BOOK "Memory bus" section. The EEPROM read below
//! (flags 0x20) is a passive read of the same kind the RE dump tooling ran
//! thousands of times; it carries no write-enable bit.
//!
//! Firmware RE (byte-verified):
//! - RAM $EC74: firmware version string "FREEMAN_V03.01.00.00"
//! - RAM $1005: chip-ID byte (= 0x08 on this unit)
//! - RAM $1016: USB config mode ID (read-only hardware block)
//! - RAM $137D/$137F: mode/session state machine (0-5; LED gate {3,4})
//! - RAM $1388/$1389: LED shift-register pair
//! - RAM $1386/$1387: EQ enable flags
//! - RAM $12D0/$12D1: EQ2/EQ1 band index (single write site $BCF9)
//! - RAM $0D08: DSP state class (01/02/04)
//! - RAM $0FC4..$0FC7: two 16-bit encoder position pairs
//!   (firmware reads $0FC7/$0FC6 and $0FC5/$0FC4 with high byte AND #$07)
//! - RAM $0F13/$0F18/$0F1D: control/state registers
//! - EEPROM $0E26 (first of 3 wear-leveled copies): Sennheiser DAC patch
//!   string "Sennheiser_EntryDAC_Patch_44-05-62_Rev_0062_CX21988_NVM-…"
//!
//! Request layout (RE-verified via eeprom_dump.py): report 0x04 with payload
//! `[flags, len, addr_hi, addr_lo]`, flags 0x00=RAM / 0x20=EEPROM / 0x10=high
//! page (>0xFFFF). Response: report 0x05, data from byte 1.

use epos_shared::device::HwSnapshot;
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tracing::warn;

/// Maximum time to wait for the firmware to answer a memory-bus request.
/// The device answers in a few ms; 2 s allows for USB scheduling hiccups.
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(2);
/// Poll interval while waiting non-blockingly for the 0x05 response.
const POLL_INTERVAL: Duration = Duration::from_millis(10);
/// Linux O_NONBLOCK (glibc/asm-generic value 0x800). Used via
/// `OpenOptionsExt::custom_flags` so a stuck firmware can never block the
/// daemon's probe.
const O_NONBLOCK: i32 = 0x800;

/// Report ID for the primary (memory-bus) output report.
const REPORT_ID_PRIMARY: u8 = 0x04;
/// Report ID of the memory-bus read response.
const REPORT_ID_RESPONSE: u8 = 0x05;
/// Request payload length (flags, len, addr_hi, addr_lo + zero padding).
const REQUEST_LEN: usize = 38;
/// Total packet length = report id + request payload.
const PACKET_LEN: usize = 39;

/// Flags: RAM access. bit6 (0x40) write-enable is NEVER set in this module.
const FLAG_RAM: u8 = 0x00;
/// Flags: EEPROM access (passive read; see eeprom_dump.py).
const FLAG_EEPROM: u8 = 0x20;
/// Flags: high page (addr >= 0x10000).
const FLAG_HIGH: u8 = 0x10;

// ── Static identity addresses (probed once at startup) ───────────────────
/// RAM address of the firmware version string.
const FW_VERSION_ADDR: u16 = 0xEC74;
/// Bytes to read for the version string (NUL-padded).
const FW_VERSION_LEN: u8 = 24;
/// RAM address of the chip-ID byte.
const CHIP_ID_ADDR: u16 = 0x1005;
/// RAM address of the USB-config mode ID (read-only hardware block).
const USB_MODE_ADDR: u16 = 0x1016;
/// EEPROM address of the Sennheiser DAC patch string (copy 0/3).
const PATCH_ADDR: u16 = 0x0E26;
/// Bytes to read for the patch string (NUL-padded, ~59 chars observed).
const PATCH_LEN: u8 = 64;

// ── Runtime snapshot regions ─────────────────────────────────────────────
/// Bundle 1: $1386..=$1389 → [eq_enable_lo, eq_enable_hi, led_lo, led_hi].
const EQ_LED_ADDR: u16 = 0x1386;
const EQ_LED_LEN: u8 = 4;
/// Bundle 2: $137D..=$137F → [mode_state, _, secondary_state].
const MODE_ADDR: u16 = 0x137D;
const MODE_LEN: u8 = 3;
/// Bundle 3: $12D0..=$12D1 → [eq2_index, eq1_index].
const EQ_INDEX_ADDR: u16 = 0x12D0;
const EQ_INDEX_LEN: u8 = 2;
/// DSP state byte.
const DSP_ADDR: u16 = 0x0D08;
/// Bundle 4: encoder positions $0FC4..=$0FC7.
const ENC_ADDR: u16 = 0x0FC4;
const ENC_LEN: u8 = 4;
/// Control registers (individual reads, 3 bytes apart each).
const CTRL_0F13: u16 = 0x0F13;
const CTRL_0F18: u16 = 0x0F18;
const CTRL_0F1D: u16 = 0x0F1D;

/// Hardware identity gathered from the device memory bus (read-only).
#[derive(Debug, Clone, Default)]
pub struct HwInfo {
    /// Firmware version string, e.g. "FREEMAN_V03.01.00.00".
    pub firmware_version: Option<String>,
    /// Chip-ID byte (0x08 = CX21988 family on this unit).
    pub chip_id: Option<u8>,
    /// USB config mode ID from the read-only hardware block $1015-$1017.
    pub usb_mode_id: Option<u8>,
    /// Sennheiser EntryDAC patch string from EEPROM (wear-leveled copy 0).
    pub dac_patch: Option<String>,
}

/// Probe the GSX 300 memory bus once at startup. Best-effort: any failure
/// (device absent, permission denied, timeout) returns [`HwInfo::default`]
/// and the daemon continues normally — hardware info is a nicety.
pub fn probe(hidraw: &PathBuf) -> HwInfo {
    let mut file = match open_hidraw(hidraw) {
        Some(f) => f,
        None => return HwInfo::default(),
    };

    let mut info = HwInfo::default();
    if let Some(raw) = read_mem(&mut file, FW_VERSION_ADDR, FW_VERSION_LEN, FLAG_RAM) {
        let s = c_string(&raw);
        if !s.is_empty() {
            info.firmware_version = Some(s);
        }
    }
    if let Some(raw) = read_mem(&mut file, CHIP_ID_ADDR, 1, FLAG_RAM) {
        info.chip_id = raw.first().copied();
    }
    if let Some(raw) = read_mem(&mut file, USB_MODE_ADDR, 1, FLAG_RAM) {
        info.usb_mode_id = raw.first().copied();
    }
    // Passive EEPROM read: flags 0x20, bit6 (write-enable) clear.
    if let Some(raw) = read_mem(&mut file, PATCH_ADDR, PATCH_LEN, FLAG_EEPROM) {
        let s = c_string(&raw);
        if s.contains("Sennheiser") {
            info.dac_patch = Some(s);
        }
    }
    info
}

/// Take a live, read-only snapshot of the runtime state registers.
///
/// Each region is read independently; a failure (device busy, timeout)
/// leaves that field `None` without failing the rest. Best-effort.
pub fn snapshot(hidraw: &PathBuf) -> HwSnapshot {
    let mut file = match open_hidraw(hidraw) {
        Some(f) => f,
        None => return HwSnapshot::default(),
    };
    let mut snap = HwSnapshot::default();

    if let Some(raw) = read_mem(&mut file, EQ_LED_ADDR, EQ_LED_LEN, FLAG_RAM) {
        if raw.len() >= 4 {
            let lo = raw[0];
            let hi = raw[1];
            snap.eq_enable = Some(if hi != 0 { lo | 0x80 } else { lo });
            snap.led_shift = Some(u16::from_le_bytes([raw[2], raw[3]]));
        }
    }
    if let Some(raw) = read_mem(&mut file, MODE_ADDR, MODE_LEN, FLAG_RAM) {
        if raw.len() >= 3 {
            snap.mode_state = Some(raw[0]);
            snap.secondary_state = Some(raw[2]);
        }
    }
    if let Some(raw) = read_mem(&mut file, EQ_INDEX_ADDR, EQ_INDEX_LEN, FLAG_RAM) {
        if raw.len() >= 2 {
            snap.eq2_index = Some(raw[0]);
            snap.eq1_index = Some(raw[1]);
        }
    }
    if let Some(raw) = read_mem(&mut file, DSP_ADDR, 1, FLAG_RAM) {
        snap.dsp_state = raw.first().copied();
    }
    if let Some(raw) = read_mem(&mut file, ENC_ADDR, ENC_LEN, FLAG_RAM) {
        if raw.len() >= 4 {
            // Raw layout: [$0FC4, $0FC5, $0FC6, $0FC7].
            // Firmware (RE): encoder A = ($0FC6 & 7) << 8 | $0FC7  → [lo=$0FC7, hi=$0FC6&7]
            //                encoder B = ($0FC4 & 7) << 8 | $0FC5  → [lo=$0FC5, hi=$0FC4&7]
            snap.encoder_a = Some(u16::from_le_bytes([raw[3], raw[2] & 0x07]));
            snap.encoder_b = Some(u16::from_le_bytes([raw[1], raw[0] & 0x07]));
        }
    }
    snap.ctrl_0f13 = read_mem(&mut file, CTRL_0F13, 1, FLAG_RAM).and_then(|r| r.first().copied());
    snap.ctrl_0f18 = read_mem(&mut file, CTRL_0F18, 1, FLAG_RAM).and_then(|r| r.first().copied());
    snap.ctrl_0f1d = read_mem(&mut file, CTRL_0F1D, 1, FLAG_RAM).and_then(|r| r.first().copied());
    snap
}

/// Open the hidraw node non-blocking (read+write).
fn open_hidraw(hidraw: &PathBuf) -> Option<File> {
    const RETRIES: u32 = 10;
    const RETRY_DELAY: Duration = Duration::from_millis(400);

    for attempt in 1..=RETRIES {
        match File::options()
            .read(true)
            .write(true)
            .custom_flags(O_NONBLOCK)
            .open(hidraw)
        {
            Ok(f) => return Some(f),
            Err(e)
                if e.kind() == std::io::ErrorKind::PermissionDenied
                    || e.raw_os_error() == Some(libc_eperm()) =>
            {
                // udev ACL race at boot / replug: the group ACL may not be
                // granted yet. Retry with a short backoff like the LED path.
                if attempt < RETRIES {
                    warn!("hwinfo: hidraw open denied (udev ACL race?), attempt {attempt}/{RETRIES}: {e}");
                }
                std::thread::sleep(RETRY_DELAY);
            }
            Err(_) => return None,
        }
    }
    None
}

// Detect EPERM without pulling in libc as a hard dependency.
fn libc_eperm() -> i32 {
    #[cfg(target_os = "linux")]
    {
        1 // EPERM
    }
    #[cfg(not(target_os = "linux"))]
    {
        0
    }
}

/// Read a NUL-padded C string from a raw byte buffer.
fn c_string(raw: &[u8]) -> String {
    raw.iter()
        .take_while(|&&b| b != 0)
        .map(|&b| {
            // Keep printable ASCII, drop control bytes.
            if (0x20..0x7F).contains(&b) {
                b as char
            } else {
                ' '
            }
        })
        .collect::<String>()
        .trim()
        .to_string()
}

/// Issue one read-only memory-bus request and collect the 0x05 response.
///
/// `flags` selects the address space: `FLAG_RAM` (0x00) or `FLAG_EEPROM`
/// (0x20). bit6 (0x40, EEPROM write) is never set: the request is a pure
/// read. High-page (0x10) is derived automatically from the address.
fn read_mem(file: &mut File, addr: u16, len: u8, flags: u8) -> Option<Vec<u8>> {
    let mut flags = flags;
    if u32::from(addr) >= 0x10000 {
        flags |= FLAG_HIGH;
    }

    let mut payload = vec![0u8; REQUEST_LEN];
    payload[0] = flags;
    payload[1] = len;
    payload[2] = (addr >> 8) as u8;
    payload[3] = addr as u8;

    let mut packet = vec![0u8; PACKET_LEN];
    packet[0] = REPORT_ID_PRIMARY;
    packet[1..].copy_from_slice(&payload);

    if !write_all_nonblocking(file, &packet) {
        return None;
    }
    let _ = file.flush();

    // Read responses until the 0x05 reply arrives or we time out. The report
    // ID is the first byte of each hidraw read.
    let deadline = Instant::now() + RESPONSE_TIMEOUT;
    let mut buf = [0u8; 64];
    while Instant::now() < deadline {
        match file.read(&mut buf) {
            Ok(n) if n >= 1 && buf[0] == REPORT_ID_RESPONSE => {
                let data_len = (n - 1).min(len as usize);
                return Some(buf[1..1 + data_len].to_vec());
            }
            Ok(_) => {
                // Another report (volume dial etc.) — keep waiting for 0x05.
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                std::thread::sleep(POLL_INTERVAL);
            }
            Err(_) => return None,
        }
    }
    None
}

/// write_all() that tolerates EAGAIN/EWOULDBLOCK on the non-blocking hidraw fd.
fn write_all_nonblocking(file: &mut File, mut buf: &[u8]) -> bool {
    while !buf.is_empty() {
        match file.write(buf) {
            Ok(0) => return false,
            Ok(n) => buf = &buf[n..],
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                std::thread::sleep(POLL_INTERVAL);
            }
            Err(_) => return false,
        }
    }
    true
}
