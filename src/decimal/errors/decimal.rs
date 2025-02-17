use core::{
    fmt,
    fmt::{Debug, Display, Formatter},
};

use crate::{decimal::Signals, utils::err_msg};

#[doc(hidden)]
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
pub enum DecimalError {
    /// Exponent of a result has been altered or constrained to fit the target
    /// type.
    Clamped,

    /// Non-zero dividend is divided by zero.
    DivByZero,

    /// The result is not exact (one or more non-zero coefficient digits were
    /// discarded during rounding).
    Inexact,

    /// The result would be undefined or impossible.
    Invalid,

    /// The exponent of a result is too large to be represented.
    Overflow,

    /// The result has been rounded (that is, some zero or non-zero coefficient
    /// digits were discarded).
    Rounded,

    /// The result is subnormal (its adjusted exponent is less than Emin) before
    /// any rounding.
    Subnormal,

    /// The result is both subnormal and inexact.
    Underflow,
}

impl DecimalError {
    #[inline]
    pub(crate) const fn description(&self) -> &'static str {
        use DecimalError::*;
        match self {
            Clamped => err_msg!("result is clamped"),
            DivByZero => err_msg!("division by zero"),
            Inexact => err_msg!("result may be inexact"),
            Invalid => err_msg!("invalid operation"),
            Overflow => err_msg!("overflow was occurred while performing arithmetic operation"),
            Rounded => err_msg!("result is rounded"),
            Subnormal => err_msg!("result is subnormal"),
            Underflow => err_msg!("underflow was occurred while performing arithmetic operation"),
        }
    }

    #[inline]
    pub(crate) const fn from_signals(signals: Signals) -> Self {
        debug_assert!(!signals.is_empty());

        if signals.is_raised(Signals::OP_DIV_BY_ZERO) {
            Self::DivByZero
        } else if signals.is_raised(Signals::OP_INVALID) {
            Self::Invalid
        } else if signals.is_raised(Signals::OP_OVERFLOW) {
            Self::Overflow
        } else if signals.is_raised(Signals::OP_UNDERFLOW) {
            Self::Underflow
        } else if signals.is_raised(Signals::OP_INEXACT) {
            Self::Inexact
        } else if signals.is_raised(Signals::OP_ROUNDED) {
            Self::Rounded
        } else if signals.is_raised(Signals::OP_SUBNORMAL) {
            Self::Subnormal
        } else if signals.is_raised(Signals::OP_CLAMPED) {
            Self::Clamped
        } else {
            unreachable!()
        }
    }

    #[track_caller]
    #[inline]
    pub(crate) const fn panic(&self) {
        #[cfg(debug_assertions)]
        panic!("{}", self.description());
    }
}

impl Display for DecimalError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl Debug for DecimalError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self, f)
    }
}

impl core::error::Error for DecimalError {
    #[inline]
    fn description(&self) -> &str {
        self.description()
    }
}
