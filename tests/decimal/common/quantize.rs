macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>], [< u $bits >], [<U $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>], [< u $bits >], [<U $bits>]); }
    };
    (UNSIGNED: $bits: tt, $dec: ident, $D: ident, $uint: ident, $U: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{*, decimal::*};

            super::test_impl!(COMMON:: $bits, $dec, $D, $uint, $U, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, $uint, $U, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident, $uint: ident, $U: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{*, decimal::*};

            super::test_impl!(COMMON:: $bits, $dec, $D, $uint, $U, THIS);
            super::test_impl!(SIGNED:: $bits, $dec, $D, $uint, $U, THIS);
        }
    };
    (COMMON:: 512, $dec: ident, $D: ident, $uint: ident, $U: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D, $uint, $U);
    };
    (UNSIGNED:: 512, $dec: ident, $D: ident, $uint: ident, $U: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D, $uint, $U);
    };
    (SIGNED:: 512, $dec: ident, $D: ident, $uint: ident, $U: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D, $uint, $U);
    };

    (COMMON:: 256, $dec: ident, $D: ident, $uint: ident, $U: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D, $uint, $U);
    };
    (COMMON:: 256, $dec: ident, $D: ident, $uint: ident, $U: ident) => {
        super::test_impl!(COMMON:: 128, $dec, $D, $uint, $U);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident, $uint: ident, $U: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D, $uint, $U);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident, $uint: ident, $U: ident) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D, $uint, $U);
    };
    (SIGNED:: 256, $dec: ident, $D: ident, $uint: ident, $U: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D, $uint, $U);
    };
    (SIGNED:: 256, $dec: ident, $D: ident, $uint: ident, $U: ident) => {
        super::test_impl!(SIGNED:: 128, $dec, $D, $uint, $U);
    };

    (COMMON:: 128, $dec: ident, $D: ident, $uint: ident, $U: ident, THIS) => {
        super::test_impl!(COMMON:: 128, $dec, $D, $uint, $U);

        #[rstest(::trace)]
        #[case($dec!(0.34028), $dec!(1e-32765))]
        fn test_quantize_nan_128(#[case] d: $D, #[case] other: $D) {
            let ctx = Context::default().without_traps();
            let d = d.with_ctx(ctx).quantize(other);

            assert!(d.is_nan());
            assert_eq!(d.op_signals(), signals![!INV]);
        }
    };
    (COMMON:: 128, $dec: ident, $D: ident, $uint: ident, $U: ident) => {
        #[rstest(::trace)]
        #[case($dec!(0), 0, $dec!(0), Signal::empty())]
        #[case($dec!(0), 1, $dec!(0.0), Signal::empty())]
        #[case($dec!(2.17), 3, $dec!(2.170), Signal::empty())]
        #[case($dec!(2.17), 2, $dec!(2.17), Signal::empty())]
        #[case($dec!(2.17), 1, $dec!(2.2), signals![!ROUND, !INEXACT])]
        #[case($dec!(2.17), 0, $dec!(2), signals![!ROUND, !INEXACT])]
        #[case($dec!(2.17), -1, $dec!(0E+1), signals![!ROUND, !INEXACT])]
        fn test_rescale(#[case] d: $D, #[case] new_scale: i16, #[case] expected: $D, #[case] signals: Signal) {
            let d = d.rescale(new_scale);

            assert_eq!(d, expected);
            assert_eq!(d.fractional_digits_count(), expected.fractional_digits_count());
            assert_eq!(d.op_signals(), signals);
        }

        #[rstest(::trace)]
        #[case($dec!(0), $dec!(1e0), $dec!(0), Signal::empty())]
        #[case($dec!(1), $dec!(1e0), $dec!(1), Signal::empty())]
        #[case($dec!(0.1), $dec!(1e+2), $dec!(0E+2), signals![!ROUND, !INEXACT])]
        #[case($dec!(0.1), $dec!(1e+1), $dec!(0E+1), signals![!ROUND, !INEXACT])]
        #[case($dec!(0.1), $dec!(1e0), $dec!(0), signals![!ROUND, !INEXACT])]
        #[case($dec!(0.1), $dec!(1e-1), $dec!(0.1), Signal::empty())]
        #[case($dec!(0.1), $dec!(1e-2), $dec!(0.10), Signal::empty())]
        #[case($dec!(0.1), $dec!(1e-3), $dec!(0.100), Signal::empty())]
        #[case($dec!(0.9), $dec!(1e+2), $dec!(0E+2), signals![!ROUND, !INEXACT])]
        #[case($dec!(0.9), $dec!(1e+1), $dec!(0E+1), signals![!ROUND, !INEXACT])]
        #[case($dec!(0.9), $dec!(1e+0), $dec!(1), signals![!ROUND, !INEXACT])]
        #[case($dec!(0.9), $dec!(1e-1), $dec!(0.9), Signal::empty())]
        #[case($dec!(0.9), $dec!(1e-2), $dec!(0.90), Signal::empty())]
        #[case($dec!(0.9), $dec!(1e-3), $dec!(0.900), Signal::empty())]
        // ------------------------------------------------
        #[case($dec!(2.17), $dec!(1e-3), $dec!(2.170), Signal::empty())]
        #[case($dec!(2.17), $dec!(1e-2), $dec!(2.17), Signal::empty())]
        #[case($dec!(2.17), $dec!(1e-1), $dec!(2.2), signals![!ROUND, !INEXACT])]
        #[case($dec!(2.17), $dec!(1e0), $dec!(2), signals![!ROUND, !INEXACT])]
        #[case($dec!(2.17), $dec!(1e+1), $dec!(0E+1), signals![!ROUND, !INEXACT])]
        #[case($D::INFINITY, $D::INFINITY, $D::INFINITY, Signal::empty())]
        #[case($dec!(217), $dec!(1e-1), $dec!(217.0), Signal::empty())]
        #[case($dec!(217), $dec!(1e0), $dec!(217), Signal::empty())]
        #[case($dec!(217), $dec!(1e+1), $dec!(2.2E+2), signals![!ROUND, !INEXACT])]
        #[case($dec!(217), $dec!(1e+2), $dec!(2E+2), signals![!ROUND, !INEXACT])]
        // ------------------------------------------------
        #[case($dec!(12), $dec!(1e+4), $dec!(0E+4), signals![!ROUND, !INEXACT])]
        #[case($dec!(12), $dec!(1e+3), $dec!(0E+3), signals![!ROUND, !INEXACT])]
        #[case($dec!(12), $dec!(1e+2), $dec!(0E+2), signals![!ROUND, !INEXACT])]
        #[case($dec!(12), $dec!(1e+1), $dec!(1E+1), signals![!ROUND, !INEXACT])]
        #[case($dec!(1.2345), $dec!(1e-2), $dec!(1.23), signals![!ROUND, !INEXACT])]
        #[case($dec!(1.2335), $dec!(1e-2), $dec!(1.23), signals![!ROUND, !INEXACT])]
        #[case($dec!(1.2345), $dec!(1e-6), $dec!(1.234500), Signal::empty())]
        #[case($dec!(9.9999), $dec!(1e-2), $dec!(10.00), signals![!ROUND, !INEXACT])]
        #[case($dec!(0.0001), $dec!(1e-2), $dec!(0.00), signals![!ROUND, !INEXACT])]
        #[case($dec!(0.001), $dec!(1e-2), $dec!(0.00), signals![!ROUND, !INEXACT])]
        #[case($dec!(0.009), $dec!(1e-2), $dec!(0.01), signals![!ROUND, !INEXACT])]
        #[case($dec!(92), $dec!(1e+2), $dec!(1E+2), signals![!ROUND, !INEXACT])]
        // ------------------------------------------------
        #[case($dec!(1.2300), $dec!(1.00), $dec!(1.23), signals![!ROUND])]
        #[case($dec!(1.2301), $dec!(1.00), $dec!(1.23), signals![!ROUND, !INEXACT])]
        #[case($dec!(1.2310), $dec!(1.00), $dec!(1.23), signals![!ROUND, !INEXACT])]
        #[case($dec!(1.2350), $dec!(1.00), $dec!(1.24), signals![!ROUND, !INEXACT])]
        #[case($dec!(1.2351), $dec!(1.00), $dec!(1.24), signals![!ROUND, !INEXACT])]
        #[case($dec!(1.2450), $dec!(1.00), $dec!(1.25), signals![!ROUND, !INEXACT])]
        #[case($dec!(1.2451), $dec!(1.00), $dec!(1.25), signals![!ROUND, !INEXACT])]
        #[case($dec!(1.2370), $dec!(1.00), $dec!(1.24), signals![!ROUND, !INEXACT])]
        #[case($dec!(1.2399), $dec!(1.00), $dec!(1.24), signals![!ROUND, !INEXACT])]
        // ------------------------------------------------
        fn test_quantize(#[case] d: $D, #[case] other: $D, #[case] expected: $D, #[case] signals: Signal) {
            let d = d.quantize(other);

            assert_eq!(d, expected);
            assert_eq!(d.fractional_digits_count(), expected.fractional_digits_count());
            assert_eq!(d.op_signals(), signals);
        }

        #[rstest(::trace)]
        #[case($dec!(2), $D::INFINITY)]
        #[case($D::INFINITY, $dec!(2))]
        #[case($D::NAN, $D::INFINITY)]
        #[case($D::INFINITY, $D::NAN)]
        #[case($D::NAN, $D::NAN)]
        fn test_quantize_nan(#[case] d: $D, #[case] other: $D) {
            let ctx = Context::default().without_traps();
            let d = d.with_ctx(ctx).quantize(other);

            assert!(d.is_nan());
            assert_eq!(d.op_signals(), signals![!INV]);
        }
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, $uint: ident, $U: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D, $uint, $U);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, $uint: ident, $U: ident) => {

    };
    (SIGNED:: 128, $dec: ident, $D: ident, $uint: ident, $U: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D, $uint, $U);

        #[rstest(::trace)]
        #[case($dec!(-0.34028), $dec!(1e-32765))]
        fn test_quantize_nan_signed_128(#[case] d: $D, #[case] other: $D) {
            let ctx = Context::default().without_traps();
            let d = d.with_ctx(ctx).quantize(other);

            assert!(d.is_nan());
            assert_eq!(d.op_signals(), signals![!INV]);
        }
    };
    (SIGNED:: 128, $dec: ident, $D: ident, $uint: ident, $U: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-0), $dec!(1e0), $dec!(-0), Signal::empty())]
        #[case($dec!(-1), $dec!(1e0), $dec!(-1), Signal::empty())]
        #[case($dec!(-0.1), $dec!(1e+2), $dec!(-0E+2), signals![!ROUND, !INEXACT])]
        #[case($dec!(-0.1), $dec!(1e+1), $dec!(-0E+1), signals![!ROUND, !INEXACT])]
        #[case($dec!(-0.1), $dec!(1e0), $dec!(-0), signals![!ROUND, !INEXACT])]
        #[case($dec!(-0.1), $dec!(1e-1), $dec!(-0.1), Signal::empty())]
        #[case($dec!(-0.1), $dec!(1e-2), $dec!(-0.10), Signal::empty())]
        #[case($dec!(-0.1), $dec!(1e-3), $dec!(-0.100), Signal::empty())]
        #[case($dec!(-0.9), $dec!(1e+2), $dec!(-0E+2), signals![!ROUND, !INEXACT])]
        #[case($dec!(-0.9), $dec!(1e+1), $dec!(-0E+1), signals![!ROUND, !INEXACT])]
        #[case($dec!(-0.9), $dec!(1e+0), $dec!(-1), signals![!ROUND, !INEXACT])]
        #[case($dec!(-0.9), $dec!(1e-1), $dec!(-0.9), Signal::empty())]
        #[case($dec!(-0.9), $dec!(1e-2), $dec!(-0.90), Signal::empty())]
        #[case($dec!(-0.9), $dec!(1e-3), $dec!(-0.900), Signal::empty())]
        #[case($dec!(-0.5), $dec!(1e+2), $dec!(-0E+2), signals![!ROUND, !INEXACT])]
        #[case($dec!(-0.5), $dec!(1e+1), $dec!(-0E+1), signals![!ROUND, !INEXACT])]
        #[case($dec!(-0.5), $dec!(1e+0), $dec!(-1), signals![!ROUND, !INEXACT])]
        #[case($dec!(-0.5), $dec!(1e-1), $dec!(-0.5), Signal::empty())]
        #[case($dec!(-0.5), $dec!(1e-2), $dec!(-0.50), Signal::empty())]
        #[case($dec!(-0.5), $dec!(1e-3), $dec!(-0.500), Signal::empty())]
        // ------------------------------------------------
        #[case($D::NEG_INFINITY, $D::INFINITY, $D::NEG_INFINITY, Signal::empty())]
        #[case($D::INFINITY, $D::NEG_INFINITY, $D::INFINITY, Signal::empty())]
        #[case($dec!(-0.1), $dec!(1), $dec!(-0), signals![!ROUND, !INEXACT])]
        #[case($dec!(-0), $dec!(1e+5), $dec!(-0E+5), Signal::empty())]
        // ------------------------------------------------
        fn test_quantize_signed(#[case] d: $D, #[case] other: $D, #[case] expected: $D, #[case] signals: Signal) {
            let d = d.quantize(other);

            assert_eq!(d, expected);
            assert_eq!(d.fractional_digits_count(), expected.fractional_digits_count());
            assert_eq!(d.op_signals(), signals);
        }

        #[rstest(::trace)]
        #[case($dec!(-2), $D::NEG_INFINITY)]
        #[case($D::NEG_INFINITY, $dec!(-2))]
        #[case($D::NAN, $D::NEG_INFINITY)]
        #[case($D::NEG_INFINITY, $D::NAN)]
        fn test_quantize_nan_signed(#[case] d: $D, #[case] other: $D) {
            let ctx = Context::default().without_traps();
            let d = d.with_ctx(ctx).quantize(other);

            assert!(d.is_nan());
            assert_eq!(d.op_signals(), signals![!INV]);
        }
    };
}

pub(crate) use test_impl;
