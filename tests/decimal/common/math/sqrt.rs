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

        #[rstest(::trace)]
        #[case($dec!(3), $dec!(1.73205080756887729352744634150587236694))]
        fn test_sqrt_128(#[case] d: $D, #[case] expected: $D) {
            let res = d.sqrt();

            assert_eq!(res, expected);
            assert_eq!(res.op_signals(), signals![!CP, !ROUND, !INEXACT]);
        }

    };
    (COMMON:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(0), $dec!(0), signals![])]
        #[case($dec!(1), $dec!(1), signals![])]
        #[case($dec!(2), $D::SQRT_2, signals![!ROUND, !INEXACT])]
        #[case($dec!(4), $dec!(2), signals![!CP, !ROUND, !INEXACT])]
        fn test_sqrt(#[case] d: $D, #[case] expected: $D, #[case] signals: Signals) {
            let res = d.sqrt();

            assert_eq!(res, expected);
            assert_eq!(res.op_signals(), signals);
        }

        #[rstest(::trace)]
        #[case($D::NAN)]
        fn test_sqrt_nan(#[case] d: $D) {
            let ctx = Context::default().without_traps();
            let res = d.with_ctx(ctx).ln();

            assert!(res.is_nan());
            assert!(res.is_op_invalid());
        }
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {
    };
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-1))]
        fn test_sqrt_neg(#[case] d: $D) {
            let ctx = Context::default().without_traps();
            let res = d.with_ctx(ctx).sqrt();

            assert!(res.is_nan());
            assert!(res.is_op_invalid());
        }
    };
}

pub(crate) use test_impl;
