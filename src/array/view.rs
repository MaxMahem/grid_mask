use bitvec::access::BitSafeU64;
use bitvec::prelude::Lsb0;
use bitvec::slice::BitSlice;

use crate::err::OutOfBounds;
use crate::num::{Point, Rect, Size};

/// A rectangular view over an [`ArrayGrid`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ArrayView<S> {
    data: S,
    data_stride: u16,
    rect: Rect<Point<u16>, Size<u16>>,
}

/// An immutable rectangular view over an [`ArrayGrid`].
pub type ArrayGridView<'a> = ArrayView<&'a BitSlice<u64, Lsb0>>;

/// A mutable rectangular view over an [`ArrayGrid`].
pub type ArrayGridViewMut<'a> = ArrayView<&'a mut BitSlice<BitSafeU64, Lsb0>>;

impl<S> ArrayView<S> {
    pub(crate) const fn new(data: S, data_stride: u16, rect: Rect<Point<u16>, Size<u16>>) -> Self {
        Self { data, data_stride, rect }
    }

    /// Returns the size/dimensions of this view.
    #[must_use]
    pub const fn size(&self) -> Size<u16> {
        self.rect.size
    }

    const fn translate_point_to_index(&self, point: Point<u16>) -> Result<usize, OutOfBounds> {
        match point.x < self.rect.size.width && point.y < self.rect.size.height {
            true => Ok((self.rect.point.y + point.y) as usize * self.data_stride as usize
                + (self.rect.point.x + point.x) as usize),
            false => Err(OutOfBounds),
        }
    }
}

impl ArrayView<&BitSlice<u64, Lsb0>> {
    /// Returns the number of set cells in the view.
    #[must_use]
    pub fn count(&self) -> usize {
        self.rows().map(bitvec::slice::BitSlice::count_ones).sum()
    }

    /// Returns the value of the cell at `point` using coordinates local to this view.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfBounds`] when `point` is outside of this view.
    pub fn get(&self, point: Point<u16>) -> Result<bool, OutOfBounds> {
        self.translate_point_to_index(point).map(|idx| self.data[idx])
    }

    /// Returns an iterator over all cells in the view.
    pub fn cells(&self) -> impl Iterator<Item = bool> + '_ {
        self.rows().flat_map(|row| row.iter().by_vals())
    }

    /// Returns an iterator over the positions of all set cells in the view.
    ///
    /// The coordinates are local to the view.
    #[allow(clippy::cast_possible_truncation)]
    pub fn points(&self) -> impl Iterator<Item = Point<u16>> + '_ {
        self.rows().enumerate().flat_map(|(y, row)| row.iter_ones().map(move |x| Point::new(x as u16, y as u16)))
    }

    /// Returns an iterator over the positions of all unset cells in the view.
    ///
    /// The coordinates are local to the view.
    #[allow(clippy::cast_possible_truncation)]
    pub fn spaces(&self) -> impl Iterator<Item = Point<u16>> + '_ {
        self.rows().enumerate().flat_map(|(y, row)| row.iter_zeros().map(move |x| Point::new(x as u16, y as u16)))
    }

    /// Returns an iterator over the rows of bits in this view.
    pub(crate) fn rows(&self) -> impl Iterator<Item = &BitSlice<u64, Lsb0>> {
        let x = self.rect.point.x as usize;
        let width = self.rect.size.width as usize;

        self.data
            .chunks(self.data_stride as usize)
            .skip(self.rect.point.y as usize)
            .take(self.rect.size.height as usize)
            .map(move |row| row.get(x..x + width).unwrap())
    }
}

impl ArrayView<&mut BitSlice<BitSafeU64, Lsb0>> {
    /// Returns the current value of the cell at `point` using coordinates local to this view.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfBounds`] when `point` is outside of this view.
    pub fn get(&self, point: Point<u16>) -> Result<bool, OutOfBounds> {
        self.translate_point_to_index(point).map(|idx| self.data[idx])
    }

    /// Sets the cell at `point` to `value` using coordinates local to this view.
    ///
    /// # Errors
    ///
    /// [`OutOfBounds`] if `point` is outside of this view.
    pub fn set(&mut self, point: Point<u16>, value: bool) -> Result<(), OutOfBounds> {
        self.translate_point_to_index(point).map(|idx| self.data.set(idx, value))
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
        let x = self.rect.point.x as usize;
        let width = self.rect.size.width as usize;
        let height = self.rect.size.height as usize;
        let y = self.rect.point.y as usize;

        self.data.chunks_mut(self.data_stride as usize).skip(y).take(height).map(move |row| {
            row.get_mut(x..x + width).expect("view must be within grid") //
        })
    }
}
