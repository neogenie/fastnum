macro_rules! test_impl {
    (I, $bits: literal) => {
        paste::paste! { test_impl!(SIGNED: $bits, [< i $bits >], [<I $bits>]); }
    };
    (U, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< u $bits >], [<U $bits>]); }
    };
    (UNSIGNED: $bits: tt, $bint: ident, $BInt: ident) => {
        mod $bint {
            use rstest::*;
            use fastnum::*;

            super::test_impl!(COMMON:: $bits, $bint, $BInt, THIS);
            super::test_impl!(UNSIGNED:: $bits, $bint, $BInt, THIS);
        }
    };
    (SIGNED: $bits: tt, $bint: ident, $BInt: ident) => {
        mod $bint {
            use rstest::*;
            use fastnum::*;

            super::test_impl!(COMMON:: $bits, $bint, $BInt, THIS);
            super::test_impl!(SIGNED:: $bits, $bint, $BInt, THIS);
        }
    };
    (COMMON:: 512, $bint: ident, $BInt: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $bint, $BInt);
    };
    (UNSIGNED:: 512, $bint: ident, $BInt: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $bint, $BInt);
    };
    (SIGNED:: 512, $bint: ident, $BInt: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $bint, $BInt);
    };


    (COMMON:: 256, $bint: ident, $BInt: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $bint, $BInt);
    };
    (COMMON:: 256, $bint: ident, $BInt: ident) => {
        super::test_impl!(COMMON:: 128, $bint, $BInt);
    };
    (UNSIGNED:: 256, $bint: ident, $BInt: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $bint, $BInt);
    };
    (UNSIGNED:: 256, $bint: ident, $BInt: ident) => {
        super::test_impl!(UNSIGNED:: 128, $bint, $BInt);
    };
    (SIGNED:: 256, $bint: ident, $BInt: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $bint, $BInt);
    };
    (SIGNED:: 256, $bint: ident, $BInt: ident) => {
        super::test_impl!(SIGNED:: 128, $bint, $BInt);
    };

    (COMMON:: 128, $bint: ident, $BInt: ident, THIS) => {
        super::test_impl!(COMMON:: 128, $bint, $BInt);
    };
    (COMMON:: 128, $bint: ident, $BInt: ident) => {
        super::test_impl!(COMMON:: 64, $bint, $BInt);
        super::test_impl!(TRY FROM $bint, $BInt, i128, u128);
    };
    (UNSIGNED:: 128, $bint: ident, $BInt: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $bint, $BInt);
    };
    (UNSIGNED:: 128, $bint: ident, $BInt: ident) => {
        super::test_impl!(UNSIGNED:: 64, $bint, $BInt);
    };
    (SIGNED:: 128, $bint: ident, $BInt: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $bint, $BInt);
    };
    (SIGNED:: 128, $bint: ident, $BInt: ident) => {
        super::test_impl!(SIGNED:: 64, $bint, $BInt);
        super::test_impl!(TRY FROM SIGNED $bint, $BInt, i128);
    };

    (COMMON:: 64, $bint: ident, $BInt: ident, THIS) => {
        super::test_impl!(COMMON:: 64, $bint, $BInt);
    };
    (COMMON:: 64, $bint: ident, $BInt: ident) => {
        super::test_impl!(FROM $bint, $BInt, u8, u16, u32);
    };
    (UNSIGNED:: 64, $bint: ident, $BInt: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 64, $bint, $BInt);
    };
    (UNSIGNED:: 64, $bint: ident, $BInt: ident) => {
        super::test_impl!(TRY FROM $bint, $BInt, i8, i16, i32, i64, isize);
        super::test_impl!(FROM $bint, $BInt, u64, usize);
    };
    (SIGNED:: 64, $bint: ident, $BInt: ident, THIS) => {
        super::test_impl!(SIGNED:: 64, $bint, $BInt);
    };
    (SIGNED:: 64, $bint: ident, $BInt: ident) => {
        super::test_impl!(FROM SIGNED $bint, $BInt, i8, i16, i32, i64, isize);
        super::test_impl!(TRY FROM $bint, $BInt, u64, usize);
    };

    (FROM $bint: ident, $BInt: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case(0, $BInt::ZERO)]
                #[case(1, $BInt::ONE)]
                #[case(2, $BInt::TWO)]
                #[case(10, $bint!(10))]
                #[case(100, $bint!(100))]
                #[case($Pt::MAX - 1, $BInt::from_str(format!("{}", $Pt::MAX - 1).as_str()).unwrap())]
                #[case($Pt::MAX,     $BInt::from_str(format!("{}", $Pt::MAX).as_str()).unwrap())]
                fn [< test_from_ $Pt >](#[case] n: $Pt, #[case] expected: $BInt) {
                    let d = $BInt::from(n);
                    assert_eq!(d, expected);
                }
            }
        )*
    };

    (TRY FROM $bint: ident, $BInt: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case(0, $BInt::ZERO)]
                #[case(1, $BInt::ONE)]
                #[case(2, $BInt::TWO)]
                #[case(10, $bint!(10))]
                #[case(100, $bint!(100))]
                #[case($Pt::MAX / 2,     $BInt::from_str(format!("{}", $Pt::MAX / 2).as_str()).unwrap())]
                fn [< test_try_from_ $Pt >](#[case] n: $Pt, #[case] expected: $BInt) {
                    let d = $BInt::try_from(n).unwrap();
                    assert_eq!(d, expected);
                }
            }
        )*
    };

    (TRY FROM UNSIGNED $bint: ident, $BInt: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case($Pt::MAX - 1, $BInt::from_str(format!("{}", $Pt::MAX - 1).as_str()).unwrap())]
                #[case($Pt::MAX,     $BInt::from_str(format!("{}", $Pt::MAX).as_str()).unwrap())]
                fn [< test_try_from_ $Pt >](#[case] n: $Pt, #[case] expected: $BInt) {
                    let d = $BInt::try_from(n).unwrap();
                    assert_eq!(d, expected);
                }
            }
        )*
    };

    (FROM SIGNED $bint: ident, $BInt: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case(-1, $BInt::NEG_ONE)]
                #[case(-2, $BInt::NEG_TWO)]
                #[case(-10, $bint!(-10))]
                #[case(-100, $bint!(-100))]
                #[case($Pt::MIN, $BInt::from_str(format!("{}", $Pt::MIN).as_str()).unwrap())]
                #[case($Pt::MIN + 1, $BInt::from_str(format!("{}", $Pt::MIN + 1).as_str()).unwrap())]
                fn [< test_from_ $Pt _signed>](#[case] n: $Pt, #[case] expected: $BInt) {
                    let d = $BInt::from(n);
                    assert_eq!(d, expected);
                }
            }
        )*
    };

    (TRY FROM SIGNED $bint: ident, $BInt: ident, $($Pt: ty),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case(-1, $BInt::NEG_ONE)]
                #[case(-2, $BInt::NEG_TWO)]
                #[case(-10, $bint!(-10))]
                #[case(-100, $bint!(-100))]
                #[case($Pt::MIN, $BInt::from_str(format!("{}", $Pt::MIN).as_str()).unwrap())]
                #[case($Pt::MIN + 1, $BInt::from_str(format!("{}", $Pt::MIN + 1).as_str()).unwrap())]
                fn [< test_try_from_ $Pt _signed>](#[case] n: $Pt, #[case] expected: $BInt) {
                    let d = $BInt::try_from(n).unwrap();
                    assert_eq!(d, expected);
                }
            }
        )*
    };
}

pub(crate) use test_impl;
