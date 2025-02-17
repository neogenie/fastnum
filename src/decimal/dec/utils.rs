use crate::{
    decimal::{
        dec::{
            math::{add::add, sub::sub},
            ControlBlock, ExtraPrecision,
        },
        Decimal, Signals,
    },
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline(always)]
pub(crate) const fn add_extra_precision<const N: usize>(this: &mut D<N>, other: &D<N>) {
    debug_assert!(this.cb.get_scale() == other.cb.get_scale());

    if this.cb.add_extra_precision(&other.cb) {
        magnitude_inc(this);
    }
}

#[inline(always)]
pub(crate) const fn sub_extra_precision<const N: usize>(this: &mut D<N>, other: &D<N>) {
    debug_assert!(this.cb.get_scale() == other.cb.get_scale());

    if this.cb.sub_extra_precision(&other.cb) {
        magnitude_dec(this);
    }
}

#[inline]
pub const fn magnitude_inc<const N: usize>(d: &mut D<N>) {
    if d.is_negative() {
        *d = sub(
            *d,
            D::new(
                UInt::ONE,
                ControlBlock::new(
                    d.cb.get_scale(),
                    d.cb.get_sign(),
                    Signals::empty(),
                    d.context(),
                    ExtraPrecision::new(),
                ),
            ),
        );
    } else {
        *d = add(
            *d,
            D::new(
                UInt::ONE,
                ControlBlock::new(
                    d.cb.get_scale(),
                    d.cb.get_sign(),
                    Signals::empty(),
                    d.context(),
                    ExtraPrecision::new(),
                ),
            ),
        );
    }
}

#[inline]
pub const fn magnitude_dec<const N: usize>(d: &mut D<N>) {
    if d.is_negative() {
        *d = add(
            *d,
            D::new(
                UInt::ONE,
                ControlBlock::new(
                    d.cb.get_scale(),
                    d.cb.get_sign(),
                    Signals::empty(),
                    d.context(),
                    ExtraPrecision::new(),
                ),
            ),
        );
    } else {
        *d = sub(
            *d,
            D::new(
                UInt::ONE,
                ControlBlock::new(
                    d.cb.get_scale(),
                    d.cb.get_sign(),
                    Signals::empty(),
                    d.context(),
                    ExtraPrecision::new(),
                ),
            ),
        );
    }
}
