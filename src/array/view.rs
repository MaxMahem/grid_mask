use std::num::NonZeroU16;

use bitvec::access::BitSafeU64;
use bitvec::prelude::Lsb0;
use bitvec::slice::BitSlice;

use crate::array::{GridGetIndex, GridSetIndex};
use crate::err::OutOfBounds;
use crate::num::{Point, Rect, Size};

/// A borrowed view over an [`ArrayGrid`](struct@crate::ArrayGrid).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BaseGridView<S> {
    pub(crate) data: S,
    data_stride: u16,
    rect: Rect<Point<u16>, Size<NonZeroU16>>,
}

/// An immutable view over an [`ArrayGrid`](struct@crate::ArrayGrid).
pub type GridView<'a> = BaseGridView<&'a BitSlice<u64, Lsb0>>;

/// A mutable view over an [`ArrayGrid`](struct@crate::ArrayGrid).
pub type GridViewMut<'a> = BaseGridView<&'a mut BitSlice<BitSafeU64, Lsb0>>;

impl<S> BaseGridView<S> {
    pub(crate) const fn new(data: S, data_stride: u16, rect: Rect<Point<u16>, Size<NonZeroU16>>) -> Self {
        Self { data, data_stride, rect }
    }

    /// Returns the size/dimensions of this view.
    #[must_use]
    pub const fn size(&self) -> Size<u16> {
        Size::new(self.rect.size.width.get(), self.rect.size.height.get())
    }

    pub(crate) const fn translate_point_to_index(&self, point: Point<u16>) -> Result<usize, OutOfBounds> {
        match point.x < self.rect.size.width.get() && point.y < self.rect.size.height.get() {
            true => Ok((self.rect.point.y + point.y) as usize * self.data_stride as usize
                + (self.rect.point.x + point.x) as usize),
            false => Err(OutOfBounds),
        }
    }
}

impl GridView<'_> {
    pub(crate) fn try_view(&self, rect: Rect<Point<u16>, Size<u16>>) -> Result<Self, OutOfBounds> {
        let width = NonZeroU16::new(rect.size.width).ok_or(OutOfBounds)?;
        let height = NonZeroU16::new(rect.size.height).ok_or(OutOfBounds)?;

        if rect.point.x + rect.size.width > self.rect.size.width.get()
            || rect.point.y + rect.size.height > self.rect.size.height.get()
        {
            return Err(OutOfBounds);
        }

        let point = Point::new(self.rect.point.x + rect.point.x, self.rect.point.y + rect.point.y);
        let size = Size::new(width, height);

        Ok(Self::new(self.data, self.data_stride, Rect::new(point, size)))
    }

    /// Returns the number of set cells in the view.
    #[must_use]
    pub fn count(&self) -> usize {
        self.rows().map(BitSlice::count_ones).sum()
    }

    /// Returns the value of the cell at `point` using coordinates local to this view.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the cell to get (could be linear or a point/tuple).
    ///
    /// # Type Parameters
    ///
    /// * `IDX` - The type of the indexer.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfBounds`] when `point` is outside of this view.
    pub fn get<IDX: GridGetIndex<Self>>(&self, index: IDX) -> IDX::GetOutput<'_> {
        index.get(self)
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
        let width = self.rect.size.width.get() as usize;

        self.data
            .chunks(self.data_stride as usize)
            .skip(self.rect.point.y as usize)
            .take(self.rect.size.height.get() as usize)
            .map(move |row| row.get(x..x + width).unwrap())
    }
}

impl GridViewMut<'_> {
    /// Returns the value of the cell at `point` using coordinates local to this view.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the cell to get (could be linear or a point/tuple).
    ///
    /// # Type Parameters
    ///
    /// * `IDX` - The type of the indexer.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfBounds`] when `point` is outside of this view.
    pub fn get<IDX: GridGetIndex<Self>>(&self, index: IDX) -> IDX::GetOutput<'_> {
        index.get(self)
    }

    /// Sets the cell at `point` to `value` using coordinates local to this view.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the cell to set (could be linear or a point/tuple).
    ///
    /// # Type Parameters
    ///
    /// * `IDX` - The type of the indexer.
    ///
    /// # Errors
    ///
    /// [`OutOfBounds`] if `point` is outside of this view.
    pub fn set<IDX: GridSetIndex<Self>>(&mut self, index: IDX, value: bool) -> IDX::SetOutput {
        index.set(self, value)
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
        let width = self.rect.size.width.get() as usize;
        let height = self.rect.size.height.get() as usize;
        let y = self.rect.point.y as usize;

        self.data.chunks_mut(self.data_stride as usize).skip(y).take(height).map(move |row| {
            row.get_mut(x..x + width).expect("view must be within grid") //
        })
    }
}
