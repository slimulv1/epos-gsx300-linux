//! Shared non-blocking I/O for raw `hidraw` nodes.
//!
//! The GSX 300 is reached through `/dev/hidrawN` on two paths: the hardware
//! probe in `hwinfo` and the LED ring in `led`. Both write, and a write goes out
//! over the interrupt-out endpoint or, where none exists, a `SET_REPORT`
//! control transfer. When a device stops servicing its endpoint that transfer
//! does not fail, it stops completing — so a blocking descriptor turns a wedged
//! headset into a wedged daemon, on a tokio worker thread, often while the
//! global state lock is held.
//!
//! Both open with `O_NONBLOCK` and push their reports through
//! [`drain_nonblocking`], which gives up at a budget instead.

use std::io;
use std::time::{Duration, Instant};

/// Linux `O_NONBLOCK` (glibc/asm-generic value 0x800), applied through
/// `OpenOptionsExt::custom_flags` so a stuck firmware can never block a write.
pub(crate) const O_NONBLOCK: i32 = 0x800;

/// How long to keep retrying before sleeping between attempts.
const POLL_INTERVAL: Duration = Duration::from_millis(10);

/// Push `buf` through a non-blocking sink, retrying `WouldBlock` until it goes
/// through or the budget runs out.
///
/// The retry is a closure rather than a `File` so the policy can be tested by
/// handing it a sink that always reports `EAGAIN` — the case that is otherwise
/// impossible to produce without a wedged USB endpoint.
///
/// Giving up reports failure, which is the truth: the report did not go out.
/// Every caller here is periodic or user-triggered, so a later attempt retries.
pub(crate) fn drain_nonblocking(
    mut buf: &[u8],
    budget: Duration,
    mut sink: impl FnMut(&[u8]) -> io::Result<usize>,
) -> bool {
    let started = Instant::now();
    while !buf.is_empty() {
        match sink(buf) {
            Ok(0) => return false,
            Ok(n) => buf = &buf[n..],
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                if started.elapsed() >= budget {
                    return false;
                }
                std::thread::sleep(POLL_INTERVAL);
            }
            Err(_) => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A sink that reports `EAGAIN` every time, standing in for a device that
    /// has stopped servicing its endpoint. The old loops had no way to be
    /// tested here, because the retry was welded to a `File`.
    fn always_would_block(_: &[u8]) -> io::Result<usize> {
        Err(io::Error::from(io::ErrorKind::WouldBlock))
    }

    /// The bug this exists for: a transfer that never drains must be abandoned
    /// at its budget, not awaited forever. Without the deadline this test does
    /// not finish at all.
    #[test]
    fn a_transfer_that_never_drains_is_abandoned_at_its_budget() {
        let started = Instant::now();
        let delivered = drain_nonblocking(&[0u8; 8], Duration::from_millis(150), always_would_block);
        assert!(!delivered, "a transfer that never drains must report failure");
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "must give up near its budget, took {:?}",
            started.elapsed()
        );
    }

    /// A sink that accepts part of the buffer is drained completely rather than
    /// giving up — a short write is normal, not a failure.
    #[test]
    fn a_sink_taking_one_byte_at_a_time_still_completes() {
        let mut offered = 0usize;
        let delivered = drain_nonblocking(&[7u8; 4], Duration::from_secs(5), |buf| {
            offered += 1;
            assert!(offered <= 4, "must stop once the buffer is empty");
            Ok(1.min(buf.len()))
        });
        assert!(delivered);
        assert_eq!(offered, 4, "every byte must be offered exactly once");
    }

    /// An immediate zero-byte write is a dead endpoint, not a retry.
    #[test]
    fn a_sink_that_writes_nothing_is_a_failure() {
        assert!(!drain_nonblocking(&[1, 2], Duration::from_secs(5), |_| Ok(0)));
    }

    /// A real error ends the attempt immediately.
    #[test]
    fn a_sink_that_fails_ends_the_attempt() {
        assert!(!drain_nonblocking(&[1, 2], Duration::from_secs(5), |_| {
            Err(io::Error::from(io::ErrorKind::BrokenPipe))
        }));
    }

    /// The ordinary path still writes to a real descriptor.
    #[test]
    fn a_real_file_still_receives_the_report() {
        let path = std::env::temp_dir().join(format!("epos-hid-io-{}", std::process::id()));
        let mut file = std::fs::File::create(&path).expect("create temp file");
        assert!(drain_nonblocking(&[1, 2, 3], Duration::from_secs(5), |b| {
            std::io::Write::write(&mut file, b)
        }));
        drop(file);
        assert_eq!(std::fs::read(&path).expect("read back"), vec![1, 2, 3]);
        let _ = std::fs::remove_file(&path);
    }
}
