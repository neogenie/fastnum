macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!([< dec $bits >], [< udec $bits >], [<D $bits>], [<UD $bits>]); }
    };
    ($dec: ident, $udec: ident, $D: ident, $UD: ident) => {
        mod $dec {
            use fastnum::{decimal::*, *};
            use rstest::*;

            #[rstest(::trace)]
            #[case($dec!(0), $dec!(0), $udec!(0))]
            #[case($dec!(0.00), $dec!(0.00), $udec!(0.00))]
            #[case($dec!(-0.00), $dec!(0.00), $udec!(0.00))]
            #[case($dec!(1), $dec!(1), $udec!(1))]
            #[case($dec!(-1), $dec!(1), $udec!(1))]
            #[case($dec!(1.00), $dec!(1.00), $udec!(1.00))]
            #[case($dec!(-1.00), $dec!(1.00), $udec!(1.00))]
            #[case($dec!(-2), $dec!(2), $udec!(2))]
            #[case($dec!(-2.00), $dec!(2.00), $udec!(2.00))]
            #[case($dec!(2000000), $dec!(2000000), $udec!(2000000))]
            #[case($dec!(-2000000), $dec!(2000000), $udec!(2000000))]
            #[case($dec!(+0.1), $dec!(0.1), $udec!(0.1))]
            #[case($dec!(-0.1), $dec!(0.1), $udec!(0.1))]
            #[case($dec!(2.1), $dec!(2.1), $udec!(2.1))]
            #[case($dec!(-100), $dec!(100), $udec!(100))]
            #[case($dec!(101.5), $dec!(101.5), $udec!(101.5))]
            #[case($dec!(-101.5), $dec!(101.5), $udec!(101.5))]
            #[case($dec!(-56267E-10), $dec!(0.0000056267), $udec!(0.0000056267))]
            #[case($dec!(-56267E-5), $dec!(0.56267), $udec!(0.56267))]
            #[case($dec!(-56267E-2), $dec!(562.67), $udec!(562.67))]
            #[case($dec!(-56267E-1), $dec!(5626.7), $udec!(5626.7))]
            #[case($dec!(-56267E-0), $dec!(56267), $udec!(56267))]
            #[case($D::INFINITY, $D::INFINITY, $UD::INFINITY)]
            #[case($D::NEG_INFINITY, $D::INFINITY, $UD::INFINITY)]
            fn test_abs(#[case] d: $D, #[case] abs: $D, #[case] uabs: $UD) {
                assert_eq!(d.abs(), abs);
                assert_eq!(d.unsigned_abs(), uabs);
            }

            #[rstest(::trace)]
            fn test_abs_nan() {
                let ctx = Context::default().without_traps();
                let d = $D::NAN.with_ctx(ctx).abs();
                assert!(d.is_nan());
                assert!(d.is_op_invalid());
            }
        }
    };
}

pub(crate) use test_impl;
