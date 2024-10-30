use rstest::*;
use std::str::FromStr;

use fastnum::{u256, U256};

#[rstest]
fn test_max_u256() {
    const X: U256 =
        u256!(115792089237316195423570985008687907853269984665640564039457584007913129639935);
    assert_eq!(X, U256::MAX);
}

#[rstest]
fn test_max_u256_hex() {
    let x = u256!(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff);
    assert_eq!(x, U256::MAX);
}

#[rstest]
#[should_panic(expected = "attempt to parse integer from string containing invalid digit")]
fn test_u256_negative() {
    let _ = U256::from_str("-1").unwrap();
}



