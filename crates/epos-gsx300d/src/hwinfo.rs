//! Read-only memory-bus probe for hardware identification.
//!
//! SAFETY: This module only ever issues REPORT 0x04 read requests (flags bit6
//! clear). It NEVER sets bit6 (0x40 = EEPROM write enable) or touches the
//! firmware flash protocol (reports 0x06/0x07/0x1A). All operations are
//! read-only — see HARDWARE-BOOK "Memory bus" section.
//!
//! Firmware RE (byte-verified):
//! - RAM $EC74: firmware version string "FREEMAN_V03.01.00.00"
//! - RAM $1005: chip-ID byte (= 0x08 on this unit)
//!
//! Request layout (RE-verified via eeprom_dump.py): report 0x04 with payload
//! `[flags, len, addr_hi, addr_lo]`, flags 0x00=RAM / 0x20=EEPROM / 0x10=high
//! page (>0xFFFF). Response: report 0x05, data from byte 1.

use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Maximum time to wait for the firmware to answer a memory-bus request.
/// The device answers in a few ms; 2 s allows for USB scheduling hiccups.
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(2);
/// Poll interval while waiting non-blockingly for the 0x05 response.
const POLL_INTERVAL: Duration = Duration::from_millis(10);
/// Linux O_NONBLOCK (glibc/asm-generic value 0x800). Used via
/// `OpenOptionsExt::custom_flags` so a stuck firmware can never block the
/// daemon's probe.
const O_NONBLOCK: i32 = 0x800;

/// RAM address of the firmware version string.
const FW_VERSION_ADDR: u16 = 0xEC74;
/// Number of bytes to read for the version string (NUL-padded).
const FW_VERSION_LEN: u8 = 24;
/// RAM address of the chip-ID byte.
const CHIP_ID_ADDR: u16 = 0x1005;

/// Hardware identity gathered from the device memory bus (read-only).
#[derive(Debug, Clone, Default)]
pub struct HwInfo {
    /// Firmware version string, e.g. "FREEMAN_V03.01.00.00".
    pub firmware_version: Option<String>,
    /// Chip-ID byte (0x08 = CX21988 family on this unit).
    pub chip_id: Option<u8>,
}

/// Probe the GSX 300 memory bus once at startup. Best-effort: any failure
/// (device absent, permission denied, timeout) returns [`HwInfo::default`]
/// and the daemon continues normally — hardware info is a nicety.
pub fn probe(hidraw: &PathBuf) -> HwInfo {
    let mut file = match File::options()
        .read(true)
        .write(true)
        .custom_flags(O_NONBLOCK)
        .open(hidraw)
    {
        Ok(f) => f,
        Err(_) => return HwInfo::default(),
    };

    let mut info = HwInfo::default();
    if let Some(raw) = read_mem(&mut file, FW_VERSION_ADDR, FW_VERSION_LEN) {
        // The string is NUL-padded; strip everything after the first NUL.
        let s: String = raw
            .iter()
            .take_while(|&&b| b != 0)
            .map(|&b| b as char)
            .collect();
        if !s.trim().is_empty() {
            info.firmware_version = Some(s.trim().to_string());
        }
    }
    if let Some(raw) = read_mem(&mut file, CHIP_ID_ADDR, 1) {
        info.chip_id = raw.first().copied();
    }
    info
}

/// Issue one read-only memory-bus request and collect the 0x05 response.
///
/// `addr` is a byte address in the device's RAM space. The EEPROM flag
/// (0x20) is never set here — this is a RAM read — and bit6 (0x40, EEPROM
/// write) is never set: the request is a pure read.
fn read_mem(file: &mut File, addr: u16, len: u8) -> Option<Vec<u8>> {
    let mut payload = vec![0u8; 38];
    payload[0] = 0x00; // flags: RAM, low page (bit6 + EEPROM flag clear)
    payload[1] = len;
    payload[2] = (addr >> 8) as u8;
    payload[3] = addr as u8;

    let mut packet = vec![0u8; 39];
    packet[0] = 0x04; // REPORT_ID_PRIMARY_CMD (memory bus)
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
            Ok(n) if n >= 1 && buf[0] == 0x05 => {
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