macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!($bits, [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!($bits, [< udec $bits >], [<UD $bits>]); }
    };
    ($bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::*;

            super::test_impl!(@ $bits, $dec, $D);
        }
    };
    (@ 512, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($D::PI, $dec!(3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067982148086513282306647093844609550582231725359408128481))]
        fn test_math_consts(#[case] d: $D, #[case] expected: $D) {
            assert_eq!(d, expected);
        }

        #[rstest(::trace)]
        #[case($D::EPSILON, $dec!(0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001))]
        fn test_base_consts(#[case] d: $D, #[case] expected: $D) {
            assert_eq!(d, expected);
        }
    };
    (@ 256, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($D::PI, $dec!(3.1415926535897932384626433832795028841971693993751058209749445923078164062862))]
        fn test_math_consts(#[case] d: $D, #[case] expected: $D) {
            assert_eq!(d, expected);
        }

        #[rstest(::trace)]
        #[case($D::EPSILON, $dec!(0.00000000000000000000000000000000000000000000000000000000000000000000000000001))]
        fn test_base_consts(#[case] d: $D, #[case] expected: $D) {
            assert_eq!(d, expected);
        }
    };

    (@ 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($D::PI, $dec!(3.1415926535897932384626433832795028842))]
        fn test_math_consts(#[case] d: $D, #[case] expected: $D) {
            assert_eq!(d, expected);
        }

        #[rstest(::trace)]
        #[case($D::EPSILON, $dec!(0.00000000000000000000000000000000000001))]
        fn test_base_consts(#[case] d: $D, #[case] expected: $D) {
            assert_eq!(d, expected);
        }
    };
}

pub(crate) use test_impl;
