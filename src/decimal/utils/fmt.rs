use core::{
    fmt,
    fmt::{Display, Formatter},
};

use crate::{bint::UInt, decimal::dec::ControlBlock};

#[inline]
pub(crate) fn debug_print<const N: usize>(
    digits: &UInt<N>,
    cb: &ControlBlock,
    ty: &str,
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
    ty: &str,
    f: &mut Formatter,
) -> fmt::Result {
    if cb.is_nan() {
        return write!(f, "{ty}(NaN)");
    } else if cb.is_infinity() {
        return write!(f, "{ty}({}Inf)", cb.get_sign());
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
    ty: &str,
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

macro_rules! type_name {
    ($t: literal) => {
        const {
            const PREFIX: &'static [u8] = $t.as_bytes();

            let buffer: &[u8] = &const {
                let mut buffer = [0; PREFIX.len() + usize::MAX.ilog10() as usize + 1];

                let mut i = buffer.len();
                let mut n = N * 64;

                while 0 < i && n != 0 {
                    i -= 1;

                    buffer[i] = (n % 10) as u8 + b'0';
                    n /= 10;
                }

                let mut j = PREFIX.len();
                while 0 < i && 0 < j {
                    i -= 1;
                    j -= 1;

                    buffer[i] = PREFIX[j];
                }

                buffer
            };

            let buffer_len = PREFIX.len() + (N * 64).ilog10() as usize + 1;

            match core::str::from_utf8(buffer.split_at(buffer.len() - buffer_len).1) {
                Ok(x) => x,
                Err(_) => panic!("buffer is always ascii"),
            }
        }
    };
}

pub(crate) use type_name;
