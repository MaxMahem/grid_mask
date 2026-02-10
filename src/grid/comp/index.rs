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

// /// A trait for types that can be fallibly converted to a grid index.
// ///
// /// This trait is implemented for types that may represent out-of-bounds positions.
// #[sealed::sealed]
// pub trait TryGridIndex: Copy {
//     /// Tries to convert the value into a [`GridIndexU64`].
//     fn try_to_index(self) -> Result<BitIndexU64, OutOfBounds>;

//     /// Tries to convert the value into a [`GridMask`] with the bit set,
//     /// or [`GridMask::EMPTY`] if the value is out of bounds.
//     fn to_grid_mask(self) -> GridMask {
//         self.try_to_index().map_or(GridMask::EMPTY, Into::into)
//     }
// }

// // Implement for GridIndexU64 itself (infallible)
// #[sealed::sealed]
// impl TryGridIndex for BitIndexU64 {
//     fn try_to_index(self) -> Result<BitIndexU64, OutOfBounds> {
//         Ok(self)
//     }
// }

// // Implement for GridPoint (infallible)
// #[sealed::sealed]
// impl TryGridIndex for GridPoint {
//     fn try_to_index(self) -> Result<BitIndexU64, OutOfBounds> {
//         self.conv::<BitIndexU64>().into_ok()
//     }
// }

// // Implement for generic tuples that can convert to GridPos
// #[sealed::sealed]
// impl<X, Y> TryGridIndex for (X, Y)
// where
//     X: TryInto<GridPos> + Copy,
//     Y: TryInto<GridPos> + Copy,
// {
//     fn try_to_index(self) -> Result<BitIndexU64, OutOfBounds> {
//         let x = self.0.try_into().map_err(OutOfBounds::new_from)?;
//         let y = self.1.try_into().map_err(OutOfBounds::new_from)?;
//         Ok(BitIndexU64::from((x, y)))
//     }
// }
