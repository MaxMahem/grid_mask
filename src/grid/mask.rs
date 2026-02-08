use std::char;
use std::ops::Range;
use std::str::FromStr;

use fluent_result::bool::Then;
use fluent_result::into::{IntoOption, IntoResult};
use itertools::Itertools;
use tap::{Conv, Pipe};

use crate::err::PatternError;
use crate::ext::NotWhitespace;
use crate::ext::bits::{BitZeros, FromBitRange, OccupiedBitSpan};
use crate::ext::range::RangeLength;
use crate::grid::TryGridIndex;
use crate::num::{BitIndexIter, GridIndexU64, GridPos, SetBitsIter};
use crate::{Adjacency, Grid, GridPoint, GridRect, GridSize, GridVector};

/// An immutable mask of cells on a 8x8 grid.
///
/// Generic over the backing storage type `T`, which defaults to `u64`.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Default,
    Hash,
    derive_more::From,
    derive_more::BitAnd,
    derive_more::BitAndAssign,
    derive_more::BitOr,
    derive_more::BitOrAssign,
    derive_more::BitXor,
    derive_more::BitXorAssign,
    derive_more::Not,
)]
pub struct GridMask64<T = u64>(pub T);

impl<T> GridMask64<T> {
    /// Creates a new [`GridMask64`] from a value.
    pub const fn new(value: T) -> Self {
        Self(value)
    }
}

impl GridMask64 {
    const COLS_U32: u32 = <Self as Grid>::COLS as u32;

    /// A bitmask of the first column.
    pub(crate) const COL_FIRST: u64 = 0x0101_0101_0101_0101;

    /// Returns a new [`GridPoint`] with the cell at `pos` set.
    ///
    /// # Arguments
    ///
    /// * 'pos' - The position to set.
    ///
    /// # Type Parmeters
    ///
    /// * `Idx` - A type that can index a [`GridMask64`]
    ///
    /// ```rust
    /// # use grid_mask::{Grid, GridMask, GridPoint};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let point = GridPoint::try_new(1, 1)?;
    ///
    /// let mask = GridMask::EMPTY.set(point);
    ///
    /// assert_eq!(mask.index(point), true, "Should be set");
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn set<Idx: TryGridIndex>(&self, pos: Idx) -> Self {
        *self | pos.to_grid_mask()
    }

    /// Returns a new [`GridMask64`] with the cell at `pos` unset.
    ///
    /// # Arguments
    ///
    /// * 'pos' - The position to unset.
    ///
    /// # Type Parmeters
    ///
    /// * `Idx` - A type that can index a [`GridMask64`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{Grid, GridMask, GridPoint};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let point = GridPoint::try_new(1, 1)?;
    ///
    /// let mask = GridMask::FULL.unset(point);
    ///
    /// assert_eq!(mask.index(point), false, "Should be unset");
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn unset<Idx: TryGridIndex>(&self, pos: Idx) -> Self {
        *self & !pos.to_grid_mask()
    }

    /// Gets the value of the cell at `pos`
    ///
    /// # Arguments
    ///
    /// * 'pos' - The position to get.
    ///
    /// # Type Parmeters
    ///
    /// * `Idx` - A type that can index a [`GridMask64`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{Grid, GridMask, GridPoint};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let point = GridPoint::try_new(1, 1)?;
    ///
    /// assert_eq!(GridMask::FULL.index(point), true, "Should be set");
    /// assert_eq!(GridMask::EMPTY.index(point), false, "Should be unset");
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn index<Idx: TryGridIndex>(&self, pos: Idx) -> bool {
        pos.try_to_index().is_ok_and(|index| (self.0 & (1 << index.get())) != 0)
    }

    /// Returns an iterator over all cells of the mask.
    ///
    /// Iterates from the top-left cell (`(0, 0)`, least significant bit)
    /// to the bottom-right cell (`(7, 7)`, most significant bit).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{Grid, GridMask};
    /// let mask = GridMask::new(0b101);
    ///
    /// let mut cells = mask.cells();
    ///
    /// assert_eq!(cells.next(), Some(true));
    /// assert_eq!(cells.next(), Some(false));
    /// assert_eq!(cells.next(), Some(true));
    /// assert_eq!(cells.nth(60), Some(false));
    /// ```
    #[must_use]
    pub const fn cells(&self) -> Cells {
        Cells::new(*self)
    }

