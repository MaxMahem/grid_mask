use std::num::NonZeroU16;
use std::str::FromStr;

use bitvec::prelude::{BitArray, Lsb0};
use bitvec::slice::ChunksExact;
use fluent_result::bool::Then;
use fluent_result::into::IntoResult;
use itertools::Itertools;
use tap::Conv;

use crate::array::delta::ArrayDelta;
use crate::err::{OutOfBounds, PatternError};
use crate::ext::{FoldMut, NotWhitespace};
use crate::num::SignedMag;
use crate::{ArrayIndex, ArrayPoint, ArrayVector};

use super::{Cells, Points, Spaces};

/// A fixed-size bit grid with `W` columns and `H` rows.
#[readonly::make]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayGrid<const W: u16, const H: u16, const WORDS: usize> {
    pub(crate) words: BitArray<[u64; WORDS], Lsb0>,
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
    pub const EMPTY: Self = Self { words: BitArray::ZERO };

    /// A full grid with all bits set.
    pub const FULL: Self = const {
        let mut words = BitArray::<[u64; WORDS], Lsb0>::ZERO;
        words.data = [u64::MAX; WORDS];
        let mut grid = Self { words };
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
    pub const CELL_COUNT: u32 = W as u32 * H as u32;

    pub(crate) const CELL_COUNT_USZ: usize = Self::CELL_COUNT as usize;
    const WORD_BITS: usize = u64::BITS as usize;

    /// The number of `u64` words used to store the grid data.
    const WORD_COUNT: usize = const {
        assert!(
            WORDS == usize::div_ceil(Self::CELL_COUNT_USZ, Self::WORD_BITS),
            "WORDS must match the minimum number of words needed to represent the grid"
        );
        WORDS
    };

    /// Gets the cell at `index` to `value`.
    #[must_use]
    pub fn get<Idx: Into<ArrayIndex<W, H>>>(&self, index: Idx) -> bool {
        let (word, bit) = index.into().word_and_bit();
        self.words.data[word] & (1_u64 << bit) != 0
    }

    /// Returns the number of set cells in the grid.
    #[must_use]
    pub fn count(&self) -> u32 {
        self.words.data.iter().copied().map(u64::count_ones).sum()
    }

    /// Returns the raw data.
    #[must_use]
    pub const fn words(&self) -> &[u64] {
        &self.words.data
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

    /// Updates the cell at `index` to `value`.
    pub fn update<Idx: Into<ArrayIndex<W, H>>>(&mut self, index: Idx, value: bool) {
        self.const_update(index.into(), value);
    }

    /// Updates the cell at `index` to `value`.
    // TODO: Remove when const traits stabilize
    pub const fn const_update(&mut self, index: ArrayIndex<W, H>, value: bool) {
        match (index.word_and_bit(), value) {
            ((word, bit), true) => self.words.data[word] |= 1u64 << bit,
            ((word, bit), false) => self.words.data[word] &= !(1u64 << bit),
        }
    }

    /// Clears all cells in the grid.
    pub fn clear(&mut self) {
        self.words.data.fill(0);
    }

    /// Fills all cells in the grid with `value`.
    pub fn fill(&mut self, value: bool) {
        match value {
            false => self.words.data.fill(0),
            true => {
                self.words.data.fill(u64::MAX);
                self.clear_trailing_bits();
            }
        }
    }

    /// Translates the grid by the given displacement vector.
    ///
    /// Bits that shift beyond the grid boundary are discarded; vacated
    /// positions are filled with `false`.
    pub fn translate_mut(&mut self, vec: ArrayVector) {
        match ArrayDelta::<W, H>::try_from(vec).map(|d| (d.linear_offset, d.dx)) {
            Ok((SignedMag::Zero, _)) => {}
            Ok((SignedMag::Positive(n), dx)) => {
                self.words.as_mut_bitslice().shift_right(n.get() as usize);
                self.clear_wrapped_columns(dx);
                self.clear_trailing_bits();
            }
            Ok((SignedMag::Negative(n), dx)) => {
                self.words.as_mut_bitslice().shift_left(n.get() as usize);
                self.clear_wrapped_columns(dx);
                self.clear_trailing_bits();
            }
            Err(_) => self.clear(),
        }
    }

    const W_U32: u32 = W as u32;
    const H_U32: u32 = H as u32;

    fn ensure_fits<const W2: u16, const H2: u16, const _WORDS2: usize>(
        _other: &ArrayGrid<W2, H2, _WORDS2>,
        at: ArrayPoint<W, H>,
    ) -> Result<(), OutOfBounds> {
        (u32::from(at.x) + ArrayPoint::<W2, H2>::W_U32 > Self::W_U32
            || u32::from(at.y) + ArrayPoint::<W2, H2>::H_U32 > Self::H_U32)
            .then_err(OutOfBounds)
    }

    fn iter_rows(&self) -> ChunksExact<'_, u64, Lsb0> {
        self.words.chunks_exact(Self::W_USIZE)
    }

    /// Performs a logical AND operation with another grid at the specified point.
    ///
    /// Only the intersection of the two grids is affected.
    /// If the other grid does not fit at the specified point, the operation returns `Err(OutOfBounds)`.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfBounds`] if the `other` grid does not fit within `self` at `at`.
    pub fn bitand_at<const W2: u16, const H2: u16, const WORDS2: usize>(
        &mut self,
        other: &ArrayGrid<W2, H2, WORDS2>,
        at: ArrayPoint<W, H>,
    ) -> Result<(), OutOfBounds> {
        Self::ensure_fits(other, at)?;

        let mut dst_start = usize::from(at.y) * Self::W_USIZE + usize::from(at.x);
        let len = usize::from(W2);

        std::iter::zip(other.iter_rows(), 0..H2).for_each(|(src_row, _)| {
            let dst_slice = &mut self.words[dst_start..dst_start + len];
            *dst_slice &= src_row;

            dst_start += Self::W_USIZE;
        });

        Ok(())
    }

    /// Performs a logical OR operation with another grid at the specified point.
    ///
    /// If the other grid does not fit at the specified point, the operation returns `Err(OutOfBounds)`.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfBounds`] if the `other` grid does not fit within `self` at `at`.
    pub fn bitor_at<const W2: u16, const H2: u16, const WORDS2: usize>(
        &mut self,
        other: &ArrayGrid<W2, H2, WORDS2>,
        at: ArrayPoint<W, H>,
    ) -> Result<(), OutOfBounds> {
        Self::ensure_fits(other, at)?;

        let mut src_start = 0;
        let mut dst_start = usize::from(at.y) * Self::W_USIZE + usize::from(at.x);
        let len = usize::from(W2);

        for _ in 0..H2 {
            let src_slice = &other.words[src_start..src_start + len];
            let dst_slice = &mut self.words[dst_start..dst_start + len];
            *dst_slice |= src_slice;

            src_start += ArrayGrid::<W2, H2, WORDS2>::W_USIZE;
            dst_start += Self::W_USIZE;
        }

        Ok(())
    }

    /// Performs a logical XOR operation with another grid at the specified point.
    ///
    /// If the other grid does not fit at the specified point, the operation returns `Err(OutOfBounds)`.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfBounds`] if the `other` grid does not fit within `self` at `at`.
    pub fn bitxor_at<const W2: u16, const H2: u16, const WORDS2: usize>(
        &mut self,
        other: &ArrayGrid<W2, H2, WORDS2>,
        at: ArrayPoint<W, H>,
    ) -> Result<(), OutOfBounds> {
        Self::ensure_fits(other, at)?;

        let mut src_start = 0;
        let mut dst_start = usize::from(at.y) * Self::W_USIZE + usize::from(at.x);

        // let w2: usize = W2 as usize;
        // let h2: usize = H2 as usize;

        // let src_rows = (0..h2).map(|row| row * w2).map(move |row| &other.words[row..row + w2]);
        // let dst_rows = (0..h2).map(|row| row * w2).map(|row| &mut self.words[row..row + w2])

        (0..H2).for_each(|_| {
            let src_slice = &other.words[src_start..src_start + ArrayGrid::<W2, H2, WORDS2>::W_USIZE];
            let dst_slice = &mut self.words[dst_start..dst_start + ArrayGrid::<W2, H2, WORDS2>::W_USIZE];
            *dst_slice ^= src_slice;

            src_start += ArrayGrid::<W2, H2, WORDS2>::W_USIZE;
            dst_start += Self::W_USIZE;
        });

        Ok(())
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

        self.words
            .as_mut_bitslice()
            .chunks_mut(Self::W_USIZE)
            .take(H as usize)
            .for_each(|row| row[min..max].fill(false));
    }

    /// Negates all cells in the grid.
    pub fn negate(&mut self) {
        self.words.data.iter_mut().for_each(|word| *word = !*word);
        self.clear_trailing_bits();
    }

    /// Provides the closure `f` with safe `mut` access to the underlying data.
    ///
    /// Note: This method provides the closure with the full `[u64]` slice. For grids
    /// where `W * H` is not a multiple of 64, some of the bits in the last element
    /// are unused (marked by [`Self::UNUSED_TAILING_BITS`]). They will be cleared
    /// after the closure returns.
    pub fn mutate_words<R>(&mut self, f: impl FnOnce(&mut [u64]) -> R) -> R {
        let r = f(&mut self.words.data);
        self.clear_trailing_bits();
        r
    }

    /// Mask of the unused tailing bits of the last word.
    pub const UNUSED_TRAILING_BITS: u64 = !Self::USED_TRAILING_BITS;

    /// Mask of the used bits of the last word.
    pub const USED_TRAILING_BITS: u64 = match Self::CELL_COUNT % u64::BITS {
        0 => u64::MAX,
        used => (1u64 << used) - 1,
    };

    /// Index of the last word in the grid.
    const LAST_WORD: usize = Self::WORD_COUNT - 1;

    /// Clears the unused tail bits of the last word.
    const fn clear_trailing_bits(&mut self) {
        self.words.data[Self::LAST_WORD] &= Self::USED_TRAILING_BITS;
    }
}

