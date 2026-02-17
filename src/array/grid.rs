use std::num::NonZeroU16;
use std::str::FromStr;

use bitvec::access::BitSafeU64;
use bitvec::prelude::{BitArray, BitSlice, Lsb0};
use fluent_result::into::IntoResult;
use tap::Conv;

use crate::array::delta::ArrayDelta;
use crate::err::{OutOfBounds, PatternError};
use crate::ext::{FoldMut, NotWhitespace, assert_then, safe_into};
use crate::num::{Point, Rect, SignedMag, Size};
use crate::{ArrayIndex, ArrayPoint, ArrayRect, ArrayVector, GridView, GridViewMut};

use super::{Cells, GridGetIndex, GridSetIndex, Points, Spaces};

/// A fixed-size bit grid with `W` columns and `H` rows.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::From, derive_more::Into)]
pub struct ArrayGrid<const W: u16, const H: u16, const WORDS: usize> {
    pub(crate) data: BitArray<[u64; WORDS], Lsb0>,
}

impl<const W: u16, const H: u16, const WORDS: usize> ArrayGrid<W, H, WORDS> {
    /// An empty grid with all bits unset.
    pub const EMPTY: Self = Self { data: BitArray::ZERO };

    /// A full grid with all bits set.
    pub const FULL: Self = const {
        let mut words = BitArray::<[u64; WORDS], Lsb0>::ZERO;
        words.data = [u64::MAX; WORDS];
        let mut grid = Self { data: words };
        grid.clear_trailing_bits();
        grid
    };

    /// The origin point of the grid.
    pub const ORIGIN: ArrayPoint<W, H> = ArrayPoint::ORIGIN;

    /// The width of the grid.
    pub const WIDTH: NonZeroU16 = NonZeroU16::new(W).expect("Width must be greater than 0");
    /// The height of the grid.
    pub const HEIGHT: NonZeroU16 = NonZeroU16::new(H).expect("Height must be greater than 0");
    /// The total number of cells in the grid.
    pub const CELLS: u32 = W as u32 * H as u32;

    pub(crate) const CELLS_USZ: usize = Self::CELLS as usize;

    const WORD_BITS: usize = u64::BITS as usize;

    /// The number of `u64` words used to store the grid data.
    const WORD_COUNT: usize = const {
        assert_then!(WORDS == usize::div_ceil(Self::CELLS_USZ, Self::WORD_BITS) => WORDS,
            "WORDS must match the minimum number of words needed to represent the grid"
        )
    };

