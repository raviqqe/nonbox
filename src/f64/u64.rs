//! NaN boxing for `u64` representation of `f64`.

/// Boxes a 50-bit unsigned integer.
pub const fn box_unsigned(value: u64) -> u64 {
    super::box_unsigned(value).to_bits()
}

/// Unboxes a 50-bit unsigned integer.
pub const fn unbox_unsigned(number: u64) -> Option<u64> {
    super::unbox_unsigned(f64::from_bits(number))
}

/// Boxes a 51-bit signed integer.
pub const fn box_signed(value: i64) -> u64 {
    super::box_signed(value).to_bits()
}

/// Unboxes a 51-bit signed integer.
pub const fn unbox_signed(number: u64) -> Option<i64> {
    super::unbox_signed(f64::from_bits(number))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unbox_unsigned_value() {
        assert_eq!(unbox_unsigned(42.0f64.to_bits()), None);
        assert_eq!(unbox_unsigned(box_unsigned(0)), Some(0));
        assert_eq!(unbox_unsigned(box_unsigned(1)), Some(1));
        assert_eq!(unbox_unsigned(box_unsigned(7)), Some(7));
        assert_eq!(unbox_unsigned(box_unsigned(42)), Some(42));
    }

    #[test]
    fn unbox_signed_value() {
        assert_eq!(unbox_signed(42.0f64.to_bits()), None);
        assert_eq!(unbox_signed(box_signed(0)), Some(0));
        assert_eq!(unbox_signed(box_signed(1)), Some(1));
        assert_eq!(unbox_signed(box_signed(7)), Some(7));
        assert_eq!(unbox_signed(box_signed(42)), Some(42));
        assert_eq!(unbox_signed(box_signed(-1)), Some(-1));
        assert_eq!(unbox_signed(box_signed(-7)), Some(-7));
        assert_eq!(unbox_signed(box_signed(-42)), Some(-42));
    }
}
