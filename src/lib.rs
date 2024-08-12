#![doc = include_str!("../README.md")]
#![no_std]

/// An offset to an exponent mask.
pub const EXPONENT_MASK_OFFSET: usize = 48;
/// An exponent mask.
pub const EXPONENT_MASK: u64 = 0x7ff0 << EXPONENT_MASK_OFFSET;
