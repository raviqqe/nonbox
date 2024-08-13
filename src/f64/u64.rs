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

/// Boxes a 52-bit signed integer.
pub fn box_signed(value: i64) -> u64 {
    super::box_signed(value).to_bits()
}

/// Unboxes a 52-bit signed integer.
pub fn unbox_signed(number: u64) -> Option<i64> {
    super::unbox_signed(f64::from_bits(number))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unbox_value_in_u64() {
        assert_eq!(u64::unbox_unsigned(42.0f64.to_bits()), None);
        assert_eq!(u64::unbox_unsigned(u64::box_unsigned(0)), Some(0));
        assert_eq!(u64::unbox_unsigned(u64::box_unsigned(1)), Some(1));
        assert_eq!(u64::unbox_unsigned(u64::box_unsigned(7)), Some(7));
        assert_eq!(u64::unbox_unsigned(u64::box_unsigned(42)), Some(42));
    }
}
