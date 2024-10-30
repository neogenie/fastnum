use crate::decimal::unsigned::{parse, UnsignedDecimal};
use crate::decimal::ParseError;
use crate::{U128, U256, U512};

macro_rules! from_uint {
    ($($uint: tt),*) => {
        $(
            impl<UINT> From<$uint> for UnsignedDecimal<UINT>
            where
                UINT: From<$uint>
            {
                #[inline]
                fn from(int: $uint) -> Self {
                    Self::new(UINT::from(int), 0)
                }
            }
        )*
    }
}

macro_rules! try_from_int {
    ($($int: tt as $uint: tt),*) => {
        $(
            impl<UINT> TryFrom<$int> for UnsignedDecimal<UINT>
            where
                UINT: From<$uint>
            {
                type Error = ParseError;

                #[inline]
                fn try_from(int: $int) -> Result<Self, Self::Error> {
                    if int.is_negative() {
                        return Err(ParseError::Signed);
                    }
                    let bits = int as $uint;
                    Ok(Self::new(UINT::from(bits), 0))
                }
            }
        )*
    }
}

from_uint!(u8, u16, u32, u64, u128, usize);
try_from_int!(
    i8 as u8,
    i16 as u16,
    i32 as u32,
    isize as usize,
    i64 as u64,
    i128 as u128
);

macro_rules! try_from_float {
    ($UINT: ident, $bits: literal, $name: ident) => {
        impl TryFrom<f32> for UnsignedDecimal<$UINT> {
            type Error = ParseError;

            #[inline]
            fn try_from(n: f32) -> Result<Self, Self::Error> {
                parse::$name::from_f32(n)
            }
        }

        impl TryFrom<f64> for UnsignedDecimal<$UINT> {
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
