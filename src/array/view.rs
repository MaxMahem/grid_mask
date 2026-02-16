use std::borrow::{Borrow, BorrowMut};
use std::ops::Not;

use bitvec::access::BitSafeU64;
use bitvec::prelude::Lsb0;
use bitvec::slice::BitSlice;
use fluent_result::bool::Then;

use crate::array::{ArrayGrid, ArrayPoint, ArrayRect};
use crate::err::OutOfBounds;

/// A rectangular view over an [`ArrayGrid`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ArrayView<G, const W: u16, const H: u16, const WORDS: usize> {
    grid: G,
    rect: ArrayRect<W, H>,
}

/// An immutable rectangular view over an [`ArrayGrid`].
pub type ArrayGridView<'a, const W: u16, const H: u16, const WORDS: usize> =
    ArrayView<&'a ArrayGrid<W, H, WORDS>, W, H, WORDS>;

/// A mutable rectangular view over an [`ArrayGrid`].
pub type ArrayGridViewMut<'a, const W: u16, const H: u16, const WORDS: usize> =
    ArrayView<&'a mut ArrayGrid<W, H, WORDS>, W, H, WORDS>;

impl<G, const W: u16, const H: u16, const WORDS: usize> ArrayView<G, W, H, WORDS> {
    pub(crate) const fn new(grid: G, rect: ArrayRect<W, H>) -> Self {
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
        self.rect
            .size()
            .contains(x, y)
            .not()
            .then_err(OutOfBounds)
            .and_then(|()| ArrayPoint::<W, H>::new(self.rect.point.x() + x, self.rect.point.y() + y))
    }
}

impl<G, const W: u16, const H: u16, const WORDS: usize> ArrayView<G, W, H, WORDS>
where
    G: Borrow<ArrayGrid<W, H, WORDS>>,
{
    /// Returns the number of set cells in the view.
    #[must_use]
    pub fn count(&self) -> usize {
        self.rows().map(bitvec::slice::BitSlice::count_ones).sum()
    }

    /// Returns the value of the cell at `x`/`y` using coordinates local to this view.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfBounds`] when `x` or `y` are outside of this view.
    pub fn get(&self, x: u16, y: u16) -> Result<bool, OutOfBounds> {
        self.translate_point(x, y).map(|point| self.grid.borrow().get(point))
    }

    /// Returns an iterator over all cells in the view.
    pub fn cells(&self) -> impl Iterator<Item = bool> + '_ {
        self.rows().flat_map(|row| row.iter().by_vals())
    }

    /// Returns an iterator over the positions of all set cells in the view.
    ///
    /// The coordinates are local to the view (relative to `(0, 0)`).
    #[allow(clippy::cast_possible_truncation)]
    pub fn points(&self) -> impl Iterator<Item = (u16, u16)> + '_ {
        self.rows().enumerate().flat_map(|(y, row)| row.iter_ones().map(move |x| (x as u16, y as u16)))
    }

    /// Returns an iterator over the positions of all unset cells in the view.
    ///
    /// The coordinates are local to the view (relative to `(0, 0)`).
    #[allow(clippy::cast_possible_truncation)]
    pub fn spaces(&self) -> impl Iterator<Item = (u16, u16)> + '_ {
        self.rows().enumerate().flat_map(|(y, row)| row.iter_zeros().map(move |x| (x as u16, y as u16)))
    }

    const W_USZ: usize = W as usize;

    /// Returns an iterator over the rows of bits in this view.
    pub(crate) fn rows(&self) -> impl Iterator<Item = &BitSlice<u64, Lsb0>> {
        let x = self.rect.point.x() as usize;
        let width = self.rect.size.width() as usize;

        self.grid
            .borrow()
            .words
            .as_bitslice()
            .chunks(W as usize)
            .skip(self.rect.point.y() as usize)
            .take(self.rect.size.height() as usize)
            .map(move |row| row.get(x..x + width).unwrap())
    }
}

impl<G, const W: u16, const H: u16, const WORDS: usize> ArrayView<G, W, H, WORDS>
where
    G: BorrowMut<ArrayGrid<W, H, WORDS>>,
{
    /// Sets the cell at `x`/`y` to `value` using coordinates local to this view.
    ///
    /// # Errors
    ///
    /// [`OutOfBounds`] if `x` or `y` are outside of this view.
    pub fn set(&mut self, x: u16, y: u16, value: bool) -> Result<(), OutOfBounds> {
        self.translate_point(x, y).map(|point| self.grid.borrow_mut().set(point, value))
    }

    /// Fills the view with `value`.
    pub fn fill(&mut self, value: bool) {
        self.rows_mut().for_each(|row| row.fill(value));
    }

    /// Clears the view.
    pub fn clear(&mut self) {
        self.fill(false);
    }

    /// Returns an iterator over the rows of bits in this view.
    pub(crate) fn rows_mut(&mut self) -> impl Iterator<Item = &mut BitSlice<BitSafeU64, Lsb0>> {
        let x = self.rect.point.x() as usize;
        let width = self.rect.size.width() as usize;

        self.grid
            .borrow_mut()
            .words
            .as_mut_bitslice()
            .chunks_mut(W as usize)
            .skip(self.rect.point.y() as usize)
            .take(self.rect.size.height() as usize)
            .map(move |row| row.get_mut(x..x + width).unwrap())
    }
}
