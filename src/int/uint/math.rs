mod ilog;
mod mul;

pub use ilog::ilog10;
pub use mul::{overflowing_mul10, strict_mul10};

use core::cmp::Ordering;

use crate::int::{uint::intrinsics::*, UInt};

macro_rules! to_int {
    { $($name: ident -> $int: ty), * }  => {
        $(
            #[allow(dead_code)]
            #[inline]
            pub const fn $name<const N: usize>(this: UInt<N>) -> Option<$int> {
                let digits = this.digits();
                let mut out = 0;
                let mut i = 0;
                if BITS > <$int>::BITS {
                    let small = digits[i] as $int;
                    let trunc = small as Digit;
                    if digits[i] != trunc {
                        return None;
                    }
                    out = small;
                    i = 1;
                } else {
                    loop {
                        let shift = i << BIT_SHIFT;
                        if i >= N || shift >= <$int>::BITS as usize {
                            break;
                        }
                        out |= digits[i] as $int << shift;
                        i += 1;
                    }
                }

                #[allow(unused_comparisons)]
                if out < 0 {
                    return None;
                }

                while i < N {
                    if digits[i] != 0 {
                        return None;
                    }
                    i += 1;
                }

                Some(out)
            }
        )*
    };
}

to_int! {
    to_u8 -> u8,
    to_u16 -> u16,
    to_u32 -> u32,
    to_u64 -> u64,
    to_u128 -> u128,
    to_usize -> usize,

    to_i8 -> i8,
    to_i16 -> i16,
    to_i32 -> i32,
    to_i64 -> i64,
    to_i128 -> i128,
    to_isize -> isize
}

// This Hell is here because of the div_rem methods are not public in the bnum.

#[inline]
pub const fn div_rem<const N: usize>(dividend: UInt<N>, divisor: UInt<N>) -> (UInt<N>, UInt<N>) {
    match dividend.cmp(&divisor) {
        Ordering::Less => (UInt::<N>::ZERO, dividend),
        Ordering::Equal => (UInt::<N>::ONE, UInt::<N>::ZERO),
        Ordering::Greater => {
            let ldi = last_digit_index(divisor.digits());
            if ldi == 0 {
                let digits = divisor.digits();
                let (div, rem) = div_rem_digit(dividend, digits[0]);
                (div, UInt::<N>::from_digit(rem))
            } else {
                let (div, rem) =
                    basecase_div_rem(*(dividend.digits()), *(divisor.digits()), ldi + 1);
                (UInt::<N>::from_digits(div), UInt::<N>::from_digits(rem))
            }
        }
    }
}

#[inline]
pub const fn div_rem_wide(low: Digit, high: Digit, rhs: Digit) -> (Digit, Digit) {
    let a = to_double_digit(low, high);
    (
        (a / rhs as DoubleDigit) as Digit,
        (a % rhs as DoubleDigit) as Digit,
    )
}

#[inline]
pub const fn div_rem_digit<const N: usize>(value: UInt<N>, rhs: Digit) -> (UInt<N>, Digit) {
    let mut out = [0; N];

    let mut rem: Digit = 0;
    let mut i = N;

    let digits = value.digits();

    while i > 0 {
        i -= 1;
        let (q, r) = div_rem_wide(digits[i], rem, rhs);
        rem = r;
        out[i] = q;
    }
    (UInt::from_digits(out), rem)
}

const fn last_digit_index<const N: usize>(digits: &Digits<N>) -> usize {
    let mut index = 0;
    let mut i = 1;

    while i < N {
        if digits[i] != 0 {
            index = i;
        }
        i += 1;
    }
    index
}

