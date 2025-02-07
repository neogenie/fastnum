macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>]); }
    };

    (UNSIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{*, decimal::*};

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{*, decimal::*};

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(SIGNED:: $bits, $dec, $D, THIS);
        }
    };

    (COMMON:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);
    };
    (UNSIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };

    (COMMON:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);
    };
    (COMMON:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(COMMON:: 128, $dec, $D);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };

    (COMMON:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 128, $dec, $D);
    };
    (COMMON:: 128, $dec: ident, $D: ident) => {
        super::test_impl!(FROM UINT $dec, $D, u8, u16, u32, u64, u128, usize);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {
        super::test_impl!(FROM INT $dec, $D, i8, i16, i32, i64, i128, isize);
    };
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        super::test_impl!(FROM SIGNED $dec, $D, i8, i16, i32, i64, i128, isize);
    };

    (FROM UINT $dec: ident, $D: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case(0, $dec!(0))]
                #[case(1, $dec!(1))]
                #[case(10, $dec!(10))]
                #[case(100, $dec!(100))]
                #[case($Pt::MAX, $D::from_str(format!("{}", $Pt::MAX).as_str(), Context::default()).unwrap())]
                #[case($Pt::MAX - 1, $D::from_str(format!("{}", $Pt::MAX).as_str(), Context::default()).unwrap() - $dec!(1))]
                fn [< test_from_ $Pt >](#[case] n: $Pt, #[case] expected: $D) {
                    let d = $D::from(n);
                    assert_eq!(d, expected);
                }
            }
        )*
    };

    (FROM SIGNED $dec: ident, $D: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case(-1, $dec!(-1))]
                #[case(-10, $dec!(-10))]
                #[case(-100, $dec!(-100))]
                #[case($Pt::MIN, $D::from_str(format!("{}", $Pt::MIN).as_str(), Context::default()).unwrap())]
                #[case($Pt::MIN + 1, $D::from_str(format!("{}", $Pt::MIN).as_str(), Context::default()).unwrap() + $dec!(1))]
                fn [< test_from_ $Pt >](#[case] n: $Pt, #[case] expected: $D) {
                    let d = $D::from(n);
                    assert_eq!(d, expected);
                }
            }
        )*
    };

    (FROM INT $dec: ident, $D: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case(0, $dec!(0))]
                #[case(1, $dec!(1))]
                #[case(10, $dec!(10))]
                #[case(100, $dec!(100))]
                #[case($Pt::MAX, $D::from_str(format!("{}", $Pt::MAX).as_str(), Context::default()).unwrap())]
                #[case($Pt::MAX - 1, $D::from_str(format!("{}", $Pt::MAX).as_str(), Context::default()).unwrap() - $dec!(1))]
                fn [< test_from_ $Pt >](#[case] n: $Pt, #[case] expected: $D) {
                    let d = $D::try_from(n).unwrap();
                    assert_eq!(d, expected);
                }

                #[rstest(::trace)]
                #[case(-1)]
                #[case(-10)]
                #[case($Pt::MIN)]
                #[should_panic(expected = "(fastnum) number would be signed for unsigned type")]
                fn [< test_from_ $Pt _negative>](#[case] n: $Pt) {
                     let _ = $D::try_from(n).unwrap();
                }
            }
        )*
    };
}

pub(crate) use test_impl;
