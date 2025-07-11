macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>]); test_impl!(UNSIGNED: $bits, [< udec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>]); }
    };

    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use fastnum::decimal::Context;
            use fastnum::*;
            use rstest::*;
            use rust_decimal::dec;

            #[rstest(::trace)]
            #[case(-42)]
            #[case(-123456789)]
            fn test_basic_conversions(#[case] val: i32) {
                let fastnum_val = $D::from(val);
                let rust_dec_val = fastnum_val.to_rust_decimal();
                let expected = rust_decimal::Decimal::from(val);
                assert_eq!(rust_dec_val, expected);
            }

            #[rstest(::trace)]
            #[case("-123.456")]
            #[case("-79228162514264337593543950335")] // Max negative mantissa
            fn test_string_conversions(#[case] val_str: &str) {
                let fastnum_val = $D::parse_str(val_str, Context::default());
                let rust_dec_val = fastnum_val.to_rust_decimal();
                let expected = rust_decimal::Decimal::from_str_exact(val_str).unwrap();
                assert_eq!(rust_dec_val, expected);
            }

            #[test]
            fn test_negative_scale() {
                // Scale represents the power of 10 divisor
                // digits 123, scale -2 represents 123 / 10^2 = 1.23
                let fastnum_val = $D::from_parts(
                    123u32.into(),
                    -2,
                    fastnum::decimal::Sign::Plus,
                    Context::default(),
                );
                let rust_dec_val = fastnum_val.to_rust_decimal();
                let expected = dec!(1.23);
                assert_eq!(rust_dec_val, expected);
            }

            #[test]
            #[should_panic(expected = "Scale out of bounds for rust_decimal::Decimal")]
            fn test_scale_too_high_should_panic() {
                // 29 decimal places should panic
                let val_str = "-0.12345678901234567890123456789"; // 29 decimal places
                let fastnum_val = $D::parse_str(val_str, Context::default());
                fastnum_val.to_rust_decimal();
            }

            #[test]
            #[should_panic(expected = "Mantissa too large for rust_decimal::Decimal")]
            fn test_mantissa_too_large_should_panic() {
                // 2^96 (one larger than max for rust_decimal)
                let val_str = "-79228162514264337593543950336";
                let fastnum_val = $D::parse_str(val_str, Context::default());
                fastnum_val.to_rust_decimal();
            }

            #[test]
            #[should_panic(expected = "Cannot convert non-finite value (NaN or Infinity) to rust_decimal::Decimal at compile time")]
            fn test_negative_infinity_should_panic() {
                $D::NEG_INFINITY.to_rust_decimal();
            }
        }
    };

    (UNSIGNED: $bits: tt, $udec: ident, $UD: ident) => {
        mod $udec {
            use fastnum::decimal::Context;
            use fastnum::*;
            use rstest::*;
            use rust_decimal::dec;

            #[rstest(::trace)]
            #[case(0)]
            #[case(42)]
            #[case(123456789)]
            fn test_basic_conversions(#[case] val: u32) {
                let fastnum_val = $UD::from(val);
                let rust_dec_val = fastnum_val.to_rust_decimal();
                let expected = rust_decimal::Decimal::from(val);
                assert_eq!(rust_dec_val, expected);
            }

            #[rstest(::trace)]
            #[case("123.456")]
            #[case("0.1234567890123456789012345678")] // 28 decimal places (max for rust_decimal)
            #[case("79228162514264337593543950335")] // Max mantissa for rust_decimal
            fn test_string_conversions(#[case] val_str: &str) {
                let fastnum_val = $UD::parse_str(val_str, Context::default());
                let rust_dec_val = fastnum_val.to_rust_decimal();
                let expected = rust_decimal::Decimal::from_str_exact(val_str).unwrap();
                assert_eq!(rust_dec_val, expected);
            }

            #[test]
            fn test_negative_scale() {
                // Scale represents the power of 10 divisor
                // digits 123, scale -2 represents 123 / 10^2 = 1.23
                let fastnum_val = $UD::from_parts(
                    123u32.into(),
                    -2,
                    Context::default(),
                );
                let rust_dec_val = fastnum_val.to_rust_decimal();
                let expected = dec!(1.23);
                assert_eq!(rust_dec_val, expected);
            }

            #[test]
            fn test_zero_scale() {
                let fastnum_val = $UD::from_parts(
                    12300u32.into(),
                    0,
                    Context::default(),
                );
                let rust_dec_val = fastnum_val.to_rust_decimal();
                let expected = dec!(12300);
                assert_eq!(rust_dec_val, expected);
            }

            #[test]
            #[should_panic(expected = "Scale out of bounds for rust_decimal::Decimal")]
            fn test_scale_too_high_should_panic() {
                // 29 decimal places should panic
                let val_str = "0.12345678901234567890123456789"; // 29 decimal places
                let fastnum_val = $UD::parse_str(val_str, Context::default());
                fastnum_val.to_rust_decimal();
            }

            #[test]
            #[should_panic(expected = "Mantissa too large for rust_decimal::Decimal")]
            fn test_mantissa_too_large_should_panic() {
                // 2^96 (one larger than max for rust_decimal)
                let val_str = "79228162514264337593543950336";
                let fastnum_val = $UD::parse_str(val_str, Context::default());
                fastnum_val.to_rust_decimal();
            }

            #[test]
            #[should_panic(expected = "Cannot convert non-finite value (NaN or Infinity) to rust_decimal::Decimal at compile time")]
            fn test_nan_should_panic() {
                $UD::NAN.to_rust_decimal();
            }

            #[test]
            #[should_panic(expected = "Cannot convert non-finite value (NaN or Infinity) to rust_decimal::Decimal at compile time")]
            fn test_infinity_should_panic() {
                $UD::INFINITY.to_rust_decimal();
            }
        }
    };
}

pub(crate) use test_impl;
