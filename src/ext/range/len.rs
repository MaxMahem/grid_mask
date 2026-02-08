use std::ops::Range;

/// Extension trait for getting the length of a range.
pub trait RangeLength {
    type Len;

    /// Returns the length of the range.
    fn length(&self) -> Self::Len;
}

impl RangeLength for Range<u8> {
    type Len = u8;

    fn length(&self) -> Self::Len {
        self.end - self.start
    }
}
