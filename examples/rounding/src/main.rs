use fastnum::{decimal::RoundingMode, udec128, UD128};

fn main() {
    const THREE_ZEROES: UD128 = udec128!(1.0001).with_rounding_mode(RoundingMode::Ceiling);
    const FOUR_ZEROES: UD128 = udec128!(1.00001).with_rounding_mode(RoundingMode::Ceiling);
    const FIVE_ZEROES: UD128 = udec128!(1.000001).with_rounding_mode(RoundingMode::Ceiling);
    
    assert_eq!(THREE_ZEROES.round(0), udec128!(1));
    assert_eq!(THREE_ZEROES.round(1), udec128!(1));
    assert_eq!(THREE_ZEROES.round(2), udec128!(1));
    assert_eq!(THREE_ZEROES.round(3), udec128!(1.001));
    
    assert_eq!(FOUR_ZEROES.round(0), udec128!(1));
    assert_eq!(FOUR_ZEROES.round(1), udec128!(1));
    assert_eq!(FOUR_ZEROES.round(2), udec128!(1));
    assert_eq!(FOUR_ZEROES.round(3), udec128!(1));
    assert_eq!(FOUR_ZEROES.round(4), udec128!(1.0001));
    
    assert_eq!(FIVE_ZEROES.round(0), udec128!(1));
    assert_eq!(FIVE_ZEROES.round(1), udec128!(1));
    assert_eq!(FIVE_ZEROES.round(2), udec128!(1));
    assert_eq!(FIVE_ZEROES.round(3), udec128!(1));
    assert_eq!(FIVE_ZEROES.round(4), udec128!(1));
    assert_eq!(FIVE_ZEROES.round(5), udec128!(1.00001));
}
