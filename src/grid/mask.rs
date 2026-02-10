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
use crate::grid::{Cells, GridData, GridDataMut, GridDataValue, Points, Spaces};
use crate::num::{BitIndexU8, BitIndexU64};
use crate::{Adjacency, GridIndex, GridPoint, GridRect, GridSize, GridVector};

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
    // derive_more::Constructor,
)]
pub struct GridMask<T = u64>(pub(crate) T);

impl<T> GridMask<T> {
    /// Creates a new [`GridMask`] from a value.
    pub const fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T: GridData> GridMask<T> {
    /// An empty mask.
    pub const EMPTY: Self = Self(T::EMPTY);
    /// A full mask.
    pub const FULL: Self = Self(T::FULL);

    /// The number of rows in the mask.
    pub const ROWS: T::RowLen = T::ROWS;
    /// The number of columns in the mask.
    pub const COLS: T::ColLen = T::COLS;

    delegate::delegate! {
        to self.0 {
            /// Returns the number of set cells.
            pub fn count(&self) -> usize;
            /// Returns the state of the cell at `index`.
            pub fn index<Idx: GridIndex<T>>(&self, index: Idx) -> bool;
        }
    }

    /// Returns `true` if the mask is [`EMPTY`](Self::EMPTY).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0 == T::EMPTY
    }

    /// Returns `true` if the mask is [`FULL`](Self::FULL).
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.0 == T::FULL
    }

    /// Returns an iterator over all cells of the mask.
    ///
    /// Iterates from the top-left cell (`(0, 0)`) to the bottom-right cell
    /// (`(T::COLS - 1, T::ROWS - 1)`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::GridMask;
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
    pub const fn cells(&self) -> Cells<'_, T> {
        Cells::new(self)
    }
}

impl<T: GridDataMut> GridMask<T> {
    delegate::delegate! {
        to self.0 {
            /// Sets the cell at `index`.
            pub fn set<Idx: GridIndex<T>>(&mut self, index: Idx);
            /// Unsets the cell at `index`.
            pub fn unset<Idx: GridIndex<T>>(&mut self, index: Idx);
            /// Translates the mask by `delta` in place.
            pub fn translate_mut(&mut self, delta: GridVector);
            /// Negates all cells in the mask in place.
            pub fn negate(&mut self);
        }
    }
}

impl<T: GridDataValue> GridMask<T> {
    delegate::delegate! {
        to self.0 {
            /// Returns a new mask with the cell at `index` set.
            #[must_use]
            #[expr(Self($))]
            pub fn with_set<Idx: GridIndex<T>>(&self, index: Idx) -> Self;

            /// Returns a new mask with the cell at `index` unset.
            #[must_use]
            #[expr(Self($))]
            pub fn with_unset<Idx: GridIndex<T>>(&self, index: Idx) -> Self;

            /// Returns a new mask translated by `delta`.
            #[must_use]
            #[expr(Self($))]
            pub fn translate(&self, delta: GridVector) -> Self;
        }
    }
}

impl GridMask<u64> {
    /// A bitmask of the first column.
    pub(crate) const COL_FIRST: u64 = 0x0101_0101_0101_0101;

