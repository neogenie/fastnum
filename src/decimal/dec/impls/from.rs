use crate::decimal::{dec::parse, Decimal};

type D<const N: usize> = Decimal<N>;

macro_rules! from_num_impls {
    ($($name:ident $num:ty,)*) => {
        $(
            impl<const N: usize> From<$num> for D<N> {
                #[inline]
                fn from(n: $num) -> Self {
                    parse::$name(n)
                }
            }
        )*

        impl<const N: usize> D<N> {
            $(
                #[inline]
                #[doc = concat!("Converts [`", stringify!($num), "`] to [Decimal].")]
                pub const fn $name(n: $num) -> Self {
                    parse::$name(n)
                }
            )*
        }
    };
}

from_num_impls!(
    from_u8 u8,
    from_u16 u16,
    from_u32 u32,
    from_u64 u64,
    from_u128 u128,
    from_usize usize,

    from_i8 i8,
    from_i16 i16,
    from_i32 i32,
    from_i64 i64,
    from_i128 i128,
    from_isize isize,

    from_f32 f32,
    from_f64 f64,
);
