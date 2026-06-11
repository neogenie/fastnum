mod powers;

use crate::{
    bint::intrinsics::{_div_rem_128_64, _div_rem_64, _widening_mul_64, ExpType},
    utils::assert_eq_size,
};

pub const MAX_POWER_10: u32 = 38;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct _U128 {
    pub(crate) low: u64,
    pub(crate) high: u64,
}

assert_eq_size!(_U128, u128);

impl _U128 {
    const HALF_BITS: ExpType = 64;
    const BITS: ExpType = 128;

    pub const ZERO: Self = Self { low: 0, high: 0 };
    pub const ONE: Self = Self { low: 1, high: 0 };

    #[inline(always)]
    pub const fn is_zero(&self) -> bool {
        self.low == 0 && self.high == 0
    }

    #[inline(always)]
    pub const fn is_one(&self) -> bool {
        self.low == 1 && self.high == 0
    }

    #[inline(always)]
    pub const fn bits(&self) -> ExpType {
        Self::BITS - self.leading_zeros()
    }

    #[inline(always)]
    pub const fn eq(&self, other: &Self) -> bool {
        self.low == other.low && self.high == other.high
    }

    #[inline(always)]
    pub const fn lt(&self, other: &Self) -> bool {
        if self.high == other.high {
            self.low < other.low
        } else {
            self.high < other.high
        }
    }

    #[inline(always)]
    pub const fn le(&self, other: &Self) -> bool {
        if self.high == other.high {
            self.low <= other.low
        } else {
            self.high < other.high
        }
    }

    #[inline(always)]
    pub const fn ge(&self, other: &Self) -> bool {
        if self.high == other.high {
            self.low >= other.low
        } else {
            self.high > other.high
        }
    }

    #[inline(always)]
    pub const fn leading_zeros(&self) -> ExpType {
        if self.high == 0 {
            Self::HALF_BITS + self.low.leading_zeros()
        } else {
            self.high.leading_zeros()
        }
    }

    #[inline(always)]
    pub const fn decimal_digits(&self) -> ExpType {
        if self.is_zero() {
            0
        } else {
            self.ilog10() + 1
        }
    }

    #[inline(always)]
    pub const fn remaining_decimal_digits(&self) -> ExpType {
        let dd = self.decimal_digits();

        let mut max_digits = MAX_POWER_10 + 1;

        if dd != 0 && Self::MAX_REDUCED_BY_POWERS_10[(max_digits - dd) as usize].lt(self) {
            max_digits -= 1;
        }

        max_digits - dd
    }

    #[inline(always)]
    pub const fn can_scaled_by_power_of_ten(&self, power: ExpType) -> bool {
        self.le(&Self::MAX_REDUCED_BY_POWERS_10[power as usize])
    }

    #[inline(always)]
    pub const fn ilog10(&self) -> ExpType {
        let res = (self.bits() * 1233) >> 12;
        if self.lt(&Self::POWERS_10[res as usize]) {
            res.saturating_sub(1)
        } else {
            res
        }
    }

    #[inline(always)]
    pub const fn shl(self, shl: ExpType) -> Self {
        if shl == 0 {
            self
        } else if shl >= Self::BITS {
            Self { low: 0, high: 0 }
        } else if shl >= Self::HALF_BITS {
            Self {
                low: 0,
                high: self.low << (shl - Self::HALF_BITS),
            }
        } else {
            Self {
                low: self.low << shl,
                high: (self.high << shl) | (self.low >> (Self::HALF_BITS - shl)),
            }
        }
    }

    #[inline(always)]
    pub const fn shr(self, shr: ExpType) -> Self {
        if shr == 0 {
            self
        } else if shr >= Self::BITS {
            Self { low: 0, high: 0 }
        } else if shr >= Self::HALF_BITS {
            Self {
                low: self.high >> (shr - Self::HALF_BITS),
                high: 0,
            }
        } else {
            Self {
                low: (self.low >> shr) | (self.high << (Self::HALF_BITS - shr)),
                high: self.high >> shr,
            }
        }
    }

    #[inline(always)]
    #[allow(unsafe_code)]
    pub const unsafe fn unchecked_mul(self, b: Self) -> Self {
        let a = self.low as u128 | (self.high as u128) << 64;
        let b = b.low as u128 | (b.high as u128) << 64;
        let r = a.unchecked_mul(b);

        Self {
            low: r as u64,
            high: (r >> 64) as u64,
        }
    }

    #[inline(always)]
    pub const fn overflowing_mul(self, b: Self) -> (Self, bool) {
        let a = self.low as u128 | (self.high as u128) << 64;
        let b = b.low as u128 | (b.high as u128) << 64;
        let (r, overflow) = a.overflowing_mul(b);
        (
            Self {
                low: r as u64,
                high: (r >> 64) as u64,
            },
            overflow,
        )
    }

    #[inline(always)]
    pub const fn new(pair: (u64, u64)) -> Self {
        let (low, high) = pair;
        Self { low, high }
    }

    #[inline(always)]
    pub const fn add_assign(&mut self, other: Self) {
        let a = self.low as u128 | (self.high as u128) << 64;
        let b = other.low as u128 | (other.high as u128) << 64;
        let res = a + b;
        self.low = res as u64;
        self.high = (res >> 64) as u64;
    }

    #[inline(always)]
    pub const fn shr64(self) -> Self {
        Self {
            low: self.high,
            high: 0,
        }
    }

