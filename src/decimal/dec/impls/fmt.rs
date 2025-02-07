use core::fmt::{self, Debug, Display, Formatter, LowerExp, UpperExp};

use crate::decimal::{dec::format, Decimal};

impl<const N: usize> Display for Decimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_nan() {
            return write!(f, "NaN");
        } else if self.is_infinite() {
            return write!(f, "{}Inf", self.sign());
        }
        format::format(self.digits.to_str_radix(10), self.scale, self.sign(), f)
    }
}

impl<const N: usize> LowerExp for Decimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        format::format_exponential(
            self.digits.to_str_radix(10),
            self.scale,
            self.sign(),
            f,
            "e",
        )
    }
}

impl<const N: usize> UpperExp for Decimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        format::format_exponential(
            self.digits.to_str_radix(10),
            self.scale,
            self.sign(),
            f,
            "E",
        )
    }
}

impl<const N: usize> Debug for Decimal<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if f.alternate() {
            if self.is_nan() {
                return write!(f, "{}(NaN)", Self::type_name());
            } else if self.is_infinite() {
                return write!(f, "{}({}Inf)", Self::type_name(), self.sign(),);
            }

            let alert = if self.is_op_ok() { "" } else { "! " };
            write!(
                f,
                "{}({}{}{}e{})",
                Self::type_name(),
                alert,
                self.sign(),
                self.digits,
                self.exponent(),
            )
        } else {
            write!(
                f,
                "{}(digits=[{:?}], exp=[{}], flags=[{}], signals=[{}], ctx=[{}], extra=[{}])",
                Self::type_name(),
                self.digits,
                self.exponent(),
                self.flags(),
                self.signals(),
                self.context(),
                self.extra_precision
            )
        }
    }
}