    /// Gets the cell value(s) identified by `index`.
    ///
    /// The behavior and return type of this method depend on the type of `IDX`.
    ///
    /// - Infallible single-cell indexer ([`ArrayIndex`], [`ArrayPoint`]):
    ///   Returns the value of the cell ([`bool`])
    /// - Fallible single-cell indexer ([`Point`], `(x, y)` tuples, integer indices):
    ///   Tries to get the value of the cell ([`Result<bool, OutOfBounds>`])
    /// - Infallible region indexer ([`ArrayRect`]):
    ///   Returns a borrowed view of the grid ([`GridView<'_>`])
    /// - Fallible region indexer ([`Rect`]):
    ///   Tries to get a borrowed view of the grid ([`Result<GridView<'_>, OutOfBounds>`])
    ///
    /// ## Arguments
    ///
    /// * `index` - The index of the cell(s) to get.
    ///
    /// ## Type Parameters
    ///
    /// * `IDX` - The type of `index`.
    ///
    /// ## Examples
    ///
    /// Infallible single-cell access:
    ///
    /// ```rust
    /// # use grid_mask::{ArrayGrid, ArrayIndex, ArrayPoint, array_grid};
    /// let grid = array_grid!(8, 8; [(0, 0), (7, 7)]);
    ///
    /// assert!(grid.get(ArrayIndex::MAX), "Max (7, 7) should be set");
    /// assert!(grid.get(ArrayPoint::ORIGIN), "Origin (0, 0) should be set");
    /// ```
    ///
    /// Fallible single-cell access:
    ///
    /// ```rust
    /// # use grid_mask::{ArrayGrid, array_grid};
    /// # use grid_mask::num::{Point, Rect};
    /// # use grid_mask::err::OutOfBounds;
    /// let grid = array_grid!(8, 8; [(1, 1)]);
    ///
    /// assert_eq!(grid.get(Point { x: 1, y: 1 }), Ok(true), "(1, 1) should be set");
    /// assert_eq!(grid.get((1, 1)), Ok(true), "(1, 1) should be set");
    /// assert_eq!(grid.get(9), Ok(true), "Index 9 (1, 1) should be set");
    ///
    /// assert_eq!(grid.get(Point { x: 8, y: 8 }), Err(OutOfBounds), "(8, 8) should be out of bounds");
    /// assert_eq!(grid.get((8, 8)), Err(OutOfBounds), "(8, 8) should be out of bounds");
    /// assert_eq!(grid.get(64), Err(OutOfBounds), "Index 64 should be out of bounds");
    /// ```
    ///
    /// Infallible region access:
    ///
    /// ```rust
    /// # use grid_mask::{ArrayGrid, ArrayPoint, ArrayRect, array_grid};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let grid = array_grid!(8, 8; [(0, 0), (0, 1), (1, 0), (1, 1)]);
    ///
    /// let rect = ArrayRect::new(ArrayPoint::ORIGIN, (2, 2))?;
    /// let view = grid.get(rect);
    ///
    /// assert_eq!(view.count(), 4, "Rect (0, 0) to (1, 1) should have 4 set cells");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Fallible region access:
    ///
    /// ```rust
    /// # use grid_mask::{ArrayGrid, array_grid};
    /// # use grid_mask::err::OutOfBounds;
    /// # use grid_mask::num::{Point, Rect, Size};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let grid = array_grid!(8, 8; [(1, 1), (1, 2), (2, 1), (2, 2)]);
    ///
    /// let rect = Rect::new(Point::new(1, 1), Size::new(2, 2));
    /// let result_view = grid.get(rect);
    ///
    /// assert!(result_view.is_ok(), "View should be valid");
    /// assert_eq!(result_view?.count(), 4, "Rect (1, 1) to (2, 2) should have 4 set cells");
    ///
    /// let oob_rect = Rect::new(Point::new(1, 1), Size::new(8, 8));
    /// let result_view = grid.get(oob_rect);
    ///
    /// assert_eq!(result_view, Err(OutOfBounds));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get<IDX: GridGetIndex<Self>>(&self, indexer: IDX) -> IDX::GetOutput<'_> {
        indexer.get(self)
    }

    /// Returns the value of the cell at `index`.
    ///
    /// This method is infallible and can be used in const contexts.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the cell to get.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{ArrayGrid, ArrayIndex, array_grid};
    /// let grid = <array_grid!(8, 8)>::FULL;
    /// let value = grid.const_get(ArrayIndex::MIN);
    /// assert!(value);
    /// ```
    #[must_use]
    pub const fn const_get(&self, index: ArrayIndex<W, H>) -> bool {
        let (word, bit) = index.word_and_bit();
        self.data.data[word] & (1u64 << bit) != 0
    }

    pub(crate) fn get_mut_ref(&mut self, index: ArrayIndex<W, H>) -> bitvec::ptr::BitRef<'_, bitvec::ptr::Mut, u64> {
        let index = index.get() as usize;
        unsafe { self.data.get_unchecked_mut(index) }
    }

    /// Returns the number of set cells in the grid.
    #[must_use]
    pub fn count(&self) -> u32 {
        safe_into!(self.data.count_ones() => u32)
    }

    /// Returns the raw data.
    #[must_use]
    pub const fn data(&self) -> &[u64] {
        &self.data.data
    }

    /// Returns a view of the bit data.
    #[must_use]
    pub fn bits(&self) -> &BitSlice<u64> {
        &self.data[..Self::CELLS_USZ]
    }

    /// Returns a mutable view of the bit data.
    #[must_use]
    pub fn bits_mut(&mut self) -> &mut BitSlice<u64> {
        &mut self.data[..Self::CELLS_USZ]
    }

    /// Returns an iterator over all cells in the grid.
    #[must_use]
    pub const fn cells(&self) -> Cells<'_, W, H, WORDS> {
        Cells::new(self)
    }

    /// Returns an iterator over the positions of all set cells in the grid.
    #[must_use]
    pub fn points(&self) -> Points<'_, W, H, WORDS> {
        Points::new(self)
    }

    /// Returns an iterator over the positions of all unset cells in the grid.
    #[must_use]
    pub fn spaces(&self) -> Spaces<'_, W, H, WORDS> {
        Spaces::new(self)
    }

    /// Returns an iterator over the positions of all set cells in the grid.
    #[must_use]
    pub fn iter(&self) -> Points<'_, W, H, WORDS> {
        self.points()
    }

    /// Returns the rectangle covered by this grid.
    #[must_use]
    pub const fn rect(&self) -> ArrayRect<W, H> {
        ArrayRect::<W, H>::const_new::<0, 0, W, H>()
    }

    /// Returns an immutable rectangular view over the entire grid.
    #[must_use]
    pub fn as_view(&self) -> GridView<'_> {
        let rect = Rect::new(Point::ORIGIN, Size::new(Self::WIDTH, Self::HEIGHT));
        GridView::new(self.bits(), W, rect)
    }

    /// Returns a mutable rectangular view over the entire grid.
    #[must_use]
    pub fn as_view_mut(&mut self) -> GridViewMut<'_> {
        let rect = Rect::new(Point::ORIGIN, Size::new(Self::WIDTH, Self::HEIGHT));
        let bits = self.bits_mut().split_at_mut(0).1;
        GridViewMut::new(bits, W, rect)
    }

    /// Returns an immutable rectangular view into this grid.
    #[must_use]
    pub fn view(&self, rect: ArrayRect<W, H>) -> GridView<'_> {
        GridView::new(self.bits(), W, Rect::from(rect))
    }

    /// Returns a mutable rectangular view into this grid.
    #[must_use]
    pub fn view_mut(&mut self, rect: ArrayRect<W, H>) -> GridViewMut<'_> {
        let bits = self.bits_mut().split_at_mut(0).1;
        GridViewMut::new(bits, W, Rect::from(rect))
    }

    /// Sets the value of the cell at `index`.
    ///
    /// This method supports two modes of operation:
    /// - infallible index ([`ArrayPoint`] or [`ArrayIndex`]): `()`
    /// - fallible index ([`Point`], `(x, y)` tuples, integer index): `Result<(), OutOfBounds>`
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the cell to set.
    /// * `value` - The new state of the cell.
    ///
    /// # Type Parameters
    ///
    /// * `IDX` - Index input type used to address a cell in this grid.
    ///
    /// # Examples
    ///
    /// Infallible index:
    /// ```rust
    /// # use grid_mask::{ArrayGrid, ArrayPoint, array_grid};
    /// let mut grid = <array_grid!(8, 8)>::EMPTY;
    ///
    /// grid.set(ArrayPoint::ORIGIN, true);
    ///
    /// assert_eq!(grid.get(ArrayPoint::ORIGIN), true);
    /// ```
    ///
    /// Fallible index:
    /// ```rust
    /// # use grid_mask::{ArrayGrid, ArrayPoint, array_grid};
    /// let mut grid = <array_grid!(8, 8)>::EMPTY;
    ///
    /// let result = grid.set((0u16, 0u16), false);
    ///
    /// assert!(result.is_ok());
    /// assert_eq!(grid.get(ArrayPoint::ORIGIN), false);
    /// ```
    pub fn set<IDX: GridSetIndex<Self>>(&mut self, indexer: IDX, value: bool) -> IDX::SetOutput {
        indexer.set(self, value)
    }

    /// Updates the cell at `index` to `value`.
    // TODO: Remove when const traits stabilize
    pub const fn const_set(&mut self, index: ArrayIndex<W, H>, value: bool) {
        match (index.word_and_bit(), value) {
            ((word, bit), true) => self.data.data[word] |= 1u64 << bit,
            ((word, bit), false) => self.data.data[word] &= !(1u64 << bit),
        }
    }

    /// Clears all cells in the grid.
    pub fn clear(&mut self) {
        self.fill(false);
    }

    /// Fills all cells in the grid with `value`.
    pub fn fill(&mut self, value: bool) {
        self.data[..Self::CELLS_USZ].fill(value);
    }

    /// Translates the grid by the given displacement vector.
    ///
    /// Bits that shift beyond the grid boundary are discarded; vacated
    /// positions are filled with `false`.
    pub fn translate(&mut self, vec: ArrayVector) {
        match ArrayDelta::<W, H>::try_from(vec).map(|d| (d.linear_offset, d.dx)) {
            Ok((SignedMag::Zero, _)) => {}
            Ok((SignedMag::Positive(n), dx)) => {
                self.data.as_mut_bitslice().shift_right(n.get() as usize);
                self.clear_wrapped_columns(dx);
                self.clear_trailing_bits();
            }
            Ok((SignedMag::Negative(n), dx)) => {
                self.data.as_mut_bitslice().shift_left(n.get() as usize);
                self.clear_wrapped_columns(dx);
                self.clear_trailing_bits();
            }
            Err(_) => self.clear(),
        }
    }

    fn bitwise_op_at<'a>(
        &mut self,
        other: impl Into<GridView<'a>>,
        at: ArrayPoint<W, H>,
        op: impl Fn(&mut BitSlice<BitSafeU64, Lsb0>, &BitSlice<u64, Lsb0>) + Copy,
    ) -> Result<(), OutOfBounds> {
        let other = other.into();
        let mut view = ArrayRect::new(at, other.size()).map(|rect| self.view_mut(rect))?;

        std::iter::zip(view.rows_mut(), other.rows()).for_each(|(dst_row, src_row)| op(dst_row, src_row));

        Ok(())
    }

    /// Performs a logical AND operation with another grid `other` at `at`.
    ///
    /// Only points in the intersection of the two grids are affected.
    ///
    /// # Errors
    ///
    /// [`OutOfBounds`] if the `other` grid does not fit within `self` at `at`.
    pub fn bitand_at<'a>(&mut self, other: impl Into<GridView<'a>>, at: ArrayPoint<W, H>) -> Result<(), OutOfBounds> {
        self.bitwise_op_at(other, at, |dst, src| *dst &= src)
    }

    /// Performs a logical OR operation with another grid at the specified point.
    ///
    /// Only points in the intersection of the two grids are affected.
    ///
    /// # Errors
    ///
    /// [`OutOfBounds`] if the `other` grid does not fit within `self` at `at`.
    pub fn bitor_at<'a>(&mut self, other: impl Into<GridView<'a>>, at: ArrayPoint<W, H>) -> Result<(), OutOfBounds> {
        self.bitwise_op_at(other, at, |dst, src| *dst |= src)
    }

    /// Performs a logical XOR operation with another grid at the specified point.
    ///
    /// Only points in the intersection of the two grids are affected.
    ///
    /// # Errors
    ///
    /// [`OutOfBounds`] if the `other` grid does not fit within `self` at `at`.
    pub fn bitxor_at<'a>(&mut self, other: impl Into<GridView<'a>>, at: ArrayPoint<W, H>) -> Result<(), OutOfBounds> {
        self.bitwise_op_at(other, at, |dst, src| *dst ^= src)
    }

    const W_USIZE: usize = W as usize;

    /// Clears the columns that incorrectly wrapped across row boundaries after
    /// the flat bit shift.
    ///
    /// When `dx` is positive, the first `dx` columns are cleared.
    /// When `dx` is negative, the last `|dx|` columns are cleared.
    fn clear_wrapped_columns(&mut self, dx: SignedMag<NonZeroU16>) {
        // TODO: use a range once they are copy
        let (min, max) = match dx {
            SignedMag::Positive(n) => (0, n.get().into()),
            SignedMag::Negative(n) => (Self::W_USIZE - n.get().conv::<usize>(), Self::W_USIZE),
            SignedMag::Zero => return,
        };

        self.data
            .as_mut_bitslice()
            .chunks_mut(Self::W_USIZE)
            .take(H as usize)
            .for_each(|row| row[min..max].fill(false));
    }

    /// Negates all cells in the grid.
    pub fn negate(&mut self) {
        self.data.data.iter_mut().for_each(|word| *word = !*word);
        self.clear_trailing_bits();
    }

    /// Provides the closure `f` with safe `mut` access to the underlying data.
    ///
    /// Note: This method provides the closure with the full `[u64]` slice. For grids
    /// where `W * H` is not a multiple of 64, some of the bits in the last element
    /// are unused (marked by [`Self::UNUSED_TRAILING_BITS`]). They will be cleared
    /// after the closure returns.
    ///
    /// Consider using [`Self::bits_mut`] for bitwise access to the grid instead.
    pub fn mutate_data<R>(&mut self, f: impl FnOnce(&mut [u64]) -> R) -> R {
        let r = f(&mut self.data.data);
        self.clear_trailing_bits();
        r
    }

    /// Mask of the unused tailing bits of the last word.
    pub const UNUSED_TRAILING_BITS: u64 = !Self::USED_TRAILING_BITS;

    /// Mask of the used bits of the last word.
    pub const USED_TRAILING_BITS: u64 = match Self::CELLS % u64::BITS {
        0 => u64::MAX,
        used => (1u64 << used) - 1,
    };

    /// Index of the last word in the grid.
    const LAST_WORD: usize = Self::WORD_COUNT - 1;

    /// Clears the unused tail bits of the last word.
    const fn clear_trailing_bits(&mut self) {
        self.data.data[Self::LAST_WORD] &= Self::USED_TRAILING_BITS;
    }
}

