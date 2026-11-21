macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(SIGNED: [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: [< udec $bits >], [<UD $bits>]); }
    };
    (UNSIGNED: $dec: ident, $D: ident) => {
        mod $dec {
            use fastnum::{decimal::*, *};
            use rstest::*;

            #[rstest(::trace)]
            #[case($dec!(0), $dec!(0))]
            #[case($dec!(3), $dec!(0))]
            #[case($dec!(3.00), $dec!(0))]
            #[case($dec!(2.25), $dec!(0.25))]
            #[case($dec!(0.001), $dec!(0.001))]
            #[case($dec!(1.5E+3), $dec!(0))]
            #[case($D::MAX, $dec!(0))]
            #[case($D::MIN_POSITIVE, $D::MIN_POSITIVE)]
            fn test_fract(#[case] d: $D, #[case] expected: $D) {
                let res = d.fract();

                assert_eq!(res, expected);
                assert!(res.is_op_ok());
                assert_eq!(res + d.trunc(), d);
            }

            #[rstest(::trace)]
            #[case($D::INFINITY)]
            #[case($D::NAN)]
            fn test_fract_special(#[case] d: $D) {
                let ctx = Context::default().without_traps();
                let res = d.with_ctx(ctx).fract();

                assert!(res.is_nan());
                assert!(res.is_op_invalid());
            }
        }
    };
    (SIGNED: $dec: ident, $D: ident) => {
        mod $dec {
            use fastnum::{decimal::*, *};
            use rstest::*;

            #[rstest(::trace)]
            #[case($dec!(0), $dec!(0))]
            #[case($dec!(3), $dec!(0))]
            #[case($dec!(3.00), $dec!(0))]
            #[case($dec!(2.25), $dec!(0.25))]
            #[case($dec!(-2.25), $dec!(-0.25))]
            #[case($dec!(-0.5), $dec!(-0.5))]
            #[case($dec!(0.001), $dec!(0.001))]
            #[case($dec!(1.5E+3), $dec!(0))]
            #[case($dec!(-1.5E+3), $dec!(0))]
            #[case($dec!(12345.6789E-2), $dec!(0.456789))]
            #[case($D::MAX, $dec!(0))]
            #[case($D::MIN, $dec!(0))]
            #[case($D::MIN_POSITIVE, $D::MIN_POSITIVE)]
            #[case(-$D::MIN_POSITIVE, -$D::MIN_POSITIVE)]
            fn test_fract(#[case] d: $D, #[case] expected: $D) {
                let res = d.fract();

                assert_eq!(res, expected);
                assert_eq!(res.is_sign_negative(), expected.is_sign_negative());
                assert!(res.is_op_ok());
                assert_eq!(res + d.trunc(), d);
            }

            #[rstest(::trace)]
            fn test_fract_largest_coefficient() {
                let ctx = Context::default();
                let d = $D::from_parts($D::MAX.digits(), -1, Sign::Minus, ctx);

                assert_eq!(d.fract(), $dec!(-0.5));
            }

            #[rstest(::trace)]
            #[case($dec!(-0))]
            #[case($dec!(-3))]
            #[case($dec!(-3.00))]
            fn test_fract_zero_is_positive(#[case] d: $D) {
                // As `self - self.trunc()` and the primitive floats give it.
                let res = d.fract();

                assert!(res.is_zero());
                assert!(res.is_sign_positive());
            }

            #[rstest(::trace)]
            #[case($D::INFINITY)]
            #[case($D::NEG_INFINITY)]
            #[case($D::NAN)]
            fn test_fract_special(#[case] d: $D) {
                let ctx = Context::default().without_traps();
                let res = d.with_ctx(ctx).fract();

                assert!(res.is_nan());
                assert!(res.is_op_invalid());
            }

            #[rstest(::trace)]
            #[should_panic(expected = "(fastnum) invalid operation")]
            fn test_fract_infinity_traps() {
                let _ = $D::INFINITY.fract();
            }
        }
    };
}

pub(crate) use test_impl;
