use crate::array::{ArrayGrid, ArrayPoint, ArrayRect};
use crate::err::OutOfBounds;

/// An immutable rectangular view over an [`ArrayGrid`].
#[derive(Debug, Clone, Copy)]
pub struct ArrayGridView<'a, const W: u16, const H: u16, const WORDS: usize> {
    grid: &'a ArrayGrid<W, H, WORDS>,
    rect: ArrayRect<W, H>,
}

impl<'a, const W: u16, const H: u16, const WORDS: usize> ArrayGridView<'a, W, H, WORDS> {
    pub(crate) const fn new(grid: &'a ArrayGrid<W, H, WORDS>, rect: ArrayRect<W, H>) -> Self {
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

    /// Returns the view width.
    #[must_use]
    pub const fn width(&self) -> u16 {
        self.rect.width()
    }

    /// Returns the view height.
    #[must_use]
    pub const fn height(&self) -> u16 {
        self.rect.height()
    }

    /// Returns a cell value using coordinates local to this view.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfBounds`] when `x` or `y` are outside of this view.
    pub fn get_local(&self, x: u16, y: u16) -> Result<bool, OutOfBounds> {
        if x >= self.width() || y >= self.height() {
            return Err(OutOfBounds);
        }

        let point = ArrayPoint::<W, H>::new(self.rect.point.x + x, self.rect.point.y + y)?;
        Ok(self.grid.get(point))
    }

    /// Returns a cell value using parent-grid coordinates.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfBounds`] when `point` lies outside this view.
    pub fn get(&self, point: ArrayPoint<W, H>) -> Result<bool, OutOfBounds> {
        if !self.rect.contains(point) {
            return Err(OutOfBounds);
        }

        Ok(self.grid.get(point))
    }
}
