/// Sign bit
pub const SIGN_MASK: u64 = 0x8000_0000_0000_0000;

/// Exponent mask
pub const EXP_MASK: u64 = 0x7ff0_0000_0000_0000;

/// Mantissa mask
pub const MAN_MASK: u64 = 0x000f_ffff_ffff_ffff;

pub const MAN_MASK_NORMAL: u64 = 0x0010_0000_0000_0000;

pub const MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;

pub const MAX_EXP: i64 = f64::MAX_EXP as i64;

#[inline]
pub const fn to_bits(n: f64) -> u64 {
    unsafe { core::mem::transmute(n) }
}

#[inline]
#[allow(clippy::eq_op)]
pub const fn is_nan(n: f64) -> bool {
    n != n
}