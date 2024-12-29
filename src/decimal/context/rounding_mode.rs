use core::fmt::{Debug, Display, Formatter};

use crate::utils::assert_eq_size;

include!(concat!(env!("OUT_DIR"), "/default_rounding_mode.rs"));

/// Determines how to calculate the last digit of the number
///
/// Default rounding mode is `HalfUp`.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum RoundingMode {
    /// Always round away from zero
    ///
    ///
    /// * 5.5 → 6.0
    /// * 2.5 → 3.0
    /// * 1.6 → 2.0
    /// * 1.1 → 2.0
    /// * -1.1 → -2.0
    /// * -1.6 → -2.0
    /// * -2.5 → -3.0
    /// * -5.5 → -6.0
    Up,

    /// Always round towards zero
    ///
    /// * 5.5  →  5.0
    /// * 2.5  →  2.0
    /// * 1.6  →  1.0
    /// * 1.1  →  1.0
    /// * -1.1 → -1.0
    /// * -1.6 → -1.0
    /// * -2.5 → -2.0
    /// * -5.5 → -5.0
    Down,

    /// Towards +∞
    ///
    /// * 5.5 → 6.0
    /// * 2.5 → 3.0
    /// * 1.6 → 2.0
    /// * 1.1 → 2.0
    /// * -1.1 → -1.0
    /// * -1.6 → -1.0
    /// * -2.5 → -2.0
    /// * -5.5 → -5.0
    Ceiling,

    /// Towards -∞
    ///
    /// * 5.5 → 5.0
    /// * 2.5 → 2.0
    /// * 1.6 → 1.0
    /// * 1.1 → 1.0
    /// * -1.1 → -2.0
    /// * -1.6 → -2.0
    /// * -2.5 → -3.0
    /// * -5.5 → -6.0
    Floor,

    /// Round to 'nearest neighbor', or up if ending decimal is 5
    ///
    /// * 5.5 → 6.0
    /// * 2.5 → 3.0
    /// * 1.6 → 2.0
    /// * 1.1 → 1.0
    /// * -1.1 → -1.0
    /// * -1.6 → -2.0
    /// * -2.5 → -3.0
    /// * -5.5 → -6.0
    HalfUp,

    /// Round to 'nearest neighbor', or down if ending decimal is 5
    ///
    /// * 5.5 → 5.0
    /// * 2.5 → 2.0
    /// * 1.6 → 2.0
    /// * 1.1 → 1.0
    /// * -1.1 → -1.0
    /// * -1.6 → -2.0
    /// * -2.5 → -2.0
    /// * -5.5 → -5.0
    HalfDown,

    /// Round to 'nearest neighbor', if equidistant, round towards
    /// nearest even digit
    ///
    /// * 5.5 → 6.0
    /// * 2.5 → 2.0
    /// * 1.6 → 2.0
    /// * 1.1 → 1.0
    /// * -1.1 → -1.0
    /// * -1.6 → -2.0
    /// * -2.5 → -2.0
    /// * -5.5 → -6.0
    HalfEven,
}

impl Default for RoundingMode {
    #[inline(always)]
    fn default() -> Self {
        Self::default()
    }
}

impl RoundingMode {
    /// Returns default rounding mode.
    #[inline(always)]
    pub const fn default() -> Self {
        DEFAULT_ROUNDING_MODE
    }
}

impl Display for RoundingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let rm = match self {
            RoundingMode::Up => "Up",
            RoundingMode::Down => "Down",
            RoundingMode::Ceiling => "Ceiling",
            RoundingMode::Floor => "Floor",
            RoundingMode::HalfUp => "HalfUp",
            RoundingMode::HalfDown => "HalfDown",
            RoundingMode::HalfEven => "HalfEven",
        };
        f.write_str(rm)
    }
}

impl Debug for RoundingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

assert_eq_size!(RoundingMode, u8);
