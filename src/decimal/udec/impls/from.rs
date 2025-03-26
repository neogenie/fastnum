use crate::decimal::{Decimal, DecimalError, ParseError, UnsignedDecimal};

type D<const N: usize> = Decimal<N>;
type UD<const N: usize> = UnsignedDecimal<N>;

impl<const N: usize> TryFrom<D<N>> for UD<N> {
    type Error = DecimalError;

    #[inline]
    fn try_from(d: D<N>) -> Result<Self, Self::Error> {
        if d.is_negative() {
            return Err(DecimalError::Invalid);
        }
        Ok(Self::new(d))
    }
}
impl<const N: usize> UD<N> {
    /// Converts from [Decimal] to [UnsignedDecimal].
    pub const fn from_decimal(d: D<N>) -> Result<Self, DecimalError> {
        if d.is_negative() {
            return Err(DecimalError::Invalid);
        }
        Ok(Self::new(d))
    }
}

impl<const N: usize> From<UD<N>> for D<N> {
    #[inline]
    fn from(ud: UD<N>) -> Self {
        ud.0
    }
}
impl<const N: usize> D<N> {
    /// Converts from [UnsignedDecimal] to [Decimal].
    pub const fn from_unsigned_decimal(ud: UD<N>) -> Self {
        ud.0
    }
}

macro_rules! from_uint {
    ($($name:ident $uint:ty,)*) => {
        $(
            impl<const N: usize> From<$uint> for UD<N> {
                #[inline]
                fn from(n: $uint) -> Self {
                    Self::new(D::from(n))
                }
            }
        )*

        impl<const N: usize> UD<N> {
            $(
                #[doc = concat!("Converts [`", stringify!($uint), "`] to [UnsignedDecimal].")]
                pub const fn $name(n: $uint) -> Self {
                    Self::new(D::$name(n))
                }
            )*
        }
    }
}

macro_rules! try_from_num {
    ($($name:ident $num:ty,)*) => {
        $(
            impl<const N: usize> TryFrom<$num> for UD<N> {
                type Error = ParseError;

                #[inline]
                fn try_from(n: $num) -> Result<Self, Self::Error> {
                    if n < (0 as $num) {
                        return Err(ParseError::Signed);
                    }
                    Ok(Self::new(D::from(n)))
                }
            }
        )*

        impl<const N: usize> UD<N> {
            $(
                #[doc = concat!("Converts [`", stringify!($num), "`] to [UnsignedDecimal].")]
                pub const fn $name(n: $num) -> Result<Self, ParseError> {
                    if n < (0 as $num) {
                        return Err(ParseError::Signed);
                    }
                    Ok(Self::new(D::$name(n)))
                }
            )*
        }
    }
}

from_uint!(
    from_u8 u8,
    from_u16 u16,
    from_u32 u32,
    from_u64 u64,
    from_u128 u128,
    from_usize usize,
);

try_from_num!(
    from_i8 i8,
    from_i16 i16,
    from_i32 i32,
    from_i64 i64,
    from_i128 i128,
    from_isize isize,

    from_f32 f32,
    from_f64 f64,
);

