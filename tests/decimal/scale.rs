use rstest::*;

use fastnum::{decimal, decimal::Decimal};

#[rstest]
fn test_scale() {
    assert_eq!(decimal!(1), Decimal::from_scale(0));
    assert_eq!(decimal!(0.001), Decimal::from_scale(-3));
    assert_eq!(decimal!(1), Decimal::from_scale(-0));
    assert_eq!(decimal!(1), Decimal::from_scale(0));
    assert_eq!(decimal!(1000).normalized(), Decimal::from_scale(3));
}