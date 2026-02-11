use std::num::NonZeroU16;

use itertools::Itertools;
use num_integer::Integer;

use crate::ext::FoldMut;
use crate::{ArrayIndex, ArrayVector};

/// A fixed-size bit grid with `W` columns and `H` rows.
#[readonly::make]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayGrid<const W: u16, const H: u16, const WORDS: usize> {
    pub(crate) words: [u64; WORDS],
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
    pub const EMPTY: Self = Self { words: [0; WORDS] };

    /// A full grid with all bits set.
    pub const FULL: Self = const {
        let mut grid = Self { words: [u64::MAX; WORDS] };
        grid.clear_trailing_bits();
        grid
    };

    /// The width of the grid.
    pub const WIDTH: NonZeroU16 = NonZeroU16::new(W).expect("Width must be greater than 0");
    /// The height of the grid.
    pub const HEIGHT: NonZeroU16 = NonZeroU16::new(H).expect("Height must be greater than 0");
    /// The total number of cells in the grid.
    pub const CELL_COUNT: u32 = W as u32 * H as u32;

    /// The number of `u64` words used to store the grid data.
    const WORD_COUNT: usize = const {
        assert!(
            WORDS == usize::div_ceil(Self::CELL_COUNT as usize, u64::BITS as usize),
            "WORDS must match the minimum number of words needed to represent the grid"
        );
        WORDS
    };

    /// Gets the cell at `index` to `value`.
    #[must_use]
    pub fn get<Idx: Into<ArrayIndex<W, H>>>(&self, index: Idx) -> bool {
        let (word, bit) = index.into().word_and_bit();
        self.words[word] & (1_u64 << bit) != 0
    }

    /// Returns the number of set cells in the grid.
    #[must_use]
    pub fn count(&self) -> u32 {
        self.words.iter().copied().map(u64::count_ones).sum()
    }

    /// Returns the raw data.
    #[must_use]
    pub const fn words(&self) -> &[u64] {
        &self.words
    }

    /// Sets the cell at `index` to `value`.
    pub fn set<Idx: Into<ArrayIndex<W, H>>>(&mut self, index: Idx, value: bool) {
        self.const_set(index.into(), value);
    }

    /// Sets the cell at `index` to `value`.
    // TODO: Remove when const traits stabilize
    pub const fn const_set(&mut self, index: ArrayIndex<W, H>, value: bool) {
        match (index.word_and_bit(), value) {
            ((word, bit), true) => self.words[word] |= 1u64 << bit,
            ((word, bit), false) => self.words[word] &= !(1u64 << bit),
        }
    }

    /// Clears all cells in the grid.
    pub fn clear(&mut self) {
        self.words.fill(0);
    }

    /// Fills all cells in the grid with `value`.
    pub fn fill(&mut self, value: bool) {
        match value {
            false => self.words.fill(0),
            true => {
                self.words.fill(u64::MAX);
                self.clear_trailing_bits();
            }
        }
    }

    /// Translates the grid by the given displacement vector.
    ///
    /// Bits that shift beyond the grid boundary are discarded; vacated
    /// positions are filled with `false`.
    pub fn translate_mut(&mut self, vec: ArrayVector) {
        let (dx, dy) = (vec.dx, vec.dy);

        // Shift exceeds grid bounds → result is empty
        if dx.unsigned_abs() >= W as u32 || dy.unsigned_abs() >= H as u32 {
            self.clear();
            return;
        }

        // Combined flat bit shift: dy rows + dx columns
        let total = i64::from(dy) * i64::from(W) + i64::from(dx);

        match total {
            total if total > 0 => self.flat_shift_left(total as u32),
            total if total < 0 => self.flat_shift_right((-total) as u32),
            _ => (),
        }

        // Mask out columns that wrapped across row boundaries
        if dx != 0 {
            self.clear_wrapped_columns(dx);
        }

        self.clear_trailing_bits();
    }

    /// Shifts the entire flat bit array left (toward higher bit indices) by `n` bits.
    fn flat_shift_left(&mut self, n: u32) {
        let (word_shift, bit_shift) = u32::div_rem(&n, &u64::BITS);

        (0..WORDS)
            .rev() // iterate backwards to avoid overwriting data we haven't moved yet
            .map(|dest| (dest.checked_sub(word_shift as usize), dest))
            .for_each(|(src, dest)| {
                self.words[dest] = match (src, bit_shift) {
                    // whole word shift
                    (Some(src), 0) => self.words[src],
                    // partial word shift
                    (Some(src), _) => {
                        // carry bits from previous word
                        if let Some(prev) = src.checked_sub(1) {
                            self.words[src] << bit_shift | self.words[prev] >> (64 - bit_shift)
                        // no carry bits
                        } else {
                            self.words[src] << bit_shift
                        }
                    }
                    // src is None → shift out of bounds
                    _ => 0u64,
                };
            });
    }

    /// Shifts the entire flat bit array right (toward lower bit indices) by `n` bits.
    fn flat_shift_right(&mut self, n: u32) {
        let (word_shift, bit_shift) = u32::div_rem(&n, &u64::BITS);

        (0..WORDS).map(|dest| (dest + word_shift as usize, dest)).for_each(|(src, dest)| {
            let mut val = 0u64;
            if src < WORDS {
                val = self.words[src] >> bit_shift;
                if bit_shift > 0 && src + 1 < WORDS {
                    val |= self.words[src + 1] << (64 - bit_shift);
                }
            }
            self.words[dest] = val;
        });
    }

    /// Clears the columns that incorrectly wrapped across row boundaries after
    /// the flat bit shift.
    ///
    /// When `dx > 0` (east), the first `dx` columns of each row must be cleared.
    /// When `dx < 0` (west), the last `|dx|` columns of each row must be cleared.
    fn clear_wrapped_columns(&mut self, dx: i32) {
        let abs_dx = dx.unsigned_abs();
        let w = W as u32;

        for word_idx in 0..WORDS {
            let base = word_idx as u32 * 64;
            let mut mask = u64::MAX;

            for bit in 0..64u32 {
                let flat = base + bit;
                if flat >= Self::CELL_COUNT {
                    break;
                }
                let col = flat % w;
                let clear = if dx > 0 { col < abs_dx } else { col >= w - abs_dx };
                if clear {
                    mask &= !(1u64 << bit);
                }
            }
            self.words[word_idx] &= mask;
        }
    }

    /// Negates all cells in the grid.
    pub fn negate(&mut self) {
        self.words.iter_mut().for_each(|word| *word = !*word);
        self.clear_trailing_bits();
    }

    /// Provides the closure `f` with safe `mut` access to the underlying data.
    ///
    /// Note: This method provides the closure with the full `[u64]` slice. For grids
    /// where `W * H` is not a multiple of 64, some of the bits in the last element
    /// are unused (marked by [`Self::UNUSED_TAILING_BITS`]). They will be cleared
    /// after the closure returns.
    pub fn mutate_words<R>(&mut self, f: impl FnOnce(&mut [u64]) -> R) -> R {
        let r = f(&mut self.words);
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
        self.words[Self::LAST_WORD] &= Self::USED_TRAILING_BITS;
    }
}

impl<const W: u16, const H: u16, const WORDS: usize> From<[u64; WORDS]> for ArrayGrid<W, H, WORDS> {
    fn from(value: [u64; WORDS]) -> Self {
        let mut grid = Self { words: value };
        grid.clear_trailing_bits();
        grid
    }
}

impl<Idx, const W: u16, const H: u16, const WORDS: usize> FromIterator<Idx> for ArrayGrid<W, H, WORDS>
where
    Idx: Into<ArrayIndex<W, H>>,
{
    fn from_iter<T: IntoIterator<Item = Idx>>(iter: T) -> Self {
        iter.into_iter().map_into().fold_mut(Self::EMPTY, |grid, index| grid.const_set(index, true))
    }
}

impl<Idx, const W: u16, const H: u16, const WORDS: usize> Extend<Idx> for ArrayGrid<W, H, WORDS>
where
    Idx: Into<ArrayIndex<W, H>>,
{
    fn extend<T: IntoIterator<Item = Idx>>(&mut self, iter: T) {
        iter.into_iter().map_into().for_each(|index| self.const_set(index, true));
    }
}
