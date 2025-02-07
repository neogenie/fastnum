use core::num::IntErrorKind;

use crate::int::{intrinsics::*, UInt};

type U<const N: usize> = UInt<N>;

macro_rules! to_int_impl {
    ($to_int: ident, $int: ident) => {
        #[inline]
        pub const fn $to_int<const N: usize>(int: U<N>) -> Result<$int, IntErrorKind> {
            let digits = int.digits();
            let mut out = 0;
            let mut i = 0;

            if Digit::BITS > <$int>::BITS {
                let small = digits[i] as $int;
                let trunc = small as Digit;
                if digits[i] != trunc {
                    return Err(IntErrorKind::PosOverflow);
                }
                out = small;
                i = 1;
            } else {
                loop {
                    let shift = i << BIT_SHIFT;
                    if i >= N || shift >= <$int>::BITS as usize {
                        break;
                    }
                    out |= (digits[i] as $int) << shift;
                    i += 1;
                }
            }

            #[allow(unused_comparisons)]
            if out < 0 {
                return Err(IntErrorKind::PosOverflow);
            }

            while i < N {
                if digits[i] != 0 {
                    return Err(IntErrorKind::PosOverflow);
                }
                i += 1;
            }

            Ok(out)
        }
    };
}

macro_rules! from_uint_impl {
    ($from_uint: ident, $uint: ident) => {
        #[inline]
        pub const fn $from_uint<const N: usize>(uint: $uint) -> U<N> {
            const UINT_BITS: usize = $uint::BITS as usize;
            debug_assert!(UINT_BITS <= N * (BITS as usize));

            let mut digits = [0; N];
            let mut i = 0;
            while i << BIT_SHIFT < UINT_BITS {
                let d = (uint >> (i << BIT_SHIFT)) as Digit;
                if d != 0 {
                    digits[i] = d;
                }
                i += 1;
            }
            U::from_digits(digits)
        }
    };
}

to_int_impl!(to_u8, u8);
to_int_impl!(to_u16, u16);
to_int_impl!(to_u32, u32);
to_int_impl!(to_u64, u64);
to_int_impl!(to_u128, u128);
to_int_impl!(to_usize, usize);

to_int_impl!(to_i8, i8);
to_int_impl!(to_i16, i16);
to_int_impl!(to_i32, i32);
to_int_impl!(to_i64, i64);
to_int_impl!(to_i128, i128);
to_int_impl!(to_isize, isize);

from_uint_impl!(from_u8, u8);
from_uint_impl!(from_u16, u16);
from_uint_impl!(from_u32, u32);
from_uint_impl!(from_u64, u64);
from_uint_impl!(from_u128, u128);
from_uint_impl!(from_usize, usize);
