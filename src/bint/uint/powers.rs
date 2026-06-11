use crate::bint::{
    intrinsics::{
        _can_scaled_by_power_of_ten_64, _decimal_digits_64, _remaining_decimal_digits_64, ExpType,
    },
    uint::{
        intrinsics::Intrinsics,
        math::utils::{as_u128_ref, as_u64_ref},
    },
    UInt,
};

type U<const N: usize> = UInt<N>;

#[inline(always)]
pub const fn decimal_digits<const N: usize>(n: &UInt<N>) -> ExpType {
    match N {
        1 => _decimal_digits_64(as_u64_ref(n)),
        2 => as_u128_ref(n).decimal_digits(),
        _ => decimal_digits_long(n),
    }
}

#[inline(always)]
pub const fn remaining_decimal_digits<const N: usize>(n: &UInt<N>) -> ExpType {
    match N {
        1 => _remaining_decimal_digits_64(as_u64_ref(n)),
        2 => as_u128_ref(n).remaining_decimal_digits(),
        _ => remaining_decimal_digits_long(n),
    }
}

#[inline(always)]
const fn decimal_digits_long<const N: usize>(n: &UInt<N>) -> ExpType {
    if n.is_zero() {
        return 0;
    }

    n.ilog10() + 1
}

#[inline]
const fn remaining_decimal_digits_long<const N: usize>(n: &UInt<N>) -> ExpType {
    let dd = decimal_digits(n);

    let mut max_digits = Intrinsics::<N>::MAX_POWER_OF_TEN + 1;

    // 64:  18446744073709551615
    // 128: 340282366920938463463374607431768211455
    // 256: 115792089237316195423570985008687907853269984665640564039457584007913129639935

    if dd != 0 && Intrinsics::<N>::unchecked_max_reduced_by_power_of_ten(max_digits - dd).lt(n) {
        max_digits -= 1;
    }

    max_digits - dd
}

#[inline(always)]
pub const fn unchecked_power_of_ten<const N: usize>(power: ExpType) -> U<N> {
    Intrinsics::<N>::unchecked_power_of_ten(power)
}

#[inline(always)]
pub const fn unchecked_power_of_five<const N: usize>(power: ExpType) -> U<N> {
    Intrinsics::<N>::unchecked_power_of_five(power)
}

#[inline(always)]
pub const fn checked_power_of_ten<const N: usize>(power: ExpType) -> Option<U<N>> {
    Intrinsics::<N>::checked_power_of_ten(power)
}

#[inline(always)]
pub const fn checked_power_of_five<const N: usize>(power: ExpType) -> Option<U<N>> {
    Intrinsics::<N>::checked_power_of_five(power)
}

#[inline(always)]
pub const fn can_scaled_by_power_of_ten<const N: usize>(n: &U<N>, power: ExpType) -> bool {
    match N {
        1 => _can_scaled_by_power_of_ten_64(as_u64_ref(n), power),
        2 => as_u128_ref(n).can_scaled_by_power_of_ten(power),
        _ => n.le(&Intrinsics::<N>::unchecked_max_reduced_by_power_of_ten(
            power,
        )),
    }
}

#[cfg(debug_assertions)]
mod __asserts {
    use crate::utils::const_assert;

    const_assert!(u64!(0).decimal_digits() == 0);
    const_assert!(u64!(1).decimal_digits() == 1);
    const_assert!(u64!(18446744073709551615).decimal_digits() == 20);

    const_assert!(u64!(0).remaining_decimal_digits() == 20);
    const_assert!(u64!(1).remaining_decimal_digits() == 19);
    const_assert!(u64!(10).remaining_decimal_digits() == 18);

    const_assert!(u64!(18).remaining_decimal_digits() == 18);
    const_assert!(u64!(19).remaining_decimal_digits() == 17);

    const_assert!(u64!(18446744073709551615).remaining_decimal_digits() == 0);
    const_assert!(u64!(1844674407370955161).remaining_decimal_digits() == 1);
    const_assert!(u64!(2844674407370955161).remaining_decimal_digits() == 0);

    const_assert!(u64!(24576).remaining_decimal_digits() == 14);
    const_assert!(u64!(14576).remaining_decimal_digits() == 15);

    const_assert!(u256!(115).remaining_decimal_digits() == 75);
    const_assert!(u256!(116).remaining_decimal_digits() == 74);

    const_assert!(u64!(1844674407370955161).can_scaled_by_power_of_ten(1));
    const_assert!(!u64!(1844674407370955162).can_scaled_by_power_of_ten(1));

    const_assert!(u128!(34028236692093846346337460743176821145).can_scaled_by_power_of_ten(1));
    const_assert!(!u128!(3402823669209384634633746074317682115).can_scaled_by_power_of_ten(2));

    const_assert!(u256!(
        11579208923731619542357098500868790785326998466564056403945758400791312963993
    )
    .can_scaled_by_power_of_ten(1));
    const_assert!(!u256!(
        11679208923731619542357098500868790785326998466564056403945758400791312963993
    )
    .can_scaled_by_power_of_ten(1));
}
