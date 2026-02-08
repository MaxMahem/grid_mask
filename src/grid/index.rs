use fluent_result::into::IntoResult;
use tap::Conv;

use crate::err::OutOfBounds;
use crate::num::{GridIndexU64, GridPos};
use crate::{Grid, GridMask, GridPoint};

/// A trait for types that can be used as an index into a grid.
///
/// This allows various types (like integer indices, coordinate pairs, etc.)
/// to be used interchangeably for grid lookups.
#[sealed::sealed]
pub trait GridIndex: Into<GridIndexU64> + Copy {
    /// Converts the value into a [`GridIndexU64`].
    fn to_index(self) -> GridIndexU64 {
        self.into()
    }

    /// Converts the value into a [`GridMask64`] with a single bit set at the index.
    fn to_grid_mask(self) -> GridMask {
        self.to_index().into()
    }
}

#[sealed::sealed]
impl<T: Into<GridIndexU64> + Copy> GridIndex for T {}

/// A trait for types that can be fallibly converted to a grid index.
///
/// This trait is implemented for types that may represent out-of-bounds positions.
#[sealed::sealed]
pub trait TryGridIndex: Copy {
    /// Tries to convert the value into a [`GridIndexU64`].
    fn try_to_index(self) -> Result<GridIndexU64, OutOfBounds>;

    /// Tries to convert the value into a [`GridMask64`] with the bit set,
    /// or [`GridMask::EMPTY`] if the value is out of bounds.
    fn to_grid_mask(self) -> GridMask {
        self.try_to_index().map_or(GridMask::EMPTY, Into::into)
    }
}

// Implement for GridIndexU64 itself (infallible)
#[sealed::sealed]
impl TryGridIndex for GridIndexU64 {
    fn try_to_index(self) -> Result<GridIndexU64, OutOfBounds> {
        Ok(self)
    }
}

// Implement for GridPoint (infallible)
#[sealed::sealed]
impl TryGridIndex for GridPoint {
    fn try_to_index(self) -> Result<GridIndexU64, OutOfBounds> {
        self.conv::<GridIndexU64>().into_ok()
    }
}

// Implement for generic tuples that can convert to GridPos
#[sealed::sealed]
impl<X, Y> TryGridIndex for (X, Y)
where
    X: TryInto<GridPos> + Copy,
    Y: TryInto<GridPos> + Copy,
{
    fn try_to_index(self) -> Result<GridIndexU64, OutOfBounds> {
        let x = self.0.try_into().map_err(OutOfBounds::new_from)?;
        let y = self.1.try_into().map_err(OutOfBounds::new_from)?;
        Ok(GridIndexU64::from((x, y)))
    }
}
