use core::fmt::{Debug, Display, Formatter};

use crate::{decimal::Sign, utils::assert_eq_size};

/// Flags.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(u8);

#[doc(hidden)]
impl Flags {
    const EMPTY: Self = Self(0b0000_0000);

    /// Sign bit. More about Sign:
    const SIGN: Self = Self(0b0000_0001);

    /// NAN bit.
    const NAN: Self = Self(0b0000_0010);

    const INFINITY: Self = Self(0b0000_0100);

    const NEG_INFINITY: Self = Self(0b0000_0101);

    const MASK_IS_SPECIAL: u8 = Self::NAN.0 | Self::INFINITY.0;

    #[inline(always)]
    pub const fn default() -> Self {
        Self::EMPTY
    }

    #[inline(always)]
    pub const fn nan() -> Self {
        Self::NAN
    }

    #[inline(always)]
    pub const fn infinity() -> Self {
        Self::INFINITY
    }

    #[inline(always)]
    pub const fn neg_infinity() -> Self {
        Self::NEG_INFINITY
    }

    #[inline(always)]
    pub const fn combine(self, other: Self) -> Self {
        self.set(other)
    }

    #[inline(always)]
    pub const fn set(mut self, other: Self) -> Self {
        self.0 |= other.0;
        self
    }
    #[inline(always)]
    pub const fn unset(mut self, other: Self) -> Self {
        self.0 &= !other.0;
        self
    }

    #[inline(always)]
    pub const fn toggle(mut self, other: Self) -> Self {
        self.0 ^= other.0;
        self
    }

    #[inline(always)]
    pub const fn neg(self) -> Self {
        self.toggle(Self::SIGN)
    }

    #[inline(always)]
    pub const fn abs(self) -> Self {
        self.unset(Self::SIGN)
    }

    #[inline(always)]
    pub const fn mul(mut self, other: Self) -> Self {
        self.0 ^= other.0 & Self::SIGN.0;
        self
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.0 == Self::EMPTY.0
    }

    #[inline(always)]
    pub const fn is_negative(&self) -> bool {
        self.0 & Self::SIGN.0 != 0
    }

    #[inline(always)]
    pub const fn sign(&self) -> Sign {
        if self.is_negative() {
            Sign::Minus
        } else {
            Sign::Plus
        }
    }

    #[inline(always)]
    pub const fn is_nan(&self) -> bool {
        self.0 & Self::NAN.0 != 0
    }

    #[inline(always)]
    pub const fn is_infinity(&self) -> bool {
        self.0 & Self::INFINITY.0 != 0
    }

    #[inline(always)]
    pub const fn is_neg_infinity(&self) -> bool {
        self.0 & Self::NEG_INFINITY.0 == Self::NEG_INFINITY.0
    }

    #[inline(always)]
    pub const fn is_special(&self) -> bool {
        self.0 & Self::MASK_IS_SPECIAL != 0
    }
}

macro_rules! delimiter {
    ($delimiter: ident, $f: ident) => {
        #[allow(unused_assignments)]
        match $delimiter {
            true => {
                write!($f, ", ")?;
            }
            false => {
                $delimiter = true;
            }
        }
    };
}

impl Display for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut delimiter = false;
        if self.is_empty() {
            return Ok(());
        }

        if self.is_nan() {
            write!(f, "NAN")?;
            delimiter = true;
        }

        if self.is_negative() {
            delimiter!(delimiter, f);
            write!(f, "S")?;
        }

        if self.is_infinity() {
            delimiter!(delimiter, f);
            write!(f, "INF")?;
        }

        Ok(())
    }
}

impl Debug for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

assert_eq_size!(Flags, u8);
