//! A 64-bit "number" representation.

/// A 64-bit "number" representation that embraces 50-bit payloads, 50-bit
/// integers, and 64-bit floating-point numbers.
#[repr(u64)]
pub struct N64(u64);
