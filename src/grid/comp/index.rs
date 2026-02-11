use crate::grid::GridData;

/// A trait for types that can be used as an index into a grid.
///
/// This allows various types (like integer indices, coordinate pairs, etc.)
/// to be used interchangeably for grid lookups.
#[sealed::sealed]
pub trait GridIndex<G: GridData>: Copy {
    /// Converts the value into a [`GridIndexU64`].
    fn to_index(self) -> G::Index;
}

#[sealed::sealed]
impl<G: GridData, T: Into<G::Index> + Copy> GridIndex<G> for T {
    fn to_index(self) -> G::Index {
        self.into()
    }
}
