use core::{
    fmt,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    ops::{Mul, Not},
};

/// A `Sign` represents `sign` associated with a decimal number.
#[derive(Copy, Clone, Debug)]
pub enum Sign {
    /// Positive: plus "+" or no sign.
    Plus,

    /// Negative: minus "-" sign.
    Minus,
}

impl Sign {
    /// Returns the default `Plus`.
    #[inline]
    pub const fn default() -> Self {
        Self::Plus
    }

    /// Returns `true` if sign is negative, and `false` otherwise.
    #[inline]
    pub const fn is_negative(self) -> bool {
        matches!(self, Self::Minus)
    }

    /// Tests for `self` and `other` signs to be equal, and is used by `==`
    /// operator.
    #[inline]
    pub const fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Plus, Self::Plus) => true,
            (Self::Minus, Self::Minus) => true,
            (_, _) => false,
        }
    }

    /// Invert `Sign` value.
    ///
    /// # Example
    ///
    /// ```
    /// use fastnum::decimal::Sign;
    ///
    /// assert_eq!(Sign::Plus.not(), Sign::Minus);
    /// assert_eq!(Sign::Minus.not(), Sign::Plus);
    /// ```
    #[inline]
    pub const fn not(self) -> Self {
        match self {
            Sign::Minus => Sign::Plus,
            Sign::Plus => Sign::Minus,
        }
    }

    /// Sign "multiplication".
    #[inline]
    pub const fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Sign::Plus, Sign::Plus) => Sign::Plus,
            (Sign::Minus, Sign::Minus) => Sign::Plus,
            (_, _) => Sign::Minus,
        }
    }

    /// Sign "division".
    #[inline]
    pub const fn div(self, rhs: Self) -> Self {
        self.mul(rhs)
    }
}

impl Default for Sign {
    fn default() -> Self {
        Self::default()
    }
}

impl PartialEq for Sign {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl Eq for Sign {}

impl Hash for Sign {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let s = match self {
            Sign::Minus => -1,
            Sign::Plus => 1,
        };
        s.hash(state);
    }
}

impl Not for Sign {
    type Output = Sign;

    #[inline]
    fn not(self) -> Self::Output {
        self.not()
    }
}

impl Mul<Sign> for Sign {
    type Output = Sign;

    #[inline]
    fn mul(self, other: Sign) -> Sign {
        self.mul(other)
    }
}

impl Display for Sign {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Sign::Minus => "-".fmt(f),
            Sign::Plus => Ok(()),
        }
    }
}