/// Conversion from a raw array of words.
///
/// Note: if `W * H` is not a multiple of 64, the trailing bits of the last word will be cleared.
impl<const W: u16, const H: u16, const WORDS: usize> From<[u64; WORDS]> for ArrayGrid<W, H, WORDS> {
    fn from(value: [u64; WORDS]) -> Self {
        let mut grid = Self { data: BitArray::new(value) };
        grid.clear_trailing_bits();
        grid
    }
}

impl<IDX, const W: u16, const H: u16, const WORDS: usize> FromIterator<IDX> for ArrayGrid<W, H, WORDS>
where
    IDX: GridSetIndex<Self, SetOutput = ()>,
{
    fn from_iter<T: IntoIterator<Item = IDX>>(iter: T) -> Self {
        iter.into_iter().fold_mut(Self::EMPTY, |grid, index| index.set(grid, true))
    }
}

impl<'a, const W: u16, const H: u16, const WORDS: usize> IntoIterator for &'a ArrayGrid<W, H, WORDS> {
    type Item = ArrayPoint<W, H>;
    type IntoIter = Points<'a, W, H, WORDS>;

    fn into_iter(self) -> Self::IntoIter {
        self.points()
    }
}

impl<IDX, const W: u16, const H: u16, const WORDS: usize> Extend<IDX> for ArrayGrid<W, H, WORDS>
where
    IDX: GridSetIndex<Self, SetOutput = ()>,
{
    fn extend<T: IntoIterator<Item = IDX>>(&mut self, iter: T) {
        iter.into_iter().for_each(|index| index.set(self, true));
    }
}

