//! Raw trapdoor.

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering as AtomicOrdering;

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

    pub fn insert(&self, value: T)  {
        let mut guard = {
            let selector = self.selector.load(AtomicOrdering::Acquire);
            self.inner[selector].write()
        };
        guard.push(value);
    }
}
