//! NaN boxing with `u64` representation of `f64`.

use super::*;

/// Boxes a 51-bit unsigned integer.
pub fn box_unsigned(value: u64) -> u64 {
    super::box_unsigned(value).to_bits()
}

/// Unboxes a 51-bit unsigned integer.
pub fn unbox_unsigned(number: u64) -> Option<u64> {
    super::unbox_unsigned(f64::from_bits(number))
}
