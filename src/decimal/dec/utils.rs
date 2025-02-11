use crate::{
    decimal::{
        dec::{
            math::{add::add, sub::sub},
            ControlBlock, ExtraPrecision,
        },
        Decimal,
    },
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline(always)]
pub(crate) const fn add_extra_precision<const N: usize>(this: &mut D<N>, other: &D<N>) {
    if this.cb.add_extra_digits(&other.cb) {
        magnitude_inc(this);
    }
}

#[inline(always)]
pub(crate) const fn sub_extra_precision<const N: usize>(this: &mut D<N>, other: &D<N>) {
    if this.cb.sub_extra_digits(&other.cb) {
        magnitude_dec(this);
    }
}


#[inline]
pub const fn magnitude_inc<const N: usize>(d: &mut D<N>) {
    if d.is_negative() {
        sub(
            d,
            D::new(
                UInt::ONE,
                d.cb.get_scale(),
                ControlBlock::default(),
                ExtraPrecision::new(),
            ),
        )
    } else {
        add(
            d,
            D::new(
                UInt::ONE,
                d.cb.get_scale(),
                ControlBlock::default(),
                ExtraPrecision::new(),
            ),
        )
    }
}

#[inline]
pub const fn magnitude_dec<const N: usize>(d: &mut D<N>) {
    if d.is_negative() {
        add(
            d,
            D::new(
                UInt::ONE,
                d.cb.get_scale(),
                ControlBlock::default(),
                ExtraPrecision::new(),
            ),
        )
    } else {
        sub(
            d,
            D::new(
                UInt::ONE,
                d.cb.get_scale(),
                ControlBlock::default(),
                ExtraPrecision::new(),
            ),
        )
    }
}
