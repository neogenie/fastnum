macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>], [< dec $bits >], [<D $bits>]); }
    };
    (UNSIGNED: $bits: tt, $dec: ident, $D: ident, $sdec: ident, $SD: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{*, decimal::*};

            super::test_impl!(COMMON:: $bits, $dec, $D, $sdec, $SD, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, $sdec, $SD, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{*, decimal::*};

            super::test_impl!(COMMON:: $bits, $dec, $D, $dec, $D, THIS);
            super::test_impl!(SIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (COMMON:: 512, $dec: ident, $D: ident, $sdec: ident, $SD: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D, $sdec, $SD);
    };
    (UNSIGNED:: 512, $dec: ident, $D: ident, $sdec: ident, $SD: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D, $sdec, $SD);
    };
    (SIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };


    (COMMON:: 256, $dec: ident, $D: ident, $sdec: ident, $SD: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D, $sdec, $SD);
    };
    (COMMON:: 256, $dec: ident, $D: ident, $sdec: ident, $SD: ident) => {
        super::test_impl!(COMMON:: 128, $dec, $D, $sdec, $SD);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident, $sdec: ident, $SD: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D, $sdec, $SD);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident, $sdec: ident, $SD: ident) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D, $sdec, $SD);
    };
    (SIGNED:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };

    (COMMON:: 128, $dec: ident, $D: ident, $sdec: ident, $SD: ident, THIS) => {
        super::test_impl!(COMMON:: 128, $dec, $D, $sdec, $SD);
    };
    (COMMON:: 128, $dec: ident, $D: ident, $sdec: ident, $SD: ident) => {
        #[rstest(::trace)]
        #[case($dec!(1), $SD::ZERO, signals![])]
        #[case($dec!(2), $SD::LN_2, signals![!ROUND, !INEXACT])]
        fn test_ln(#[case] d: $D, #[case] expected: $SD, #[case] signals: Signal) {
            let res = d.ln();

            assert_eq!(res, expected);
            assert_eq!(res.op_signals(), signals);
        }

        #[rstest(::trace)]
        #[case($D::ZERO)]
        fn test_ln_zero(#[case] d: $D) {
            let ctx = Context::default().without_traps();
            let res = d.with_ctx(ctx).ln();

            assert!(res.is_negative());
            assert!(res.is_infinite());
        }

        #[rstest(::trace)]
        #[case($D::NAN)]
        fn test_ln_nan(#[case] d: $D) {
            let ctx = Context::default().without_traps();
            let res = d.with_ctx(ctx).ln();

            assert!(res.is_nan());
            assert!(res.is_op_invalid());
        }
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, $sdec: ident, $SD: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D, $sdec, $SD);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, $sdec: ident, $SD: ident) => {
    };
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-1))]
        fn test_ln_neg(#[case] d: $D) {
            let ctx = Context::default().without_traps();
            let res = d.with_ctx(ctx).ln();

            assert!(res.is_nan());
            assert!(res.is_op_invalid());
        }
    };
}

pub(crate) use test_impl;
