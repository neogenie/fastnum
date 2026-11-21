macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!([< dec $bits >], [<D $bits>]); }
    };
    ($dec: ident, $D: ident) => {
        mod $dec {
            use fastnum::{decimal::*, *};
            use rstest::*;

            #[rstest(::trace)]
            #[case($dec!(2), $dec!(1), $dec!(2))]
            #[case($dec!(2), $dec!(-1), $dec!(-2))]
            #[case($dec!(-2), $dec!(1), $dec!(2))]
            #[case($dec!(-2), $dec!(-1), $dec!(-2))]
            #[case($dec!(2), $dec!(-0), $dec!(-2))]
            #[case($dec!(-2), $dec!(0), $dec!(2))]
            #[case($dec!(1.50), $dec!(-7), $dec!(-1.50))]
            #[case($dec!(0), $dec!(-1), $dec!(-0))]
            #[case($D::INFINITY, $dec!(-1), $D::NEG_INFINITY)]
            #[case($D::NEG_INFINITY, $D::INFINITY, $D::INFINITY)]
            #[case($D::MAX, $D::NEG_INFINITY, $D::MIN)]
            fn test_copysign(#[case] d: $D, #[case] sign: $D, #[case] expected: $D) {
                let res = d.copysign(sign);

                assert_eq!(res, expected);
                assert_eq!(res.is_sign_negative(), sign.is_sign_negative());
                assert_eq!(res.fractional_digits_count(), d.fractional_digits_count());
                assert!(res.is_op_ok());
            }

            #[rstest(::trace)]
            fn test_copysign_nan() {
                // A sign-bit operation, quiet on NaN as IEEE 754 makes it.
                let res = $D::NAN.copysign($dec!(-1));
                assert!(res.is_nan());
                assert!(res.is_sign_negative());
                assert!(!res.is_op_invalid());

                let res = res.copysign($dec!(1));
                assert!(res.is_nan());
                assert!(res.is_sign_positive());

                let res = $dec!(2).copysign(-$D::NAN);
                assert_eq!(res, $dec!(-2));
                assert!(!res.is_op_invalid());
            }
        }
    };
}

pub(crate) use test_impl;
