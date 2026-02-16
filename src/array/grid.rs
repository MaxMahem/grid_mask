use std::num::NonZeroU16;
use std::str::FromStr;

use bitvec::access::BitSafeU64;
use bitvec::prelude::{BitArray, BitSlice, Lsb0};
use fluent_result::into::IntoResult;
use itertools::Itertools;
use tap::Conv;

use crate::array::delta::ArrayDelta;
use crate::err::{OutOfBounds, PatternError};
use crate::ext::{FoldMut, NotWhitespace, assert_then, safe_into};
use crate::num::{Point, Rect, SignedMag, Size};
use crate::{ArrayIndex, ArrayPoint, ArrayRect, ArrayVector, GridView, GridViewMut};

use super::{Cells, GridIndexer, Points, Spaces};

/// A fixed-size bit grid with `W` columns and `H` rows.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::From, derive_more::Into)]
pub struct ArrayGrid<const W: u16, const H: u16, const WORDS: usize> {
    pub(crate) data: BitArray<[u64; WORDS], Lsb0>,
}

/// Helper macro for creating an [`ArrayGrid`] with the correct number of words.
#[macro_export]
macro_rules! ArrayGrid {
    ($W:expr, $H:expr) => {
        $crate::array::ArrayGrid<$W, $H, { usize::div_ceil($W * $H, u64::BITS as usize) }>
    };
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

    /// Returns the value of the cell at `point`.
    ///
    /// This method supports two modes of operation:
    /// - infallible point inputs ([`ArrayPoint`] or [`ArrayIndex`]) return `bool`
    /// - fallible point inputs ([`Point`] or `(x, y)` tuples) return `Result<bool, OutOfBounds>`
    ///
    /// # Arguments
    ///
    /// * `point` - Any input that implements [`ArrayGridPointArg`].
    ///
    /// # Type Parameters
    ///
    /// * `Idx` - Point input type used to address a cell in this grid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{ArrayGrid, ArrayIndex};
    /// type Grid = ArrayGrid<8, 8, 1>;
    ///
    /// let grid = Grid::FULL;
    /// let infallible = grid.get(ArrayIndex::MIN);
    /// assert!(infallible);
    ///
    /// let fallible = grid.get((0u16, 0u16));
    /// assert_eq!(fallible, Ok(true));
    /// ```
    #[must_use]
    pub fn get<Idx: GridIndexer<W, H>>(&self, point: Idx) -> Idx::GetOutput {
        point.get(self)
    }

    /// Returns the value of the cell at `point`.
    // Todo: Remove once const traits stabilize.
    #[must_use]
    pub const fn const_get(&self, point: ArrayPoint<W, H>) -> bool {
        let (word, bit) = point.to_index().word_and_bit();
        self.data.data[word] & (1u64 << bit) != 0
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
        let rect = Rect::new(Point::ORIGIN, Size::new(W, H));
        GridView::new(self.data.as_bitslice(), W, rect)
    }

    /// Returns a mutable rectangular view over the entire grid.
    #[must_use]
    pub fn as_view_mut(&mut self) -> GridViewMut<'_> {
        let rect = Rect::new(Point::ORIGIN, Size::new(W, H));
        let bits = self.data.as_mut_bitslice().split_at_mut(0).1;
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

    /// Sets the value of the cell at `point`.
    ///
    /// This method supports two modes of operation:
    /// - infallible point inputs ([`ArrayPoint`] or [`ArrayIndex`]) return `()`
    /// - fallible point inputs ([`Point`] or `(x, y)` tuples) return `Result<(), OutOfBounds>`
    ///
    /// # Arguments
    ///
    /// * `point` - Any input that implements [`ArrayGridPointArg`].
    /// * `value` - New bit value for the addressed cell.
    ///
    /// # Type Parameters
    ///
    /// * `Idx` - Point input type used to address a cell in this grid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::ArrayGrid;
    /// type Grid = ArrayGrid<8, 8, 1>;
    ///
    /// let mut grid = Grid::EMPTY;
    /// grid.set((0u16, 0u16), true).unwrap();
    /// assert_eq!(grid.get((0u16, 0u16)), Ok(true));
    /// ```
    pub fn set<Idx: GridIndexer<W, H>>(&mut self, point: Idx, value: bool) -> Idx::SetOutput {
        point.set(self, value)
    }

    /// Updates the cell at `index` to `value`.
    // TODO: Remove when const traits stabilize
    pub const fn const_set(&mut self, point: ArrayPoint<W, H>, value: bool) {
        match (point.to_index().word_and_bit(), value) {
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

    const W_U32: u32 = W as u32;
    const H_U32: u32 = H as u32;

    /// Returns the rectangle of this grid covered by the `view` with its
    /// origin corner at `at`.
    fn view_rect(at: ArrayPoint<W, H>, view: &GridView<'_>) -> Result<ArrayRect<W, H>, OutOfBounds> {
        ArrayRect::new(at, view.size())
    }

    fn bitwise_op_at<'a>(
        &mut self,
        other: impl Into<GridView<'a>>,
        at: ArrayPoint<W, H>,
        op: impl Fn(&mut BitSlice<BitSafeU64, Lsb0>, &BitSlice<u64, Lsb0>) + Copy,
    ) -> Result<(), OutOfBounds> {
        let other = other.into();
        let mut view = Self::view_rect(at, &other).map(|rect| self.view_mut(rect))?;

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

impl<Idx, const W: u16, const H: u16, const WORDS: usize> FromIterator<Idx> for ArrayGrid<W, H, WORDS>
where
    Idx: Into<ArrayPoint<W, H>>,
{
    fn from_iter<T: IntoIterator<Item = Idx>>(iter: T) -> Self {
        iter.into_iter().map_into().fold_mut(Self::EMPTY, |grid, point| grid.const_set(point, true))
    }
}

impl<'a, const W: u16, const H: u16, const WORDS: usize> IntoIterator for &'a ArrayGrid<W, H, WORDS> {
    type Item = ArrayPoint<W, H>;
    type IntoIter = Points<'a, W, H, WORDS>;

    fn into_iter(self) -> Self::IntoIter {
        self.points()
    }
}

impl<Idx, const W: u16, const H: u16, const WORDS: usize> Extend<Idx> for ArrayGrid<W, H, WORDS>
where
    Idx: Into<ArrayPoint<W, H>>,
{
    fn extend<T: IntoIterator<Item = Idx>>(&mut self, iter: T) {
        iter.into_iter().map_into().for_each(|point| self.const_set(point, true));
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
