use fastnum::*;

fn main() {
    assert_eq!(format!("{:?}", D128::from(1)), "1".to_string());
    assert_eq!(format!("{:?}", D128::parse_str("0.002", decimal::Context::default())), "0.002".to_string());
}