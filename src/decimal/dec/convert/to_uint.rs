use core::num::IntErrorKind;

use crate::{decimal::Decimal, int::convert};

type D<const N: usize> = Decimal<N>;

macro_rules! to_uint_impl {
    ($to_uint: ident, $uint: ty) => {
        #[inline]
        pub const fn $to_uint<const N: usize>(d: D<N>) -> Result<$uint, IntErrorKind> {
            if d.flags().is_special() || d.is_negative() {
                return Err(IntErrorKind::PosOverflow);
            }

            convert::$to_uint(d.rescale(0).digits)
        }
    };
}

to_uint_impl!(to_usize, usize);
to_uint_impl!(to_u8, u8);
to_uint_impl!(to_u16, u16);
to_uint_impl!(to_u32, u32);
to_uint_impl!(to_u64, u64);
to_uint_impl!(to_u128, u128);
