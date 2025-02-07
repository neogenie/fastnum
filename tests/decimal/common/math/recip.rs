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
            use fastnum::*;

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::*;

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
        #[case($dec!(3), $dec!(0.333333333333333333333333333333333333333))]
        fn test_recip_128(#[case] d: $D, #[case] expected: $D) {
            let res = d.recip();

            assert_eq!(res, expected);
            assert_eq!(res.op_signals(), signals![!CP, !INEXACT, !ROUND]);
        }

    };
    (COMMON:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(2), $dec!(0.5))]
        #[case($dec!(2.0), $dec!(0.5))]
        #[case($dec!(2.00), $dec!(0.50))]
        #[case($dec!(2.000), $dec!(0.500))]
        #[case($dec!(2.0000), $dec!(0.5000))]
        #[case($dec!(2.00000), $dec!(0.50000))]
        #[case($dec!(2.000000), $dec!(0.500000))]
        #[case($dec!(2.0000000), $dec!(0.5000000))]
        #[case($dec!(4), $dec!(0.25))]
        #[case($dec!(8), $dec!(0.125))]
        #[case($dec!(16), $dec!(0.0625))]
        #[case($dec!(25), $dec!(0.04))]
        #[case($dec!(32), $dec!(0.03125))]
        #[case($dec!(64), $dec!(0.015625))]
        #[case($dec!(1024), $dec!(0.0009765625))]
        #[case($dec!(2e-2), $dec!(50.00))]
        #[case($dec!(0.2), $dec!(5.0))]
        #[case($dec!(0.02), $dec!(50.00))]
        #[case($dec!(0.020), $dec!(50.000))]
        fn test_recip(#[case] d: $D, #[case] expected: $D) {
            let res = d.recip();

            assert_eq!(res, expected);
            assert_eq!(res.op_signals(), signals![!CP, !INEXACT, !ROUND]);
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
        #[case($dec!(-2), $dec!(-0.5))]
        #[case($dec!(-4), $dec!(-0.25))]
        #[case($dec!(-8), $dec!(-0.125))]
        #[case($dec!(-16), $dec!(-0.0625))]
        #[case($dec!(-32), $dec!(-0.03125))]
        #[case($dec!(-64), $dec!(-0.015625))]
        fn test_recip_signed(#[case] d: $D, #[case] expected: $D) {
            let res = d.recip();

            assert_eq!(res, expected);
            assert_eq!(res.op_signals(), signals![!CP, !INEXACT, !ROUND]);
        }
    };
}

pub(crate) use test_impl;
