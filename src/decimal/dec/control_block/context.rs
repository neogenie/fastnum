use crate::decimal::{Context, RoundingMode};
use crate::decimal::dec::ControlBlock;

/// Control block (CB)
///
/// Memory layout:
///
/// |  Bit  |          Data         |         Bit Mask        |
/// |:-----:|:---------------------:|:-----------------------:|
/// | `...` |      `...`            |          `...`          |
/// | 27    | T OP_CLAMPED          | `0x0000_0000_0800_0000` |
/// | 28    | T OP_DIV_BY_ZERO      | `0x0000_0000_1000_0000` |
/// | 29    | T OP_INEXACT          | `0x0000_0000_2000_0000` |
/// | 30    | T OP_INVALID          | `0x0000_0000_4000_0000` |
/// | 31    | T OP_OVERFLOW         | `0x0000_0000_8000_0000` |
/// | 32    | T OP_ROUNDED          | `0x0000_0001_0000_0000` |
/// | 33    | T OP_SUBNORMAL        | `0x0000_0002_0000_0000` |
/// | 34    | T OP_UNDERFLOW        | `0x0000_0004_0000_0000` |
/// | 35    |      Reserved         | `0x0000_0008_0000_0000` |
/// | 36-39 | Rounding mode (4 bit) | `0x0000_00F0_0000_0000` |
/// | `...` |      `...`            |          `...`          |
impl ControlBlock {
    #[inline(always)]
    pub const fn get_context(&self) -> Context {
        todo!()
    }
    
    #[inline(always)]
    pub const fn set_context(&mut self, ctx: Context) {
        todo!()
    }

    #[inline(always)]
    pub(super) const fn combine_ctx(&mut self, other: &Self) {

    }

    #[inline(always)]
    pub const fn set_rounding_mode(&mut self, rm: RoundingMode) {
        todo!()
    }
}