    /// Returns a [`GridMask`] of all points connected to `seed` within the current mask
    /// using the provided [`Adjacency`].
    ///
    /// # Arguments
    ///
    /// * `seed` - The starting point for the flood fill.
    ///
    /// # Type Parameters
    ///
    /// * `A` - The [`Adjacency`] strategy to use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # use grid_mask::{GridPoint, GridMask, GridRect, Cardinal};
    /// let mask: GridMask = GridRect::new((0, 0), (2, 2))?.into();
    /// let connected = mask.contiguous::<Cardinal>(GridPoint::ORIGIN);
    /// assert_eq!(connected, mask);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn contiguous<A: Adjacency>(self, seed: impl Into<BitIndexU64>) -> Self {
        match seed.into().conv::<Self>() & self {
            connected if connected.is_empty() => Self::EMPTY,
            mut connected => loop {
                match A::connected(connected.0) & self.0 {
                    grown if grown == connected.0 => break connected,
                    grown => connected = Self(grown),
                }
            },
        }
    }

    /// Returns a [`GridMask`] of all points connected to `seed` within the current mask
    /// using the provided [`Adjacency`].
    ///
    /// # Arguments
    ///
    /// * `seed` - The starting point for the flood fill.
    ///
    /// # Type Parameters
    ///
    /// * `A` - The [`Adjacency`] strategy to use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # use grid_mask::{GridPoint, GridMask, GridRect, Cardinal};
    /// let mask: GridMask = GridRect::new((0, 0), (2, 2))?.into();
    /// let connected = mask.contiguous::<Cardinal>(GridPoint::ORIGIN);
    /// assert_eq!(connected, mask);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn grow<A: Adjacency>(self) -> Self {
        A::connected::<u64>(self.0).pipe(Self)
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

    /// Returns an iterator over the positions of all unset cells of the mask.
    ///
    /// Iterates from the top-left cell (`(0, 0)`, least significant bit)
    /// to the bottom-right cell (`(7, 7)`, most significant bit).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{GridMask, GridPoint};
    /// let mask = GridMask::FULL.with_unset(GridPoint::ORIGIN);
    /// let spaces: Vec<GridPoint> = mask.spaces().collect();
    ///
    /// assert_eq!(spaces.len(), 1);
    /// assert_eq!(spaces[0], (0, 0));
    /// ```
    #[must_use]
    pub fn spaces(&self) -> Spaces {
        Spaces::new(*self)
    }

    /// Returns a bitmask of the columns that are occupied in the mask.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{GridMask, GridPoint};
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
    /// # use grid_mask::GridMask;
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
    /// # use grid_mask::{GridMask, GridRect};
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
        BitIndexU64::from_first_set(self.0).is_some_and(|seed| self.contiguous::<A>(seed) == *self)
    }

    /// Creates a [`GridMask`] from a string pattern.
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
            self.cells().map(map_char).enumerate().try_for_each(|(i, c)| {
                match (i + 1) % (Self::ROWS.conv::<usize>()) == 0 {
                    true => writeln!(f, "{c}"),
                    false => write!(f, "{c}"),
                }
            })
        })
    }
}

impl From<GridMask<Self>> for u64 {
    fn from(value: GridMask<Self>) -> Self {
        value.0
    }
}

impl<I: Into<Self>> FromIterator<I> for GridMask<u64> {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        iter.into_iter().map_into().fold(Self::EMPTY, |mask, item| mask | item)
    }
}

impl IntoIterator for GridMask<u64> {
    type Item = GridPoint;
    type IntoIter = Points;

    fn into_iter(self) -> Self::IntoIter {
        self.points()
    }
}

impl From<GridRect> for GridMask<u64> {
    fn from(rect: GridRect) -> Self {
        let (x2, y2): (BitIndexU8, _) = rect.bottom_right().into();
        let (x1, y1): (BitIndexU8, _) = rect.point().into();

        let row_mask = u8::from_bit_range(x1..=x2);

        (y1..=y2)
            .map(|row: u8| row * Self::ROWS)
            .map(|row_start| u64::from(row_mask) << row_start)
            .fold(0u64, std::ops::BitOr::bitor)
            .pipe(Self)
    }
}

impl From<BitIndexU64> for GridMask<u64> {
    fn from(idx: BitIndexU64) -> Self {
        Self(1u64 << idx.get())
    }
}

impl From<GridPoint> for GridMask<u64> {
    fn from(pos: GridPoint) -> Self {
        Self(1u64 << pos.0.get())
    }
}

impl From<[bool; 64]> for GridMask<u64> {
    fn from(bools: [bool; 64]) -> Self {
        #[expect(clippy::cast_possible_truncation, reason = "i is always <= 63")]
        bools
            .iter()
            .enumerate()
            .filter_map(|(i, &set)| set.then_some(i))
            // Safety: i is always <= 63, so it is always a valid GridIndexU64
            .map(|i| unsafe { BitIndexU64::new_unchecked(i as u8) })
            .map_into()
            .fold(Self::EMPTY, |mask, i| mask | i)
    }
}

impl FromStr for GridMask<u64> {
    type Err = PatternError;

    /// Parses a string pattern into a [`GridMask`].
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
    /// # use grid_mask::GridMask;
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