/// Conversion from a raw array of words.
///
/// Note: if `W * H` is not a multiple of 64, the trailing bits of the last word will be cleared.
impl<const W: u16, const H: u16, const WORDS: usize> From<[u64; WORDS]> for ArrayGrid<W, H, WORDS> {
    fn from(value: [u64; WORDS]) -> Self {
        let mut grid = Self { words: BitArray::new(value) };
        grid.clear_trailing_bits();
        grid
    }
}

impl<Idx, const W: u16, const H: u16, const WORDS: usize> FromIterator<Idx> for ArrayGrid<W, H, WORDS>
where
    Idx: Into<ArrayIndex<W, H>>,
{
    fn from_iter<T: IntoIterator<Item = Idx>>(iter: T) -> Self {
        iter.into_iter().map_into().fold_mut(Self::EMPTY, |grid, index| grid.const_update(index, true))
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
    Idx: Into<ArrayIndex<W, H>>,
{
    fn extend<T: IntoIterator<Item = Idx>>(&mut self, iter: T) {
        iter.into_iter().map_into().for_each(|index| self.const_update(index, true));
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
            .take(Self::CELL_COUNT_USZ + 1)
            .enumerate()
            .map(|(i, c)| (ArrayIndex::try_new(i), c))
            .try_fold((Self::EMPTY, None), |(mut grid, _), (i, c)| match (i, c) {
                (Err(_), _) => Err(PatternError::TooLong),
                (Ok(i), '#') => {
                    grid.const_update(i, true);
                    (grid, Some(i)).into_ok()
                }
                (Ok(i), '.') => (grid, Some(i)).into_ok(),
                (_, c) => PatternError::InvalidChar(c).into_err(),
            })
            .and_then(|(grid, index)| match index.map_or(0, |i| i.get() + 1) {
                i if i == Self::CELL_COUNT => Ok(grid),
                i => PatternError::TooShort(i).into_err(),
            })
    }
}
