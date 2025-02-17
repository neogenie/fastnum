macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! {
            test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>]);
        }
    };
    (UD, $bits: literal) => {
        paste::paste! {
            test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>], [< dec $bits >], [<D $bits>]);
        }
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

        #[rstest(::trace)]
        #[case($dec!(10), 20, $dec!(1e20), signals![])]
        #[case($dec!(10), 77, $dec!(1E+77), signals![])]
        #[case($dec!(10), 99, $dec!(1E+99), signals![!ROUND])]
        #[case($dec!(10), -99, $dec!(1E-99), signals![!ROUND])]
        #[case($dec!(10), -77, $dec!(1E-77), signals![])]
        #[case($dec!(10), -22, $dec!(1E-22), signals![])]
        #[case($dec!(10), 22, $dec!(1E+22), signals![])]
        fn test_powi_256(#[case] d: $D, #[case] n: i32, #[case] expected: $D, #[case] signals: Signals) {
            let d = d.powi(n);

            assert_eq!(d, expected);
            assert_eq!(d.op_signals(), signals);
        }
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

        #[rstest(::trace)]
        #[case($dec!(3), -1, $dec!(0.333333333333333333333333333333333333333), signals![!ROUND, !INEXACT])]
        #[case($dec!(2), -64, $dec!(5.4210108624275221700372640043497085571E-20), signals![!ROUND, !INEXACT])]
        #[case($dec!(10), 20, $dec!(1e20), signals![])]
        #[case($dec!(10), 22, $dec!(1E+22), signals![])]
        #[case($dec!(10), 77, $dec!(1E+77), signals![!ROUND])]
        #[case($dec!(10), 99, $dec!(1E+99), signals![!ROUND])]
        #[case($dec!(10), -77, $dec!(1E-77), signals![!ROUND])]
        #[case($dec!(10), -22, $dec!(1E-22), signals![])]
        fn test_powi_128(#[case] d: $D, #[case] n: i32, #[case] expected: $D, #[case] signals: Signals) {
            let d = d.powi(n);

            assert_eq!(d, expected);
            assert_eq!(d.op_signals(), signals);
        }
    };
    (COMMON:: 128, $dec: ident, $D: ident, $sdec: ident, $SD: ident) => {

        #[rstest(::trace)]
        #[case($dec!(0), 1, $dec!(0))]
        #[case($dec!(0), 2, $dec!(0))]
        #[case($dec!(1), 0, $dec!(1))]
        #[case($dec!(1), 1, $dec!(1))]
        #[case($dec!(1), 2, $dec!(1))]
        // --------------------------------------
        #[case($dec!(2), 0, $dec!(1))]
        #[case($dec!(2), 1, $dec!(2))]
        #[case($dec!(2), 2, $dec!(4))]
        #[case($dec!(2), 3, $dec!(8))]
        #[case($dec!(2), 4, $dec!(16))]
        #[case($dec!(2), 5, $dec!(32))]
        #[case($dec!(2), 6, $dec!(64))]
        #[case($dec!(2), 7, $dec!(128))]
        #[case($dec!(2), 8, $dec!(256))]
        #[case($dec!(2), 9, $dec!(512))]
        #[case($dec!(2), 10, $dec!(1024))]
        #[case($dec!(2), 11, $dec!(2048))]
        #[case($dec!(2), 12, $dec!(4096))]
        #[case($dec!(2), 15, $dec!(32768))]
        #[case($dec!(2), 16, $dec!(65536))]
        #[case($dec!(2), 31, $dec!(2147483648))]
        #[case($dec!(2), 32, $dec!(4294967296))]
        // --------------------------------------
        #[case($dec!(2), 3, $dec!(8))]
        #[case($dec!(9), 2, $dec!(81))]
        #[case($dec!(1), -2, $dec!(1))]
        #[case($dec!(4), -2, $dec!(0.0625))]
        #[case($dec!(2), -3, $dec!(0.125))]
        // --------------------------------------
        #[case($dec!(3), 2, $dec!(9))]
        #[case($dec!(4), 2, $dec!(16))]
        #[case($dec!(5), 2, $dec!(25))]
        #[case($dec!(6), 2, $dec!(36))]
        #[case($dec!(7), 2, $dec!(49))]
        #[case($dec!(8), 2, $dec!(64))]
        #[case($dec!(9), 2, $dec!(81))]
        #[case($dec!(10), 2, $dec!(100))]
        #[case($dec!(11), 2, $dec!(121))]
        #[case($dec!(12), 2, $dec!(144))]
        // --------------------------------------
        #[case($dec!(3), 3, $dec!(27))]
        #[case($dec!(4), 3, $dec!(64))]
        #[case($dec!(5), 3, $dec!(125))]
        #[case($dec!(6), 3, $dec!(216))]
        #[case($dec!(7), 3, $dec!(343))]
        // --------------------------------------
        #[case($dec!(10), 0, $dec!(1))]
        #[case($dec!(10), 1, $dec!(10))]
        #[case($dec!(10), 2, $dec!(100))]
        #[case($dec!(10), 3, $dec!(1000))]
        #[case($dec!(10), 4, $dec!(10000))]
        #[case($dec!(10), 5, $dec!(100000))]
        #[case($dec!(10), 6, $dec!(1000000))]
        #[case($dec!(10), 7, $dec!(10000000))]
        #[case($dec!(10), 8, $dec!(100000000))]
        #[case($dec!(10), 9, $dec!(1000000000))]
        // --------------------------------------
        #[case($dec!(0.3), 0, $dec!(1))]
        #[case($dec!(0.3), 1, $dec!(0.3))]
        #[case($dec!(0.3), 1, $dec!(0.3))]
        #[case($dec!(0.3), 2, $dec!(0.09))]
        #[case($dec!(0.3), 2, $dec!(0.09))]
        #[case($dec!(6.0), 1, $dec!(6.0))]
        #[case($dec!(6.0), 2, $dec!(36.00))]
        // --------------------------------------
        #[case($dec!(0.1), 0, $dec!(1))]
        #[case($dec!(0.1), 1, $dec!(0.1))]
        #[case($dec!(0.1), 2, $dec!(0.01))]
        #[case($dec!(0.1), 3, $dec!(0.001))]
        #[case($dec!(0.1), 4, $dec!(0.0001))]
        #[case($dec!(0.1), 5, $dec!(0.00001))]
        #[case($dec!(0.1), 6, $dec!(0.000001))]
        #[case($dec!(0.1), 7, $dec!(0.0000001))]
        #[case($dec!(0.1), 8, $dec!(0.00000001))]
        #[case($dec!(0.1), 9, $dec!(0.000000001))]
        // --------------------------------------
        #[case($dec!(101), 2, $dec!(10201))]
        #[case($dec!(101), 3, $dec!(1030301))]
        #[case($dec!(101), 4, $dec!(104060401))]
        // --------------------------------------
        #[case($dec!(1), -1, $dec!(1))]
        #[case($dec!(2), -1, $dec!(0.5))]
        #[case($dec!(2), -2, $dec!(0.25))]
        #[case($dec!(2), -4, $dec!(0.0625))]
        #[case($dec!(2), -8, $dec!(0.00390625))]
        #[case($dec!(2), -16, $dec!(0.0000152587890625))]
        #[case($dec!(2), -32, $dec!(2.3283064365386962890625e-10))]
        // --------------------------------------
        #[case($dec!(10), -8, $dec!(0.00000001))]
        #[case($dec!(10), -7, $dec!(0.0000001))]
        #[case($dec!(10), -6, $dec!(0.000001))]
        #[case($dec!(10), -5, $dec!(0.00001))]
        #[case($dec!(10), -4, $dec!(0.0001))]
        #[case($dec!(10), -3, $dec!(0.001))]
        #[case($dec!(10), -2, $dec!(0.01))]
        #[case($dec!(10), -1, $dec!(0.1))]
        // --------------------------------------
        #[case($dec!(0.5), 0, $dec!(1))]
        #[case($dec!(0.5), 1, $dec!(0.5))]
        #[case($dec!(0.5), 2, $dec!(0.25))]
        #[case($dec!(0.5), 3, $dec!(0.125))]
        #[case($dec!(0.5), 4, $dec!(0.0625))]
        #[case($dec!(0.5), 5, $dec!(0.03125))]
        #[case($dec!(0.5), 6, $dec!(0.015625))]
        #[case($dec!(0.5), 7, $dec!(0.0078125))]
        #[case($dec!(0.5), 8, $dec!(0.00390625))]
        #[case($dec!(0.5), 9, $dec!(0.001953125))]
        #[case($dec!(0.5), 10, $dec!(0.0009765625))]
        // --------------------------------------
        #[case($dec!(1), 100000000, $dec!(1))]
        #[case($dec!(1), 999999998, $dec!(1))]
        #[case($dec!(1), 999999999, $dec!(1))]
        // --------------------------------------
        #[case($dec!(0), -1, $D::INFINITY)]
        #[case($dec!(0), -2, $D::INFINITY)]
        #[case($dec!(0), -3, $D::INFINITY)]
        #[case($D::INFINITY, -1, $dec!(0))]
        #[case($D::INFINITY, 0, $dec!(1))]
        #[case($D::INFINITY, 1, $D::INFINITY)]
        fn test_powi(#[case] d: $D, #[case] n: i32, #[case] expected: $D) {
            let d = d.powi(n);

            assert_eq!(d, expected);
            assert_eq!(d.fractional_digits_count(), expected.fractional_digits_count());
            assert!(d.is_op_ok());
        }

        #[rstest(::trace)]
        #[case($dec!(0), $sdec!(1), $dec!(0))]
        #[case($dec!(0), $sdec!(2), $dec!(0))]
        #[case($dec!(1), $sdec!(0), $dec!(1))]
        #[case($dec!(1), $sdec!(1), $dec!(1))]
        #[case($dec!(1), $sdec!(2), $dec!(1))]
        // --------------------------------------
        #[case($dec!(2), $sdec!(0), $dec!(1))]
        #[case($dec!(2), $sdec!(1), $dec!(2))]
        #[case($dec!(2), $sdec!(2), $dec!(4))]
        #[case($dec!(2), $sdec!(3), $dec!(8))]
        #[case($dec!(2), $sdec!(4), $dec!(16))]
        #[case($dec!(2), $sdec!(5), $dec!(32))]
        #[case($dec!(2), $sdec!(6), $dec!(64))]
        #[case($dec!(2), $sdec!(7), $dec!(128))]
        #[case($dec!(2), $sdec!(8), $dec!(256))]
        #[case($dec!(2), $sdec!(9), $dec!(512))]
        #[case($dec!(2), $sdec!(10), $dec!(1024))]
        #[case($dec!(2), $sdec!(11), $dec!(2048))]
        #[case($dec!(2), $sdec!(12), $dec!(4096))]
        #[case($dec!(2), $sdec!(15), $dec!(32768))]
        #[case($dec!(2), $sdec!(16), $dec!(65536))]
        #[case($dec!(2), $sdec!(31), $dec!(2147483648))]
        #[case($dec!(2), $sdec!(32), $dec!(4294967296))]
        // --------------------------------------
        #[case($dec!(2), $sdec!(3), $dec!(8))]
        #[case($dec!(9), $sdec!(2), $dec!(81))]
        #[case($dec!(1), $sdec!(-2), $dec!(1))]
        #[case($dec!(4), $sdec!(-2), $dec!(0.0625))]
        #[case($dec!(2), $sdec!(-3), $dec!(0.125))]
        // --------------------------------------
        #[case($dec!(3),  $sdec!(2), $dec!(9))]
        #[case($dec!(4),  $sdec!(2), $dec!(16))]
        #[case($dec!(5),  $sdec!(2), $dec!(25))]
        #[case($dec!(6),  $sdec!(2), $dec!(36))]
        #[case($dec!(7),  $sdec!(2), $dec!(49))]
        #[case($dec!(8),  $sdec!(2), $dec!(64))]
        #[case($dec!(9),  $sdec!(2), $dec!(81))]
        #[case($dec!(10), $sdec!(2), $dec!(100))]
        #[case($dec!(11), $sdec!(2), $dec!(121))]
        #[case($dec!(12), $sdec!(2), $dec!(144))]
        // --------------------------------------
        #[case($dec!(3), $sdec!(3), $dec!(27))]
        #[case($dec!(4), $sdec!(3), $dec!(64))]
        #[case($dec!(5), $sdec!(3), $dec!(125))]
        #[case($dec!(6), $sdec!(3), $dec!(216))]
        #[case($dec!(7), $sdec!(3), $dec!(343))]
        // --------------------------------------
        #[case($dec!(10), $sdec!(0), $dec!(1))]
        #[case($dec!(10), $sdec!(1), $dec!(10))]
        #[case($dec!(10), $sdec!(2), $dec!(100))]
        #[case($dec!(10), $sdec!(3), $dec!(1000))]
        #[case($dec!(10), $sdec!(4), $dec!(10000))]
        #[case($dec!(10), $sdec!(5), $dec!(100000))]
        #[case($dec!(10), $sdec!(6), $dec!(1000000))]
        #[case($dec!(10), $sdec!(7), $dec!(10000000))]
        #[case($dec!(10), $sdec!(8), $dec!(100000000))]
        #[case($dec!(10), $sdec!(9), $dec!(1000000000))]
        // --------------------------------------
        #[case($dec!(0.3), $sdec!(0), $dec!(1))]
        #[case($dec!(0.3), $sdec!(1), $dec!(0.3))]
        #[case($dec!(0.3), $sdec!(1), $dec!(0.3))]
        #[case($dec!(0.3), $sdec!(2), $dec!(0.09))]
        #[case($dec!(0.3), $sdec!(2), $dec!(0.09))]
        #[case($dec!(6.0), $sdec!(1), $dec!(6.0))]
        #[case($dec!(6.0), $sdec!(2), $dec!(36.00))]
        // --------------------------------------
        #[case($dec!(0.1), $sdec!(0), $dec!(1))]
        #[case($dec!(0.1), $sdec!(1), $dec!(0.1))]
        #[case($dec!(0.1), $sdec!(2), $dec!(0.01))]
        #[case($dec!(0.1), $sdec!(3), $dec!(0.001))]
        #[case($dec!(0.1), $sdec!(4), $dec!(0.0001))]
        #[case($dec!(0.1), $sdec!(5), $dec!(0.00001))]
        #[case($dec!(0.1), $sdec!(6), $dec!(0.000001))]
        #[case($dec!(0.1), $sdec!(7), $dec!(0.0000001))]
        #[case($dec!(0.1), $sdec!(8), $dec!(0.00000001))]
        #[case($dec!(0.1), $sdec!(9), $dec!(0.000000001))]
        // --------------------------------------
        #[case($dec!(101), $sdec!(2), $dec!(10201))]
        #[case($dec!(101), $sdec!(3), $dec!(1030301))]
        #[case($dec!(101), $sdec!(4), $dec!(104060401))]
        // --------------------------------------
        #[case($dec!(1), $sdec!(-1), $dec!(1))]
        #[case($dec!(2), $sdec!(-1), $dec!(0.5))]
        #[case($dec!(2), $sdec!(-2), $dec!(0.25))]
        #[case($dec!(2), $sdec!(-4), $dec!(0.0625))]
        #[case($dec!(2), $sdec!(-8), $dec!(0.00390625))]
        #[case($dec!(2), $sdec!(-16), $dec!(0.0000152587890625))]
        #[case($dec!(2), $sdec!(-32), $dec!(2.3283064365386962890625e-10))]
        // --------------------------------------
        #[case($dec!(10), $sdec!(-8), $dec!(0.00000001))]
        #[case($dec!(10), $sdec!(-7), $dec!(0.0000001))]
        #[case($dec!(10), $sdec!(-6), $dec!(0.000001))]
        #[case($dec!(10), $sdec!(-5), $dec!(0.00001))]
        #[case($dec!(10), $sdec!(-4), $dec!(0.0001))]
        #[case($dec!(10), $sdec!(-3), $dec!(0.001))]
        #[case($dec!(10), $sdec!(-2), $dec!(0.01))]
        #[case($dec!(10), $sdec!(-1), $dec!(0.1))]
        // --------------------------------------
        #[case($dec!(0.5), $sdec!(0), $dec!(1))]
        #[case($dec!(0.5), $sdec!(1), $dec!(0.5))]
        #[case($dec!(0.5), $sdec!(2), $dec!(0.25))]
        #[case($dec!(0.5), $sdec!(3), $dec!(0.125))]
        #[case($dec!(0.5), $sdec!(4), $dec!(0.0625))]
        #[case($dec!(0.5), $sdec!(5), $dec!(0.03125))]
        #[case($dec!(0.5), $sdec!(6), $dec!(0.015625))]
        #[case($dec!(0.5), $sdec!(7), $dec!(0.0078125))]
        #[case($dec!(0.5), $sdec!(8), $dec!(0.00390625))]
        #[case($dec!(0.5), $sdec!(9), $dec!(0.001953125))]
        #[case($dec!(0.5), $sdec!(10), $dec!(0.0009765625))]
        // --------------------------------------
        #[case($dec!(1), $sdec!(100000000), $dec!(1))]
        #[case($dec!(1), $sdec!(999999998), $dec!(1))]
        #[case($dec!(1), $sdec!(999999999), $dec!(1))]
        // --------------------------------------
        #[case($dec!(0), $sdec!(-1), $D::INFINITY)]
        #[case($dec!(0), $sdec!(-2), $D::INFINITY)]
        #[case($dec!(0), $sdec!(-3), $D::INFINITY)]
        #[case($D::INFINITY, $sdec!(-1), $dec!(0))]
        #[case($D::INFINITY, $sdec!(0), $dec!(1))]
        #[case($D::INFINITY, $sdec!(1), $D::INFINITY)]
        fn test_pow(#[case] d: $D, #[case] n: $SD, #[case] expected: $D) {
            let d = d.pow(n);
            assert_eq!(d, expected);
        }

        #[rstest(::trace)]
        #[case($D::NAN, -9)]
        #[case($D::NAN, 0)]
        #[case($D::NAN, 10)]
        #[case($dec!(0), 0)]
        fn test_powi_nan(#[case] d: $D, #[case] n: i32) {
            let ctx = Context::default().without_traps();
            let d = d.with_ctx(ctx).powi(n);
            assert!(d.is_nan());
            assert!(d.is_op_invalid());
        }

        #[rstest(::trace)]
        #[case($D::NAN, $sdec!(9.99))]
        #[case($D::NAN, $sdec!(0.1))]
        #[case($D::NAN, $sdec!(10.1))]
        #[case($dec!(0), $sdec!(0.0))]
        fn test_pow_nan(#[case] d: $D, #[case] n: $SD) {
            let ctx = Context::default().without_traps();
            let d = d.with_ctx(ctx).pow(n);
            assert!(d.is_nan());
            assert!(d.is_op_invalid());
        }
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, $sdec: ident, $SD: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D, $sdec, $SD);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, $sdec: ident, $SD: ident) => {

    };
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(-10), 77, $dec!(-1E+77), signals![!ROUND])]
        #[case($dec!(-10), 99, $dec!(-1E+99), signals![!ROUND])]
        #[case($dec!(-10), 22, $dec!(1E+22), signals![])]
        fn test_powi_128_signed(#[case] d: $D, #[case] n: i32, #[case] expected: $D, #[case] signals: Signals) {
            let d = d.powi(n);

            assert_eq!(d, expected);
            assert_eq!(d.op_signals(), signals);
        }
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {

        #[rstest(::trace)]
        #[case($dec!(-3), 2, $dec!(9))]
        #[case($dec!(-2), 3, $dec!(-8))]
        #[case($dec!(-3), 3, $dec!(-27))]
        #[case($dec!(-4), 3, $dec!(-64))]
        #[case($dec!(-5), 3, $dec!(-125))]
        #[case($dec!(-6), 3, $dec!(-216))]
        #[case($dec!(-7), 3, $dec!(-343))]
        // --------------------------------------
        #[case($dec!(-10), 0, $dec!(1))]
        #[case($dec!(-10), 1, $dec!(-10))]
        #[case($dec!(-10), 2, $dec!(100))]
        #[case($dec!(-10), 3, $dec!(-1000))]
        #[case($dec!(-10), 4, $dec!(10000))]
        #[case($dec!(-10), 5, $dec!(-100000))]
        #[case($dec!(-10), 6, $dec!(1000000))]
        #[case($dec!(-10), 7, $dec!(-10000000))]
        #[case($dec!(-10), 8, $dec!(100000000))]
        #[case($dec!(-10), 9, $dec!(-1000000000))]
        // --------------------------------------
        #[case($dec!(-0), -1, $D::NEG_INFINITY)]
        #[case($dec!(-0), -2, $D::INFINITY)]
        #[case($dec!(-0), -3, $D::NEG_INFINITY)]
        #[case($D::NEG_INFINITY, -1, $dec!(-0))]
        #[case($D::NEG_INFINITY, 0, $dec!(1))]
        #[case($D::NEG_INFINITY, 1, $D::NEG_INFINITY)]
        #[case($D::NEG_INFINITY, 2, $D::INFINITY)]
        #[case($D::NEG_INFINITY, 3, $D::NEG_INFINITY)]
        fn test_powi_signed(#[case] d: $D, #[case] n: i32, #[case] expected: $D) {
            let d = d.powi(n);

            assert_eq!(d, expected);
            assert_eq!(d.fractional_digits_count(), expected.fractional_digits_count());
            assert!(d.is_op_ok());
        }

        #[rstest(::trace)]
        #[case($dec!(-0), 0)]
        fn test_powi_nan_signed(#[case] d: $D, #[case] n: i32) {
            let ctx = Context::default().without_traps();
            let d = d.with_ctx(ctx).powi(n);
            assert!(d.is_nan());
            assert!(d.is_op_invalid());
        }
    };
}

pub(crate) use test_impl;
