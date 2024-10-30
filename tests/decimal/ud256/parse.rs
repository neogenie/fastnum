use std::str::FromStr;

use rstest::*;

use fastnum::{U256, UD256, u256};

#[rstest]
#[trace]
#[case("0", u256!(0), 0)]
#[case("1", u256!(1), 0)]
#[case("01", u256!(1), 0)]
#[case("001", u256!(1), 0)]
#[case("0010", u256!(10), 0)]
#[case("1331.107", u256!(1331107), -3)]
#[case(".107",  u256!(107), -3)]
#[case("0.0000012345", u256!(12345), -10)]
#[case("5e9", u256!(5), 9)]
#[case("10000000000000000000", u256!(10000000000000000000), 0)]
#[case("10000000000000000001", u256!(10000000000000000001), 0)]
#[case("100000000000000000000", u256!(100000000000000000000), 0)]
#[case("0.000000000000000000000000000000000000001", u256!(1), -39)]
#[case("18446744073709551615", u256!(18446744073709551615), 0)]
#[case("18446744073709551616", u256!(18446744073709551616), 0)]
#[case("18_446_744_073_709_551_615", u256!(18446744073709551615), 0)]
#[case("20935706972060549068014", u256!(20935706972060549068014), 0)]
#[case("1.0", u256!(10), -1)]
#[case("2e1", u256!(2), 1)]
#[case("2e0", u256!(2), 0)]
#[case("0.00123", u256!(123), -5)]
#[case("123", u256!(123), 0)]
#[case("1230", u256!(1230), 0)]
#[case("12.3", u256!(123), -1)]
#[case("123e-1", u256!(123), -1)]
#[case("1.23e+1", u256!(123), -1)]
#[case("1.23E+3", u256!(123), 1)]
#[case("1.23E-8", u256!(123), -10)]
#[case("1.23E-10", u256!(123), -12)]
#[case("123_", u256!(123), 0)]
#[case("31_862_140.830_686_979", u256!(31862140830686979), -9)]
#[case("1_1.2_2", u256!(1122), -2)]
#[case("999.521_939", u256!(999521939), -6)]
#[case("679.35_84_03E-2", u256!(679358403), -8)]
#[case("271576662.__E4", u256!(271576662), 4)]
#[case("1_._2", u256!(12), -1)]
#[case("25.8", u256!(258), -1)]
#[case("0.000000034283", u256!(34283), -12)]
#[case("20935.706972060549068014", u256!(20935706972060549068014), -18)]
#[case("0.20935706972060549068014", u256!(20935706972060549068014), -23)]
#[case("115792089237316195423570985008687907853269984665640564039457584007913129639935", u256!(115792089237316195423570985008687907853269984665640564039457584007913129639935), 0)]
fn test_parse_ok(#[case] s: &str, #[case] _int: U256, #[case] exp: i64) {
    let dec = UD256::from_str(s).unwrap();
    assert_eq!(dec.significant_digits(), _int);
    assert_eq!(dec.fractional_digit_count(), -exp);
}

#[rstest]
#[trace]
#[case::empty("")]
#[case::only_decimal_and_underscore("_._")]
#[case::empty_exponent("123.123E")]
#[case::only_decimal_point(".")]
#[case::only_decimal_and_exponent(".e4")]
#[case::only_exponent("e4")]
#[should_panic(expected = "(fastnum) cannot parse decimal from empty string")]
fn test_parse_empty(#[case] s: &str) {
    let _ = UD256::from_str(s).unwrap();
}

#[rstest]
#[trace]
#[case::hello("hello")]
#[case::nan("nan")]
#[case::several_dots("123.45.67")]
#[case::invalid_char("12z3.12")]
#[case::invalid_char_("12💖3.12")]
#[case::nan_exponent("123.123eg")]
#[case::multiple_decimal_points("123.12.45")]
#[case::string_hex("0xCafeBeef")]
#[case::several_exponent("123.34e-1e-2")]
#[case::invalid_exponent("123.34e-1.5")]
#[case::invalid_exponent_("💖")]
#[case::minus_sign("-0")]
#[case::plus_sign("+0")]
#[should_panic(expected = "(fastnum) invalid literal found in string")]
fn test_parse_invalid_digit(#[case] s: &str) {
    let _ = UD256::from_str(s).unwrap();
}

#[rstest]
#[trace]
#[case::invalid_exponent("1e-9223372036854775809")]
#[case::invalid_exponent("1e9223372036854775808")]
#[should_panic(expected = "(fastnum) exponent is too large to fit in target type")]
fn test_parse_exponent_overflow(#[case] s: &str) {
    let _ = UD256::from_str(s).unwrap();
}

#[rstest]
#[trace]
#[case("1157920892373161954235709850086879078532699846656405640394575840079131296399351")]
#[case("115792089237316195423570985008687907853269984665640564039457584007913129639935.1")]
#[should_panic(expected = "(fastnum) number too large to fit in target type")]
fn test_parse_overflow(#[case] s: &str) {
    let _ = UD256::from_str(s).unwrap();
}