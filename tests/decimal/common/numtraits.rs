macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>]); }
    };
    (UNSIGNED: $bits: tt, $dec: ident, $D: ident) => {
        paste::paste! {
            mod [< $dec _unsigned >] {
                use rstest::*;
                use fastnum::{$dec, $D};
                use num_traits::{FromPrimitive, ToPrimitive};

                super::test_impl!(UNSIGNED:: $bits, $dec, $D);
            }
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        paste::paste! {
            mod [< $dec _signed >]{
                use rstest::*;
                use fastnum::{$dec, $D};
                use num_traits::{FromPrimitive, ToPrimitive};

                super::test_impl!(SIGNED:: $bits, $dec, $D);
            }
        }
    };
    (UNSIGNED:: 512, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 512, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED TO_UINT $dec, $D, u8, u16, u32, u64, u128, usize);
        super::test_impl!(UNSIGNED TO_INT $dec, $D, i8, i16, i32, i64, i128, isize);

        super::test_impl!(UNSIGNED FROM_UINT $dec, $D, u8, u16, u32, u64, u128, usize);
        super::test_impl!(UNSIGNED FROM_INT $dec, $D, i8, i16, i32, i64, i128, isize);
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED TO_UINT $dec, $D, u8, u16, u32, u64, u128, usize);
        super::test_impl!(SIGNED TO_INT $dec, $D, i8, i16, i32, i64, i128, isize);

        super::test_impl!(SIGNED FROM_UINT $dec, $D, u8, u16, u32, u64, u128, usize);
        super::test_impl!(SIGNED FROM_INT $dec, $D, i8, i16, i32, i64, i128, isize);
    };
    (UNSIGNED TO_UINT $dec: ident, $D: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case($dec!(0), 0)]
                #[case($dec!(1), 1)]
                #[case($dec!(10), 10)]
                #[case($dec!(10.5), 11)]
                #[case($D::from($Pt::MAX), $Pt::MAX)]
                #[case($D::from($Pt::MAX) - $dec!(1), $Pt::MAX - 1)]
                #[case($D::from($Pt::MAX) + $dec!(0.1), $Pt::MAX)]
                #[case($D::from($Pt::MAX) - $dec!(0.1), $Pt::MAX)]
                fn [< test_to_ $Pt >](#[case] d: $D, #[case] expected: $Pt) {
                    assert_eq!(d.[< to_ $Pt>](), Some(expected));
                }
            }
        )*
    };
    (UNSIGNED TO_INT $dec: ident, $D: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case($dec!(0), 0)]
                #[case($dec!(1), 1)]
                #[case($dec!(10), 10)]
                #[case($dec!(10.5), 11)]
                #[case($D::try_from($Pt::MAX).unwrap(), $Pt::MAX)]
                #[case($D::try_from($Pt::MAX).unwrap() - $dec!(1), $Pt::MAX - 1)]
                #[case($D::try_from($Pt::MAX).unwrap() + $dec!(0.1), $Pt::MAX)]
                #[case($D::try_from($Pt::MAX).unwrap() - $dec!(0.1), $Pt::MAX)]
                fn [< test_to_ $Pt >](#[case] d: $D, #[case] expected: $Pt) {
                    assert_eq!(d.[< to_ $Pt>](), Some(expected));
                }
            }
        )*
    };
    (SIGNED TO_UINT $dec: ident, $D: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case($dec!(0), 0)]
                #[case($dec!(1), 1)]
                #[case($dec!(10), 10)]
                #[case($dec!(10.5), 11)]
                #[case($D::from($Pt::MAX), $Pt::MAX)]
                #[case($D::from($Pt::MAX) - $dec!(1), $Pt::MAX - 1)]
                #[case($D::from($Pt::MAX) + $dec!(0.1), $Pt::MAX)]
                #[case($D::from($Pt::MAX) - $dec!(0.1), $Pt::MAX)]
                #[case($D::from($Pt::MAX) - $dec!(1), $Pt::MAX - 1)]
                fn [< test_to_ $Pt _signed>](#[case] d: $D, #[case] expected: $Pt) {
                    assert_eq!(d.[< to_ $Pt>](), Some(expected));
                }

                #[rstest(::trace)]
                #[case($dec!(-0))]
                #[case($dec!(-1))]
                #[case($dec!(-10))]
                fn [< test_to_ $Pt _signed_negative>](#[case] d: $D) {
                    assert!(d.[< to_ $Pt>]().is_none());
                }
            }
        )*
    };
    (SIGNED TO_INT $dec: ident, $D: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case($dec!(0), 0)]
                #[case($dec!(1), 1)]
                #[case($dec!(10), 10)]
                #[case($dec!(10.1), 10)]
                #[case($dec!(10.5), 11)]
                #[case($dec!(-1), -1)]
                #[case($dec!(-0), 0)]
                #[case($dec!(-10.5), -11)]
                #[case($D::from($Pt::MAX), $Pt::MAX)]
                #[case($D::from($Pt::MAX) + $dec!(0.1), $Pt::MAX)]
                #[case($D::from($Pt::MAX) - $dec!(0.1), $Pt::MAX)]
                #[case($D::from($Pt::MAX) - $dec!(1), $Pt::MAX - 1)]
                #[case($D::from($Pt::MIN), $Pt::MIN)]
                #[case($D::from($Pt::MIN) + $dec!(1), $Pt::MIN + 1)]
                #[case($D::from($Pt::MIN) - $dec!(0.1), $Pt::MIN)]
                #[case($D::from($Pt::MAX).neg(), $Pt::MIN + 1)]
                fn [< test_to_ $Pt _signed>](#[case] d: $D, #[case] expected: $Pt) {
                    assert_eq!(d.[< to_ $Pt>](), Some(expected));
                }

                #[rstest(::trace)]
                #[case($D::from($Pt::MIN).neg())]
                #[case($D::from($Pt::MAX) + $dec!(1))]
                #[case($D::from($Pt::MAX) + $dec!(0.5))]
                #[case($D::from($Pt::MIN) - $dec!(1))]
                #[case($D::from($Pt::MIN) - $dec!(0.5))]
                fn [< test_to_ $Pt _signed_negative>](#[case] d: $D) {
                    assert!(d.[< to_ $Pt>]().is_none());
                }
            }
        )*
    };

    (UNSIGNED FROM_UINT $dec: ident, $D: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case(0, $dec!(0))]
                #[case(1, $dec!(1))]
                #[case(10, $dec!(10))]
                #[case(100, $dec!(100))]
                #[case($Pt::MAX, $D::from($Pt::MAX))]
                #[case($Pt::MAX - 1, $D::from($Pt::MAX) - $dec!(1))]
                fn [< test_from_ $Pt >](#[case] n: $Pt, #[case] expected: $D) {
                    assert_eq!($D::[< from_ $Pt>](n), Some(expected));
                }
            }
        )*
    };
    (UNSIGNED FROM_INT $dec: ident, $D: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case(0, $dec!(0))]
                #[case(1, $dec!(1))]
                #[case(10, $dec!(10))]
                #[case(100, $dec!(100))]
                #[case($Pt::MAX, $D::try_from($Pt::MAX).unwrap())]
                #[case($Pt::MAX - 1, $D::try_from($Pt::MAX).unwrap() - $dec!(1))]
                fn [< test_from_ $Pt >](#[case] n: $Pt, #[case] expected: $D) {
                    assert_eq!($D::[< from_ $Pt>](n), Some(expected));
                }

                #[rstest(::trace)]
                #[case($Pt::MIN)]
                fn [< test_from_ $Pt _negative>](#[case] n: $Pt) {
                    assert!($D::[< from_ $Pt>](n).is_none());
                }
            }
        )*
    };
    (SIGNED FROM_UINT $dec: ident, $D: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case(0, $dec!(0))]
                #[case(1, $dec!(1))]
                #[case(10, $dec!(10))]
                #[case(100, $dec!(100))]
                #[case($Pt::MAX, $D::from($Pt::MAX))]
                #[case($Pt::MAX - 1, $D::from($Pt::MAX) - $dec!(1))]
                fn [< test_from_ $Pt >](#[case] n: $Pt, #[case] expected: $D) {
                    assert_eq!($D::[< from_ $Pt>](n), Some(expected));
                }
            }
        )*
    };
    (SIGNED FROM_INT $dec: ident, $D: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case(0, $dec!(0))]
                #[case(1, $dec!(1))]
                #[case(-1, $dec!(-1))]
                #[case(10, $dec!(10))]
                #[case(-10, $dec!(-10))]
                #[case(100, $dec!(100))]
                #[case(-100, $dec!(-100))]
                #[case($Pt::MIN, $D::from($Pt::MIN))]
                #[case($Pt::MIN + 1, $D::from($Pt::MIN) + $dec!(1))]
                #[case($Pt::MAX, $D::from($Pt::MAX))]
                #[case($Pt::MAX - 1, $D::from($Pt::MAX) - $dec!(1))]
                fn [< test_from_ $Pt >](#[case] n: $Pt, #[case] expected: $D) {
                    assert_eq!($D::[< from_ $Pt>](n), Some(expected));
                }
            }
        )*
    };
}

pub(crate) use test_impl;
