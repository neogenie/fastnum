use core::fmt::{Debug, Display, Formatter};

use crate::decimal::Signals;

/// # SignalsTraps
///
/// `SignalsTraps` is a list of set trap enablers for signals.
///  When a signal's trap enabler is set, the condition causes `panic!` in debug
/// mode.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct SignalsTraps(Signals);

impl SignalsTraps {
    const EMPTY: Self = Self(Signals::EMPTY);
    const DEFAULT: Self = Self(Signals::DEFAULT_TRAPS);

    #[inline(always)]
    pub(crate) const fn new(signals: Signals) -> Self {
        Self(signals)
    }

    #[inline(always)]
    pub(crate) const fn signals(&self) -> Signals {
        self.0
    }

    /// Returns the empty list of signal traps.
    #[inline(always)]
    pub const fn empty() -> Self {
        Self::EMPTY
    }

    /// Returns the default set of signal traps.
    #[inline(always)]
    pub const fn default() -> Self {
        Self::DEFAULT
    }

    /// Adds the signal trap for the given signal.
    #[inline(always)]
    pub const fn set(mut self, signal: Signals) -> Self {
        self.0 = self.0.set(signal);
        self
    }

    #[inline]
    pub(crate) const fn trap(&self, raised: Signals) -> Signals {
        self.0.intersect(raised)
    }

    #[inline(always)]
    pub(crate) const fn merge(mut self, other: Self) -> Self {
        self = self.set(other.0);
        self
    }
}

impl Display for SignalsTraps {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for SignalsTraps {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}
