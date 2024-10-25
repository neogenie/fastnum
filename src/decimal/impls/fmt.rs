use std::fmt;
use std::fmt::{Display, Formatter};

use crate::decimal::Decimal;

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} E {}", self.sign, self.value, -self.scale)
    }
}

impl fmt::Debug for Decimal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{}", self)
        } else {
            write!(f,
                   "Decimal(sign={:?}, scale={}, digits={:?})",
                   self.sign(), self.scale, self.value
            )
        }
    }
}