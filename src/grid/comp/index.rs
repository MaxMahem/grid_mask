use crate::num::BitIndexU64;

/// A trait for types that can be used as an index into a grid.
///
/// This allows various types (like integer indices, coordinate pairs, etc.)
/// to be used interchangeably for grid lookups.
#[sealed::sealed]
pub trait GridIndex: Copy {
    /// Converts the value into a [`GridIndexU64`].
    fn to_index(self) -> BitIndexU64;
}

#[sealed::sealed]
impl<T: Into<BitIndexU64> + Copy> GridIndex for T {
    fn to_index(self) -> BitIndexU64 {
        self.into()
    }
}
