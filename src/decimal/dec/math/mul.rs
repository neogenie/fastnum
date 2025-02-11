use crate::{
    decimal::{
        dec::{
            construct::construct,
            math::{add::add, utils::correct},
            ControlBlock,
        },
        Decimal,
    },
    int::{math::div_rem_wide, UInt},
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn mul<const N: usize>(mut lhs: D<N>, mut rhs: D<N>) -> D<N> {
    if lhs.is_nan() {
        return lhs.compound(&rhs).raise_op_invalid();
    }

    if rhs.is_nan() {
        return rhs.compound(&lhs).raise_op_invalid();
    }

    let sign = lhs.sign().mul(rhs.sign());

    if lhs.is_infinite() || rhs.is_infinite() {
        return if lhs.is_zero() || rhs.is_zero() {
            D::SIGNALING_NAN.compound(&lhs).compound(&rhs)
        } else {
            D::INFINITY.set_sign(sign)
        };
    }

    let mut exp = lhs.cb.get_exponent() + rhs.cb.get_exponent();

    if lhs.is_zero() {
        return construct(UInt::ZERO, exp, sign)
            .compound(&lhs)
            .compound(&rhs);
    }

    if rhs.is_zero() {
        return construct(UInt::ZERO, exp, sign)
            .compound(&lhs)
            .compound(&rhs);
    }

    let correction = mul_correction(lhs, rhs);

    let (mut low, mut high) = lhs.digits.widening_mul(rhs.digits);

    // TODO: Remove this temp partial CB
    let mut cb = ControlBlock::default();
    cb.compound(&lhs.cb);
    cb.compound(&rhs.cb);

    if !high.is_zero() {
        cb.raise_op_rounded();

        let mut out;
        let mut rem;

        while !high.is_zero() {
            exp += 1;

            out = [0; N];
            rem = 0;

            let mut i = N;
            while i > 0 {
                i -= 1;
                let (q, r) = div_rem_wide(high.digits()[i], rem, 10);
                rem = r;
                out[i] = q;
            }

            high = UInt::from_digits(out);

            i = N;
            out = [0; N];

            while i > 0 {
                i -= 1;
                let (q, r) = div_rem_wide(low.digits()[i], rem, 10);
                rem = r;
                out[i] = q;
            }

            low = UInt::from_digits(out);

            if rem != 0 {
                cb.raise_op_inexact();
            }

            cb.push_extra_precision_digit(rem);
        }
    }

    let mut result = construct(low, exp, sign);
    result.cb.set_extra_digits(cb.get_extra_digits());
    result.cb.compound(&cb);

    correct(result, correction)
}

#[inline]
const fn mul_correction<const N: usize>(mut lhs: D<N>, mut rhs: D<N>) -> D<N> {
    let xi_lhs = lhs.cb.take_extra_digits();
    let xi_rhs = rhs.cb.take_extra_digits();

    if xi_lhs.is_zero() && xi_rhs.is_zero() {
        D::ZERO
    } else if xi_lhs.is_zero() {
        mul(lhs, xi_rhs)
    } else if xi_rhs.is_zero() {
        mul(rhs, xi_lhs)
    } else {
        add(mul(lhs, xi_rhs), add(mul(rhs, xi_lhs), mul(xi_rhs, xi_lhs)))
    }
}
