use core::cmp::Ordering::{Equal, Greater, Less};

use crate::decimal::{
    RoundingMode::{self, *},
    Sign,
};

// TODO: refactor
pub(crate) fn round_pair_digits(
    pair: (u8, u8),
    sign: Sign,
    rounding_mode: RoundingMode,
    trailing_zeros: bool,
) -> u8 {
    let (lhs, rhs) = pair;
    // if all zero after digit, never round
    if rhs == 0 && trailing_zeros {
        return lhs;
    }
    let up = lhs + 1;
    let down = lhs;
    match (rounding_mode, rhs.cmp(&5)) {
        (No, _) => down,
        (Up, _) => up,
        (Down, _) => down,
        (Floor, _) => {
            if sign == Sign::Minus {
                up
            } else {
                down
            }
        }
        (Ceiling, _) => {
            if sign == Sign::Minus {
                down
            } else {
                up
            }
        }
        (_, Less) => down,
        (_, Greater) => up,
        (_, Equal) if !trailing_zeros => up,
        (HalfUp, Equal) => up,
        (HalfDown, Equal) => down,
        (HalfEven, Equal) => {
            if lhs % 2 == 0 {
                down
            } else {
                up
            }
        }
    }
}