    /// Returns an iterator over the positions of all set cells of the mask.
    ///
    /// Iterates from the top-left cell (`(0, 0)`, least significant bit)
    /// to the bottom-right cell (`(7, 7)`, most significant bit).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{GridMask, GridPoint};
    /// let mask = GridMask::new(0b101);
    /// let points: Vec<_> = mask.points().collect();
    ///
    /// assert_eq!(points.len(), 2);
    /// assert_eq!(points[0], (0, 0));
    /// assert_eq!(points[1], (2, 0));
    /// ```
    #[must_use]
    pub fn points(&self) -> Points {
        Points::new(*self)
    }

    /// Returns a bitmask of the columns that are occupied in the mask.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{Grid, GridMask, GridPoint};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// assert_eq!(GridMask::EMPTY.occupied_cols(), 0b0000_0000);
    /// assert_eq!(GridMask::FULL.occupied_cols(), 0b1111_1111);
    /// assert_eq!(GridMask::new(1 | 1 << 63).occupied_cols(), 0b1000_0001);
    /// assert_eq!(GridMask::try_from(GridPoint::ORIGIN)?.occupied_cols(), 0b0000_0001);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub const fn occupied_cols(&self) -> u8 {
        // Merge the rows upwards
        let rows_2 = self.0 | (self.0 >> 8);
        let rows_4 = rows_2 | (rows_2 >> 16);
        let rows_8 = rows_4 | (rows_4 >> 32);
        (rows_8 & 0xFF) as u8
    }

    /// Returns a bitmask of the rows that are occupied in the mask.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{Grid, GridMask};
    /// assert_eq!(GridMask::EMPTY.occupied_rows(), 0b0000_0000);
    /// assert_eq!(GridMask::FULL.occupied_rows(), 0b1111_1111);
    /// assert_eq!(GridMask::new(1 | 1 << 63).occupied_rows(), 0b1000_0001);
    /// ```
    #[must_use]
    pub const fn occupied_rows(&self) -> u8 {
        const PACKED_ROWS: u64 = 0x0102_0408_1020_4080;

        // Merge bits horizontally within each row (byte)
        let bits_2 = self.0 | (self.0 >> 1);
        let bits_4 = bits_2 | (bits_2 >> 2);
        let bits_8 = bits_4 | (bits_4 >> 4);

        let row_bits = bits_8 & Self::COL_FIRST;

        (u64::wrapping_mul(row_bits, PACKED_ROWS) >> 56) as u8
    }

    /// Returns a range of the rows that are occupied in the mask.
    fn occupied_rows_span(self) -> Range<u8> {
        let start = self.0.trailing_zeros_u8() / 8;
        let end = (63 - self.0.leading_zeros_u8()) / 8 + 1;
        start..end
    }

    /// Returns the bounds of the mask.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{Grid, GridMask, GridRect};
    /// assert_eq!(GridMask::EMPTY.bounds(), None);
    /// assert_eq!(GridMask::FULL.bounds(), Some(GridRect::MAX));
    /// assert_eq!(GridMask::new(1 | 1 << 63).bounds(), Some(GridRect::MAX));
    /// ```
    #[must_use]
    pub fn bounds(&self) -> Option<GridRect> {
        self.is_empty().then_none()?;

        let y_span = self.occupied_rows_span();
        let x_span = self.occupied_cols().occupied_span();

        let point = GridPoint::new_unchecked(x_span.start, y_span.start);
        let size = GridSize::new_unchecked(x_span.length(), y_span.length());

        GridRect::new_unchecked(point, size).into_some()
    }

    /// Returns `true` if the mask is continuous.
    ///
    /// A mask is continuous if all set cells are connected via the
    /// [`Adjacency`] rule `A`.
    ///
    /// An empty mask is not considered continuous.
    ///
    /// # Type Parameters
    ///
    /// * `A` - The [`Adjacency`] strategy to use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{GridMask, Cardinal};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let connected: GridMask = "
    ///     . . . . . . . .
    ///     . # # # # # # .
    ///     . # . . . . # .
    ///     . # . . . . # .
    ///     . # . . . . . .
    ///     . # . . . . # .
    ///     . # # # # # # .
    ///     . . . . . . . .
    /// ".parse()?;
    ///
    /// assert!(connected.is_contiguous::<Cardinal>());
    ///
    /// let disconnected: GridMask = "
    ///     . . . . . . . .
    ///     . # # # # # # .
    ///     . # . . . . # .
    ///     . # . . . . # .
    ///     . . . . . . . .
    ///     . # . . . . # .
    ///     . # # # # # # .
    ///     . . . . . . . .
    /// ".parse()?;
    ///
    /// assert!(!disconnected.is_contiguous::<Cardinal>());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn is_contiguous<A: Adjacency>(&self) -> bool {
        GridIndexU64::from_first_set(self.0).is_some_and(|seed| self.connected::<A>(seed) == *self)
    }

    /// Creates a [`GridMask64`] from a string pattern.
    ///
    /// The pattern must contain exactly 64 characters matching either `set` or `unset`,
    /// ignoring any whitespace.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The string pattern to parse.
    /// * `set` - The character representing a set bit.
    /// * `unset` - The character representing an unset bit.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// * The pattern contains characters other than `set`, `unset`, or whitespace.
    ///   ([`PatternError::InvalidChar`])
    /// * The pattern contains too many or less than 64 valid characters.
    ///   ([`PatternError::TooLong`], [`PatternError::TooShort`])
    ///
    /// # Panics
    ///
    /// Panics if:
    /// * `set` is equal to `unset`
    /// * `set` or `unset` are [whitespace](char::is_whitespace)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use grid_mask::{GridMask, GridPoint};
    /// let pattern = "
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . # # . . . .
    ///     . . # # . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    /// ";
    ///
    /// let mask = GridMask::from_pattern(pattern, '#', '.')?;
    ///
    /// let points: Vec<(u8, u8)> = mask.points().map(Into::into).collect();
    /// assert_eq!(points, [(2, 2), (3, 2), (2, 3), (3, 3)]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_pattern<S: AsRef<str>>(pattern: S, set: char, unset: char) -> Result<Self, PatternError> {
        assert!(set != unset, "set and unset must be different");
        assert!(!set.is_whitespace(), "set cannot be whitespace");
        assert!(!unset.is_whitespace(), "unset cannot be whitespace");

        pattern
            .as_ref()
            .chars()
            .filter(NotWhitespace::is_not_whitespace)
            .enumerate()
            .take(65)
            .try_fold((Self::EMPTY, 0), |(mask, _), (i, c)| match (i, c) {
                (64.., _) => Err(PatternError::TooLong),
                (i, c) if c == set => (mask | Self(1 << i), i).into_ok(),
                (i, c) if c == unset => (mask, i).into_ok(),
                (_, c) => PatternError::InvalidChar(c).into_err(),
            })
            .and_then(|(mask, index)| match index {
                63 => Ok(mask),
                index => PatternError::TooShort(index + 1).into_err(),
            })
    }

    /// Return a [`Display`](std::fmt::Display) implementation that visualizes the mask.
    ///
    /// # Arguments
    ///
    /// * `set` - The character to use for set cells.
    /// * `unset` - The character to use for unset cells.
    #[must_use]
    pub fn visualize(&self, set: char, unset: char) -> impl std::fmt::Display + '_ {
        let map_char = move |is_set: bool| if is_set { set } else { unset };
        std::fmt::from_fn(move |f| {
            self.cells().map(map_char).enumerate().try_for_each(|(i, c)| match (i + 1) % (Self::ROWS as usize) == 0 {
                true => writeln!(f, "{c}"),
                false => write!(f, "{c}"),
            })
        })
    }
}