const fn basecase_div_rem<const N: usize>(
    digits: Digits<N>,
    mut v: Digits<N>,
    n: usize,
) -> (Digits<N>, Digits<N>) {
    let mut q = [0; N];
    let m = last_digit_index(&digits) + 1 - n;

    let shift = v[n - 1].leading_zeros() as ExpType;

    v = unchecked_shl_internal(v, shift); // D1

    let v_n_m1 = v[n - 1];
    let v_n_m2 = v[n - 2];

    let mut u = Remainder::new(digits, shift);

    let mut j = m + 1; // D2
    while j > 0 {
        j -= 1; // D7

        let u_jn = u.digit(j + n);

        // q_hat will be either `q` or `q + 1`
        let mut q_hat = if u_jn < v_n_m1 {
            let (mut q_hat, r_hat) = div_rem_wide(u.digit(j + n - 1), u_jn, v_n_m1); // D3

            if tuple_gt(
                widening_mul(q_hat, v_n_m2),
                (u.digit(j + n - 2), r_hat as Digit),
            ) {
                q_hat -= 1;

                if let Some(r_hat) = r_hat.checked_add(v_n_m1) {
                    // this checks if `r_hat <= b`, where `b` is the digit base
                    if tuple_gt(
                        widening_mul(q_hat, v_n_m2),
                        (u.digit(j + n - 2), r_hat as Digit),
                    ) {
                        q_hat -= 1;
                    }
                }
            }
            q_hat
        } else {
            // `u[j + n - 1] >= v[n - 1]` so we know that estimate for q_hat would be larger
            // than `Digit::MAX`. This is either equal to `q` or `q + 1` (very unlikely to
            // be `q + 1`).
            Digit::MAX
        };
        let (u_new, overflow) = u.sub(Mul::new(v, q_hat), j, n); // D4
        u = u_new;

        if overflow {
            // D5 - unlikely, probability of this being true is ~ 2 / b where b is the digit
            // base (i.e. `Digit::MAX + 1`)
            q_hat -= 1;
            u = u.add(v, j, n);
        }
        q[j] = q_hat;
    }
    (q, u.shr(shift))
}

#[inline]
const fn carrying_add(a: Digit, b: Digit, carry: bool) -> (Digit, bool) {
    let (s1, o1) = a.overflowing_add(b);
    if carry {
        let (s2, o2) = s1.overflowing_add(1);
        (s2, o1 || o2)
    } else {
        (s1, o1)
    }
}

#[inline]
const fn carrying_mul<const N: usize>(
    a: Digit,
    b: Digit,
    carry: Digit,
    current: Digit,
) -> (Digit, Digit) {
    let prod =
        carry as DoubleDigit + current as DoubleDigit + (a as DoubleDigit) * (b as DoubleDigit);
    (prod as Digit, (prod >> BITS) as Digit)
}

#[inline]
const fn borrowing_sub(a: Digit, b: Digit, borrow: bool) -> (Digit, bool) {
    let (s1, o1) = a.overflowing_sub(b);
    if borrow {
        let (s2, o2) = s1.overflowing_sub(1);
        (s2, o1 || o2)
    } else {
        (s1, o1)
    }
}

#[inline]
const fn widening_mul(a: Digit, b: Digit) -> (Digit, Digit) {
    let prod = a as DoubleDigit * b as DoubleDigit;
    (prod as Digit, (prod >> BITS) as Digit)
}

#[inline]
const fn to_double_digit(low: Digit, high: Digit) -> DoubleDigit {
    ((high as DoubleDigit) << BITS) | low as DoubleDigit
}

#[inline]
const fn tuple_gt(a: (Digit, Digit), b: (Digit, Digit)) -> bool {
    a.1 > b.1 || a.1 == b.1 && a.0 > b.0
}

struct Remainder<const N: usize> {
    first: Digit,
    rest: Digits<N>,
}

impl<const N: usize> Remainder<N> {
    const fn digit(&self, index: usize) -> Digit {
        if index == 0 {
            self.first
        } else {
            self.rest[index - 1]
        }
    }

    const fn shr(self, shift: ExpType) -> Digits<N> {
        let mut out = [0; N];

        let mut i = 0;
        while i < N {
            out[i] = self.digit(i) >> shift;
            i += 1;
        }
        if shift > 0 {
            i = 0;
            while i < N {
                out[i] |= self.rest[i] << (BITS - shift);
                i += 1;
            }
        }

        out
    }

    const fn new(digits: Digits<N>, shift: ExpType) -> Self {
        let first = digits[0] << shift;
        let rest = wrapping_shr(digits, BITS - shift);
        Self { first, rest }
    }

    const fn sub(mut self, rhs: Mul<N>, start: usize, range: usize) -> (Self, bool) {
        let mut borrow = false;
        let mut i = 0;
        while i <= range {
            let (sub, overflow) = borrowing_sub(self.digit(i + start), rhs.digit(i), borrow);
            if start == 0 && i == 0 {
                self.first = sub;
            } else {
                self.rest[i + start - 1] = sub;
            }
            borrow = overflow;
            i += 1;
        }
        (self, borrow)
    }

