use num_traits::ToPrimitive;

use crate::decimal::{dec::convert, Decimal};

macro_rules! to_int_impl {
    ($to_int: ident, $int: ty) => {
        #[inline]
        fn $to_int(&self) -> Option<$int> {
            match convert::$to_int(*self) {
                Ok(i) => Some(i),
                Err(_) => None,
            }
        }
    };
}

macro_rules! to_uint_impl {
    ($to_uint: ident, $uint: ty) => {
        #[inline]
        fn $to_uint(&self) -> Option<$uint> {
            match convert::$to_uint(*self) {
                Ok(u) => Some(u),
                Err(_) => None,
            }
        }
    };
}

macro_rules! to_float_impl {
    ($to_f: ident, $f: ident) => {
        #[inline]
        fn $to_f(&self) -> Option<$f> {
            Some(convert::$to_f(*self))
        }
    };
}

impl<const N: usize> ToPrimitive for Decimal<N> {
    to_int_impl!(to_isize, isize);
    to_int_impl!(to_i8, i8);
    to_int_impl!(to_i16, i16);
    to_int_impl!(to_i32, i32);
    to_int_impl!(to_i64, i64);
    to_int_impl!(to_i128, i128);

    to_uint_impl!(to_usize, usize);
    to_uint_impl!(to_u8, u8);
    to_uint_impl!(to_u16, u16);
    to_uint_impl!(to_u32, u32);
    to_uint_impl!(to_u64, u64);
    to_uint_impl!(to_u128, u128);

    to_float_impl!(to_f32, f32);
    to_float_impl!(to_f64, f64);
}
