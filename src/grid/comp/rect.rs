use fluent_result::bool::Then;
use fluent_result::into::IntoResult;

use crate::err::OutOfBounds;
use crate::num::{GridLen, GridPos};
use crate::{GridPoint, GridSize, GridVector};

/// A rectangle on an 8x8 grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
#[display("{point} {size}")] // GridPoint is (x, y), GridSize is (WxH)
pub struct GridRect {
    /// The top-left corner of the rectangle.
    point: GridPoint,
    /// The size of the rectangle.
    size: GridSize,
}

impl GridRect {
    /// A maximum size [`GridRect`].
    pub const MAX: Self = Self { point: GridPoint::ORIGIN, size: GridSize::MAX };

    /// Creates a new [`GridRect`] without bounds checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    /// - that `x` and `y` are less than 8.
    /// - that `w` and `h` are between 1 and 8.
    /// - that `x + w` and `y + h` are less than or equal to 8.
    #[must_use]
    pub(crate) const fn new_unchecked(point: GridPoint, size: GridSize) -> Self {
        debug_assert!(point.x().get() + size.width.get() <= 8, "x + w must be less than or equal to 8");
        debug_assert!(point.y().get() + size.height.get() <= 8, "y + h must be less than or equal to 8");

        Self { point, size }
    }

    /// Creates a new [`GridRect`].
    ///
    /// # Arguments
    ///
    /// * `point` - The top-left corner of the rectangle.
    /// * `size` - The dimensions of the rectangle.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfBounds`] if the rectangle extends beyond the 8x8 grid.
    pub fn new<P: TryInto<GridPoint>, S: TryInto<GridSize>>(point: P, size: S) -> Result<Self, OutOfBounds> {
        let point = point.try_into().map_err(OutOfBounds::new_from)?;
        let size = size.try_into().map_err(OutOfBounds::new_from)?;

        (point.x().get() + size.width.get() > 8).then_err(OutOfBounds)?;
        (point.y().get() + size.height.get() > 8).then_err(OutOfBounds)?;
        Self { point, size }.into_ok()
    }

    /// Creates a new [`GridRect`] from raw coordinates.
    ///
    /// Validity is ensured at compile time.
    ///
    /// # Type Parameters
    ///
    /// * `X` - The x coordinate of the top-left corner.
    /// * `Y` - The y coordinate of the top-left corner.
    /// * `W` - The width of the rectangle.
    /// * `H` - The height of the rectangle.
    ///
    /// # Panics
    ///
    /// Fails at compile time if:
    ///
    /// - `X` or `Y` are greater than 7.
    /// - `W` or `H` are greater than 8.
    /// - the rectangle extends beyond the 8x8 grid.
    #[must_use]
    pub const fn const_new<const X: u8, const Y: u8, const W: u8, const H: u8>() -> Self {
        assert!(X + W <= 8, "Rectangle extends beyond the 8x8 grid");
        assert!(Y + H <= 8, "Rectangle extends beyond the 8x8 grid");
        Self { point: GridPoint::const_new::<X, Y>(), size: GridSize::const_new::<W, H>() }
    }

    /// Returns the position of the bottom-right cell occupied by the rectangle.
    ///
    /// Since [`GridRect`] is guaranteed to be within the grid, this method is infallible.
    #[allow(clippy::missing_panics_doc, reason = "Method is infallible due to type invariants")]
    #[must_use]
    pub fn bottom_right(&self) -> GridPoint {
        let x = GridPos::new(self.point.x().get() + self.size.width.get() - 1).expect("guaranteed valid");
        let y = GridPos::new(self.point.y().get() + self.size.height.get() - 1).expect("guaranteed valid");
        GridPoint::new(x, y)
    }

    /// Returns the x coordinate of the top-left corner.
    #[must_use]
    pub const fn x(&self) -> GridPos {
        self.point.x()
    }

    /// Returns the y coordinate of the top-left corner.
    #[must_use]
    pub const fn y(&self) -> GridPos {
        self.point.y()
    }

    /// Returns the width of the rectangle.
    #[must_use]
    pub const fn w(&self) -> GridLen {
        self.size.width
    }

    /// Returns the height of the rectangle.
    #[must_use]
    pub const fn h(&self) -> GridLen {
        self.size.height
    }

    /// Returns the top-left corner of the rectangle.
    #[must_use]
    pub const fn point(&self) -> GridPoint {
        self.point
    }

    /// Returns the size of the rectangle.
    #[must_use]
    pub const fn size(&self) -> GridSize {
        self.size
    }

    /// Translates the rectangle by the given vector.
    ///
    /// The rectangle can not be "clipped" by the grid boundaries.
    ///
    /// # Arguments
    ///
    /// * `vec` - The vector to translate the rectangle by.
    ///
    /// # Errors
    ///
    /// [`OutOfBounds`] if the resulting rectangle would be out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{GridRect, GridVector};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let rect = GridRect::new((3, 4), (2, 2))?;
    /// let vec = GridVector::new(1, -1);
    ///
    /// let translated = rect.translate(vec)?;
    ///
    /// assert_eq!(translated.point(), (4, 3), "Point should be translated");
    /// assert_eq!(translated.size(), (2, 2), "Size should remain the same");
    ///
    /// rect.translate(GridVector::new(4, 0)).expect_err("Should be out of bounds");
    /// # Ok(())
    /// # }
    /// ```
    pub fn translate(&self, vec: GridVector) -> Result<Self, OutOfBounds> {
        let point = self.point.translate(vec)?;
        Self::new(point, self.size)
    }
}
