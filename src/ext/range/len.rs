use std::ops::{Range, RangeInclusive};

use crate::num::{GridLen, GridPos};

/// Extension trait for getting the length of a range as a `u32`.
pub trait Len32 {
    /// Returns the length of the range as a `u32`.
    fn len_32(&self) -> u32;
}

impl Len32 for Range<u32> {
    fn len_32(&self) -> u32 {
        self.end - self.start
    }
}

pub trait Length {
    type Len;

    fn length(&self) -> Self::Len;
}

impl Length for Range<GridPos> {
    type Len = GridLen;

    fn length(&self) -> Self::Len {
        let len = (self.end - self.start).get();
        unsafe { GridLen::new_unchecked(len) }
    }
}

impl Length for RangeInclusive<GridPos> {
    type Len = GridLen;

    fn length(&self) -> Self::Len {
        let len = (self.end() - self.start()).get() + 1;
        unsafe { GridLen::new_unchecked(len) }
    }
}
