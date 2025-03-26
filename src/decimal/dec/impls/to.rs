use core::num::IntErrorKind;

use crate::decimal::{dec::convert, Decimal, UnsignedDecimal, DecimalError};

type D<const N: usize> = Decimal<N>;

macro_rules! try_to_int_impls {
    ($($name:ident $int:ty,)*) => {
        $(
            impl<const N: usize> TryFrom<D<N>> for $int {
                type Error = IntErrorKind;

                #[inline]
                fn try_from(d: D<N>) -> Result<$int, Self::Error> {
                    convert::$name(d)
                }
            }
        )*

        impl<const N: usize> D<N> {
            $(
                #[inline]
                #[doc = concat!("Converts [Decimal] into [`", stringify!($int), "`].")]
                pub const fn $name(self) -> Result<$int, IntErrorKind> {
                    convert::$name(self)
                }
            )*
        }
    };
}

try_to_int_impls!(
    to_u8 u8,
    to_u16 u16,
    to_u32 u32,
    to_u64 u64,
    to_u128 u128,
    to_usize usize,

    to_i8 i8,
    to_i16 i16,
    to_i32 i32,
    to_i64 i64,
    to_i128 i128,
    to_isize isize,
);

impl<const N: usize> From<D<N>> for f32 {
    #[inline]
    fn from(d: D<N>) -> f32 {
        convert::to_f32(d)
    }
}
impl<const N: usize> From<D<N>> for f64 {
    #[inline]
    fn from(d: D<N>) -> f64 {
        convert::to_f64(d)
    }
}

impl<const N: usize> D<N> {
    /// Converts [Decimal] into [`f32`].
    pub const fn to_f32(self) -> f32 {
        convert::to_f32(self)
    }

    /// Converts [Decimal] into [`f64`].
    pub const fn to_f64(self) -> f64 {
        convert::to_f64(self)
    }

    /// Converts [Decimal] into [UnsignedDecimal].
    pub const fn to_unsigned_decimal(self) -> Result<UnsignedDecimal<N>, DecimalError> {
        UnsignedDecimal::from_decimal(self)
    }
}
