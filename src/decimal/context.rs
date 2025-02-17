mod rounding_mode;
mod signal_traps;

pub use rounding_mode::RoundingMode;
pub use signal_traps::SignalsTraps;

use core::fmt::{Debug, Display, Formatter};

use crate::utils::assert_eq_size;

/// # Decimal Context
///
/// The context represents the user-selectable parameters and rules which govern
/// the results of arithmetic operations (for example, the rounding mode when
/// rounding occurs).
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
#[repr(C)]
pub struct Context {
    rounding_mode: RoundingMode,
    signal_traps: SignalsTraps,
}

impl Context {
    const DEFAULT: Self = Self {
        rounding_mode: RoundingMode::default(),
        signal_traps: SignalsTraps::default(),
    };

    /// Returns the [Default Decimal Context](#crate::default-decimal-context).
    #[inline(always)]
    #[must_use]
    pub const fn default() -> Self {
        Self::DEFAULT
    }

    /// Apply the given [RoundingMode] to the `Context`.
    #[must_use]
    #[inline(always)]
    pub const fn with_rounding_mode(mut self, rm: RoundingMode) -> Self {
        self.rounding_mode = rm;
        self
    }

    /// Apply no traps to given `Context`.
    #[inline(always)]
    #[must_use]
    pub const fn without_traps(mut self) -> Self {
        self.signal_traps = SignalsTraps::empty();
        self
    }

    /// Method applies specified [SignalsTraps] to the given context.
    /// # Examples
    ///
    /// ```
    /// use fastnum::{*, decimal::*};
    ///
    /// let ctx = Context::default().without_traps();
    ///
    /// // No panic! We can divide by zero!
    /// let res = dec256!(1.0).with_ctx(ctx) / dec256!(0).with_ctx(ctx);
    ///
    /// assert!(res.is_infinite());
    /// assert!(res.is_op_div_by_zero());
    /// assert!(res.is_op_invalid());
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn with_signal_traps(mut self, traps: SignalsTraps) -> Self {
        self.signal_traps = traps;
        self
    }

    /// Get [RoundingMode] of given `Context`.
    #[must_use]
    #[inline(always)]
    pub const fn rounding_mode(&self) -> RoundingMode {
        self.rounding_mode
    }

    /// Get [SignalsTraps] of given `Context`.
    #[must_use]
    #[inline(always)]
    pub const fn signal_traps(&self) -> SignalsTraps {
        self.signal_traps
    }

    #[inline(always)]
    pub(crate) const fn new(rounding_mode: RoundingMode, signal_traps: SignalsTraps) -> Self {
        Self {
            rounding_mode,
            signal_traps,
        }
    }

    #[inline(always)]
    pub(crate) const fn merge(mut self, other: Self) -> Self {
        self.signal_traps = self.signal_traps.merge(other.signal_traps);

        if !other.rounding_mode.is_default() {
            self.rounding_mode = other.rounding_mode;
        }

        self
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "R={}, S={}", self.rounding_mode, self.signal_traps)
    }
}

impl Debug for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

assert_eq_size!(Context, u16);
