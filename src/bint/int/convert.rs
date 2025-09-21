mod from_float;
mod from_int;
mod from_uint;
mod to_int;
mod to_uint;

use from_float::*;
use from_int::*;
use from_uint::*;
use to_int::*;
use to_uint::*;

use bnum::BInt;
use core::str::from_utf8_unchecked;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

use crate::bint::{
    convert, doc, error::from_int_error_kind, int::intrinsics, Int, ParseError, UInt,
};

impl<const N: usize> Int<N> {
    convert::from_str::from_str_impl!(Int, I, BInt);

    convert::from_bytes::from_bytes_impl!(Int, I, BInt);

    from_uint_impl!(
        from_u8 <- u8,
        from_u16 <- u16,
        from_u32 <- u32
    );

    try_from_uint_impl!(
        from_u64 <- u64,
        from_usize <- usize,
        from_u128 <- u128 #TRY
    );

    from_int_impl!(
        from_i8 <- i8 (from_u8 <- u8),
        from_i16 <- i16 (from_u16 <- u16),
        from_i32 <- i32 (from_u32 <- u32),
        from_i64 <- i64 (from_u64 <- u64),
        from_isize <- isize (from_usize <- usize)
    );

    try_from_int_impl!(
        from_i128 <- i128
    );

    from_float_impl!(from_f32, f32);
    from_float_impl!(from_f64, f64);
}

impl<const N: usize> Int<N> {
    convert::to_str::to_str_impl!(Int, I, BInt);

    convert::to_bytes::to_bytes_impl!(Int, I, BInt);

    to_int_impl!(
        to_i8 -> i8,
        to_i16 -> i16,
        to_i32 -> i32,
        to_i64 -> i64,
        to_i128 -> i128,
        to_isize -> isize
    );

    to_uint_impl!(
        to_u8 -> u8,
        to_u16 -> u16,
        to_u32 -> u32,
        to_u64 -> u64,
        to_u128 -> u128,
        to_usize -> usize
    );
}
