#![doc = include_str!("../README.md")]
#![no_std]

/// An offset to an exponent mask.
pub const EXPONENT_MASK_OFFSET: usize = 48;

/// An exponent mask.
pub const EXPONENT_MASK: u64 = 0x7ff0 << EXPONENT_MASK_OFFSET;

pub const fn r#box(value: u64) -> f64 {
    f64::NAN.to_bits() | value
}

pub const fn unbox(value: u64) -> f64 {
    f64::NAN.to_bits() | value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unbox_value() {
        assert_eq!();
    }
}
