/// Sign bit
pub const SIGN_MASK: u32 = 0x8000_0000;

/// Exponent mask
pub const EXP_MASK: u32 = 0x7f80_0000;

/// Mantissa mask
pub const MAN_MASK: u32 = 0x007f_ffff;

pub const MAN_MASK_NORMAL: u32 = 0x80_0000;

pub const MANTISSA_DIGITS: u32 = f32::MANTISSA_DIGITS;

pub const MAX_EXP: i64 = f32::MAX_EXP as i64;

#[inline]
pub const fn to_bits(n: f32) -> u32 {
    unsafe { core::mem::transmute(n) }
}

#[inline]
#[allow(clippy::eq_op)]
pub const fn is_nan(n: f32) -> bool {
    n != n
}