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
        #[case($D::INFINITY, concat!(stringify!($D), r#"(digits=[340282366920938463463374607431768211455], exp=[32768], flags=[INF], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        fn test_fmt_debug_128(#[case] d: $D, #[case] expected: &str) {
            let formated = format!("{d:?}");
            assert_eq!(formated.as_str(), expected);
        }
    };
    (COMMON:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($D::NAN, "NaN")]
        #[case($D::INFINITY, "Inf")]
        #[case($dec!(0), "0")]
        #[case($dec!(0.00), "0.00")]
        #[case($dec!(1), "1")]
        #[case($dec!(10), "10")]
        #[case($dec!(0.123), "0.123")]
        #[case($dec!(0.0123), "0.0123")]
        #[case($dec!(0.00123), "0.00123")]
        #[case($dec!(0.000123), "0.000123")]
        #[case($dec!(1.23E-4), "0.000123")]
        #[case($dec!(123.), "123")]
        #[case($dec!(123.e1), "1230")]
        fn test_fmt(#[case] d: $D, #[case] expected: &str) {
            let formated = format!("{d}");
            assert_eq!(formated.as_str(), expected);
        }

        #[rstest(::trace)]
        #[case($D::NAN, concat!(stringify!($D), r#"(digits=[0], exp=[0], flags=[NAN], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(0), concat!(stringify!($D), r#"(digits=[0], exp=[0], flags=[], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(0.00), concat!(stringify!($D), r#"(digits=[0], exp=[-2], flags=[], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(1), concat!(stringify!($D), r#"(digits=[1], exp=[0], flags=[], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(123.400), concat!(stringify!($D), r#"(digits=[123400], exp=[-3], flags=[], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(123.4e-2), concat!(stringify!($D), r#"(digits=[1234], exp=[-3], flags=[], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(123.456), concat!(stringify!($D), r#"(digits=[123456], exp=[-3], flags=[], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(01.20), concat!(stringify!($D), r#"(digits=[120], exp=[-2], flags=[], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(1.20), concat!(stringify!($D), r#"(digits=[120], exp=[-2], flags=[], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(01.2E3), concat!(stringify!($D), r#"(digits=[12], exp=[2], flags=[], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(6.02214076e1023), concat!(stringify!($D), r#"(digits=[602214076], exp=[1015], flags=[], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(1e9999), concat!(stringify!($D), r#"(digits=[1], exp=[9999], flags=[], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        fn test_fmt_debug(#[case] d: $D, #[case] expected: &str) {
            let formated = format!("{d:?}");
            assert_eq!(formated.as_str(), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(0), concat!(stringify!($D), r#"(0e0)"#))]
        #[case($dec!(1), concat!(stringify!($D), r#"(1e0)"#))]
        #[case($dec!(123.400), concat!(stringify!($D), r#"(123400e-3)"#))]
        #[case($dec!(123.4e-2), concat!(stringify!($D), r#"(1234e-3)"#))]
        #[case($dec!(123.456), concat!(stringify!($D), r#"(123456e-3)"#))]
        #[case($dec!(01.20), concat!(stringify!($D), r#"(120e-2)"#))]
        #[case($dec!(1.20), concat!(stringify!($D), r#"(120e-2)"#))]
        #[case($dec!(01.2E3), concat!(stringify!($D), r#"(12e2)"#))]
        #[case($dec!(6.02214076e1023), concat!(stringify!($D), r#"(602214076e1015)"#))]
        #[case($dec!(1e9999), concat!(stringify!($D), r#"(1e9999)"#))]
        fn test_fmt_debug_alt(#[case] d: $D, #[case] expected: &str) {
            let formated = format!("{d:#?}");
            assert_eq!(formated.as_str(), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1),     "1",      "1.0",    "1.0000",   " 1.0",   "+01.0",   "1.0 ")]
        #[case($dec!(0.1),   "0.1",    "0.1",    "0.1000",   " 0.1",   "+00.1",   "0.1 ")]
        #[case($dec!(0.01),  "0.01",   "0.0",    "0.0100",   " 0.0",   "+00.0",   "0.0 ")]
        #[case($dec!(100),   "100",  "100.0",  "100.0000",  "100.0",  "+100.0",  "100.0")]
        fn test_fmt_options(
            #[case] d: $D,
            #[case] expected: &str,
            #[case] expected_d1: &str,
            #[case] expected_d4: &str,
            #[case] expected_4d1: &str,
            #[case] expected_p05d1: &str,
            #[case] expected_l4d1: &str,
        ) {
            assert_eq!(format!("{}", d), expected);
            assert_eq!(format!("{:.1}", d), expected_d1);
            assert_eq!(format!("{:.4}", d), expected_d4);
            assert_eq!(format!("{:4.1}", d), expected_4d1);
            assert_eq!(format!("{:+05.1}", d), expected_p05d1);
            assert_eq!(format!("{:<4.1}", d), expected_l4d1);
        }

        #[rstest(::trace)]
        #[case($dec!(1), "1")]
        #[case($dec!(10), "10")]
        #[case($dec!(0.1), "0.1")]
        #[case($dec!(0.9), "0.9")]
        #[case($dec!(800e-3), "0.800")]
        #[case($dec!(123456), "123456")]
        #[case($dec!(9999999), "9999999")]
        #[case($dec!(19073.97235939614856), "19073.97235939614856")]
        #[case($dec!(1764031078e-13), "0.0001764031078")]
        #[case($dec!(1e15), "1000000000000000")]
        #[case($dec!(1e16), "1e+16")]
        #[case($dec!(491326e-12), "4.91326E-7")]
        #[case($dec!(0.00003102564500), "0.00003102564500")]
        #[case($dec!(1E-10000), "1E-10000")]
        #[case($dec!(1e10000), "1e+10000")]
        #[case($dec!(1234506789e5), "123450678900000")]
        #[case($dec!(1234506789e15), "1234506789000000000000000")]
        #[case($dec!(1234506789e16), "1234506789e+16")]
        #[case($dec!(13400476439814628800e2502), "13400476439814628800e+2502")]
        fn test_fmt_options_default(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1), "1")]
        #[case($dec!(10), "10")]
        #[case($dec!(0.1), "0")]
        #[case($dec!(0.9), "1")]
        #[case($dec!(800e-3), "1")]
        #[case($dec!(19073.97235939614856), "19074")]
        #[case($dec!(1e15), "1000000000000000")]
        #[case($dec!(1e16), "10000000000000000")]
        #[case($dec!(491326e-12), "5E-7")]
        #[case($dec!(0.00003102564500), "0")]
        fn test_fmt_options_d0(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.0}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(9999999), "1e+7")]
        #[case($dec!(0.00003102564500), "3e-5")]
        fn test_fmt_options_d0e(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.0e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1), "1.0")]
        #[case($dec!(10), "10.0")]
        #[case($dec!(0.1), "0.1")]
        #[case($dec!(0.9), "0.9")]
        #[case($dec!(800e-3), "0.8")]
        #[case($dec!(123456), "123456.0")]
        #[case($dec!(19073.97235939614856), "19074.0")]
        #[case($dec!(1764031078e-13), "0.0")]
        #[case($dec!(1e15), "1000000000000000.0")]
        #[case($dec!(491326e-12), "4.9E-7")]
        #[case($dec!(1E-10000), "1.0E-10000")]
        #[case($dec!(1e10000), "1e+10000")]
        #[case($dec!(1234506789e5), "123450678900000.0")]
        #[case($dec!(1234506789e15), "1234506789000000000000000.0")]
        #[case($dec!(13400476439814628800e2502), "13400476439814628800e+2502")]
        fn test_fmt_options_d1(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.1}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(9999999), "1.0e+7")]
        #[case($dec!(0.00003102564500), "3.1e-5")]
        fn test_fmt_options_d1e(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.1e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1), "1.00")]
        #[case($dec!(10), "10.00")]
        #[case($dec!(0.1), "0.10")]
        #[case($dec!(1e16), "10000000000000000.00")]
        fn test_fmt_options_d2(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.2}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(9999999), "1.00e+7")]
        fn test_fmt_options_d2e(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.2e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(800e-3), "0.800")]
        #[case($dec!(19073.97235939614856), "19073.972")]
        #[case($dec!(1764031078e-13), "0.000")]
        #[case($dec!(491326e-12), "4.913E-7")]
        #[case($dec!(1234506789e5), "123450678900000.000")]
        #[case($dec!(1234506789e15), "1234506789000000000000000.000")]
        fn test_fmt_options_d3(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.3}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1), "1.0000")]
        #[case($dec!(0.9), "0.9000")]
        #[case($dec!(123456), "123456.0000")]
        #[case($dec!(19073.97235939614856), "19073.9724")]
        #[case($dec!(1764031078e-13), "0.0002")]
        #[case($dec!(0.00003102564500), "0.0000")]
        #[case($dec!(1E-10000), "1.0000E-10000")]
        #[case($dec!(1e10000), "1e+10000")]
        #[case($dec!(1234506789e5), "123450678900000.0000")]
        fn test_fmt_options_d4(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.4}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(9999999), "1.0000e+7")]
        #[case($dec!(0.00003102564500), "3.1026e-5")]
        fn test_fmt_options_d4e(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.4e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1764031078e-13), "0.00018")]
        #[case($dec!(491326e-12), "4.91326E-7")]
        #[case($dec!(0.00003102564500), "0.00003")]
        fn test_fmt_options_d5(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.5}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(491326e-12), "4.913260E-7")]
        fn test_fmt_options_d6(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.6}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(9999999), "9.999999e+6")]
        fn test_fmt_options_d6e(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.6e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(9999999), "9.9999990e+6")]
        fn test_fmt_options_d7e(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.7e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(9999999), "9999999.00000000")]
        fn test_fmt_options_d8(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.8}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(800e-3), "0.800000000")]
        #[case($dec!(491326e-12), "4.913260000E-7")]
        fn test_fmt_options_d9(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.9}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(0.00003102564500), "0.0000310256")]
        fn test_fmt_options_d10(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.10}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(9999999), "9.9999990000e+6")]
        fn test_fmt_options_d10e(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.10e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1764031078e-13), "0.0001764031078")]
        fn test_fmt_options_d13(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.13}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(0.00003102564500), "0.00003102564500")]
        fn test_fmt_options_d14(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.14}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(0.00003102564500), "0.00003102564500000")]
        fn test_fmt_options_d17(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.17}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1764031078e-13), "0.00017640310780000000")]
        #[case($dec!(491326e-12), "4.91326000000000000000E-7")]
        fn test_fmt_options_d20(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.20}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1), " 1.0")]
        #[case($dec!(123456), "123456.0")]
        fn test_fmt_options_4d1(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:4.1}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(19073.97235939614856), "19073.972")]
        fn test_fmt_options_8d3(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:8.3}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(19073.97235939614856), " 19073.972")]
        fn test_fmt_options_10d3(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:10.3}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(19073.97235939614856), "019073.972")]
        fn test_fmt_options_010d3(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:010.3}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(123456), "      123456.00")]
        fn test_fmt_options_15d2(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:15.2}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1), " 1.0")]
        fn test_fmt_options_r4d1(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:>4.1}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(123456), "      123456.00")]
        fn test_fmt_options_r15d2(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:>15.2}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1234506789e5), "   123450678900000.0000")]
        fn test_fmt_options_r23d4(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:>23.4}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1), "1.0 ")]
        fn test_fmt_options_l4d1(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:<4.1}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(123456), "123456.00      ")]
        fn test_fmt_options_l15d2(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:<15.2}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1234506789e5), "123450678900000.0000   ")]
        fn test_fmt_options_l23d4(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:<23.4}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1), "+01.0")]
        fn test_fmt_options_p05d1(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:+05.1}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(123456), "+123456.0000000")]
        fn test_fmt_options_p05d7(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:+05.7}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(19073.97235939614856), "+19073.9723594")]
        fn test_fmt_options_pd7(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:+.7}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1234506789e15), "+1234506789000000000000000.00   ")]
        fn test_fmt_options_l32d2(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:<+32.2}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1234506789e15), "   +1234506789000000000000000.00")]
        fn test_fmt_options_r32d2(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:>+32.2}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1), "1e+0")]
        #[case($dec!(9999999), "9.999999e+6")]
        #[case($dec!(0.00003102564500), "3.102564500e-5")]
        fn test_fmt_options_exp(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(0.00003102564500), "3.102564500e-5")]
        fn test_fmt_options_d_exp(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:.e}", d), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(1), "1E+0")]
        #[case($dec!(9999999), "9.999999E+6")]
        #[case($dec!(0.00003102564500), "3.102564500E-5")]
        fn test_fmt_options_exp_upper(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(format!("{:E}", d), expected);
        }

        // TODO: subnormals & other

        #[rstest(::trace)]
        #[case(format!("1E{}", i8::MAX), "1e+127")]
        #[case(format!("1E{}", i16::MAX), "1e+32767")]
        fn test_fmt_boundaries(#[case] src: String, #[case] expected: &str) {
            let d: $D = src.parse().unwrap();
            let result = d.to_string();
            assert_eq!(result, expected);

            let round_trip = $D::from_str(&result, Context::default()).unwrap();
            assert_eq!(round_trip, d);

            let sci = d.to_scientific_notation();
            let sci_round_trip = $D::from_str(&sci, Context::default()).unwrap();
            assert_eq!(sci_round_trip, d);

            let eng = d.to_engineering_notation();
            let eng_round_trip = $D::from_str(&eng, Context::default()).unwrap();
            assert_eq!(eng_round_trip, d);
        }

        #[rstest(::trace)]
        #[case($dec!(4159248078.2835), "4.1592480782835e9")]
        #[case($dec!(0.00001234), "1.234e-5")]
        #[case($dec!(0), "0e0")]
        #[case($dec!(1), "1e0")]
        #[case($dec!(2.00), "2.00e0")]
        fn test_fmt_scientific_notation(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(d.to_scientific_notation(), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(4159248078.2835), "4.1592480782835e9")]
        #[case($dec!(0.00001234), "12.34e-6")]
        #[case($dec!(0), "0e0")]
        #[case($dec!(1), "1e0")]
        #[case($dec!(2.00), "2.00e0")]
        #[case($dec!(5.31e4), "53.1e3")]
        #[case($dec!(5.31e5), "531e3")]
        #[case($dec!(5.31e6), "5.31e6")]
        #[case($dec!(5.31e7), "53.1e6")]
        #[case($dec!(1e2), "100e0")]
        #[case($dec!(1e19), "10e18")]
        #[case($dec!(1e3000), "1e3000")]
        #[case($dec!(4.2e7), "42e6")]
        #[case($dec!(4.2e8), "420e6")]
        #[case($dec!(4e9999), "4e9999")]
        #[case($dec!(4e9998), "400e9996")]
        #[case($dec!(44e9998), "4.4e9999")]
        #[case($dec!(4e9997), "40e9996")]
        #[case($dec!(41e9997), "410e9996")]
        #[case($dec!(413e9997), "4.13e9999")]
        fn test_fmt_engineering_notation(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(d.to_engineering_notation(), expected);
        }
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {
    };
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);

        #[rstest(::trace)]
        #[case($D::NEG_INFINITY, concat!(stringify!($D), r#"(digits=[340282366920938463463374607431768211455], exp=[32768], flags=[S, INF], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        fn test_fmt_debug_signed_128(#[case] d: $D, #[case] expected: &str) {
            let formated = format!("{d:?}");
            assert_eq!(formated.as_str(), expected);
        }
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($D::NEG_INFINITY, "-Inf")]
        #[case($dec!(-0), "-0")]
        #[case($dec!(-0.00), "-0.00")]
        #[case($dec!(-123.e1), "-1230")]
        #[case($dec!(-90037659.6905), "-90037659.6905")]
        fn test_fmt_signed(#[case] d: $D, #[case] expected: &str) {
            let formated = format!("{d}");
            assert_eq!(formated.as_str(), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(-0), concat!(stringify!($D), r#"(digits=[0], exp=[0], flags=[S], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(-1), concat!(stringify!($D), r#"(digits=[1], exp=[0], flags=[S], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(-123.400), concat!(stringify!($D), r#"(digits=[123400], exp=[-3], flags=[S], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(+123.4e-2), concat!(stringify!($D), r#"(digits=[1234], exp=[-3], flags=[], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(-123.456), concat!(stringify!($D), r#"(digits=[123456], exp=[-3], flags=[S], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(+01.20), concat!(stringify!($D), r#"(digits=[120], exp=[-2], flags=[], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(-1.20), concat!(stringify!($D), r#"(digits=[120], exp=[-2], flags=[S], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(-01.2E3), concat!(stringify!($D), r#"(digits=[12], exp=[2], flags=[S], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(-6.02214076e1023), concat!(stringify!($D), r#"(digits=[602214076], exp=[1015], flags=[S], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(+1e9999), concat!(stringify!($D), r#"(digits=[1], exp=[9999], flags=[], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(-144.3308279), concat!(stringify!($D), r#"(digits=[1443308279], exp=[-7], flags=[S], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(-349983058835858339619e2), concat!(stringify!($D), r#"(digits=[349983058835858339619], exp=[2], flags=[S], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        #[case($dec!(-90037659.6905), concat!(stringify!($D), r#"(digits=[900376596905], exp=[-4], flags=[S], signals=[], ctx=[R=HalfUp, S=!DBZ, !INV, !OFW], extra=[0.0000000])"#))]
        fn test_fmt_debug_signed(#[case] d: $D, #[case] expected: &str) {
            let formated = format!("{d:?}");
            assert_eq!(formated.as_str(), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(+0), concat!(stringify!($D), r#"(0e0)"#))]
        #[case($dec!(-0), concat!(stringify!($D), r#"(-0e0)"#))]
        #[case($dec!(-1), concat!(stringify!($D), r#"(-1e0)"#))]
        #[case($dec!(-123.400), concat!(stringify!($D), r#"(-123400e-3)"#))]
        #[case($dec!(+123.4e-2), concat!(stringify!($D), r#"(1234e-3)"#))]
        #[case($dec!(-123.456), concat!(stringify!($D), r#"(-123456e-3)"#))]
        #[case($dec!(-01.20), concat!(stringify!($D), r#"(-120e-2)"#))]
        #[case($dec!(-1.20), concat!(stringify!($D), r#"(-120e-2)"#))]
        #[case($dec!(-01.2E3), concat!(stringify!($D), r#"(-12e2)"#))]
        #[case($dec!(-6.02214076e1023), concat!(stringify!($D), r#"(-602214076e1015)"#))]
        #[case($dec!(+1e9999), concat!(stringify!($D), r#"(1e9999)"#))]
        #[case($dec!(-1e9999), concat!(stringify!($D), r#"(-1e9999)"#))]
        #[case($dec!(-144.3308279), concat!(stringify!($D), r#"(-1443308279e-7)"#))]
        #[case($dec!(-349983058835858339619e2), concat!(stringify!($D), r#"(-349983058835858339619e2)"#))]
        #[case($dec!(-90037659.6905), concat!(stringify!($D), r#"(-900376596905e-4)"#))]
        fn test_fmt_debug_alt_signed(#[case] d: $D, #[case] expected: &str) {
            let formated = format!("{d:#?}");
            assert_eq!(formated.as_str(), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(-1),              "-1",    "-1.0",   "-1.0000",   "-1.0",   "-01.0",  "-1.0" )]
        #[case($dec!(-0.1),            "-0.1",  "-0.1",   "-0.1000",   "-0.1",   "-00.1",  "-0.1")]
        #[case($dec!(-0.01),           "-0.01", "-0.0",   "-0.0100",   "-0.0",   "-00.0",  "-0.0")]
        fn test_fmt_options_signed(#[case] d: $D,
                            #[case] expected: &str,
                            #[case] expected_d1: &str,
                            #[case] expected_d4: &str,
                            #[case] expected_4d1: &str,
                            #[case] expected_p05d1: &str,
                            #[case] expected_l4d1: &str,
        ) {
            assert_eq!(format!("{}", d), expected);
            assert_eq!(format!("{:.1}", d), expected_d1);
            assert_eq!(format!("{:.4}", d), expected_d4);
            assert_eq!(format!("{:4.1}", d), expected_4d1);
            assert_eq!(format!("{:+05.1}", d), expected_p05d1);
            assert_eq!(format!("{:<4.1}", d), expected_l4d1);
        }

        #[rstest(::trace)]
        fn test_fmt_options_signed_n90037659d6905() {
            let d = $dec!(-90037659.6905);

            assert_eq!(format!("{:+.7}", d), "-90037659.6905000");
            assert_eq!(format!("{:.0}", d), "-90037660");
            assert_eq!(format!("{:.3}", d), "-90037659.691");
            assert_eq!(format!("{:.4}", d), "-90037659.6905");
            assert_eq!(format!("{:14.4}", d), "-90037659.6905");
            assert_eq!(format!("{:15.4}", d), " -90037659.6905");
            assert_eq!(format!("{:<17.5}", d), "-90037659.69050  ");
        }

        #[rstest(::trace)]
        #[case($dec!(-57.0), "-5.70e1")]
        #[case($dec!(-0.00001234), "-1.234e-5")]
        #[case($dec!(-0), "-0e0")]
        #[case($dec!(-1), "-1e0")]
        #[case($dec!(-2.00), "-2.00e0")]
        fn test_fmt_scientific_notation_signed(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(d.to_scientific_notation(), expected);
        }

        #[rstest(::trace)]
        #[case($dec!(-57.0), "-57.0e0")]
        #[case($dec!(-0), "-0e0")]
        #[case($dec!(-1), "-1e0")]
        #[case($dec!(-2.00), "-2.00e0")]
        #[case($dec!(-5.31e4), "-53.1e3")]
        #[case($dec!(-413e9997), "-4.13e9999")]
        fn test_fmt_engineering_notation_signed(#[case] d: $D, #[case] expected: &str) {
            assert_eq!(d.to_engineering_notation(), expected);
        }
    };
}

pub(crate) use test_impl;
