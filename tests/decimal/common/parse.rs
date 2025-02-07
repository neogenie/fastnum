macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [< u $bits >], [<D $bits>], [<U $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [< u $bits >], [<UD $bits>], [<U $bits>]); }
    };
    (UNSIGNED: $bits: tt, $dec: ident, $uint: ident, $D: ident, $U: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{*, decimal::*};

            super::test_impl!(COMMON:: $bits, $uint, $D, $U, THIS);
            super::test_impl!(UNSIGNED:: $bits, $uint, $D, $U, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $uint: ident, $D: ident, $U: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{*, decimal::*};

            super::test_impl!(COMMON:: $bits, $uint, $D, $U, THIS);
            super::test_impl!(SIGNED:: $bits, $uint, $D, $U, THIS);
        }
    };

    (COMMON:: 512, $uint: ident, $D: ident, $U: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $uint, $D, $U);

        #[rstest(::trace)]
        #[case("115792089237316195423570985008687907853269984665640564039457584007913129639935", $uint!(115792089237316195423570985008687907853269984665640564039457584007913129639935), 0)]
        #[case("1157920892373161954235709850086.87907853269984665640564039457584007913129639935", $uint!(115792089237316195423570985008687907853269984665640564039457584007913129639935), -47)]
        #[case("13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095", $uint!(13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095), 0)]
        #[case("1340780792994259709957402499820584612747936582.0592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095", $uint!(13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095), -109)]
        fn test_parse_ok_512(#[case] s: &str, #[case] _int: $U, #[case] exp: i16) {
            let dec = $D::from_str(s, Context::default()).unwrap();
            assert_eq!(dec.digits(), _int);
            assert_eq!(dec.fractional_digits_count(), -exp);
            assert!(dec.is_op_ok());
        }

        #[rstest(::trace)]
        #[case("13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084096")]
        #[case("13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095.1")]
        #[should_panic(expected = "(fastnum) number too large to fit in target type")]
        fn test_parse_overflow_512(#[case] s: &str) {
            let _ = $D::from_str(s, Context::default()).unwrap();
        }
    };
    (UNSIGNED:: 512, $uint: ident, $D: ident, $U: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $uint, $D, $U);
    };
    (SIGNED:: 512, $uint: ident, $D: ident, $U: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $uint, $D, $U);
    };

    (COMMON:: 256, $uint: ident, $D: ident, $U: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $uint, $D, $U);

        #[rstest(::trace)]
        #[case("115792089237316195423570985008687907853269984665640564039457584007913129639936")]
        #[case("11579208923731619542357098500868790785326998466564056403945758400791312963993.6")]
        #[case("1157920892373161954235709850086879078532699846656405640394575840079131296399351")]
        #[case("115792089237316195423570985008687907853269984665640564039457584007913129639935.1")]
        #[should_panic(expected = "(fastnum) number too large to fit in target type")]
        fn test_parse_overflow_256(#[case] s: &str) {
            let _ = $D::from_str(s, Context::default()).unwrap();
        }

    };
    (COMMON:: 256, $uint: ident, $D: ident, $U: ident) => {
        super::test_impl!(COMMON:: 128, $uint, $D, $U);

        #[rstest(::trace)]
        #[case("340282366920938463463374607431768211455", $uint!(340282366920938463463374607431768211455), 0)]
        #[case("34028236692093846346337460743176821145.5", $uint!(340282366920938463463374607431768211455), -1)]
        #[case("34028236692093846346337460743176821145.5e1000", $uint!(340282366920938463463374607431768211455), 999)]
        #[case("340282366920938463.463374607431768211455e-1000", $uint!(340282366920938463463374607431768211455), -1021)]
        #[case("115792089237316195423570985008687907853269984665640564039457584007913129639935", $uint!(115792089237316195423570985008687907853269984665640564039457584007913129639935), 0)]
        #[case("1157920892373161954235709850086.87907853269984665640564039457584007913129639935", $uint!(115792089237316195423570985008687907853269984665640564039457584007913129639935), -47)]
        fn test_parse_ok_256(#[case] s: &str, #[case] _int: $U, #[case] exp: i16) {
            let dec = $D::from_str(s, Context::default()).unwrap();
            assert_eq!(dec.digits(), _int);
            assert_eq!(dec.fractional_digits_count(), -exp);
            assert!(dec.is_op_ok());
        }
    };
    (UNSIGNED:: 256, $uint: ident, $D: ident, $U: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $uint, $D, $U);
    };
    (UNSIGNED:: 256, $uint: ident, $D: ident, $U: ident) => {
        super::test_impl!(UNSIGNED:: 128, $uint, $D, $U);
    };
    (SIGNED:: 256, $uint: ident, $D: ident, $U: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $uint, $D, $U);
    };
    (SIGNED:: 256, $uint: ident, $D: ident, $U: ident) => {
        super::test_impl!(SIGNED:: 128, $uint, $D, $U);
    };

    (COMMON:: 128, $uint: ident, $D: ident, $U: ident, THIS) => {
        super::test_impl!(COMMON:: 128, $uint, $D, $U);

        #[rstest(::trace)]
        #[case("340282366920938463463374607431768211456")]
        #[case("340282366920938463463374607431768211455.5")]
        #[case("340282366920938463.4633746074317682114551")]
        #[case("1157920892373161954235709850086879078532699846656405640394575840079131296399351")]
        #[case("115792089237316195423570985008687907853269984665640564039457584007913129639935.1")]
        #[should_panic(expected = "(fastnum) number too large to fit in target type")]
        fn test_parse_overflow_128(#[case] s: &str) {
            let _ = $D::from_str(s, Context::default()).unwrap();
        }

    };
    (COMMON:: 128, $uint: ident, $D: ident, $U: ident) => {
        #[rstest(::trace)]
        #[case("0", $uint!(0), 0)]
        #[case("1", $uint!(1), 0)]
        #[case("01", $uint!(1), 0)]
        #[case("001", $uint!(1), 0)]
        #[case("0010", $uint!(10), 0)]
        #[case("+0", $uint!(0), 0)]
        #[case("+1", $uint!(1), 0)]
        #[case("+01", $uint!(1), 0)]
        #[case("1331.107", $uint!(1331107), -3)]
        #[case(".107",  $uint!(107), -3)]
        #[case("0.0000012345", $uint!(12345), -10)]
        #[case("5e9", $uint!(5), 9)]
        #[case("10000000000000000000", $uint!(10000000000000000000), 0)]
        #[case("10000000000000000001", $uint!(10000000000000000001), 0)]
        #[case("100000000000000000000", $uint!(100000000000000000000), 0)]
        #[case("0.000000000000000000000000000000000000001", $uint!(1), -39)]
        #[case("18446744073709551615", $uint!(18446744073709551615), 0)]
        #[case("18446744073709551616", $uint!(18446744073709551616), 0)]
        #[case("18_446_744_073_709_551_615", $uint!(18446744073709551615), 0)]
        #[case("20935706972060549068014", $uint!(20935706972060549068014), 0)]
        #[case("1.0", $uint!(10), -1)]
        #[case("2e1", $uint!(2), 1)]
        #[case("2e0", $uint!(2), 0)]
        #[case("0.00123", $uint!(123), -5)]
        #[case("123", $uint!(123), 0)]
        #[case("1230", $uint!(1230), 0)]
        #[case("12.3", $uint!(123), -1)]
        #[case("123e-1", $uint!(123), -1)]
        #[case("1.23e+1", $uint!(123), -1)]
        #[case("1.23E+3", $uint!(123), 1)]
        #[case("1.23E-8", $uint!(123), -10)]
        #[case("1.23E-10", $uint!(123), -12)]
        #[case("123_", $uint!(123), 0)]
        #[case("31_862_140.830_686_979", $uint!(31862140830686979), -9)]
        #[case("1_1.2_2", $uint!(1122), -2)]
        #[case("999.521_939", $uint!(999521939), -6)]
        #[case("679.35_84_03E-2", $uint!(679358403), -8)]
        #[case("271576662.__E4", $uint!(271576662), 4)]
        #[case("1_._2", $uint!(12), -1)]
        #[case("25.8", $uint!(258), -1)]
        #[case("0.000000034283", $uint!(34283), -12)]
        #[case("20935.706972060549068014", $uint!(20935706972060549068014), -18)]
        #[case("0.20935706972060549068014", $uint!(20935706972060549068014), -23)]
        #[case("340282366920938463463374607431768211455", $uint!(340282366920938463463374607431768211455), 0)]
        #[case("34028236692093846346337460743176821145.5", $uint!(340282366920938463463374607431768211455), -1)]
        #[case("34028236692093846346337460743176821145.5e1000", $uint!(340282366920938463463374607431768211455), 999)]
        #[case("340282366920938463.463374607431768211455e-1000", $uint!(340282366920938463463374607431768211455), -1021)]
        fn test_parse_ok(#[case] s: &str, #[case] _int: $U, #[case] exp: i16) {
            let dec = $D::from_str(s, Context::default()).unwrap();
            assert_eq!(dec.digits(), _int);
            assert_eq!(dec.fractional_digits_count(), -exp);
            assert!(dec.is_op_ok());
        }

        #[rstest(::trace)]
        #[case::nan("nan")]
        #[case::nan("NAN")]
        #[case::nan("NaN")]
        fn test_parse_nan(#[case] s: &str) {
            let dec = $D::from_str(s, Context::default()).unwrap();
            assert!(dec.is_nan());
        }

        #[rstest(::trace)]
        #[case::nan("Inf")]
        #[case::nan("+Inf")]
        #[case::nan("+Infinity")]
        fn test_parse_inf(#[case] s: &str) {
            let dec = $D::from_str(s, Context::default()).unwrap();
            assert!(dec.is_infinite());
            assert_eq!(dec, $D::INFINITY);
        }

        #[rstest(::trace)]
        #[case::empty("")]
        #[case::only_minus("-")]
        #[case::only_plus("+")]
        #[case::only_decimal_and_underscore("_._")]
        #[case::empty_exponent("123.123E")]
        #[case::only_decimal_point(".")]
        #[case::only_decimal_and_exponent(".e4")]
        #[case::only_exponent("e4")]
        #[should_panic(expected = "(fastnum) cannot parse decimal from empty string")]
        fn test_parse_empty(#[case] s: &str) {
            let _ = $D::from_str(s, Context::default()).unwrap();
        }

        #[rstest(::trace)]
        #[case::hello("hello")]
        #[case::incorrect_nan("nan1")]
        #[case::incorrect_inf("Inf1")]
        #[case::neg_nan("-nan")]
        #[case::incorrect_inf("-Inf1")]
        #[case::incorrect_inf("-InfinityInf")]
        #[case::several_dots("123.45.67")]
        #[case::invalid_char("12z3.12")]
        #[case::invalid_char("12💖3.12")]
        #[case::invalid_char("💖")]
        #[case::nan_exponent("123.123eg")]
        #[case::multiple_decimal_points("123.12.45")]
        #[case::string_hex("0xCafeBeef")]
        #[case::several_exponent("123.34e-1e-2")]
        #[case::invalid_exponent("123.34e-1.5")]
        #[should_panic(expected = "(fastnum) invalid literal found in string")]
        fn test_parse_invalid_digit(#[case] s: &str) {
            let _ = $D::from_str(s, Context::default()).unwrap();
        }

        #[rstest(::trace)]
        #[case("1e-9223372036854775808")]
        #[case("1e9223372036854775809")]
        #[should_panic(expected = "(fastnum) exponent is too large to fit in target type")]
        fn test_parse_exponent_overflow(#[case] s: &str) {
            let _ = $D::from_str(s, Context::default()).unwrap();
        }
    };
    (UNSIGNED:: 128, $uint: ident, $D: ident, $U: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $uint, $D, $U);
    };
    (UNSIGNED:: 128, $uint: ident, $D: ident, $U: ident) => {
        #[rstest(::trace)]
        #[case::minus_sign("-0")]
        #[case::minus_sign("-0.0")]
        #[case::minus_sign("-1")]
        #[case::minus_sign("-1.434343")]
        #[should_panic(expected = "(fastnum) number would be signed for unsigned type")]
        fn test_parse_unsigned(#[case] s: &str) {
            let _ = $D::from_str(s, Context::default()).unwrap();
        }
    };
    (SIGNED:: 128, $uint: ident, $D: ident, $U: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $uint, $D, $U);

        #[rstest(::trace)]
        #[case("-340282366920938463463374607431768211456")]
        #[case("-340282366920938463463374607431768211455.5")]
        #[case("-340282366920938463.4633746074317682114551")]
        #[case("-1157920892373161954235709850086879078532699846656405640394575840079131296399351")]
        #[case("-115792089237316195423570985008687907853269984665640564039457584007913129639935.1")]
        #[should_panic(expected = "(fastnum) number too small to fit in target type")]
        fn test_parse_overflow_128_signed(#[case] s: &str) {
            let _ = $D::from_str(s, Context::default()).unwrap();
        }
    };
    (SIGNED:: 128, $uint: ident, $D: ident, $U: ident) => {
        #[rstest(::trace)]
        #[case("-0", Sign::Minus, $uint!(0), 0)]
        #[case("-1", Sign::Minus, $uint!(1), 0)]
        #[case("-.107", Sign::Minus, $uint!(107), -3)]
        #[case("-0.000000000000000000000000000000000000001", Sign::Minus, $uint!(1), -39)]
        #[case("-123", Sign::Minus, $uint!(123), 0)]
        #[case("-1230", Sign::Minus, $uint!(1230), 0)]
        #[case("-1.23E-10", Sign::Minus, $uint!(123), -12)]
        #[case("-1_1.2_2", Sign::Minus, $uint!(1122), -2)]
        #[case("-34028236692093846346337460743176821145.5", Sign::Minus, $uint!(340282366920938463463374607431768211455), -1)]
        #[case("-340282366920938463.463374607431768211455e-1000", Sign::Minus, $uint!(340282366920938463463374607431768211455), -1021)]
        fn test_parse_ok_signed(
            #[case] s: &str,
            #[case] sign: Sign,
            #[case] _int: $U,
            #[case] exp: i16,
        ) {
            let dec = $D::from_str(s, Context::default()).unwrap();
            assert_eq!(dec.digits(), _int);
            assert_eq!(dec.sign(), sign);
            assert_eq!(dec.fractional_digits_count(), -exp);
            assert!(dec.is_op_ok());
        }

        #[rstest(::trace)]
        #[case::nan("-Infinity")]
        #[case::nan("-Inf")]
        fn test_parse_inf_neg(#[case] s: &str) {
            let dec = $D::from_str(s, Context::default()).unwrap();
            assert!(dec.is_infinite());
            assert_eq!(dec, $D::NEG_INFINITY);
        }

        #[rstest(::trace)]
        #[case("-0-1")]
        #[case("-1+")]
        #[case("+0+1")]
        #[case("--1")]
        #[case("+-35.55")]
        #[should_panic(expected = "(fastnum) invalid literal found in string")]
        fn test_parse_invalid_signed(#[case] s: &str) {
            let _ = $D::from_str(s, Context::default()).unwrap();
        }

        #[rstest(::trace)]
        #[case::invalid_exponent("-1e9223372036854775809")]
        #[case::invalid_exponent("-1e-9223372036854775808")]
        #[should_panic(expected = "(fastnum) exponent is too large to fit in target type")]
        fn test_parse_exponent_overflow_signed(#[case] s: &str) {
            let _ = $D::from_str(s, Context::default()).unwrap();
        }
    };
}

pub(crate) use test_impl;
