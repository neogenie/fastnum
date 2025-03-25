use core::cmp::Ordering::*;

use crate::decimal::{
    dec::{
        cmp::cmp,
        math::{add::add, atan::atan, div::div, sub::sub},
    },
    Decimal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn atan2<const N: usize>(y: D<N>, x: D<N>) -> D<N> {
    match (cmp(&x, &D::ZERO), cmp(&y, &D::ZERO)) {
        (Some(Equal), Some(Equal)) => x.compound(&y).signaling_nan(),
        (Some(Greater), _) => atan(div(y, x)),
        (Some(Less), Some(Greater) | Some(Equal)) => add(atan(div(y, x)), D::PI),
        (Some(Less), Some(Less)) => sub(atan(div(y, x)), D::PI),
        (Some(Equal), Some(Greater)) => D::FRAC_PI_2,
        (Some(Equal), Some(Less)) => D::FRAC_PI_2.neg(),
        _ => y.signaling_nan(),
    }
}
