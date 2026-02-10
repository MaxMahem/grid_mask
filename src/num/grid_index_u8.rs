use crate::ext::{Bound, debug_check_then};
use crate::num::{GridPos, VecMagU64};

bounded_integer::bounded_integer! {
    /// A position in a u8 bitmask.
    ///
    /// The valid range is 0 to 7.
    #[repr(u8)]
    pub struct BitIndexU8(0, 7);
}

impl Bound for BitIndexU8 {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    const COUNT: usize = (Self::MAX.get() - Self::MIN.get()) as usize + 1;

    fn increment(&self) -> Option<Self> {
        self.get().checked_add(1).and_then(Self::new)
    }

    fn decrement(&self) -> Option<Self> {
        self.get().checked_sub(1).and_then(Self::new)
    }

    fn remaining(&self) -> usize {
        (Self::MAX.get() - self.get()) as usize
    }
}

impl From<GridPos> for BitIndexU8 {
    fn from(val: GridPos) -> Self {
        let val: u8 = val.get();

        debug_check_then!(
            // Safety: GridPos is 0..=7 which is exactly BitIndexU8's range
            val <= Self::MAX.get() => unsafe { Self::new_unchecked(val)},
            "GridPos is out of range"
        )
    }
}

impl From<VecMagU64> for BitIndexU8 {
    fn from(value: VecMagU64) -> Self {
        // Safety: VecMagU64 is bounded to 1..=7
        unsafe { Self::new_unchecked(value.get()) }
    }
}
