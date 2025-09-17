use core::fmt::{self, Debug, Display, Formatter, LowerExp, UpperExp};

use crate::decimal::{dec::format, utils, Decimal};

impl<const N: usize> Display for Decimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_nan() {
            write!(f, "NaN")
        } else if self.is_infinite() {
            write!(f, "{}Inf", self.sign())
        } else {
            format::format(
                self.digits.to_str_radix(10),
                self.cb.get_scale(),
                self.sign(),
                f,
            )
        }
    }
}

impl<const N: usize> LowerExp for Decimal<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        format::format_exponential(
            self.digits.to_str_radix(10),
            self.cb.get_scale(),
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
            self.cb.get_scale(),
            self.sign(),
            f,
            "E",
        )
    }
}

impl<const N: usize> Debug for Decimal<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        #[cfg(not(feature = "debug_display"))]
        {
            utils::fmt::debug_print(&self.digits, &self.cb, Self::type_name(), f)
        }

        #[cfg(feature = "debug_display")]
        {
            write!(f, "{}", self)
        }
    }
}
