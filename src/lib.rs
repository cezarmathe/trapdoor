//! Some kind of accumulator.
//!
//! You write to the first slot, then you switch writing to the next slot and
//! take the value from the first one.

use self::raw::RawTrapdoor;

mod raw;

/// A trapdoor?
pub struct Trapdoor<T> {
    raw: RawTrapdoor<T>,
}

impl<T> Trapdoor<T> {
    /// Create a new trapdoor.
    pub const fn new() -> Self {
        Self {
            raw: RawTrapdoor::new(),
        }
    }

    /// Take a collection from the trapdoor.
    pub fn take(&self) -> Vec<T> {
        self.raw.take()
    }

    /// Insert a value in the trapdoor.
    pub fn insert(&self, value: T) {
        self.raw.insert(value)
    }
}

impl<T> Default for Trapdoor<T> {
    /// Create a new, default trapdoor.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::Trapdoor;

    #[test]
    fn test_take_insert() {
        let trapdoor = Trapdoor::new();

        assert_eq!(trapdoor.take().len(), 0);

        for i in 0..43 {
            trapdoor.insert(i);
        }

        let col = trapdoor.take();
        assert_eq!(col.len(), 43);
        assert_eq!(col.last(), Some(&42));
    }
}
