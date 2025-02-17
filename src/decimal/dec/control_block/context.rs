use crate::decimal::{dec::ControlBlock, Context, RoundingMode, Signals, SignalsTraps};

const SIGNAL_TRAPS_SHIFT: u8 = 27;
const ROUNDING_MODE_SHIFT: u8 = 37;

/// Control block (CB)
///
/// Memory layout:
///
/// |  Bit  |          Data         |         Bit Mask        |
/// |:-----:|:---------------------:|:-----------------------:|
/// | `...` |      `...`            |          `...`          |
/// | 27    | T OP_CLAMPED          | `0x0000_0000_0800_0000` |
/// | 28    | T OP_DIV_BY_ZERO      | `0x0000_0000_1000_0000` |
/// | 29    | T OP_INVALID          | `0x0000_0000_2000_0000` |
/// | 30    | T OP_INEXACT          | `0x0000_0000_4000_0000` |
/// | 31    | T OP_OVERFLOW         | `0x0000_0000_8000_0000` |
/// | 32    | T OP_ROUNDED          | `0x0000_0001_0000_0000` |
/// | 33    | T OP_SUBNORMAL        | `0x0000_0002_0000_0000` |
/// | 34    | T OP_UNDERFLOW        | `0x0000_0004_0000_0000` |
/// | 35    |      Reserved         | `0x0000_0008_0000_0000` |
/// | 36    |      Reserved         | `0x0000_0010_0000_0000` |
/// | 37-39 | Rounding mode (3 bit) | `0x0000_00E0_0000_0000` |
/// | `...` |      `...`            |          `...`          |
impl ControlBlock {
    pub(super) const CONTEXT_MASK: u64 = 0x0000_00E7_F800_0000;
    pub(super) const SIGNAL_TRAPS_MASK: u64 = 0x0000_0007_F800_0000;
    pub(super) const ROUNDING_MODE_MASK: u64 = 0x0000_00E0_0000_0000;

    pub(super) const DEFAULT_CONTEXT: u64 = make_context(Context::default());

    #[inline(always)]
    pub const fn get_context(&self) -> Context {
        Context::new(self.get_rounding_mode(), self.get_signal_traps())
    }

    #[inline(always)]
    pub const fn set_context(&mut self, ctx: Context) {
        self.0 = (self.0 & !Self::CONTEXT_MASK) | make_context(ctx);
    }

    #[inline(always)]
    pub const fn get_rounding_mode(&self) -> RoundingMode {
        #[allow(unsafe_code)]
        unsafe {
            core::mem::transmute(((self.0 & Self::ROUNDING_MODE_MASK) >> ROUNDING_MODE_SHIFT) as u8)
        }
    }

    #[inline(always)]
    pub const fn set_rounding_mode(&mut self, rm: RoundingMode) {
        self.0 = (self.0 & !Self::ROUNDING_MODE_MASK) | rounding_mode(rm);
    }

    #[inline(always)]
    pub const fn get_signal_traps(&self) -> SignalsTraps {
        SignalsTraps::new(Signals::new(
            ((self.0 & Self::SIGNAL_TRAPS_MASK) >> SIGNAL_TRAPS_SHIFT) as u8,
        ))
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub const fn set_signal_traps(&mut self, traps: SignalsTraps) {
        self.0 = (self.0 & !Self::SIGNAL_TRAPS_MASK) | signals_traps(traps);
    }

    #[inline(always)]
    pub(super) const fn combine_ctx(&mut self, other: &Self) {
        self.0 |= other.0 & Self::SIGNAL_TRAPS_MASK;
        let rm = other.get_rounding_mode();
        if !rm.is_default() {
            self.set_rounding_mode(rm);
        }
    }
}

#[inline(always)]
const fn make_context(ctx: Context) -> u64 {
    rounding_mode(ctx.rounding_mode()) | signals_traps(ctx.signal_traps())
}

#[inline(always)]
const fn rounding_mode(rm: RoundingMode) -> u64 {
    let rm_u8 = rm as u8;
    (rm_u8 as u64) << ROUNDING_MODE_SHIFT
}

#[inline(always)]
const fn signals_traps(traps: SignalsTraps) -> u64 {
    (traps.signals().mask() as u64) << SIGNAL_TRAPS_SHIFT
}
