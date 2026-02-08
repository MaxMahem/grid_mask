use std::marker::PhantomData;
use std::str::FromStr;

use collect_failable::TryFromIterator;

use crate::err::{Discontiguous, ShapePatternError};
use crate::grid::data::GridData;
use crate::num::GridIndexU64;
use crate::{Adjacency, Cardinal, Grid, GridMask, GridRect, GridVector};

/// A contiguous shape on an 8x8 grid.
///
/// A `GridShape` is a [`GridMask64`] that guarantees that all set cells are
/// connected via the [`Adjacency`] strategy, `A`.
///
/// # Type Parameters
///
/// * `A` - The type of [`Adjacency`] strategy
///
/// # Examples
///
/// ```rust
/// # use grid_mask::{Grid, GridShape, GridMask, GridPoint, GridRect, GridSize};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create a shape from a rectangle (always contiguous)
/// let shape: GridShape = GridRect::new(GridPoint::ORIGIN, (2, 2))?.into();
///
/// assert!(shape.index(GridPoint::ORIGIN));
/// assert!(shape.index((1, 1)));
/// assert_eq!(shape.count(), 4);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, derive_more::Into, derive_more::Deref, derive_more::AsRef)]
pub struct GridShape<A: Adjacency = Cardinal>(#[deref] GridMask, PhantomData<A>);

impl<A: Adjacency> GridShape<A> {
    /// A shape that contains all cells.
    pub const FULL: Self = Self(GridMask::FULL, PhantomData);

    /// Creates a new mask
    pub(crate) const fn new(mask: GridMask) -> Self {
        Self(mask, PhantomData)
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
    /// # use grid_mask::{Grid, GridShape};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    /// let shape: GridShape = GridShape::from_pattern(pattern, '#', '.')?;
    ///
    /// assert!(shape.index((2, 2)));
    /// assert!(shape.index((3, 3)));
    /// assert_eq!(shape.count(), 4);
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_pattern<S: AsRef<str>>(pattern: S, set: char, unset: char) -> Result<Self, ShapePatternError> {
        GridMask::from_pattern(pattern, set, unset)?.try_into().map_err(Into::into)
    }

    /// Returns the bounds of the shape.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{GridShape, GridRect, Cardinal};
    /// let bounds = GridShape::<Cardinal>::FULL.bounds();
    /// assert_eq!(bounds, GridRect::MAX);
    /// ```
    #[must_use]
    pub fn bounds(&self) -> GridRect {
        let bounds = self.0.bounds();

        // SAFETY: A shape always has a set cell, and thus always has bounds
        debug_assert!(bounds.is_some());
        unsafe { bounds.unwrap_unchecked() }
    }

    /// Translates the shape by `vector`.
    ///
    /// # Arguments
    ///
    /// * `vector` - The vector to translate the shape by
    ///
    /// # Errors
    ///
    /// [`Discontiguous`] if the translated shape is empty or not contiguous
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
    pub fn translate(self, vector: GridVector) -> Result<Self, Discontiguous> {
        self.0.translate(vector).try_into()
    }

    /// Changes the shape into a new shape with a different [`Adjacency`] rule.
    ///
    /// # Type Parameters
    ///
    /// * `A2` - The new [`Adjacency`] rule.
    ///
    /// # Errors
    ///
    /// [`Discontiguous`] if the new adjacency rule makes the new shape non-contiguous.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use grid_mask::{Grid, GridShape, GridMask, GridPoint, Cardinal, Octile};
    /// let shape: GridShape<Cardinal> = GridMask::from(GridPoint::ORIGIN).try_into()?;
    /// let shape2: GridShape<Octile> = shape.cast::<Octile>()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cast<A2: Adjacency>(self) -> Result<GridShape<A2>, Discontiguous> {
        self.0.try_into()
    }
}

impl<A: Adjacency> TryFrom<GridMask> for GridShape<A> {
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
        let connected = mask.connected::<A>(seed);
        (connected == mask).then_some(Self(mask, PhantomData)).ok_or(Discontiguous(mask))
    }
}

impl<A: Adjacency> TryFrom<u64> for GridShape<A> {
    type Error = Discontiguous;

    fn try_from(mask: u64) -> Result<Self, Self::Error> {
        GridMask::from(mask).try_into()
    }
}

impl<Adj: Adjacency> TryFrom<[bool; 64]> for GridShape<Adj> {
    type Error = Discontiguous;

    fn try_from(bools: [bool; 64]) -> Result<Self, Self::Error> {
        GridMask::from(bools).try_into()
    }
}

impl FromStr for GridShape {
    type Err = ShapePatternError;

    /// Parses a string pattern into a [`GridShape`].
    ///
    /// Uses `#` for set cells and `.` for unset cells. Whitespace is ignored.
    ///
    /// # Errors
    ///
    /// Errors if:
    ///
    /// * The pattern is empty or not contiguous ([`ShapePatternError::Discontiguous`])
    /// * The pattern contains characters other than `#`, `.` and whitespace
    ///   ([`ShapePatternError::Pattern`])
    /// * The pattern is longer or shorter than 64 characters ([`ShapePatternError::Pattern`])
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use grid_mask::{Grid, GridShape};
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
    ///
    /// let shape: GridShape = pattern.parse()?;
    ///
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
