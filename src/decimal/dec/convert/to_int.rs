use core::num::IntErrorKind;

use crate::{decimal::Decimal, int::convert};

type D<const N: usize> = Decimal<N>;

macro_rules! to_int_impl {
    ($to_int: ident, $int: ty, $to_uint: ident) => {
        #[inline]
        pub const fn $to_int<const N: usize>(d: D<N>) -> Result<$int, IntErrorKind> {
            if d.cb.is_special() {
                return Err(IntErrorKind::PosOverflow);
            }

            if d.is_negative() {
                match convert::$to_uint(d.rescale(0).digits) {
                    Err(e) => Err(e),
                    Ok(uint) => match (0 as $int).checked_sub_unsigned(uint) {
                        None => Err(IntErrorKind::NegOverflow),
                        Some(i) => Ok(i),
                    },
                }
            } else {
                convert::$to_int(d.rescale(0).digits)
            }
        }
    };
}

to_int_impl!(to_isize, isize, to_usize);
to_int_impl!(to_i8, i8, to_u8);
to_int_impl!(to_i16, i16, to_u16);
to_int_impl!(to_i32, i32, to_u32);
to_int_impl!(to_i64, i64, to_u64);
to_int_impl!(to_i128, i128, to_u128);
