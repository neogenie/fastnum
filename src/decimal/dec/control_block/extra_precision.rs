use crate::{
    decimal::{
        dec::{construct::construct, ControlBlock},
        Decimal,
    },
    int::UInt,
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
    pub const EXTRA_PRECISION_DIGITS: u8 = 7;
    pub const EXTRA_PRECISION_SCALE: u64 = 1_000_000;
    
    const EXTRA_PRECISION_CARRY: u64 = 10_000_000;
    const EXTRA_DIGITS_SHIFT: u8 = 40;
    const EXTRA_DIGITS_MASK: u64 = 0xFFFF_FF00_0000_0000;

    #[inline(always)]
    pub const fn push_extra_precision_digit(&mut self, digit: u64) {
        debug_assert!(digit < 10);
        let extra_digits = self.get_extra_digits();
        self.set_extra_digits(digit * Self::EXTRA_PRECISION_SCALE + extra_digits / 10);
    }

    #[inline(always)]
    pub const fn eq_extra_precision(&self, other: &Self) -> bool {
        self.get_extra_digits() == other.get_extra_digits()
    }

    #[inline(always)]
    pub const fn take_round_reminder(&mut self) -> u8 {
        let extra_digits = self.get_extra_digits();
        let mut extra_digit = 0;

        if extra_digits != 0 {
            extra_digit = (extra_digits / Self::EXTRA_PRECISION_SCALE) as u8;

            self.op_rounded();
            self.op_inexact();
            self.reset_extra_digits();
        }

        extra_digit
    }

    #[inline]
    pub const fn take_extra_digits<const N: usize>(&mut self) -> D<N> {
        let extra_digits = self.get_extra_digits();

        if extra_digits != 0 {
            let exp = self.get_exponent() - Self::EXTRA_PRECISION_DIGITS as i32;
            self.reset_extra_digits();
            construct(UInt::from_digit(extra_digits), exp, self.get_sign())
        } else {
            D::ZERO
        }
    }

    #[inline]
    pub const fn add_extra_digits(&mut self, other: &Self) -> bool {
        
    }

    #[inline]
    pub const fn sub_extra_digits(&mut self, other: &Self) -> bool {
        
    }

    #[inline(always)]
    pub const fn get_extra_digits(&self) -> u64 {
        (self.0 & Self::EXTRA_DIGITS_MASK) >> Self::EXTRA_DIGITS_SHIFT
    }

    #[inline(always)]
    pub const fn set_extra_digits(&mut self, extra_digits: u64) {
        debug_assert!(extra_digits < Self::EXTRA_PRECISION_CARRY);
        
        self.0 = (self.0 & !Self::EXTRA_DIGITS_MASK)
            | (extra_digits << Self::EXTRA_DIGITS_SHIFT) & Self::EXTRA_DIGITS_MASK;
    }

    #[inline(always)]
    pub const fn reset_extra_digits(&mut self) {
        self.0 &= !Self::EXTRA_DIGITS_MASK;
    }
}
