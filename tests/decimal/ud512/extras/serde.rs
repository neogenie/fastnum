use rstest::*;

use serde_test::{
    Token, assert_tokens, assert_de_tokens, assert_de_tokens_error,
};

use fastnum::{udec256, UD256};

#[rstest]
#[case(udec256!(1.0), "1.0")]
#[case(udec256!(0.5), "0.5")]
#[case(udec256!(50.), "50")]
#[case(udec256!(50000), "50000")]
#[case(udec256!(1e-3), "0.001")]
#[case(udec256!(10e11), "1000000000000")]
#[case(udec256!(0.25), "0.25")]
#[case(udec256!(12.34e1), "123.4")]
#[case(udec256!(40.0010), "40.0010")]
#[case(udec256!(115.792089237316195423570985008687907853269984665640564039457584007913129639935), "115.792089237316195423570985008687907853269984665640564039457584007913129639935")]
fn test_serialize_deserialize_str(#[case] dec: UD256, #[case] expected: &'static str) {
    let expected = Token::Str(expected);
    assert_tokens(&dec, &[expected]);
}

// #[cfg(not(feature = "string-only"))]
// mod serde_deserialize_int {
//     use super::*;
// 
//     macro_rules! impl_case {
//             ( $( $ttype:ident ),+ : -$input:literal ) => {
//                 $( paste! { impl_case!([< case_n $input _ $ttype:lower >] : $ttype : -$input); } )*
//             };
//             ( $( $ttype:ident ),+ : $input:literal ) => {
//                 $( paste! { impl_case!([< case_ $input _ $ttype:lower >] : $ttype : $input); } )*
//             };
//             ($name:ident : $type:ident : $input:literal) => {
//                 #[test]
//                 fn $name() {
//                     let expected = BigDecimal::from($input);
//                     let token = Token::$type($input);
//                     assert_de_tokens(&expected, &[token]);
//                 }
//             };
//         }
// 
//     impl_case!(I8, I16, I32, I64, U8, U16, U32, U64 : 0);
//     impl_case!(I8, I16, I32, I64, U8, U16, U32, U64 : 1);
//     impl_case!(I8, I16, I32, I64 : -1);
//     impl_case!(I64: -99999999999i64);
//     impl_case!(I64: -9_223_372_036_854_775_808i64);
// }
// 
// #[cfg(not(feature = "string-only"))]
// mod serde_deserialize_float {
//     use super::*;
// 
//     macro_rules! impl_case {
//             ( $name:ident : $input:literal => $ttype:ident : $expected:literal ) => {
//                 paste! {
//                     #[test]
//                     fn [< $name _ $ttype:lower >]() {
//                         let expected: BigDecimal = $expected.parse().unwrap();
//                         let token = Token::$ttype($input);
//                         assert_de_tokens(&expected, &[token]);
//                     }
//                 }
//             };
//             ( $name:ident : $input:literal => $( $ttype:ident : $expected:literal )+ ) => {
//                 $( impl_case!($name : $input => $ttype : $expected); )*
//             };
//             ( $name:ident : $input:literal => $( $ttype:ident ),+ : $expected:literal ) => {
//                 $( impl_case!($name : $input => $ttype : $expected); )*
//             };
//         }
// 
//     impl_case!(case_1d0 : 1.0 => F32, F64 : "1");
//     impl_case!(case_1d1 : 1.1 => F32 : "1.10000002384185791015625"
//                                      F64 : "1.100000000000000088817841970012523233890533447265625");
// 
//     impl_case!(case_0d001834988943300:
//             0.001834988943300 => F32 : "0.001834988943301141262054443359375"
//                                  F64 : "0.00183498894330000003084768511740776375518180429935455322265625");
// 
//     impl_case!(case_n869651d9131236838:
//             -869651.9131236838 => F32 : "-869651.9375"
//                                   F64 : "-869651.91312368377111852169036865234375");
// 
//     impl_case!(case_n1en20:
//             -1e-20 => F32 : "-9.999999682655225388967887463487205224055287544615566730499267578125E-21"
//                       F64 : "-999999999999999945153271454209571651729503702787392447107715776066783064379706047475337982177734375e-119");
// }
// 
// #[cfg(not(feature = "string-only"))]
// mod serde_deserialize_nan {
//     use super::*;
// 
//     #[test]
//     fn case_f32() {
//         let tokens = [ Token::F32(f32::NAN) ];
//         assert_de_tokens_error::<BigDecimal>(&tokens, "NAN");
//     }
// 
//     #[test]
//     fn case_f64() {
//         let tokens = [ Token::F64(f64::NAN) ];
//         assert_de_tokens_error::<BigDecimal>(&tokens, "NAN");
//     }
// }
// 
// 
// #[cfg(feature = "serde_json")]
// mod json_support {
//     use super::*;
//     use impl_serde::{Serialize, Deserialize};
//     use serde_json;
// 
//     #[derive(Serialize,Deserialize)]
//     struct TestStruct {
//         name: String,
//         value: BigDecimal,
//         #[serde(with = "crate::serde::json_num")]
//         number: BigDecimal,
//     }
// 
// 
//     #[test]
//     fn test_struct_parsing() {
//         let json_src = r#"
//                 { "name": "foo", "value": 0.0008741329382918, "number": "12.34" }
//             "#;
//         let my_struct: TestStruct = serde_json::from_str(&json_src).unwrap();
//         assert_eq!(&my_struct.name, "foo");
//         assert_eq!(&my_struct.value, &"0.0008741329382918".parse().unwrap());
//         assert_eq!(&my_struct.number, &"12.34".parse().unwrap());
// 
//         let s = serde_json::to_string(&my_struct).unwrap();
//         assert_eq!(s, r#"{"name":"foo","value":"0.0008741329382918","number":12.34}"#);
//     }
// 
// }