impl Grid for GridMask64<u64> {
    type Backing = u64;
    type Idx = GridIndexU64;

    const ROWS: u8 = 8;
    const COLS: u8 = 8;
    const EMPTY: Self = Self(0);
    const FULL: Self = Self(!0);

    fn count(&self) -> usize {
        self.0.count_ones() as usize
    }

    /// Translates the mask by the given vector.
    ///
    /// Cells that are shifted out of bounds are discarded.
    ///
    /// # Arguments
    ///
    /// * `vector` - The vector to translate by.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{Grid, GridMask, GridVector};
    /// # use std::str::FromStr;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mask: GridMask = "
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . # # . . # # .
    ///     . # # . . # # .
    ///     . . . . . . . .
    ///     . # . . . . # .
    ///     . # # # # # # .
    ///     . . . . . . . .
    /// ".parse()?;
    ///
    /// let translated = mask.translate(GridVector::new(3, 1));
    ///
    /// let expected: GridMask = "
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . # # . .
    ///     . . . . # # . .
    ///     . . . . . . . .
    ///     . . . . # . . .
    ///     . . . . # # # #
    /// ".parse()?;
    /// assert_eq!(translated, expected);
    /// # Ok(())
    /// # }
    /// ```
    fn translate(&self, vector: GridVector) -> Self {
        let mask_shifted_y = match vector.y {
            dy @ 1..=7 => self.0 << (dy.unsigned_abs().conv::<u32>() * Self::COLS_U32),
            dy @ -7..=-1 => self.0 >> (dy.unsigned_abs().conv::<u32>() * Self::COLS_U32),
            0 => self.0,
            _ => return Self::EMPTY,
        };

        match vector.x {
            dx @ 1..=7 => {
                let shift: u32 = dx.unsigned_abs().into();
                let mask_shifted_x_y = mask_shifted_y << shift;

                // Safety: shift is always <= 7, so it is always a valid GridPos
                #[expect(clippy::cast_possible_truncation, reason = "shift is always <= 7")]
                let shift_pos = unsafe { GridPos::new_unchecked(shift as u8) };

                let col_mask = u64::from_bit_range(..shift_pos) * Self::COL_FIRST;

                Self(mask_shifted_x_y & !col_mask)
            }
            dx @ -7..=-1 => {
                let shift: u32 = dx.unsigned_abs().into();
                let mask_shifted_x_y = mask_shifted_y >> shift;

                #[expect(clippy::cast_possible_truncation, reason = "shift is always <= 7")]
                let start_pos = unsafe { GridPos::new_unchecked(8 - shift as u8) };

                let col_mask = u64::from_bit_range(start_pos..) * Self::COL_FIRST;

                Self(mask_shifted_x_y & !col_mask)
            }
            0 => Self(mask_shifted_y),
            _ => Self::EMPTY,
        }
    }
}

