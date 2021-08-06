//! Some kind of accumulator.
//!
//! You write to the first slot, then you switch writing to the next slot and
//! take the value from the first one.

mod raw;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
