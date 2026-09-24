//! Small synchronisation helpers shared across the daemon.

use std::sync::{Mutex, MutexGuard};

/// Lock a mutex, tolerating poisoning.
///
/// A `std::sync::Mutex` stays flagged once a holder panicked, and from then on
/// every `.lock().unwrap()` on it panics too. In a daemon whose workers are
/// `tokio::spawn`ed, that turns a single panic anywhere in the process into the
/// silent, permanent death of whichever worker touched the mutex next: the mic
/// watchdog, the PipeWire reload worker, the config watcher and the volume
/// watcher all stop, with no log line and no restart.
///
/// Poisoning only records that a panic unwound through the guard. Everything
/// these mutexes hold is a small, self-consistent value — a pending-restart set,
/// a watchdog counter, the last sink name, the bytes of the last config write —
/// so the data behind a poisoned lock is still the data to use. Recovering it
/// keeps one panic from cascading into a dead control surface.
pub(crate) fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    /// Poison a mutex the way a panicking holder would.
    fn poison<T: Send + 'static>(value: T) -> Arc<Mutex<T>> {
        let mutex = Arc::new(Mutex::new(value));
        let worker = Arc::clone(&mutex);
        let _ = std::thread::spawn(move || {
            let _guard = worker.lock().unwrap();
            panic!("boom");
        })
        .join();
        mutex
    }

    /// The premise, stated as an assertion: the mutex really is poisoned after a
    /// panic, so the tests below are exercising recovery and not a no-op.
    #[test]
    fn a_panicking_holder_really_does_poison_the_mutex() {
        let mutex = poison(0u32);

        assert!(mutex.is_poisoned(), "precondition: the lock is poisoned");
        assert!(
            mutex.lock().is_err(),
            "a plain lock().unwrap() would panic here"
        );
    }

    /// The recovery: a poisoned lock is still usable, and the value behind it
    /// survives, so a panic elsewhere cannot take a worker down with it.
    #[test]
    fn a_poisoned_lock_is_still_usable() {
        let mutex = poison(String::from("pending restarts"));

        assert_eq!(&*lock(&mutex), "pending restarts");
        lock(&mutex).push_str(", voice");
        assert_eq!(&*lock(&mutex), "pending restarts, voice");
    }

    /// And the ordinary path is unchanged: locking, reading, writing, dropping.
    #[test]
    fn an_unpoisoned_lock_behaves_normally() {
        let mutex = Mutex::new(vec![1u8, 2, 3]);
        lock(&mutex).push(4);
        assert_eq!(*lock(&mutex), vec![1, 2, 3, 4]);
        assert!(!mutex.is_poisoned());
    }
}