/// An iterator over all cells of a [`GridMask64`].
#[derive(Debug, Clone)]
pub struct Cells {
    mask: GridMask64<u64>,
    iter: BitIndexIter,
}

impl Cells {
    const fn new(mask: GridMask64<u64>) -> Self {
        Self { mask, iter: BitIndexIter::new() }
    }
}

impl Iterator for Cells {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|i| self.mask.index(i))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl DoubleEndedIterator for Cells {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|i| self.mask.index(i))
    }
}

impl ExactSizeIterator for Cells {}
impl std::iter::FusedIterator for Cells {}

/// An iterator over all set cells of a [`GridMask64`].
#[derive(Debug, Clone)]
pub struct Points(SetBitsIter);

impl Points {
    fn new(mask: GridMask64<u64>) -> Self {
        Self(GridIndexU64::iter_set_bits(mask.0))
    }
}

impl Iterator for Points {
    type Item = GridPoint;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(GridPoint::from)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for Points {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(GridPoint::from)
    }
}

impl ExactSizeIterator for Points {}
impl std::iter::FusedIterator for Points {}

impl<I: Into<Self>> FromIterator<I> for GridMask64<u64> {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        iter.into_iter().map_into().fold(Self::EMPTY, |mask, item| mask | item)
    }
}

impl IntoIterator for GridMask64<u64> {
    type Item = GridPoint;
    type IntoIter = Points;

    fn into_iter(self) -> Self::IntoIter {
        self.points()
    }
}

impl From<GridRect> for GridMask64<u64> {
    fn from(rect: GridRect) -> Self {
        let (x2, y2): (GridIndexU64, _) = rect.bottom_right().into();
        let (x1, y1): (GridIndexU64, _) = rect.point().into();

        let row_mask = u64::from_bit_range(x1..=x2);

        (y1..=y2)
            .map(|row: u8| row * Self::ROWS)
            .map(|row_start| row_mask << row_start)
            .fold(0u64, std::ops::BitOr::bitor)
            .pipe(Self)
    }
}

impl From<GridIndexU64> for GridMask64<u64> {
    fn from(idx: GridIndexU64) -> Self {
        Self(1u64 << idx.get())
    }
}

impl From<GridPoint> for GridMask64<u64> {
    fn from(pos: GridPoint) -> Self {
        Self(1u64 << pos.0.get())
    }
}

impl From<[bool; 64]> for GridMask64<u64> {
    fn from(bools: [bool; 64]) -> Self {
        #[expect(clippy::cast_possible_truncation, reason = "i is always <= 63")]
        bools
            .iter()
            .enumerate()
            .filter_map(|(i, &set)| set.then_some(i))
            // Safety: i is always <= 63, so it is always a valid GridIndexU64
            .map(|i| unsafe { GridIndexU64::new_unchecked(i as u8) })
            .map_into()
            .fold(Self::EMPTY, |mask, i| mask | i)
    }
}

impl FromStr for GridMask64<u64> {
    type Err = PatternError;

    /// Parses a string pattern into a [`GridMask64`].
    ///
    /// Uses `#` for set cells and `.` for unset cells.
    /// Whitespace is ignored.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The pattern contains characters other than `set`, `unset`, or whitespace.
    /// * The pattern contains too many or too few valid characters (must be exactly 64).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{Grid, GridMask};
    /// # use std::str::FromStr;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let pattern = "
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . # # . . . .
    ///     . . # # . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    ///     . . . . . . . .
    /// ";
    ///
    /// let mask: GridMask = GridMask::from_str(pattern)?;
    ///
    /// assert_eq!(mask.count(), 4);
    /// # Ok(())
    /// # }
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_pattern(s, '#', '.')
    }
}
