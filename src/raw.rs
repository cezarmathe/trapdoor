//! Raw trapdoor.

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering as AtomicOrdering;

use parking_lot::RwLock;

pub struct RawTrapdoor<T> {
    inner: [RwLock<Option<Vec<T>>>; 2],
    selector: AtomicUsize,
}

impl<T> RawTrapdoor<T> {
    /// Create a new raw trapdoor.
    pub const fn new() -> Self {
        Self {
            inner: [RwLock::new(None), RwLock::new(None)],
            selector: AtomicUsize::new(0),
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
