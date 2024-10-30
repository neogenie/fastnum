use std::ops::Not;

/// A `Sign` represents sign associated with decimal number.
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Debug, Hash)]
pub enum Sign {
    Minus,
    #[default]
    NoSign,
    Plus,
}

impl Sign {
    #[inline]
    pub const fn not(self) -> Self {
        match self {
            Sign::Minus => Sign::Plus,
            Sign::NoSign => Sign::Minus,
            Sign::Plus => Sign::Minus,
        }
    }
}

impl Not for Sign {
    type Output = Sign;

    #[inline]
    fn not(self) -> Self::Output {
        self.not()
    }
}
