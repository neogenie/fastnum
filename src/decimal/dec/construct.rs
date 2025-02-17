use crate::{
    decimal::{
        dec::{
            intrinsics::{clength, Intrinsics, E_LIMIT, E_MIN},
            math::utils::{overflow, underflow},
            ControlBlock, ExtraPrecision,
        },
        signals::Signals,
        Context, Decimal, Sign,
    },
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline(always)]
pub(crate) const fn construct<const N: usize>(
    digits: UInt<N>,
    exp: i32,
    sign: Sign,
    signals: Signals,
    ctx: Context,
    extra_precision: ExtraPrecision,
) -> D<N> {
    construct_with_clength(
        digits,
        exp,
        sign,
        signals,
        ctx,
        extra_precision,
        clength(digits),
    )
}

#[inline]
pub(crate) const fn construct_with_clength<const N: usize>(
    mut digits: UInt<N>,
    mut exp: i32,
    sign: Sign,
    mut signals: Signals,
    ctx: Context,
    mut extra_precision: ExtraPrecision,
    clength: u32,
) -> D<N> {
    if digits.is_zero() {
        return construct_zero(exp, sign, signals, ctx, extra_precision);
    }

    // Overflow exp > Emax
    if exp > Intrinsics::<N>::E_MAX {
        return overflow(sign, signals, ctx);
    }

    // Underflow exp < Emin
    if exp < E_MIN {
        return underflow(sign, signals, ctx);
    }

    if exp <= E_LIMIT {
        if exp < E_MIN + (clength as i32 - 1) {
            signals.raise(Signals::OP_SUBNORMAL);
        }

        let cb = ControlBlock::new(-exp as i16, sign, signals, ctx, extra_precision);
        return D::new(digits, cb);
    }

    signals.raise(Signals::OP_ROUNDED);
    signals.raise(Signals::OP_CLAMPED);

    while exp > E_LIMIT {
        if digits.gt(&Intrinsics::<N>::COEFF_MEDIUM) {
            return overflow(sign, signals, ctx);
        } else {
            digits = digits.strict_mul(UInt::<N>::TEN);

            if let Some(correction) = extra_precision.scale_up::<N>(1) {
                let ofw;
                (digits, ofw) = digits.overflowing_add(correction.digits);

                if ofw {
                    return overflow(sign, signals, ctx);
                }
            }

            exp -= 1;
        }
    }

    let cb = ControlBlock::new(-exp as i16, sign, signals, ctx, extra_precision);
    D::new(digits, cb)
}

#[inline]
const fn construct_zero<const N: usize>(
    exp: i32,
    sign: Sign,
    mut signals: Signals,
    ctx: Context,
    extra_precision: ExtraPrecision,
) -> D<N> {
    let cb = if exp > i16::MAX as i32 + 1 {
        signals.raise(Signals::OP_CLAMPED);
        ControlBlock::new(i16::MIN, sign, signals, ctx, extra_precision)
    } else if exp <= i16::MIN as i32 {
        signals.raise(Signals::OP_CLAMPED);
        ControlBlock::new(i16::MAX, sign, signals, ctx, extra_precision)
    } else {
        ControlBlock::new(-exp as i16, sign, signals, ctx, extra_precision)
    };

    D::new(UInt::ZERO, cb)
}