    #[inline(always)]
    pub const fn widening_mul(self, b: Self) -> (Self, Self) {
        let mut low = Self::new(_widening_mul_64(self.low, b.low));
        let mut t = low.shr64();
        low.high = 0;
        t.add_assign(Self::new(_widening_mul_64(self.high, b.low)));
        low.add_assign(Self {
            low: 0,
            high: t.low,
        });
        let mut high = t.shr64();
        t = low.shr64();
        low.high = 0;
        t.add_assign(Self::new(_widening_mul_64(self.low, b.high)));
        low.add_assign(Self {
            low: 0,
            high: t.low,
        });
        high.add_assign(t.shr64());
        high.add_assign(Self::new(_widening_mul_64(self.high, b.high)));

        (low, high)
    }

    #[inline(always)]
    pub const fn overflowing_mul_u64(self, b: u64) -> (Self, bool) {
        let (low, high) = _widening_mul_64(self.low, b);
        let (mid, overflow) = _widening_mul_64(self.high, b);

        let (mut sum, mut carry) = high.overflowing_add(mid);
        (sum, carry) = sum.overflowing_add(if carry { 1 } else { 0 });

        (Self { low, high: sum }, overflow != 0 || carry)
    }

    #[allow(unsafe_code)]
    #[inline(always)]
    pub const unsafe fn unchecked_mul_u64(self, b: u64) -> Self {
        let (low, high) = _widening_mul_64(self.low, b);
        let mid = self.high * b;
        Self {
            low,
            high: high + mid,
        }
    }

    #[inline(always)]
    pub const fn div(self, divisor: Self) -> Self {
        // TODO
        self.div_rem(divisor).0
    }

    #[inline(always)]
    pub const fn div_u64(self, divisor: u64) -> Self {
        if self.high < divisor {
            // TODO
            let (q, _) = _div_rem_128_64(self.low, self.high, divisor);
            Self { low: q, high: 0 }
        } else {
            let (q1, k) = _div_rem_64(self.high, divisor);
            let (q0, _) = _div_rem_128_64(self.low, k, divisor);
            Self { low: q0, high: q1 }
        }
    }

    #[allow(unsafe_code)]
    #[inline(always)]
    pub const unsafe fn unchecked_add_u64(mut self, other: u64) -> Self {
        let overflow;
        (self.low, overflow) = self.low.overflowing_add(other);

        if overflow {
            self.high = self.high.unchecked_add(1);
        }

        self
    }

    #[allow(dead_code)]
    #[allow(unsafe_code)]
    #[inline(always)]
    pub const unsafe fn overflowing_add_u64(mut self, other: u64) -> (Self, bool) {
        let mut overflow;

        (self.low, overflow) = self.low.overflowing_add(other);

        if overflow {
            (self.high, overflow) = self.high.overflowing_add(1);
        }

        (self, overflow)
    }

    #[inline(always)]
    pub const fn sub(self, other: Self) -> Self {
        let (low, borrow) = self.low.overflowing_sub(other.low);
        Self {
            low,
            high: self.high - other.high - if borrow { 1 } else { 0 },
        }
    }

    #[inline(always)]
    pub const fn sub_u64(self, other: u64) -> Self {
        let (low, borrow) = self.low.overflowing_sub(other);
        Self {
            low,
            high: self.high - if borrow { 1 } else { 0 },
        }
    }

    #[inline(always)]
    pub const fn div_rem(self, divisor: Self) -> (Self, Self) {
        debug_assert!(!divisor.is_zero());

        if self.lt(&divisor) {
            return (Self::ZERO, self);
        } else if divisor.is_one() {
            return (self, Self::ZERO);
        } else if self.eq(&divisor) {
            return (Self::ONE, Self::ZERO);
        }

        if divisor.high == 0 {
            return if self.high == 0 {
                let (q, r) = _div_rem_64(self.low, divisor.low);
                (Self { low: q, high: 0 }, Self { low: r, high: 0 })
            } else {
                let (q, r) = self.div_rem_u64(divisor.low);
                (q, Self { low: r, high: 0 })
            };
        }

        let shift = divisor.leading_zeros();

        let v1 = divisor.shl(shift).high;
        // To ensure no overflow.
        let u1 = self.shr(1);

        let u1_high = u1.high;
        let u1_low = u1.low;

        // Get quotient from divide.
        let (q1, _) = _div_rem_128_64(u1_low, u1_high, v1);

        // Undo normalization and division of u by 2.
        let mut q = Self { low: q1, high: 0 }.shl(shift).shr(63);

        if !q.is_zero() {
            q = q.sub_u64(1);
        }

        let (dividend, carry) = q.overflowing_mul(divisor);
        debug_assert!(!carry);
        let mut r = self.sub(dividend);

        if r.ge(&divisor) {
            // SAFETY: quotient is always less than u128.
            #[allow(unsafe_code)]
            {
                q = unsafe { q.unchecked_add_u64(1) };
            }

            r = r.sub(divisor);
        }

        (q, r)
    }

    #[inline(always)]
    pub const fn div_rem_u64(self, divisor: u64) -> (Self, u64) {
        if self.high < divisor {
            let (q, r) = _div_rem_128_64(self.low, self.high, divisor);
            (Self { low: q, high: 0 }, r)
        } else {
            let (q1, k) = _div_rem_64(self.high, divisor);
            let (q0, _) = _div_rem_128_64(self.low, k, divisor);
            let q = Self { low: q0, high: q1 };

            let (dividend, carry) = q.overflowing_mul_u64(divisor);
            debug_assert!(!carry);

            let r = self.sub(dividend);
            debug_assert!(r.high == 0);

            (q, r.low)
        }
    }
}
