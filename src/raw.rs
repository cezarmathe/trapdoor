//! Raw trapdoor.

use std::mem;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering as AtomicOrdering;

use parking_lot::Mutex;
use parking_lot::RwLock;

pub struct RawTrapdoor<T> {
    inner: [RwLock<Option<Vec<T>>>; 2],
    selector: AtomicUsize,
    take_lock: Mutex<()>,
}

impl<T> RawTrapdoor<T> {
    /// Create a new raw trapdoor.
    pub const fn new() -> Self {
        Self {
            inner: [
                parking_lot::const_rwlock(None),
                parking_lot::const_rwlock(None),
            ],
            selector: AtomicUsize::new(0),
            take_lock: parking_lot::const_mutex(()),
        }
    }

    /// Insert a value in the trapdoor.
    pub fn insert(&self, value: T) {
        loop {
            let mut guard = {
                let selector = self.get_selector();
                self.inner[selector].write()
            };
            if let Some(collection) = &mut *guard {
                collection.push(value);
                break;
            }
        }
    }

    /// Take the collected data.
    pub fn take(&self) -> Vec<T> {
        let _ = self.take_lock.lock();
        loop {
            let collection = {
                let selector = self.get_selector();
                let mut guard = self.inner[selector].write();
                mem::take(&mut *guard)
            };
            if let Some(value) = collection {
                {
                    let selector = self.switch_selector();
                    let mut guard = self.inner[selector ^ 1].write();
                    assert!(guard.is_none());
                    *guard = Some(Vec::new());
                }
                break value;
            }
        }
    }

    /// Get the current selector.
    #[inline(always)]
    fn get_selector(&self) -> usize {
        self.selector.load(AtomicOrdering::Acquire)
    }

    /// Switch the selector, returning the old value.
    #[inline(always)]
    fn switch_selector(&self) -> usize {
        self.selector.fetch_xor(1, AtomicOrdering::SeqCst)
    }
}
