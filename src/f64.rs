//! NaN boxing for `f64`.

pub mod u64;

const EXPONENT_MASK_OFFSET: usize = 48;
const SIGN_MASK: u64 = 1 << 63;
const EXPONENT_MASK: u64 = 0x7ff0 << EXPONENT_MASK_OFFSET;
const PAYLOAD_MASK: u64 = !(0xfff8 << EXPONENT_MASK_OFFSET);
const QUIET_MASK: u64 = 0x0008 << EXPONENT_MASK_OFFSET;

/// Returns `true` if a number contains a boxed value, or otherwise `false`.
pub fn is_boxed(number: f64) -> bool {
    number.to_bits() & QUIET_MASK != 0
}

/// Boxes a 51-bit unsigned integer.
pub fn box_unsigned(value: u64) -> f64 {
    f64::from_bits(EXPONENT_MASK | value)
}

/// Unboxes a 51-bit unsigned integer.
pub fn unbox_unsigned(number: f64) -> Option<u64> {
    number.is_nan().then_some(number.to_bits() & PAYLOAD_MASK)
}

/// Boxes a 52-bit signed integer.
pub fn box_signed(value: i64) -> f64 {
    f64::from_bits(
        (if value < 0 { SIGN_MASK } else { 0 }) | u64::box_unsigned(value.unsigned_abs()),
    )
}

/// Unboxes a 52-bit signed integer.
pub fn unbox_signed(number: f64) -> Option<i64> {
    unbox_unsigned(number).map(|value| {
        (if number.to_bits() & SIGN_MASK == 0 {
            1
        } else {
            -1
        }) * (value as i64)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_boxed {
        ($value:expr) => {
            let value = $value;

            assert!(is_boxed(value));
            assert!(value.is_nan());
        };
    }

    #[test]
    fn check_mask() {
        assert_eq!(EXPONENT_MASK, f64::NAN.to_bits());
    }

    #[test]
    fn check_calculated_nan_bits() {
        assert_eq!(f64::NAN.to_bits(), (0.0f64 / 0.0).to_bits());
    }

    #[test]
    fn box_unsigned_value() {
        assert_boxed!(box_unsigned(0));
        assert_boxed!(box_unsigned(1));
        assert_boxed!(box_unsigned(7));
        assert_boxed!(box_unsigned(42));
    }

    #[test]
    fn unbox_unsigned_value() {
        assert_eq!(unbox_unsigned(box_unsigned(0)), Some(0));
        assert_eq!(unbox_unsigned(box_unsigned(1)), Some(1));
        assert_eq!(unbox_unsigned(box_unsigned(7)), Some(7));
        assert_eq!(unbox_unsigned(box_unsigned(42)), Some(42));
    }

    #[test]
    fn box_signed_value() {
        assert_boxed!(r#box_signed(0));
        assert_boxed!(r#box_signed(1));
        assert_boxed!(r#box_signed(7));
        assert_boxed!(r#box_signed(42));
        assert_boxed!(r#box_signed(-1));
        assert_boxed!(r#box_signed(-7));
        assert_boxed!(r#box_signed(-42));
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
        assert_eq!(unbox_unsigned(0.0), None);
        assert_eq!(unbox_unsigned(-1.0), None);
        assert_eq!(unbox_unsigned(1.0), None);
        assert_eq!(unbox_unsigned(42.0), None);
    }
}
