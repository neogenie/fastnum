use crate::decimal::{
    dec::{
        convert::to_i32,
        math::{exp::exp, ln::ln, mul::mul, powi::powi},
    },
    Decimal, Sign,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn pow<const N: usize>(d: D<N>, n: D<N>) -> D<N> {
    if n.is_integral() {
        if let Ok(n) = to_i32(n) {
            return powi(d, n);
        }
    }

    if d.is_nan() {
        return d.op_invalid();
    }

    let sign = if d.is_negative() && n.is_even() {
        Sign::Minus
    } else {
        Sign::Plus
    };

    if d.cb.is_infinity() {
        return if n.is_zero() {
            D::ONE
        } else if n.is_negative() {
            D::ZERO.set_sign(sign)
        } else if sign.is_negative() ^ d.is_negative() {
            d.neg()
        } else {
            d
        };
    }

    if n.is_zero() {
        return if d.is_zero() {
            d.signaling_nan()
        } else {
            D::ONE
        };
    }

    if d.is_zero() {
        return if n.is_negative() {
            D::INFINITY.set_ctx(d.context()).set_sign(sign)
        } else {
            D::ZERO.set_ctx(d.context()).set_sign(sign)
        };
    }

    powf(d, n)
}

#[inline]
const fn powf<const N: usize>(d: D<N>, n: D<N>) -> D<N> {
    debug_assert!(!d.is_negative());
    debug_assert!(!d.is_zero());
    exp(mul(ln(d), n))
}
