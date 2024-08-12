#![doc = include_str!("../README.md")]
#![no_std]

pub const EXPONENT_MASK_OFFSET: usize = 48;
pub const EXPONENT_MASK: u64 = 0x7ff0 << EXPONENT_MASK_OFFSET;

const fn build_mask(sub_mask: usize) -> u64 {
    ((sub_mask as u64) << TYPE_MASK_OFFSET) | EXPONENT_MASK
}
