mod f2dec;

use crate::decimal::{
    dec::{ControlBlock, ExtraPrecision},
    signals::Signals,
    Context, Decimal, Sign,
};

type D<const N: usize> = Decimal<N>;

macro_rules! from_float_impl {
    ($n: ident, $f: ident) => {
        #[inline]
        pub const fn $n<const N: usize>(n: $f) -> D<N> {
            use crate::decimal::utils::types::$f::*;

            if n.is_nan() {
                return D::NAN;
            }

            let b = n.to_bits();

            let sign = if b & SIGN_MASK != 0 {
                Sign::Minus
            } else {
                Sign::Plus
            };

            let frac = b & MAN_MASK;
            let exp = b & EXP_MASK;

            if frac == 0 && exp == EXP_MASK {
                return D::INFINITY.set_sign(sign);
            }

            if frac == 0 && exp == 0 {
                return D::ZERO.set_sign(sign);
            }

            if exp == 0 {
                // subnormal

                let pow = (MAX_EXP - 2) as i16 + (MANTISSA_DIGITS - 1) as i16;
                f2dec::f2dec(frac as u64, -pow, sign)
            } else {
                // normal

                let frac = frac | MAN_MASK_NORMAL;
                let pow = (exp >> (MANTISSA_DIGITS - 1)) as i16
                    - (MAX_EXP - 1) as i16
                    - (MANTISSA_DIGITS - 1) as i16;

                if pow == 0 {
                    Decimal::new(
                        uint(frac),
                        ControlBlock::new(
                            0,
                            sign,
                            Signals::empty(),
                            Context::default(),
                            ExtraPrecision::new(),
                        ),
                    )
                } else if pow < 0 {
                    let mut trailing_zeros = frac.trailing_zeros();
                    if trailing_zeros > -pow as u32 {
                        trailing_zeros = -pow as u32;
                    }

                    let reduced_frac = frac >> trailing_zeros;
                    let reduced_pow = pow + trailing_zeros as i16;

                    f2dec::f2dec(reduced_frac as u64, reduced_pow, sign)
                } else {
                    f2dec::f2dec(frac as u64, pow, sign)
                }
            }
        }
    };
}

from_float_impl!(from_f32, f32);
from_float_impl!(from_f64, f64);
