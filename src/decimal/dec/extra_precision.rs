use core::{
    fmt,
    fmt::{Display, Formatter},
};

use crate::{
    decimal::{
        dec::{construct::construct, ControlBlock},
        Context, Decimal, Sign, Signals,
    },
    int::UInt,
    utils::assert_eq_size,
};

type D<const N: usize> = Decimal<N>;

// TODO: can be compacted into u32 (24 bit digits & count)
/// Extra precision digits
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub(crate) struct ExtraPrecision {
    digits: u32,
    count: u32,
}

impl ExtraPrecision {
    pub const EXTRA_PRECISION_DIGITS: u32 = 7;

    const EXTRA_PRECISION_SCALE: u32 = 1_000_000;
    const EXTRA_PRECISION_CARRY: u64 = 10_000_000;
    const EXTRA_PRECISION_DIGIT_7: u64 = 1_000_000;
    const EXTRA_PRECISION_DIGIT_6: u64 = 100_000;
    const EXTRA_PRECISION_DIGIT_5: u64 = 10_000;
    const EXTRA_PRECISION_DIGIT_4: u64 = 1_000;
    const EXTRA_PRECISION_DIGIT_3: u64 = 100;
    const EXTRA_PRECISION_DIGIT_2: u64 = 10;

    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            digits: 0,
            count: 0,
        }
    }

    #[inline(always)]
    pub const fn digits(&self) -> u64 {
        self.digits as u64
    }

    #[inline(always)]
    pub const fn count(&self) -> u32 {
        self.count
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.digits == 0 && self.count == 0
    }

    #[inline(always)]
    pub const fn get_round_reminder(&self) -> u8 {
        (self.digits / Self::EXTRA_PRECISION_SCALE) as u8
    }

    #[inline(always)]
    pub const fn from_digits(digits: u64) -> Self {
        debug_assert!(digits < Self::EXTRA_PRECISION_CARRY);
        Self {
            digits: digits as u32,
            count: count_digits(digits),
        }
    }

    #[inline(always)]
    pub const fn push_back(&mut self, tail: Self) {
        match self.count {
            6 => {
                self.digits += tail.digits / 1_000_000;
                self.count += min(1, tail.count);
            }
            5 => {
                self.digits += tail.digits / 100_000;
                self.count += min(2, tail.count);
            }
            4 => {
                self.digits += tail.digits / 10_000;
                self.count += min(3, tail.count);
            }
            3 => {
                self.digits += tail.digits / 1_000;
                self.count += min(4, tail.count);
            }
            2 => {
                self.digits += tail.digits / 100;
                self.count += min(5, tail.count);
            }
            1 => {
                self.digits += tail.digits / 10;
                self.count += min(6, tail.count);
            }
            0 => {
                self.digits = tail.digits;
                self.count = tail.count;
            }
            _ => {}
        }
    }

    #[inline(always)]
    pub const fn push_digit(&mut self, digit: u64) {
        debug_assert!(digit < 10);
        self.digits = (digit as u32) * Self::EXTRA_PRECISION_SCALE + self.digits / 10;
        self.count += 1;
    }

    #[inline]
    pub const fn scale_up<const N: usize>(&mut self, mut power: u32) -> Option<D<N>> {
        if self.digits == 0 {
            return None;
        }

        let mut res = D::ZERO;

        // TODO: performance optimization
        while power > 0 {
            let digit = self.digits / Self::EXTRA_PRECISION_SCALE;
            res = res.mul_add(
                D::TEN,
                D::new(
                    UInt::from_digit(digit as u64),
                    ControlBlock::basic(0, Sign::Plus),
                ),
            );
            self.digits = (self.digits % Self::EXTRA_PRECISION_SCALE) * 10;
            power -= 1;
            self.count = self.count.saturating_sub(1);
        }

        if res.is_zero() || res.is_op_underflow() {
            return None;
        }

        Some(res)
    }

    #[inline(always)]
    pub const fn overflowing_add(&mut self, other: Self) -> bool {
        let res = self.digits as u64 + other.digits as u64;
        if res >= Self::EXTRA_PRECISION_CARRY {
            let res = res - Self::EXTRA_PRECISION_CARRY;
            self.digits = res as u32;
            self.count = count_digits(res);
            true
        } else {
            self.digits = res as u32;
            self.count = count_digits(res);
            false
        }
    }

    #[inline(always)]
    pub const fn overflowing_sub(&mut self, other: Self) -> bool {
        if self.digits >= other.digits {
            self.digits -= other.digits;
            self.count = count_digits(self.digits as u64);
            false
        } else {
            let res = Self::EXTRA_PRECISION_CARRY - (other.digits - self.digits) as u64;
            self.digits = res as u32;
            self.count = count_digits(res);
            true
        }
    }

    #[inline(always)]
    pub const fn as_decimal<const N: usize>(self, exp: i32, sign: Sign, ctx: Context) -> D<N> {
        let extra_digits = self.digits as u64;

        if extra_digits != 0 {
            construct(
                UInt::from_digit(extra_digits),
                exp - Self::EXTRA_PRECISION_DIGITS as i32,
                sign,
                Signals::empty(),
                ctx,
                ExtraPrecision::new(),
            )
        } else {
            D::ZERO.set_sign(sign).set_ctx(ctx)
        }
    }
}

#[inline(always)]
pub const fn min(lhs: u32, rhs: u32) -> u32 {
    if lhs < rhs {
        lhs
    } else {
        rhs
    }
}

#[inline(always)]
const fn count_digits(digits: u64) -> u32 {
    debug_assert!(digits < ExtraPrecision::EXTRA_PRECISION_CARRY);
    if digits == 0 {
        0
    } else if digits >= ExtraPrecision::EXTRA_PRECISION_DIGIT_7 {
        7
    } else if digits >= ExtraPrecision::EXTRA_PRECISION_DIGIT_6 {
        6
    } else if digits >= ExtraPrecision::EXTRA_PRECISION_DIGIT_5 {
        5
    } else if digits >= ExtraPrecision::EXTRA_PRECISION_DIGIT_4 {
        4
    } else if digits >= ExtraPrecision::EXTRA_PRECISION_DIGIT_3 {
        3
    } else if digits >= ExtraPrecision::EXTRA_PRECISION_DIGIT_2 {
        2
    } else {
        1
    }
}

impl Display for ExtraPrecision {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "0.{:07}", self.digits)
    }
}

assert_eq_size!(ExtraPrecision, u64);
