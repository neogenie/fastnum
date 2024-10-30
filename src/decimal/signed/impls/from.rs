use crate::decimal::signed::{parse, Decimal, Sign};
use crate::decimal::unsigned::UnsignedDecimal;
use crate::decimal::ParseError;
use crate::{U128, U256, U512};

macro_rules! from_uint {
    ($($uint: tt),*) => {
        $(
            impl<UINT> From<$uint> for Decimal<UINT>
            where
                UnsignedDecimal<UINT>: From<$uint>
            {
                #[inline]
                fn from(n: $uint) -> Self {
                    Self::new(UnsignedDecimal::<UINT>::from(n), Sign::NoSign)
                }
            }
        )*
    }
}

macro_rules! from_int {
    ($($int: tt as $uint: tt),*) => {
        $(
            impl<UINT> From<$int> for Decimal<UINT>
            where
                UnsignedDecimal<UINT>: From<$uint>
            {
                #[inline]
                fn from(n: $int) -> Self {
                    if n.is_negative() {
                        Self::new(UnsignedDecimal::<UINT>::from(n.unsigned_abs()), Sign::Minus)
                    } else { 
                        Self::new(UnsignedDecimal::<UINT>::from(n as $uint), Sign::NoSign)
                    }
                }
            }
        )*
    }
}

from_uint!(u8, u16, u32, u64, u128, usize);
from_int!(
    i8 as u8,
    i16 as u16,
    i32 as u32,
    isize as usize,
    i64 as u64,
    i128 as u128
);

macro_rules! try_from_float {
    ($UINT: ident, $bits: literal, $name: ident) => {
        impl TryFrom<f32> for Decimal<$UINT> {
            type Error = ParseError;

            #[inline]
            fn try_from(n: f32) -> Result<Self, Self::Error> {
                parse::$name::from_f32(n)
            }
        }

        impl TryFrom<f64> for Decimal<$UINT> {
            type Error = ParseError;

            #[inline]
            fn try_from(n: f64) -> Result<Self, Self::Error> {
                parse::$name::from_f64(n)
            }
        }
    };
}

try_from_float!(U128, 128, d128);
try_from_float!(U256, 256, d256);
try_from_float!(U512, 512, d512);
