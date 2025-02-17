use core::{
    fmt,
    fmt::{Display, Formatter},
};

use crate::{decimal::dec::ControlBlock, int::UInt};

#[inline]
pub(crate) fn debug_print<const N: usize>(
    digits: &UInt<N>,
    cb: &ControlBlock,
    ty: String,
    f: &mut Formatter,
) -> fmt::Result {
    if f.alternate() {
        debug_print_alternate(digits, cb, ty, f)
    } else {
        debug_print_default(digits, cb, ty, f)
    }
}

fn debug_print_alternate<const N: usize>(
    digits: &UInt<N>,
    cb: &ControlBlock,
    ty: String,
    f: &mut Formatter,
) -> fmt::Result {
    if cb.is_nan() {
        return write!(f, "{}(NaN)", ty);
    } else if cb.is_infinity() {
        return write!(f, "{}({}Inf)", ty, cb.get_sign());
    }

    let alert = if cb.is_op_ok() { "" } else { "! " };
    write!(
        f,
        "{}({}{}{}e{})",
        ty,
        alert,
        cb.get_sign(),
        digits,
        cb.get_exponent()
    )
}

fn debug_print_default<const N: usize>(
    digits: &UInt<N>,
    cb: &ControlBlock,
    ty: String,
    f: &mut Formatter,
) -> fmt::Result {
    write!(
        f,
        "{}(digits=[{:?}], exp=[{}], flags=[{}], signals=[{}], ctx=[{}], extra=[{}])",
        ty,
        digits,
        cb.get_exponent(),
        Flags(*cb),
        cb.get_signals(),
        cb.get_context(),
        cb.get_extra_precision()
    )
}

struct Flags(ControlBlock);

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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut delimiter = false;

        if self.0.is_nan() {
            write!(f, "NAN")?;
            delimiter = true;
        }

        if self.0.is_negative() {
            delimiter!(delimiter, f);
            write!(f, "S")?;
        }

        if self.0.is_infinity() {
            delimiter!(delimiter, f);
            write!(f, "INF")?;
        }

        Ok(())
    }
}
