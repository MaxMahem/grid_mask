use std::num::NonZeroU64;
use tap::Pipe;

use crate::num::GridIndexU64;

pub trait UnsetBit: Sized {
    /// Unsets the lowest set bit.
    #[must_use]
    fn unset_low_bit(self) -> Self;

    /// Unsets the highest set bit.
    #[must_use]
    fn unset_high_bit(self) -> Self;

    /// Unsets the bit at the given index.
    #[must_use]
    fn unset_bit(self, index: GridIndexU64) -> Self;
}

impl UnsetBit for Option<NonZeroU64> {
    fn unset_low_bit(self) -> Self {
        self?.get().pipe(|val| NonZeroU64::new(val & (val - 1)))
    }

    fn unset_high_bit(self) -> Self {
        self?.get().pipe(|v| NonZeroU64::new(v & !(1 << v.ilog2())))
    }

    fn unset_bit(self, index: GridIndexU64) -> Self {
        self?.get().pipe(|v| NonZeroU64::new(v & !(1 << index.get())))
    }
}
