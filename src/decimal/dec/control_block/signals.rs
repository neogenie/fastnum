use crate::decimal::dec::ControlBlock;

/// Control block (CB)
///
/// Signals memory layout:
///
/// |  Bit  |          Data         |         Bit Mask        |
/// |:-----:|:---------------------:|:-----------------------:|
/// | `...` |      `...`            |          `...`          |
/// | 19    |   OP_CLAMPED          | `0x0000_0000_0008_0000` |
/// | 20    | OP_DIV_BY_ZERO        | `0x0000_0000_0010_0000` |
/// | 21    | OP_INEXACT            | `0x0000_0000_0020_0000` |
/// | 22    | OP_INVALID            | `0x0000_0000_0040_0000` |
/// | 23    | OP_OVERFLOW           | `0x0000_0000_0080_0000` |
/// | 24    | OP_ROUNDED            | `0x0000_0000_0100_0000` |
/// | 25    | OP_SUBNORMAL          | `0x0000_0000_0200_0000` |
/// | 26    | OP_UNDERFLOW          | `0x0000_0000_0400_0000` |
/// | `...` |      `...`            |          `...`          |
impl ControlBlock {
    pub(super) const SIGNALS_MASK: u64 = 0x0000_0000_07F8_0000;

    pub(super) const OP_CLAMPED_MASK: u64 = 0x0000_0000_0008_0000;
    pub(super) const OP_DIV_BY_ZERO_MASK: u64 = 0x0000_0000_0010_0000;
    pub(super) const OP_INEXACT_MASK: u64 = 0x0000_0000_0020_0000;
    pub(super) const OP_INVALID_MASK: u64 = 0x0000_0000_0040_0000;
    pub(super) const OP_OVERFLOW_MASK: u64 = 0x0000_0000_0080_0000;
    pub(super) const OP_ROUNDED_MASK: u64 = 0x0000_0000_0100_0000;
    pub(super) const OP_SUBNORMAL_MASK: u64 = 0x0000_0000_0200_0000;
    pub(super) const OP_UNDERFLOW_MASK: u64 = 0x0000_0000_0400_0000;

    #[inline(always)]
    pub const fn is_op_div_by_zero(&self) -> bool {
        self.0 & Self::OP_DIV_BY_ZERO_MASK != 0
    }

    #[inline(always)]
    pub const fn is_op_overflow(&self) -> bool {
        self.0 & Self::OP_OVERFLOW_MASK != 0
    }

    #[inline(always)]
    pub const fn is_op_underflow(&self) -> bool {
        self.0 & Self::OP_UNDERFLOW_MASK != 0
    }

    #[inline(always)]
    pub const fn is_op_invalid(&self) -> bool {
        self.0 & Self::OP_INVALID_MASK != 0
    }

    #[inline(always)]
    pub const fn is_op_subnormal(&self) -> bool {
        self.0 & Self::OP_SUBNORMAL_MASK != 0
    }

    #[inline(always)]
    pub const fn is_op_inexact(&self) -> bool {
        self.0 & Self::OP_INEXACT_MASK != 0
    }

    #[inline(always)]
    pub const fn is_op_rounded(&self) -> bool {
        self.0 & Self::OP_ROUNDED_MASK != 0
    }

    #[inline(always)]
    pub const fn is_op_clamped(&self) -> bool {
        self.0 & Self::OP_CLAMPED_MASK != 0
    }

    #[inline(always)]
    pub const fn is_op_ok(&self) -> bool {
        self.0 & Self::SIGNALS_MASK == 0
    }

    #[inline(always)]
    pub const fn raise_op_div_by_zero(&mut self) {
        self.0 |= Self::OP_DIV_BY_ZERO_MASK;
    }

    #[inline(always)]
    pub const fn raise_op_overflow(&mut self) {
        self.0 |= Self::OP_OVERFLOW_MASK;
    }

    #[inline(always)]
    pub const fn raise_op_underflow(&mut self) {
        self.0 |= Self::OP_UNDERFLOW_MASK;
    }

    #[inline(always)]
    pub const fn raise_op_invalid(&mut self) {
        self.0 |= Self::OP_INVALID_MASK;
    }

    #[inline(always)]
    pub const fn raise_op_subnormal(&mut self) {
        self.0 |= Self::OP_SUBNORMAL_MASK;
    }

    #[inline(always)]
    pub const fn raise_op_inexact(&mut self) {
        self.0 |= Self::OP_INEXACT_MASK;
    }

    #[inline(always)]
    pub const fn raise_op_rounded(&mut self) {
        self.0 |= Self::OP_ROUNDED_MASK;
    }

    #[inline(always)]
    pub const fn raise_op_clamped(&mut self) {
        self.0 |= Self::OP_CLAMPED_MASK;
    }

    #[inline(always)]
    pub const fn quiet_op_div_by_zero(&mut self) {
        self.0 &= !Self::OP_DIV_BY_ZERO_MASK;
    }

    #[inline(always)]
    pub const fn quiet_op_overflow(&mut self) {
        self.0 &= !Self::OP_OVERFLOW_MASK;
    }

    #[inline(always)]
    pub const fn quiet_op_underflow(&mut self) {
        self.0 &= !Self::OP_UNDERFLOW_MASK;
    }

    #[inline(always)]
    pub const fn quiet_op_invalid(&mut self) {
        self.0 &= !Self::OP_INVALID_MASK;
    }

    #[inline(always)]
    pub const fn quiet_op_subnormal(&mut self) {
        self.0 &= !Self::OP_SUBNORMAL_MASK;
    }

    #[inline(always)]
    pub const fn quiet_op_inexact(&mut self) {
        self.0 &= !Self::OP_INEXACT_MASK;
    }

    #[inline(always)]
    pub const fn quiet_op_rounded(&mut self) {
        self.0 &= !Self::OP_ROUNDED_MASK;
    }

    #[inline(always)]
    pub const fn quiet_op_clamped(&mut self) {
        self.0 &= !Self::OP_CLAMPED_MASK;
    }
    
    
    // ???
    // #[inline(always)]
    // pub const fn op_ok(&mut self) {
    //     self.0 |= Self::SIGNALS_MASK;
    // }

    #[inline(always)]
    pub(super) const fn combine_signals(&mut self, other: &Self) {}
}
