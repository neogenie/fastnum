use rstest::*;

use fastnum::{udec256, UD256};

use crate::decimal::common::from::f64::{test_impl, test_impl_unsigned};

test_impl!(udec256, UD256);
test_impl_unsigned!(udec256, UD256);

#[rstest(::trace)]
#[case(0.1, udec256!(0.1000000000000000055511151231257827021181583404541015625))]
#[case(0.001, udec256!(0.001000000000000000020816681711721685132943093776702880859375))]
#[case(12.34, udec256!(12.339999999999999857891452847979962825775146484375))]
#[case(0.333333333333333333333333333333, udec256!(0.333333333333333314829616256247390992939472198486328125))]
#[case(1.0 / 3.0, udec256!(0.333333333333333314829616256247390992939472198486328125))]
#[case(core::f64::consts::PI, udec256!(3.141592653589793115997963468544185161590576171875))]
#[case(core::f64::consts::E, udec256!(2.718281828459045090795598298427648842334747314453125))]
#[case(core::f64::consts::PI * 10000.0, udec256!(31415.926535897931898944079875946044921875))]
#[case(core::f64::consts::PI * 30000.0, udec256!(94247.779607693795696832239627838134765625))]
#[case(3.0000000000000004, udec256!(3.000000000000000444089209850062616169452667236328125))]
#[case(0.07155292, udec256!(0.07155292000000000596227067717336467467248439788818359375))]
#[case(21509.2, udec256!(21509.20000000000072759576141834259033203125))]
#[case(2.3283064e-10, udec256!(2.328306399999999934987650668772826180463741962967105791904032230377197265625e-10))]
#[case(0.14693861798803098, udec256!(0.146938617988030983951830421574413776397705078125))]
#[case(6.99999952316, udec256!(6.9999995231599996259319595992565155029296875))]
fn test_from_f64_ok_256(#[case] n: f64, #[case] expected: UD256) {
    let d = UD256::try_from(n).unwrap();
    assert_eq!(d, expected);

    let n = f64::from_bits(n.to_bits() | (1 << 63));
    let r = UD256::try_from(n);
    assert!(r.is_err());
}

// #[rstest(::trace)]
// #[should_panic(expected = "(fastnum) number too large to fit in target type")]
// fn test_from_f64_overflow_256(#[case] n: f64) {
//     let _ = UD256::try_from(n).unwrap();
// }