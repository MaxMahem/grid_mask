use std::ops::Not;

use fluent_result::bool::Then;
use fluent_result::into::IntoResult;

use crate::array::{ArrayGrid, ArrayPoint, ArrayRect};
use crate::err::OutOfBounds;

/// An immutable rectangular view over an [`ArrayGrid`].
#[derive(Debug, PartialEq, Eq)]
pub struct ArrayGridView<'a, const W: u16, const H: u16, const WORDS: usize> {
    grid: &'a mut ArrayGrid<W, H, WORDS>,
    rect: ArrayRect<W, H>,
}

impl<'a, const W: u16, const H: u16, const WORDS: usize> ArrayGridView<'a, W, H, WORDS> {
    pub(crate) const fn new(grid: &'a mut ArrayGrid<W, H, WORDS>, rect: ArrayRect<W, H>) -> Self {
        Self { grid, rect }
    }

    /// Returns the rectangle covered by this view.
    #[must_use]
    pub const fn rect(&self) -> ArrayRect<W, H> {
        self.rect
    }

    /// Returns the origin point of this view in parent-grid coordinates.
    #[must_use]
    pub const fn origin(&self) -> ArrayPoint<W, H> {
        self.rect.point()
    }

    fn translate_point(&self, x: u16, y: u16) -> Result<ArrayPoint<W, H>, OutOfBounds> {
        self.rect.size().contains(x, y).not().then_err(OutOfBounds)?;
        ArrayPoint::<W, H>::new(self.rect.point.x() + x, self.rect.point.y() + y)
    }

    /// Returns the value of the cell at `x`/`y` using coordinates local to this view.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfBounds`] when `x` or `y` are outside of this view.
    pub fn get(&self, x: u16, y: u16) -> Result<bool, OutOfBounds> {
        let point = self.translate_point(x, y)?;
        self.grid.get(point).into_ok()
    }

    /// Updates the cell at `x`/`y` to `value` using coordinates local to this view.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfBounds`] when `x` or `y` are outside of this view.
    pub fn update(&mut self, x: u16, y: u16, value: bool) -> Result<(), OutOfBounds> {
        let point = self.translate_point(x, y)?;

        self.grid.update(point, value);
        Ok(())
    }
}
