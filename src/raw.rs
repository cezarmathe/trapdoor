//! Raw trapdoor.

use std::sync::RwLock;
use std::sync::atomic::AtomicUsize;

pub struct RawTrapdoor<T> {
    inner: [RwLock<Vec<T>>; 2],
    selector: AtomicUsize,
}
