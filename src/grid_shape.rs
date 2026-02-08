use std::marker::PhantomData;
use std::str::FromStr;

use collect_failable::TryFromIterator;

use crate::adjacency::{Adjacency, Cardinal};
use crate::num::GridIndexU64;
use crate::{Discontiguous, GridMask, GridPoint, GridSize, OutOfBounds};

use crate::PatternError;

/// A contiguous shape on an 8x8 grid.
///
/// Unlike [`GridMask64`], a `GridShape` guarantees that all set cells are
/// connected via an [`Adjacency`] strategy.
///
/// # Type Parameters
///
/// * `Adj` - The type of [`Adjacency`] strategy
///
/// # Examples
///
/// ```rust
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// # use grid_mask::{GridShape, GridMask, GridPoint, GridRect};
/// // Create a shape from a rectangle (always contiguous)
/// let shape: GridShape = GridRect::try_new(0, 0, 2, 2)?.into();
///
/// assert!(shape.index(GridPoint::ORIGIN));
/// assert_eq!(shape.cells().filter(|&b| b).count(), 4);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, derive_more::Into, derive_more::Deref, derive_more::AsRef)]
pub struct GridShape<Adj: Adjacency = Cardinal>(#[deref] GridMask, PhantomData<Adj>);

impl<Adj: Adjacency> GridShape<Adj> {
    /// Creates a new mask
    const fn new(mask: GridMask) -> Self {
        Self(mask, PhantomData)
    }
}

impl GridShape {
    /// Creates a [`GridShape`] from a rectangle.
    ///
    /// Rectangles are always contiguous, so this cannot fail due to
    /// discontinuity.
    ///
    /// # Arguments
    ///
    /// * `pos` - The top-left corner of the rectangle.
    /// * `size` - The dimensions of the rectangle.
    ///
    /// # Errors
    ///
    /// [`OutOfBounds`] if the rectangle would extend beyond the grid limits.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # use grid_mask::{GridShape, GridPoint, GridSize};
    /// let shape = GridShape::from_rect(GridPoint::try_new(0, 0)?, GridSize::try_new(3, 2)?)?;
    ///
    /// // 3x2 rectangle = 6 cells
    /// assert_eq!(shape.cells().filter(|&b| b).count(), 6);
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_rect(pos: GridPoint, size: GridSize) -> Result<Self, OutOfBounds> {
        GridMask::from_rect(pos, size).map(Self::new)
    }

    /// Creates a [`GridShape`] from a string pattern.
    ///
    /// The pattern must contain exactly 64 characters matching either `set` or
    /// `unset`, ignoring any whitespace.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The string pattern to parse.
    /// * `set` - The character representing a set bit.
    /// * `unset` - The character representing an unset bit.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The pattern parsing fails (see [`GridMask::from_pattern`]).
    /// * The resulting mask is not contiguous.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # use grid_mask::GridShape;
    /// let pattern = "
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . # # . . . .
    ///     . . # # . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    /// ";
    /// let shape = GridShape::from_pattern(pattern, '#', '.')?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_pattern<S: AsRef<str>>(pattern: S, set: char, unset: char) -> Result<Self, PatternError> {
        let mask = GridMask::from_pattern(pattern, set, unset)?;
        mask.try_into().map_err(|_: Discontiguous| PatternError::NotContiguous)
    }
}

impl<Adj: Adjacency> TryFrom<GridMask> for GridShape<Adj> {
    type Error = Discontiguous;

    /// Creates a [`GridShape`] from a [`GridMask64`] if the mask is contiguous.
    ///
    /// A mask is contiguous if all set cells are connected via the adjacency rule `Adj`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{GridShape, GridMask, GridPoint};
    /// // A single point is contiguous
    /// let mask = GridMask::from(GridPoint::ORIGIN);
    /// let shape: Result<GridShape, _> = mask.try_into();
    /// assert!(shape.is_ok());
    /// ```
    fn try_from(mask: GridMask) -> Result<Self, Self::Error> {
        // Find any set bit as seed
        let seed = GridIndexU64::from_first_set(mask.0).ok_or(Discontiguous(mask))?;

        // Mask is contiguous iff connected region equals original mask
        let connected = mask.connected::<Adj>(seed);
        (connected == mask).then_some(Self::new(mask)).ok_or(Discontiguous(mask))
    }
}

impl FromStr for GridShape {
    type Err = PatternError;

    /// Parses a string pattern into a [`GridShape`].
    ///
    /// Uses `#` for set cells and `.` for unset cells. Whitespace is ignored.
    ///
    /// # Errors
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # use grid_mask::GridShape;
    /// let pattern = "
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . # # . . . .
    ///     . . # # . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    /// ";
    /// let shape: GridShape = pattern.parse()?;
    /// assert_eq!(shape.count(), 4);
    /// # Ok(())
    /// # }
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_pattern(s, '#', '.')
    }
}

impl<T: Into<GridMask>, I: IntoIterator<Item = T>, Adj: Adjacency> TryFromIterator<I> for GridShape<Adj> {
    type Error = Discontiguous;

    fn try_from_iter(iter: I) -> Result<Self, Self::Error> {
        GridMask::from_iter(iter).try_into()
    }
}
