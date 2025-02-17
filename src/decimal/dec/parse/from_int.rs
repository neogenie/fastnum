use crate::{
    decimal::{
        dec::{Context, ControlBlock, ExtraPrecision, Signals},
        Decimal, Sign,
    },
    int::convert,
};

type D<const N: usize> = Decimal<N>;

macro_rules! from_int {
    ($n: ident, $nu: ident, $int: ty) => {
        #[inline]
        pub const fn $n<const N: usize>(n: $int) -> D<N> {
            let sign = if n.is_negative() {
                Sign::Minus
            } else {
                Sign::Plus
            };

            D::new(
                convert::$nu(n.unsigned_abs()),
                ControlBlock::new(
                    0,
                    sign,
                    Signals::empty(),
                    Context::default(),
                    ExtraPrecision::new(),
                ),
            )
        }
    };
}

from_int!(from_i8, from_u8, i8);
from_int!(from_i16, from_u16, i16);
from_int!(from_i32, from_u32, i32);
from_int!(from_i64, from_u64, i64);
from_int!(from_i128, from_u128, i128);
from_int!(from_isize, from_usize, isize);
