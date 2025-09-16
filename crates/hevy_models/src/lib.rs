#![doc = include_str!("../README.md")]

/// Adds two numbers together.
#[must_use]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    #![allow(missing_docs)]
    #![allow(clippy::pedantic)]
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
