#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(test)]
extern crate std;

/// An offset to an exponent mask.
pub const EXPONENT_MASK_OFFSET: usize = 48;

/// An exponent mask.
pub const EXPONENT_MASK: u64 = 0x7ff0 << EXPONENT_MASK_OFFSET;

/// A value mask.
pub const BOXED_VALUE_MASK: u64 = (0x8000 << EXPONENT_MASK_OFFSET) | EXPONENT_MASK;

/// Boxes a value.
///
/// The `value` needs to be less than `1 << 52`.
pub fn r#box(value: u64) -> f64 {
    f64::from_bits(EXPONENT_MASK | value)
}

pub fn unbox(number: f64) -> Option<u64> {
    number
        .is_nan()
        .then_some(number.to_bits() & !BOXED_VALUE_MASK)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_value() {
        assert!(r#box(42).is_nan());
    }

    #[test]
    fn unbox_value() {
        assert_eq!(unbox(r#box(0)), Some(0));
        assert_eq!(unbox(r#box(1)), Some(1));
        assert_eq!(unbox(r#box(7)), Some(7));
        assert_eq!(unbox(r#box(42)), Some(42));
    }
}
