use crate::{
    decimal::{
        dec::{
            intrinsics::{clength, Intrinsics, E_LIMIT, E_MIN},
            math::utils::{overflow, underflow},
            scale::extend_scale_to,
            ControlBlock, ExtraPrecision,
        },
        Decimal, Sign,
    },
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline(always)]
pub(crate) const fn construct<const N: usize>(digits: UInt<N>, exp: i32, sign: Sign) -> D<N> {
    construct_with_clength(digits, exp, sign, clength(digits))
}

#[inline]
pub(crate) const fn construct_with_clength<const N: usize>(
    mut digits: UInt<N>,
    mut exp: i32,
    sign: Sign,
    clength: u32,
) -> D<N> {
    if digits.is_zero() {
        return construct_zero(exp, sign);
    }

    // Overflow exp > Emax
    if exp > Intrinsics::<N>::E_MAX {
        return overflow(cb);
    }

    // Underflow exp < Emin
    if exp < E_MIN {
        return underflow(cb);
    }

    if exp <= E_LIMIT {
        if exp < E_MIN + (clength as i32 - 1) {
            cb = cb.raise_signal(Signal::OP_SUBNORMAL);
        }

        return D::new(digits, -exp as i16, cb);
    }

    cb = cb
        .raise_signal(Signal::OP_CLAMPED)
        .raise_signal(Signal::OP_ROUNDED);

    while exp > E_LIMIT {
        if digits.gt(&Intrinsics::<N>::COEFF_MEDIUM) {
            return D::INFINITY.with_cb(cb.raise_signal(Signal::OP_OVERFLOW));
        } else {
            digits = digits.strict_mul(UInt::<N>::TEN);
            exp -= 1;
        }
    }

    D::new(digits, -exp as i16, cb, extra_precision)
}

#[inline]
const fn construct_zero<const N: usize>(exp: i32, sign: Sign) -> D<N> {
    let cb = if exp > i16::MAX as i32 + 1 {
        let mut cb = ControlBlock::new(i16::MIN, sign);
        cb.raise_op_clamped();
        cb
    } else if exp <= i16::MIN as i32 {
        let mut cb = ControlBlock::new(i16::MAX, sign);
        cb.raise_op_clamped();
        cb
    } else {
        ControlBlock::new(-exp as i16, sign)
    };

    D::new(UInt::ZERO, cb)
}
