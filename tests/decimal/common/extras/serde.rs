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
            use fastnum::{$dec, $D};

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{$dec, $D};

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(SIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (COMMON:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(1.3407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095), "1.3407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095")]
        fn test_serialize_deserialize_str_512(#[case] dec: $D, #[case] expected: &'static str) {
            let expected = serde_test::Token::Str(expected);
            serde_test::assert_tokens(&dec, &[expected]);
        }
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

        #[rstest(::trace)]
        #[case($dec!(115.792089237316195423570985008687907853269984665640564039457584007913129639935), "115.792089237316195423570985008687907853269984665640564039457584007913129639935")]
        fn test_serialize_deserialize_str_256(#[case] dec: $D, #[case] expected: &'static str) {
            let expected = serde_test::Token::Str(expected);
            serde_test::assert_tokens(&dec, &[expected]);
        }
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
    };
    (COMMON:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(1.0), "1.0")]
        #[case($dec!(0.5), "0.5")]
        #[case($dec!(50.), "50")]
        #[case($dec!(50000), "50000")]
        #[case($dec!(1e-3), "0.001")]
        #[case($dec!(10e11), "1000000000000")]
        #[case($dec!(0.25), "0.25")]
        #[case($dec!(12.34e1), "123.4")]
        #[case($dec!(40.0010), "40.0010")]
        fn test_serialize_deserialize_str(#[case] dec: $D, #[case] expected: &'static str) {
            let expected = serde_test::Token::Str(expected);
            serde_test::assert_tokens(&dec, &[expected]);
        }

        #[rstest(::trace)]
        #[case("1")]
        #[case("2")]
        #[case("3")]
        #[case("1.0")]
        #[case("12.34")]
        #[case("0.1")]
        #[case("0.01")]
        #[case("0.001")]
        #[case("0.0001")]
        fn test_json(#[case] num: &'static str) {
            use serde::{Deserialize, Serialize};

            use fastnum::decimal::extras::serde::DeserializeMode;

            #[derive(Serialize, Deserialize)]
            struct TestStruct {
                name: String,
                value: $D,
            }

            let d: $D = num.parse().unwrap();

            let json_src = format!("{{\"name\":\"foo\",\"value\":\"{}\"}}", num);

            let my_struct: TestStruct = serde_json::from_str(&json_src).unwrap();
            assert_eq!(&my_struct.name, "foo");
            assert_eq!(my_struct.value, d);

            let s = serde_json::to_string(&my_struct).unwrap();
            assert_eq!(s, json_src);

            let json = format!("{{\"name\":\"foo\",\"value\":{}}}", num);

            match DeserializeMode::default() {
                DeserializeMode::Strict => {
                    assert!(serde_json::from_str::<$D>(&json).is_err());
                }
                DeserializeMode::Stringify | DeserializeMode::Any => {
                    let my_struct: TestStruct = serde_json::from_str(&json).unwrap();
                    assert_eq!(&my_struct.name, "foo");
                    assert_eq!(my_struct.value, d);

                    let s = serde_json::to_string(&my_struct).unwrap();
                    assert_eq!(s, json_src);
                }
            }
        }

        super::test_impl!(FROM INT:: $dec, $D, U8, U16, U32, U64);
        super::test_impl!(TRY FROM FLOAT:: $dec, $D, F32, F64);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {
        super::test_impl!(TRY FROM INT:: $dec, $D, I8, I16, I32, I64);
    };
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        super::test_impl!(FROM INT:: $dec, $D, I8, I16, I32, I64);
    };
    (FROM INT:: $dec: ident, $D: ident, $($tt: ident),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case(0)]
                #[case(1)]
                #[case([<$tt:lower>]::MIN)]
                #[case([<$tt:lower>]::MAX)]
                fn [< test_deserialize_from_ $tt:lower >](#[case] int: [<$tt:lower>]) {
                    use fastnum::decimal::extras::serde::DeserializeMode;
                    let tokens = [serde_test::Token::$tt(int)];

                    match DeserializeMode::default() {
                        DeserializeMode::Strict => {
                            let err = format!("invalid type: integer `{}`, expected formatted decimal string in strict mode", int);
                            serde_test::assert_de_tokens_error::<$D>(&tokens, &err);
                        }
                        DeserializeMode::Stringify | DeserializeMode::Any => {
                            let expected = $D::from(int);
                            serde_test::assert_de_tokens(&expected, &tokens);
                        }
                    }
                }
            }
        )*
    };
    (TRY FROM INT:: $dec: ident, $D: ident, $($tt: ident),*) => {
        $(
            paste::paste! {
                #[rstest::rstest(::trace)]
                #[case(0)]
                #[case(1)]
                #[case(10)]
                #[case(100)]
                fn [< test_deserialize_from_ $tt:lower >](#[case] int: [<$tt:lower>]) {
                    use fastnum::decimal::extras::serde::DeserializeMode;
                    let tokens = [serde_test::Token::$tt(int)];

                    match DeserializeMode::default() {
                        DeserializeMode::Strict => {
                            let err = format!("invalid type: integer `{}`, expected formatted decimal string in strict mode", int);
                            serde_test::assert_de_tokens_error::<$D>(&tokens, &err);
                        }
                        DeserializeMode::Stringify | DeserializeMode::Any => {
                            let expected = $D::try_from(int).unwrap();
                            serde_test::assert_de_tokens(&expected, &tokens);
                        }
                    }
                }
            }
        )*
    };
    (TRY FROM FLOAT:: $dec: ident, $D: ident, $($tt: ident),*) => {
        $(
            paste::paste! {
                #[rstest(::trace)]
                #[case(0.)]
                #[case(1.)]
                #[case(10.)]
                #[case(100.)]
                #[case(1.2)]
                fn [< test_deserialize_from_ $tt:lower >](#[case] n: [<$tt:lower>]) {
                    use fastnum::decimal::extras::serde::DeserializeMode;
                    use crate::decimal::common::extras::serde::WithDecimalPoint;

                    let tokens = [serde_test::Token::$tt(n)];

                    match DeserializeMode::default() {
                        DeserializeMode::Strict => {
                            let err = format!("invalid type: floating point `{}`, expected formatted decimal string in strict mode", WithDecimalPoint(n as f64));
                            serde_test::assert_de_tokens_error::<$D>(&tokens, &err);
                        }
                        DeserializeMode::Stringify | DeserializeMode::Any => {
                            let expected = $D::try_from(n).unwrap();
                            serde_test::assert_de_tokens(&expected, &tokens);
                        }
                    }
                }

                // #[rstest(::trace)]
                // fn [< $name _ $ttype:lower _ nan >]() {
                //     use fastnum::decimal::extras::serde::DeserializeMode;
                //     let tokens = [ serde_test::Token::$ttype([<$ttype:lower>]::NAN) ];
                //
                //     match DeserializeMode::default() {
                //         DeserializeMode::Strict => {
                //             let err = "invalid type: floating point `NaN`, expected formatted decimal string in strict mode";
                //             serde_test::assert_de_tokens_error::<$DEC>(&tokens, err);
                //         }
                //         DeserializeMode::Stringify | DeserializeMode::Any => {
                //             serde_test::assert_de_tokens_error::<$DEC>(&tokens, "(fastnum) number is NaN");
                //         }
                //     }
                // }
            }
        )*
    };
}

pub(crate) use test_impl;

use core::{
    fmt,
    fmt::{Display, Write},
};

pub(crate) struct WithDecimalPoint(pub f64);

impl Display for WithDecimalPoint {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        struct LookForDecimalPoint<'f, 'a> {
            formatter: &'f mut fmt::Formatter<'a>,
            has_decimal_point: bool,
        }

        impl fmt::Write for LookForDecimalPoint<'_, '_> {
            fn write_str(&mut self, fragment: &str) -> fmt::Result {
                self.has_decimal_point |= fragment.contains('.');
                self.formatter.write_str(fragment)
            }

            fn write_char(&mut self, ch: char) -> fmt::Result {
                self.has_decimal_point |= ch == '.';
                self.formatter.write_char(ch)
            }
        }

        if self.0.is_finite() {
            let mut writer = LookForDecimalPoint {
                formatter,
                has_decimal_point: false,
            };
            write!(writer, "{}", self.0)?;
            if !writer.has_decimal_point {
                formatter.write_str(".0")?;
            }
        } else {
            write!(formatter, "{}", self.0)?;
        }
        Ok(())
    }
}
