use fluent_result::into::IntoResult;
use tap::Pipe;

use crate::GridVector;
use crate::err::OutOfBounds;
use crate::ext::{Bound, BoundedIter};
use crate::num::{BitIndexU64, GridPos};

/// A point in a 8x8 grid.
#[derive(
    Debug, // col format
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Display,
)]
#[display("({x}, {y})", x = self.x(), y = self.y())]
pub struct GridPoint(pub BitIndexU64);

impl GridPoint {
    /// The origin point `(0, 0)`.
    pub const ORIGIN: Self = Self(BitIndexU64::MIN);
    /// The maximum point `(7, 7)`.
    pub const MAX: Self = Self(BitIndexU64::MAX);

    /// Creates a new [`GridPoint`] without bounds checking.
    ///
    /// The caller must ensure that `x` and `y` are within the range `0..=7`.
    #[must_use]
    pub(crate) fn new_unchecked(x: u8, y: u8) -> Self {
        debug_assert!(x <= 7, "x should be within 0..=7");
        debug_assert!(y <= 7, "y should be within 0..=7");

        let index = x + y * 8;
        debug_assert!(index <= 63, "index should be within 0..=63");

        unsafe { BitIndexU64::new_unchecked(index) }.pipe(Self)
    }

    /// Creates a new [`GridPoint`]
    ///
    /// ```rust
    /// # use grid_mask::GridPoint;
    /// # use grid_mask::num::GridPos;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let x = GridPos::try_from(3u8)?;
    /// let y = GridPos::try_from(4u8)?;
    ///
    /// let point = GridPoint::new(x, y);
    ///
    /// assert_eq!(point, (3, 4), "Should match");
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn new(x: GridPos, y: GridPos) -> Self {
        BitIndexU64::at(x, y).pipe(Self)
    }

    /// Tries to create a new [`GridPoint`].
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the point.
    /// * `y` - The y coordinate of the point.
    ///
    /// # Type Parameters
    ///
    /// * `X` - The type of the x coordinate.
    /// * `Y` - The type of the y coordinate.
    ///
    /// # Errors
    ///
    /// [`OutOfBounds`] if the point would extend beyond the limits of the grid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::GridPoint;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let point = GridPoint::try_new(3u32, 4u64)?;
    ///
    /// assert_eq!(point, (3, 4), "Should match");
    ///
    /// GridPoint::try_new(8, -4).expect_err("Should be invalid");
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_new<X: TryInto<GridPos>, Y: TryInto<GridPos>>(x: X, y: Y) -> Result<Self, OutOfBounds> {
        let x = x.try_into().map_err(OutOfBounds::from)?;
        let y = y.try_into().map_err(OutOfBounds::from)?;
        Self::new(x, y).into_ok()
    }

    /// Creates a new [`GridPoint`] from constant coordinates.
    ///
    /// This function enforces bounds checking at compile time.
    ///
    /// # Panics
    ///
    /// This function panics (failing at compile time) if `X` or `Y` are >= 8.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::GridPoint;
    /// const POINT: GridPoint = GridPoint::const_new::<3, 4>();
    /// assert_eq!(POINT, (3, 4));
    /// ```
    ///
    /// Failing to provide valid coordinates will result in a compile error:
    ///
    /// ```rust,compile_fail
    /// # use grid_mask::GridPoint;
    /// const POINT: GridPoint = GridPoint::const_new::<8, 8>();
    /// ```
    #[must_use]
    pub const fn const_new<const X: u8, const Y: u8>() -> Self {
        assert!(X < 8, "x coordinate is out of bounds (must be < 8)");
        assert!(Y < 8, "y coordinate is out of bounds (must be < 8)");

        let index = BitIndexU64::new(X + Y * 8).unwrap();
        Self(index)
    }

    /// Returns the x coordinate of the point.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::GridPoint;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let point = GridPoint::try_new(3, 4)?;
    ///
    /// assert_eq!(point.x(), 3);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub const fn x(&self) -> GridPos {
        let x = self.0.get() % 8;
        // x is always in 0..=7, so this is safe
        unsafe { GridPos::new_unchecked(x) }
    }

    /// Returns the y coordinate of the point.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::GridPoint;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let point = GridPoint::try_new(3, 4)?;
    ///
    /// assert_eq!(point.y(), 4);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub const fn y(&self) -> GridPos {
        let y = self.0.get() / 8;
        // y is always in 0..=7, so this is safe
        unsafe { GridPos::new_unchecked(y) }
    }

    /// Translates the point by `vec`.
    ///
    /// # Arguments
    ///
    /// * `vec` - The [`GridVector`] to translate the point by.
    ///
    /// # Errors
    ///
    /// [`OutOfBounds`] if the translated point would be out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # use grid_mask::{GridPoint, GridVector};
    /// let point = GridPoint::try_new(3, 4)?;
    /// let vec = GridVector::new(1, -1);
    ///
    /// let translated = point.translate(vec)?;
    ///
    /// assert_eq!(translated, (4, 3));
    ///
    /// let vec = GridVector::new(5, 0);
    /// point.translate(vec).expect_err("Should be out of bounds");
    /// # Ok(())
    /// # }
    /// ```
    pub fn translate(&self, vec: GridVector) -> Result<Self, OutOfBounds> {
        // because GridPos is bounded to 0..=7, a cast to i8 is safe
        let x = self.x().get().cast_signed().saturating_add(vec.x);
        let y = self.y().get().cast_signed().saturating_add(vec.y);

        Self::try_new(x, y)
    }

    /// Returns an iterator over all possible [`GridPoint`] values.
    #[must_use]
    pub const fn all_values() -> BoundedIter<Self> {
        BoundedIter::new()
    }
}

impl<X: From<GridPos>, Y: From<GridPos>> From<GridPoint> for (X, Y) {
    fn from(point: GridPoint) -> Self {
        (point.x().into(), point.y().into())
    }
}

impl<X: TryInto<GridPos>, Y: TryInto<GridPos>> TryFrom<(X, Y)> for GridPoint {
    type Error = OutOfBounds;

    fn try_from(value: (X, Y)) -> Result<Self, Self::Error> {
        Self::try_new(value.0, value.1)
    }
}

impl<X, Y> PartialEq<(X, Y)> for GridPoint
where
    X: From<GridPos> + PartialEq,
    Y: From<GridPos> + PartialEq,
{
    fn eq(&self, other: &(X, Y)) -> bool {
        let (x, y): (X, Y) = (*self).into();
        x == other.0 && y == other.1
    }
}

impl Bound for GridPoint {
    const MIN: Self = Self::ORIGIN;
    const MAX: Self = Self::MAX;
    const COUNT: usize = BitIndexU64::COUNT;

    fn increment(&self) -> Option<Self> {
        self.0.increment().map(Self)
    }

    fn decrement(&self) -> Option<Self> {
        self.0.decrement().map(Self)
    }

    fn remaining(&self) -> usize {
        self.0.remaining()
    }
}

/// An iterator over all possible [`GridPoint`] values.
pub type GridPointIter = BoundedIter<GridPoint>;
