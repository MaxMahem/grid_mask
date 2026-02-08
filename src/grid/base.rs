use std::ops::{BitAnd, BitOr, BitXor, Not};

use crate::{Adjacency, GridVector, ext::Bound};

/// A trait representing a bitmask over a grid.
///
/// This trait defines core operations for grid masks, including:
/// - Grid dimensions (`ROWS`, `COLS`)
/// - Empty/full states (`EMPTY`, `FULL`)
/// - Bit counting and checking (`count`, `is_empty`, `is_full`)
///
/// Implementations must also support standard bit operations via supertraits.
pub trait Grid:
    Sized
    + Copy
    + Default
    + PartialEq
    + Eq
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
{
    /// The backing storage type for the grid mask.
    type Backing: Copy;

    /// The type used to index the grid.
    type Idx: Bound + Into<Self>;

    /// Number of rows in the grid.
    const ROWS: u8;

    /// Number of columns in the grid.
    const COLS: u8;

    /// An empty grid mask (no cells set).
    const EMPTY: Self;

    /// A full grid mask (all cells set).
    const FULL: Self;

    /// Returns the number of set cells in the mask.
    fn count(&self) -> usize;

    /// Returns `true` if the mask is empty (no cells set).
    fn is_empty(&self) -> bool {
        *self == Self::EMPTY
    }

    /// Returns `true` if the mask is full (all cells set).
    fn is_full(&self) -> bool {
        *self == Self::FULL
    }

    /// Translates the mask by a vector.
    ///
    /// # Arguments
    ///
    /// * `vector` - The vector to translate the mask by
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{Grid, GridShape, GridVector, Cardinal};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let shape = GridShape::<Cardinal>::FULL;
    ///
    /// let translated = shape.translate(GridVector::SOUTH_EAST)?;
    ///
    /// let bounds = translated.bounds();
    ///
    /// assert_eq!(bounds.point(), (1, 1));
    /// assert_eq!(bounds.size(), (7, 7));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    fn translate(&self, vector: GridVector) -> Self;

    /// Grows the mask according to [`Adjacency`].
    ///
    /// # Type Parameters
    ///
    /// * `A` - The [`Adjacency`] strategy to use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use grid_mask::{Grid, GridMask, Cardinal};
    /// let crosses: GridMask = "
    ///     . . . . . . . .
    ///     . . . # . . . .
    ///     . . # # # . . .
    ///     . . . # . . . .
    ///     . . . . . . . .
    ///     . . . . . . # .
    ///     . . . . . # # #
    ///     . . . . . . # .
    /// ".parse()?;
    ///
    /// let grown = crosses.grow::<Cardinal>();
    ///
    /// let diamonds: GridMask = "
    ///     . . . # . . . .
    ///     . . # # # . . .
    ///     . # # # # # . .
    ///     . . # # # . . .
    ///     . . . # . . # .
    ///     . . . . . # # #
    ///     . . . . # # # #
    ///     . . . . . # # #
    /// "
    /// .parse()?;
    ///
    /// assert_eq!(grown, diamonds, "crosses should grow to diamonds");
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    fn grow<A: Adjacency>(self) -> Self {
        A::grow(self)
    }

    /// Returns a [`GridMask64`] of all points connected to `seed` within the current mask
    /// using the provided [`Adjacency`].
    ///
    /// # Arguments
    ///
    /// * `seed` - The starting point for the flood fill.
    ///
    /// # Type Parameters
    ///
    /// * `A` - The [`Adjacency`] strategy to use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # use grid_mask::{Grid, GridPoint, GridMask, GridRect, Cardinal};
    /// let mask: GridMask = GridRect::new((0, 0), (2, 2))?.into();
    /// let connected = mask.connected::<Cardinal>(GridPoint::ORIGIN);
    /// assert_eq!(connected, mask);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    fn connected<A: Adjacency>(&self, seed: impl Into<Self::Idx>) -> Self {
        let mut connected = match seed.into().into() & *self {
            mask if mask == Grid::EMPTY => return mask,
            mask => mask,
        };

        loop {
            match A::grow(connected) & *self {
                grown if grown == connected => break connected,
                grown => connected = grown,
            }
        }
    }
}
