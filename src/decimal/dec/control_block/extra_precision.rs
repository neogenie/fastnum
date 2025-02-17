use core::cmp::Ordering;

use crate::decimal::{
    dec::{ControlBlock, ExtraPrecision},
    Decimal, Signals,
};

type D<const N: usize> = Decimal<N>;

/// Control block (CB)
///
/// Memory layout:
///
/// |  Bit  |          Data         |         Bit Mask        |
/// |:-----:|:---------------------:|:-----------------------:|
/// | `...` |      `...`            |          `...`          |
/// | 40-63 | Extra digits (24 bit) | `0xFFFF_FF00_0000_0000` |
impl ControlBlock {
    const EXTRA_DIGITS_SHIFT: u8 = 40;
    const EXTRA_DIGITS_MASK: u64 = 0xFFFF_FF00_0000_0000;

    #[inline(always)]
    pub const fn has_extra_precision(&self) -> bool {
        self.0 & Self::EXTRA_DIGITS_MASK != 0
    }

    #[inline(always)]
    pub const fn eq_extra_precision(&self, other: &Self) -> bool {
        self.get_extra_digits() == other.get_extra_digits()
    }

    #[inline(always)]
    pub const fn cmp_extra_precision(&self, other: &Self) -> Ordering {
        let lhs = self.get_extra_digits();
        let rhs = other.get_extra_digits();

        if lhs < rhs {
            Ordering::Less
        } else if lhs > rhs {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    #[inline(always)]
    pub const fn take_round_reminder(&mut self) -> u8 {
        let extra_digits = self.take_extra_precision();
        let mut extra_digit = 0;

        if !extra_digits.is_empty() {
            self.raise_signals(Signals::OP_ROUNDED.combine(Signals::OP_INEXACT));
            extra_digit = extra_digits.get_round_reminder();
        }

        extra_digit
    }

    #[inline]
    pub const fn take_extra_precision_decimal<const N: usize>(&mut self) -> D<N> {
        let extra_precision = self.take_extra_precision();
        extra_precision.as_decimal(self.get_exponent(), self.get_sign(), self.get_context())
    }

    #[inline]
    pub const fn add_extra_precision(&mut self, other: &Self) -> bool {
        let mut extra_precision = self.take_extra_precision();
        let other_extra_precision = other.get_extra_precision();

        let overflow = extra_precision.overflowing_add(other_extra_precision);
        self.set_extra_precision(extra_precision);

        overflow
    }

    #[inline]
    pub const fn sub_extra_precision(&mut self, other: &Self) -> bool {
        let mut extra_precision = self.take_extra_precision();
        let other_extra_precision = other.get_extra_precision();

        let overflow = extra_precision.overflowing_sub(other_extra_precision);
        self.set_extra_precision(extra_precision);

        overflow
    }

    #[inline(always)]
    pub const fn get_extra_precision(&self) -> ExtraPrecision {
        ExtraPrecision::from_digits(self.get_extra_digits())
    }

    #[inline(always)]
    pub const fn set_extra_precision(&mut self, extra_precision: ExtraPrecision) {
        let extra_digits = extra_precision.digits();
        self.set_extra_digits(extra_digits);
    }

    #[inline(always)]
    pub const fn reset_extra_precision(&mut self) {
        self.reset_extra_digits();
    }

    #[inline]
    pub const fn take_extra_precision(&mut self) -> ExtraPrecision {
        let extra_precision = self.get_extra_precision();
        self.reset_extra_digits();
        extra_precision
    }

    #[inline(always)]
    const fn get_extra_digits(&self) -> u64 {
        (self.0 & Self::EXTRA_DIGITS_MASK) >> Self::EXTRA_DIGITS_SHIFT
    }

    #[inline(always)]
    const fn set_extra_digits(&mut self, extra_digits: u64) {
        self.0 = (self.0 & !Self::EXTRA_DIGITS_MASK)
            | (extra_digits << Self::EXTRA_DIGITS_SHIFT) & Self::EXTRA_DIGITS_MASK;
    }

    #[inline(always)]
    const fn reset_extra_digits(&mut self) {
        self.0 &= !Self::EXTRA_DIGITS_MASK;
    }
}
