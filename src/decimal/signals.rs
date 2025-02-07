use core::fmt::{Debug, Display, Formatter};

use crate::utils::assert_eq_size;

/// # Signal
///
/// The exceptional conditions are grouped into signals, which can be controlled
/// individually.
/// For each of the signals, the corresponding flag is set to `1`
/// when the signal occurs.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct Signal(u8);

impl Signal {
    /// By default, no operation signals are raised.
    pub const EMPTY: Self = Self(0b0000_0000);

    /// Raised when the exponent of a result has been altered or constrained to
    /// fit the target type.
    pub const OP_CLAMPED: Self = Self(0b0000_0001);

    /// Raised when a non-zero dividend is divided by zero.
    pub const OP_DIV_BY_ZERO: Self = Self(0b0000_0010);

    /// Raised when a result is not exact (one or more non-zero coefficient
    /// digits were discarded during rounding).
    pub const OP_INEXACT: Self = Self(0b0000_1000);

    /// Raised when a result would be undefined or impossible.
    pub const OP_INVALID: Self = Self(0b0000_0100);

    /// Raised when the exponent of a result is too large to be represented.
    pub const OP_OVERFLOW: Self = Self(0b0001_0000);

    /// Raised when a result has been rounded (that is, some zero or non-zero
    /// coefficient digits were discarded).
    pub const OP_ROUNDED: Self = Self(0b0010_0000);

    /// Raised when a result is subnormal (its adjusted exponent is less than
    /// E<sub>min</sub>), before any rounding.
    pub const OP_SUBNORMAL: Self = Self(0b0100_0000);

    /// Raised when a result is both subnormal and inexact.
    pub const OP_UNDERFLOW: Self = Self(0b1000_0000);

    pub(crate) const DEFAULT_TRAPS: Self =
        Self(Self::OP_DIV_BY_ZERO.0 | Self::OP_INVALID.0 | Self::OP_OVERFLOW.0);

    /// Return an empty set of signaling flags.
    #[must_use]
    #[inline(always)]
    pub const fn empty() -> Self {
        Self::EMPTY
    }

    #[must_use]
    #[inline(always)]
    pub(crate) const fn div_by_zero() -> Self {
        Self(Self::OP_DIV_BY_ZERO.0 | Self::OP_INVALID.0)
    }

    #[must_use]
    #[inline(always)]
    pub(crate) const fn underflow() -> Self {
        Self(Self::OP_UNDERFLOW.0 | Self::OP_INEXACT.0 | Self::OP_ROUNDED.0 | Self::OP_SUBNORMAL.0)
    }

    #[must_use]
    #[inline(always)]
    pub(crate) const fn overflow() -> Self {
        Self(Self::OP_OVERFLOW.0 | Self::OP_INEXACT.0 | Self::OP_ROUNDED.0)
    }

    /// Combines the given signal with another one.
    #[must_use]
    #[inline(always)]
    pub const fn combine(mut self, other: Self) -> Self {
        self.0 |= other.0;
        self
    }

    /// Intersect the given signal with another one.
    #[must_use]
    #[inline(always)]
    pub const fn intersect(mut self, other: Self) -> Self {
        self.0 &= other.0;
        self
    }

    #[doc(hidden)]
    #[must_use]
    #[inline(always)]
    pub const fn set(mut self, other: Self) -> Self {
        self.0 |= other.0;
        self
    }

    #[doc(hidden)]
    #[must_use]
    #[inline(always)]
    pub const fn unset(mut self, other: Self) -> Self {
        self.0 &= !other.0;
        self
    }

    #[doc(hidden)]
    #[must_use]
    #[inline(always)]
    pub const fn toggle(mut self, other: Self) -> Self {
        self.0 ^= other.0;
        self
    }

    #[doc(hidden)]
    #[must_use]
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.0 == Self::EMPTY.0
    }

    #[doc(hidden)]
    #[must_use]
    #[inline(always)]
    pub const fn is_raised(&self, other: Self) -> bool {
        self.0 & other.0 != 0
    }
}

macro_rules! display {
    ($self: ident, $f: ident, $($v: ident => $l: literal),*) => {
        #[allow(unused_assignments)]
        {
            let mut delimiter = false;
            $(
                if $self.is_raised(Self::$v) {
                    match delimiter {
                        true => {
                            write!($f, ", ")?;
                        }
                        false => {
                            delimiter = true;
                        }
                    }
                    write!($f, $l)?;
                }
            )*
        }
    };
}

impl Display for Signal {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        display!(self, f, OP_CLAMPED => "!CP", OP_DIV_BY_ZERO => "!DBZ", OP_INEXACT => "!INEXACT", OP_INVALID => "!INV", OP_OVERFLOW => "!OFW", OP_ROUNDED => "!ROUND", OP_SUBNORMAL => "!SN", OP_UNDERFLOW => "!UFW");

        Ok(())
    }
}

impl Debug for Signal {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

assert_eq_size!(Signal, u8);
