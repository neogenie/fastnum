use rstest::*;

use fastnum::{D256, dec256};

#[rstest]
#[trace]
#[case(0.0, dec256!(0))]
fn test_from_ok(#[case] n: f64, #[case] expected: D256) {
    let d = D256::try_from(n).unwrap();
    assert_eq!(d, expected);

    let n = f64::from_bits(n.to_bits() | (1<<63));
    let d = D256::try_from(n).unwrap();
    assert_eq!(d, expected.negative());
}
