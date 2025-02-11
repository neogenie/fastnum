use core::{
    fmt,
    fmt::{Display, Formatter},
};

use crate::{
    decimal::{dec::ControlBlock, Decimal},
    int::UInt,
    utils::assert_eq_size,
};

type D<const N: usize> = Decimal<N>;

/// Extra precision digits
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct ExtraPrecision(u16);

impl ExtraPrecision {
    #[inline(always)]
    pub(crate) const fn new() -> Self {
        Self(0)
    }

    #[inline(always)]
    pub(crate) const fn from_digits(digits: u16) -> Self {
        debug_assert!(digits < 10000);
        Self(digits)
    }

    #[inline(always)]
    pub(crate) const fn has_digits(self) -> bool {
        self.0 != 0
    }

    #[inline(always)]
    pub(crate) const fn round_reminder(self) -> u16 {
        self.0 / 1000
    }

    #[inline(always)]
    pub(crate) const fn push(self, digit: u64) -> Self {
        debug_assert!(digit < 10);
        Self(digit as u16 * 1000 + self.0 / 10)
    }

    // #[inline(always)]
    // pub(crate) const fn as_decimal<const N: usize>(self) -> D<N> {
    //     D::new(
    //         UInt::from_digit(self.0 as u64),
    //         4,
    //         ControlBlock::default(),
    //         ExtraPrecision::new(),
    //     )
    // }

    #[inline]
    pub(crate) const fn eq(self, other: Self) -> bool {
        self.0 == other.0
    }

    #[inline]
    pub(crate) const fn overflowing_add(self, other: Self) -> (Self, bool) {
        let res = self.0 + other.0;
        if res >= 10000 {
            (Self(res - 10000), true)
        } else {
            (Self(res), false)
        }
    }

    #[inline]
    pub(crate) const fn overflowing_sub(self, other: Self) -> (Self, bool) {
        let res = self.0 as i16 - other.0 as i16;
        if res < 0 {
            (Self((res + 10000) as u16), true)
        } else {
            (Self(res as u16), false)
        }
    }

    #[inline]
    pub(crate) const fn overflowing_scale<const N: usize>(
        mut self,
        mut power: i16,
    ) -> (Self, D<N>) {
        debug_assert!(power > 0);

        // TODO: performance optimization

        if self.0 == 0 {
            return (self, D::ZERO);
        }

        let mut res = D::ZERO;

        while power > 0 {
            let digit = self.0 / 1000;
            res = res.mul_add(
                D::TEN,
                D::new(
                    UInt::from_digit(digit as u64),
                    0,
                    ControlBlock::default(),
                    ExtraPrecision::new(),
                ),
            );
            self.0 = (self.0 % 1000) * 10;
            power -= 1;
        }

        (self, res)
    }
}

impl Display for ExtraPrecision {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "0.{:04}", self.0)
    }
}

assert_eq_size!(ExtraPrecision, u16);
