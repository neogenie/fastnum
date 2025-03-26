use core::num::IntErrorKind;

use crate::decimal::{UnsignedDecimal, Decimal};

type UD<const N: usize> = UnsignedDecimal<N>;

macro_rules! to_num_impls {
    ($($name:ident $num:ty,)*) => {
        $(
            impl<const N: usize> TryFrom<UD<N>> for $num {
                type Error = IntErrorKind;

                #[inline]
                fn try_from(ud: UD<N>) -> Result<Self, Self::Error> {
                    <$num>::try_from(ud.0)
                }
            }
        )*

        impl<const N: usize> UD<N> {
            $(
                #[inline]
                #[doc = concat!("Converts [UnsignedDecimal] into [`", stringify!($num), "`].")]
                pub const fn $name(self) -> Result<$num, IntErrorKind> {
                    self.0.$name()
                }
            )*
        }
    }
}

to_num_impls!(
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


impl<const N: usize> From<UD<N>> for f32 {
    #[inline]
    fn from(d: UD<N>) -> f32 {
        f32::from(d.0)
    }
}
impl<const N: usize> From<UD<N>> for f64 {
    #[inline]
    fn from(d: UD<N>) -> f64 {
        f64::from(d.0)
    }
}

impl<const N: usize> UD<N> {
    /// Converts [UnsignedDecimal] into [`f32`].
    pub const fn to_f32(self) -> f32 {
        self.0.to_f32()
    }

    /// Converts [UnsignedDecimal] into [`f64`].
    pub const fn to_f64(self) -> f64 {
        self.0.to_f64()
    }

    /// Converts [UnsignedDecimal] into [Decimal].
    pub const fn to_decimal(self) -> Decimal<N> {
        Decimal::from_unsigned_decimal(self)
    }
}