impl<const W: u16, const H: u16, const WORDS: usize> FromStr for ArrayGrid<W, H, WORDS> {
    type Err = PatternError;

    /// Parses a string pattern into an [`ArrayGrid`].
    ///
    /// Uses `#` for set cells and `.` for unset cells.
    /// Whitespace is ignored.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .filter(NotWhitespace::is_not_whitespace)
            .take(Self::CELLS_USZ + 1)
            .enumerate()
            .map(|(i, c)| (ArrayIndex::try_new(i), c))
            .try_fold((Self::EMPTY, None), |(mut grid, _), (i, c)| match (i, c) {
                (Err(_), _) => Err(PatternError::TooLong),
                (Ok(i), '#') => {
                    grid.set(i, true);
                    (grid, Some(i)).into_ok()
                }
                (Ok(i), '.') => (grid, Some(i)).into_ok(),
                (_, c) => PatternError::InvalidChar(c).into_err(),
            })
            .and_then(|(grid, index)| match index.map_or(0, |i| i.get() + 1) {
                i if i == Self::CELLS => Ok(grid),
                i => PatternError::TooShort(i).into_err(),
            })
    }
}

impl<'a, const W: u16, const H: u16, const WORDS: usize> From<&'a ArrayGrid<W, H, WORDS>> for GridView<'a> {
    fn from(grid: &'a ArrayGrid<W, H, WORDS>) -> Self {
        grid.as_view()
    }
}
