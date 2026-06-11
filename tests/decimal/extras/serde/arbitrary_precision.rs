use fastnum::*;
use rstest::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Arbitrary {
    value: D128,

    #[serde(with = "fastnum::decimal::serde::dec::arbitrary_precision")]
    arb_value: D128,

    #[serde(serialize_with = "fastnum::decimal::serde::dec::arbitrary_precision::serialize")]
    arb_ser_value: D128,

    #[serde(deserialize_with = "fastnum::decimal::serde::dec::arbitrary_precision::deserialize")]
    arb_deser_value: D128,
}

#[rstest(::trace)]
fn test_serialize() {
    let arb = Arbitrary {
        value: dec128!(1.09861228866810969139524523692252570465),
        arb_value: dec128!(1.09861228866810969139524523692252570465),
        arb_ser_value: dec128!(1.09861228866810969139524523692252570465),
        arb_deser_value: dec128!(1.09861228866810969139524523692252570465),
    };

    let expected = r#"{"value":"1.09861228866810969139524523692252570465","arb_value":1.09861228866810969139524523692252570465,"arb_ser_value":1.09861228866810969139524523692252570465,"arb_deser_value":"1.09861228866810969139524523692252570465"}"#;
    let actual = serde_json::to_string(&arb).unwrap();
    assert_eq!(expected, actual);
}

#[rstest(::trace)]
#[case(r#"{"value": "1.09861228866810969139524523692252570465", "arb_value": "1.09861228866810969139524523692252570465", "arb_ser_value": "1.09861228866810969139524523692252570465", "arb_deser_value": "1.09861228866810969139524523692252570465"}"#)]
#[case(r#"{"value": "1.09861228866810969139524523692252570465", "arb_value": 1.09861228866810969139524523692252570465, "arb_ser_value": "1.09861228866810969139524523692252570465", "arb_deser_value": "1.09861228866810969139524523692252570465"}"#)]
#[case(r#"{"value": "1.09861228866810969139524523692252570465", "arb_value": 1.09861228866810969139524523692252570465, "arb_ser_value": "1.09861228866810969139524523692252570465", "arb_deser_value": 1.09861228866810969139524523692252570465}"#)]
fn test_deserialize(#[case] json: &'static str) {
    let actual: Arbitrary = serde_json::from_str(json).unwrap();

    let expected = Arbitrary {
        value: dec128!(1.09861228866810969139524523692252570465),
        arb_value: dec128!(1.09861228866810969139524523692252570465),
        arb_ser_value: dec128!(1.09861228866810969139524523692252570465),
        arb_deser_value: dec128!(1.09861228866810969139524523692252570465),
    };

    assert_eq!(expected, actual);
}

#[rstest(::trace)]
#[case(r#"{"value": 1.09861228866810969139524523692252570465, "arb_value": 1.09861228866810969139524523692252570465, "arb_ser_value": 1.09861228866810969139524523692252570465, "arb_deser_value": 1.09861228866810969139524523692252570465}"#)]
#[case(r#"{"value": "1.09861228866810969139524523692252570465", "arb_value": 1.09861228866810969139524523692252570465, "arb_ser_value": 1.09861228866810969139524523692252570465, "arb_deser_value": 1.09861228866810969139524523692252570465}"#)]
#[case(r#"{"value": "1.09861228866810969139524523692252570465", "arb_value": "1.09861228866810969139524523692252570465", "arb_ser_value": 1.09861228866810969139524523692252570465, "arb_deser_value": 1.09861228866810969139524523692252570465}"#)]
#[case(r#"{"value": "1.09861228866810969139524523692252570465", "arb_value": "1.09861228866810969139524523692252570465", "arb_ser_value": 1.09861228866810969139524523692252570465, "arb_deser_value": "1.09861228866810969139524523692252570465"}"#)]
fn test_should_panic(#[case] json: &'static str) {
    let res: Result<Arbitrary, _> = serde_json::from_str(json);
    assert!(res.is_err());
}
