use num_traits::{Num, Zero};
use rstest::*;
use std::str::FromStr;

use fastnum::{decimal, decimal::Decimal};

#[rstest]
#[case(decimal!(0), decimal!(1), decimal!(0))]
#[case(decimal!(0), decimal!(10), decimal!(0))]
#[case(decimal!(2), decimal!(1), decimal!(2))]
#[case(decimal!(2e1), decimal!(1), decimal!(2e1))]
#[case(decimal!(10), decimal!(10), decimal!(1))]
#[case(decimal!(100), decimal!(10.0), decimal!(1e1))]
#[case(decimal!(20.0), decimal!(200), decimal!(0.1))]
#[case(decimal!(4), decimal!(2), decimal!(2.0))]
#[case(decimal!(15), decimal!(3), decimal!(5.0))]
#[case(decimal!(1), decimal!(2), decimal!(0.5))]
#[case(decimal!(1), decimal!(2e-2), decimal!(5e1))]
#[case(decimal!(1), decimal!(0.2), decimal!(5))]
#[case(decimal!(1.0), decimal!(0.02), decimal!(50))]
#[case(decimal!(1), decimal!(0.020), decimal!(5e1))]
#[case(decimal!(5.0), decimal!(4.00), decimal!(1.25))]
#[case(decimal!(5.0), decimal!(4.000), decimal!(1.25))]
#[case(decimal!(5), decimal!(4.000), decimal!(1.25))]
#[case(decimal!(5), decimal!(4), decimal!(125e-2))]
#[case(decimal!(100), decimal!(5), decimal!(20))]
#[case(decimal!(-50), decimal!(5), decimal!(-10))]
#[case(decimal!(200), decimal!(-5), decimal!(-40.))]
#[case(decimal!(1), decimal!(3), decimal!(0.3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333))]
#[case(decimal!(-2), decimal!(-3), decimal!(0.6666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666667))]
#[case(decimal!(-12.34), decimal!(1.233), decimal!(-10.00811030008110300081103000811030008110300081103000811030008110300081103000811030008110300081103001))]
#[case(decimal!(125348), decimal!(352.2283), decimal!(355.8714617763535752237966114591019517738921035021887792661748076460636467881768727839301952739175132))]
fn test_div(#[case] a: Decimal, #[case] b: Decimal, #[case] expected: Decimal) {
    assert_eq!(a / b, expected);
    assert_eq!(a / &b, expected);
    assert_eq!(&a / b, expected);
    assert_eq!(&a / &b, expected);

    let mut q = a;
    q /= b;
    assert_eq!(q, expected);
}

#[rstest]
#[should_panic(expected = "Division by zero")]
fn test_division_by_zero_int_panics() {
    let x = decimal!(3.14);
    let _r = x / 0;
}

#[rstest]
#[should_panic(expected = "Division by zero")]
fn test_division_by_zero_float_panics() {
    let x = decimal!(3.14);
    let _r = x / 0.;
}

#[rstest]
#[should_panic(expected = "Division by zero")]
fn test_division_by_zero_panics() {
    let x = decimal!(3.14);
    let _r = x / Decimal::zero();
}

#[rstest]
fn test_division_by_large_number() {
    let n = 1u8;
    let d = decimal!(79437738588056219546528239237352667078);

    let quotient_n_ref_d = n / &d;
    let quotient_n_d = n / d;
    assert_eq!(quotient_n_ref_d, quotient_n_d);
    assert_eq!(quotient_n_ref_d, decimal!(1.258847517281104957975270408416632052090243053529147458917576143852500316808428812104171430669001064E-38));
}