    const fn add(mut self, rhs: Digits<N>, start: usize, range: usize) -> Self {
        let mut carry = false;
        let mut i = 0;
        while i < range {
            let (sum, overflow) = carrying_add(self.digit(i + start), rhs[i], carry);
            if start == 0 && i == 0 {
                self.first = sum;
            } else {
                self.rest[i + start - 1] = sum;
            }
            carry = overflow;
            i += 1;
        }
        if carry {
            if start == 0 && range == 0 {
                self.first = self.first.wrapping_add(1);
            } else {
                self.rest[range + start - 1] = self.rest[range + start - 1].wrapping_add(1);
            }
        }
        self
    }
}

#[derive(Clone, Copy)]
struct Mul<const N: usize> {
    last: Digit,
    rest: Digits<N>,
}

impl<const N: usize> Mul<N> {
    const fn new(digits: Digits<N>, rhs: Digit) -> Self {
        let mut rest = [0; N];

        let mut carry: Digit = 0;
        let mut i = 0;
        while i < N {
            let (prod, c) = carrying_mul::<N>(digits[i], rhs, carry, 0);
            carry = c;
            rest[i] = prod;
            i += 1;
        }
        Self { last: carry, rest }
    }

    const fn digit(&self, index: usize) -> Digit {
        if index == N {
            self.last
        } else {
            self.rest[index]
        }
    }
}

#[inline]
const fn wrapping_shr<const N: usize>(digits: Digits<N>, rhs: ExpType) -> Digits<N> {
    overflowing_shr(digits, rhs).0
}

#[inline]
const fn overflowing_shr<const N: usize>(digits: Digits<N>, rhs: ExpType) -> (Digits<N>, bool) {
    if rhs >= UInt::<N>::BITS {
        (
            unchecked_shr_internal(digits, rhs & (UInt::<N>::BITS - 1)),
            true,
        )
    } else {
        (unchecked_shr_internal(digits, rhs), false)
    }
}

#[inline]
const fn unchecked_shl_internal<const N: usize>(digits: Digits<N>, rhs: ExpType) -> Digits<N> {
    let mut out = [0; N];

    let digit_shift = (rhs >> BIT_SHIFT) as usize;
    let bit_shift = rhs & BITS_MINUS_1;

    if bit_shift != 0 {
        let carry_shift = BITS - bit_shift;
        let mut carry = 0;

        let mut i = digit_shift;
        while i < N {
            let current_digit = digits[i - digit_shift];
            out[i] = (current_digit << bit_shift) | carry;
            carry = current_digit >> carry_shift;
            i += 1;
        }
    } else {
        let mut i = digit_shift;
        while i < N {
            // we start i at digit_shift, not 0, since the compiler can elide bounds checks
            // when i < N
            out[i] = digits[i - digit_shift];
            i += 1;
        }
    }

    out
}

#[inline]
const fn unchecked_shr_pad_internal<const N: usize, const NEG: bool>(
    digits: Digits<N>,
    rhs: ExpType,
) -> Digits<N> {
    let mut out = if NEG { [Digit::MAX; N] } else { [0; N] };

    let digit_shift = (rhs >> BIT_SHIFT) as usize;
    let bit_shift = rhs & BITS_MINUS_1;

    let num_copies = N.saturating_sub(digit_shift);

    if bit_shift != 0 {
        let carry_shift = BITS - bit_shift;
        let mut carry = 0;

        let mut i = digit_shift;
        while i < N {
            // we use an increment while loop because the compiler can elide the array
            // bounds check, which results in big performance gains
            let index = N - 1 - i;
            let current_digit = digits[index + digit_shift];
            out[index] = (current_digit >> bit_shift) | carry;
            carry = current_digit << carry_shift;
            i += 1;
        }

        if NEG {
            out[num_copies - 1] |= Digit::MAX << carry_shift;
        }
    } else {
        let mut i = digit_shift;
        while i < N {
            // we start i at digit_shift, not 0, since the compiler can elide bounds checks
            // when i < N
            out[i - digit_shift] = digits[i];
            i += 1;
        }
    }

    out
}

const fn unchecked_shr_internal<const N: usize>(digits: Digits<N>, rhs: ExpType) -> Digits<N> {
    unchecked_shr_pad_internal::<N, false>(digits, rhs)
}
