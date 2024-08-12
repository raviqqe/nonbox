#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(test)]
extern crate std;

const EXPONENT_MASK_OFFSET: usize = 48;
const EXPONENT_MASK: u64 = 0x7ff8 << EXPONENT_MASK_OFFSET;
const BOXED_VALUE_MASK: u64 = !(0xfff8 << EXPONENT_MASK_OFFSET);

/// Boxes an unsigned value into `f64`.
///
/// The `value` needs to be less than `1 << 51`. Otherwise, it is truncated.
pub fn r#box(value: u64) -> f64 {
    f64::from_bits(EXPONENT_MASK | value)
}

/// Unboxes an unsigned value from `f64`.
pub fn unbox(number: f64) -> Option<u64> {
    number
        .is_nan()
        .then_some(number.to_bits() & BOXED_VALUE_MASK)
}

/// Boxes a signed value into `f64`.
pub fn r#box_signed(value: i64) -> f64 {
    (if value < 0 { -1.0 } else { 1.0 }) * r#box(value as u64)
}

/// Unboxes a signed value from `f64`.
pub fn unbox_signed(number: f64) -> Option<i64> {
    unbox(number).map(|value| (if number < 0.0 { -1 } else { 1 }) * (value as i64))
}

/// Boxes a value into `u64` representation of `f64`.
pub fn box_u64(value: u64) -> u64 {
    r#box(value).to_bits()
}

/// Unboxes a value from `u64` representation of `f64`.
pub fn unbox_u64(number: u64) -> Option<u64> {
    unbox(f64::from_bits(number))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_mask() {
        assert_eq!(EXPONENT_MASK, f64::NAN.to_bits());
    }

    #[test]
    fn box_unsigned_value() {
        assert!(r#box(0).is_nan());
        assert!(r#box(1).is_nan());
        assert!(r#box(7).is_nan());
        assert!(r#box(42).is_nan());
    }

    #[test]
    fn unbox_unsigned_value() {
        assert_eq!(unbox(r#box(0)), Some(0));
        assert_eq!(unbox(r#box(1)), Some(1));
        assert_eq!(unbox(r#box(7)), Some(7));
        assert_eq!(unbox(r#box(42)), Some(42));
    }

    #[test]
    fn box_signed_value() {
        assert!(r#box_signed(0).is_nan());
        assert!(r#box_signed(1).is_nan());
        assert!(r#box_signed(7).is_nan());
        assert!(r#box_signed(42).is_nan());
        assert!(r#box_signed(-1).is_nan());
        assert!(r#box_signed(-7).is_nan());
        assert!(r#box_signed(-42).is_nan());
    }

    #[test]
    fn unbox_signed_value() {
        assert_eq!(unbox_signed(r#box_signed(0)), Some(0));
        assert_eq!(unbox_signed(r#box_signed(1)), Some(1));
        assert_eq!(unbox_signed(r#box_signed(7)), Some(7));
        assert_eq!(unbox_signed(r#box_signed(42)), Some(42));
        assert_eq!(unbox_signed(r#box_signed(-1)), Some(-1));
        assert_eq!(unbox_signed(r#box_signed(-7)), Some(-7));
        assert_eq!(unbox_signed(r#box_signed(-42)), Some(-42));
    }

    #[test]
    fn unbox_f64_value() {
        assert_eq!(unbox(0.0), None);
        assert_eq!(unbox(-1.0), None);
        assert_eq!(unbox(1.0), None);
        assert_eq!(unbox(42.0), None);
    }

    #[test]
    fn unbox_value_in_u64() {
        assert_eq!(unbox_u64(42.0f64.to_bits()), None);
        assert_eq!(unbox_u64(r#box_u64(0)), Some(0));
        assert_eq!(unbox_u64(r#box_u64(1)), Some(1));
        assert_eq!(unbox_u64(r#box_u64(7)), Some(7));
        assert_eq!(unbox_u64(r#box_u64(42)), Some(42));
    }
}
