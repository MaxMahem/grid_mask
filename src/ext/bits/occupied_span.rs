use std::ops::Range;

use num_traits::PrimInt;

use crate::ext::bits::{BitZeros, NumBits};

pub trait OccupiedBitSpan {
    /// Returns a range (`start_index..end_index`) of the occupied bits.
    ///
    /// If the value is 0, the range will be empty.
    fn occupied_span(self) -> Range<u8>;
}

impl<T: PrimInt + NumBits + BitZeros> OccupiedBitSpan for T {
    fn occupied_span(self) -> Range<u8> {
        match self == T::zero() {
            true => 0..0,
            false => self.trailing_zeros_u8()..T::BITS - self.leading_zeros_u8(),
        }
    }
}
