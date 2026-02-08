use std::ops::Range;

pub trait OccupiedBitSpan {
    /// Returns a half-open range `start..end` of the occupied bits on an unsigned value.
    /// Returns an empty range if the value is 0.
    fn occupied_span(self) -> Range<u32>;
}

impl OccupiedBitSpan for u64 {
    fn occupied_span(self) -> Range<u32> {
        if self == 0 {
            return 0..0;
        }
        let start = self.trailing_zeros();
        let end = 64 - self.leading_zeros();
        start..end
    }
}

impl OccupiedBitSpan for u8 {
    fn occupied_span(self) -> Range<u32> {
        if self == 0 {
            return 0..0;
        }
        let start = self.trailing_zeros();
        let end = 8 - self.leading_zeros();
        start..end
    }
}
