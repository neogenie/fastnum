use crate::bint::intrinsics::{_ilog10_64, Digit, ExpType, DIGIT_POWER_10};

pub const DIGIT_POWERS_10: [Digit; 20] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    10000000000000000000,
];

macro_rules! make_max_reduced_by_powers_10 {
    ($name: ident, $ty: ident, $len: literal) => {
        const fn $name(powers: &[$ty; $len]) -> [$ty; $len] {
            let mut i = 0;
            let mut result = [0; $len];
            while i < powers.len() {
                result[i] = $ty::MAX / powers[i];
                i += 1;
            }

            result
        }
    };
}

make_max_reduced_by_powers_10!(make_max_reduced_by_powers_10_64, u64, 20);

const DIGIT_MAX_REDUCED_BY_POWERS_10: [Digit; 20] =
    make_max_reduced_by_powers_10_64(&DIGIT_POWERS_10);

#[inline(always)]
pub const fn _decimal_digits_64(n: u64) -> ExpType {
    if n == 0 {
        0
    } else {
        _ilog10_64(n) + 1
    }
}

#[inline(always)]
pub const fn _remaining_decimal_digits_64(n: u64) -> ExpType {
    let dd = _decimal_digits_64(n);

    let mut max_digits = DIGIT_POWER_10 + 1;

    if dd != 0 && DIGIT_MAX_REDUCED_BY_POWERS_10[(max_digits - dd) as usize] < n {
        max_digits -= 1;
    }

    max_digits - dd
}

#[inline(always)]
pub const fn _can_scaled_by_power_of_ten_64(n: u64, power: ExpType) -> bool {
    n <= DIGIT_MAX_REDUCED_BY_POWERS_10[power as usize]
}
