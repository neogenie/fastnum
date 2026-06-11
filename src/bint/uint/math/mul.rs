use crate::bint::{
    intrinsics::{_carrying_mul_64, _widening_mul_64, Digit},
    uint::math::utils::{as_u128, as_u64, uint128, uint64, uint_pair64},
    UInt,
};

#[inline(always)]
pub const fn overflowing_mul<const N: usize>(lhs: UInt<N>, rhs: UInt<N>) -> (UInt<N>, bool) {
    match N {
        1 => {
            let (r, overflow) = as_u64(lhs).overflowing_mul(as_u64(rhs));
            (uint64(r), overflow)
        }
        2 => {
            let (r, overflow) = as_u128(lhs).overflowing_mul(as_u128(rhs));
            (uint128(r), overflow)
        }
        _ => mul_long(lhs, rhs),
    }
}

#[inline(always)]
#[allow(unsafe_code)]
pub const unsafe fn unchecked_mul<const N: usize>(value: UInt<N>, rhs: UInt<N>) -> UInt<N> {
    match N {
        1 => uint64(as_u64(value).unchecked_mul(as_u64(rhs))),
        2 => uint128(as_u128(value).unchecked_mul(as_u128(rhs))),
        // TODO
        _ => mul_long(value, rhs).0,
    }
}

#[inline(always)]
pub const fn widening_mul<const N: usize>(lhs: UInt<N>, rhs: UInt<N>) -> (UInt<N>, UInt<N>) {
    match N {
        1 => uint_pair64(_widening_mul_64(as_u64(lhs), as_u64(rhs))),
        2 => {
            let (low, high) = as_u128(lhs).widening_mul(as_u128(rhs));
            (uint128(low), uint128(high))
        }
        _ => widening_mul_long(lhs, rhs),
    }
}

#[inline(always)]
#[allow(unsafe_code)]
pub const unsafe fn unchecked_mul_digit<const N: usize>(value: UInt<N>, rhs: Digit) -> UInt<N> {
    match N {
        1 => uint64(as_u64(value).unchecked_mul(rhs)),
        2 => uint128(as_u128(value).unchecked_mul_u64(rhs)),
        // TODO
        _ => mul_digit_long(value, rhs).0,
    }
}

#[inline(always)]
pub const fn overflowing_mul_digit<const N: usize>(value: UInt<N>, rhs: Digit) -> (UInt<N>, bool) {
    match N {
        1 => {
            let (r, carry) = as_u64(value).overflowing_mul(rhs);
            (uint64(r), carry)
        }
        2 => {
            let (r, carry) = as_u128(value).overflowing_mul_u64(rhs);
            (uint128(r), carry)
        }
        _ => mul_digit_long(value, rhs),
    }
}

#[inline(always)]
const fn mul_digit_long<const N: usize>(value: UInt<N>, rhs: Digit) -> (UInt<N>, bool) {
    let mut out = [0; N];
    let mut carry = 0;

    let mut i = 0;

    let digits = value.digits();

    while i < N {
        let (low, high) = _carrying_mul_64(digits[i], rhs, carry);
        out[i] = low;
        carry = high;
        i += 1;
    }

    (UInt::from_digits(out), carry != 0)
}

#[inline(always)]
const fn widening_mul_long<const N: usize>(lhs: UInt<N>, rhs: UInt<N>) -> (UInt<N>, UInt<N>) {
    let (low, high) = lhs.0.widening_mul(rhs.0);
    (UInt(low), UInt(high))
}

#[inline(always)]
const fn mul_long<const N: usize>(lhs: UInt<N>, rhs: UInt<N>) -> (UInt<N>, bool) {
    let (r, overflow) = lhs.0.overflowing_mul(rhs.0);
    (UInt(r), overflow)
}
