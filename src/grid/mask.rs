use std::char;
use std::ops::Range;
use std::str::FromStr;

use fluent_result::bool::Then;
use fluent_result::into::{IntoOption, IntoResult};
use itertools::Itertools;
use tap::{Conv, Pipe, TryConv};

use crate::err::PatternError;
use crate::ext::NotWhitespace;
use crate::ext::bits::{BitZeros, FromBitRange, OccupiedBitSpan};
use crate::ext::range::RangeLength;
use crate::grid::{Cells, Points, Spaces};
use crate::num::{BitIndexU8, BitIndexU64, GridLen, GridPos, SignedMag, VecMagU64};
use crate::{Adjacency, GridDelta, GridPoint, GridRect, GridSize, GridVector};

/// An immutable mask of cells on a 8x8 grid.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Default,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::BitAnd,
    derive_more::BitAndAssign,
    derive_more::BitOr,
    derive_more::BitOrAssign,
    derive_more::BitXor,
    derive_more::BitXorAssign,
    derive_more::Not,
    // derive_more::Constructor,
)]
pub struct GridMask(pub u64);

impl GridMask {
    /// An empty mask.
    pub const EMPTY: Self = Self(0);
    /// A full mask.
    pub const FULL: Self = Self(u64::MAX);

    /// The number of rows in the mask.
    pub const ROWS: GridLen = GridLen::const_new::<8>();
    /// The number of columns in the mask.
    pub const COLS: GridLen = GridLen::const_new::<8>();

    /// A bitmask of the first column.
    pub(crate) const COL_FIRST: u64 = 0x0101_0101_0101_0101;

    /// Returns the number of set cells.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.0.count_ones() as usize
    }

    /// Returns the state of the cell at `index`.
    pub fn get<Idx: Into<BitIndexU64>>(&self, index: Idx) -> bool {
        (*self & index.into().conv::<Self>()) != Self::EMPTY
    }

    /// Updates the cell at `index` to `value`.
    pub fn update<Idx: Into<BitIndexU64>>(&mut self, index: Idx, value: bool) {
        *self = self.with(index, value);
    }

    /// Returns a new mask with the cell at `index` set to `value`.
    #[must_use]
    pub fn with<Idx: Into<BitIndexU64>>(self, index: Idx, value: bool) -> Self {
        if value { self.const_set::<true>(index.into()) } else { self.const_set::<false>(index.into()) }
    }

    /// Sets a new mask with the cell at `index` set to `value`.
    #[must_use]
    const fn const_set<const VALUE: bool>(self, index: BitIndexU64) -> Self {
        match (VALUE, 1 << index.get()) {
            (true, bit) => Self(self.0 | bit),
            (false, bit) => Self(self.0 & !bit),
        }
    }

    const COLS_U32: u32 = 8;

    /// Returns a new mask translated by `delta`.
    #[must_use]
    pub fn translate(&self, delta: GridVector) -> Self {
        delta
            .try_conv::<GridDelta<VecMagU64>>()
            .map_or(0, |delta| {
                let data = self.0;

                let data_shifted_y = match delta.y {
                    SignedMag::Positive(dy) => data << (dy.get().conv::<u32>() * Self::COLS_U32),
                    SignedMag::Negative(dy) => data >> (dy.get().conv::<u32>() * Self::COLS_U32),
                    SignedMag::Zero => data,
                };

                match delta.x {
                    SignedMag::Positive(dx) => {
                        let mask_shifted_x_y = data_shifted_y << dx.get();

                        let col_mask = u8::from_bit_range(..dx).conv::<u64>() * Self::COL_FIRST;

                        mask_shifted_x_y & !col_mask
                    }
                    SignedMag::Negative(dx) => {
                        let col_mask = u8::from_bit_range(..dx).conv::<u64>() * Self::COL_FIRST;
                        (data_shifted_y & !col_mask) >> dx.get()
                    }
                    SignedMag::Zero => data_shifted_y,
                }
            })
            .pipe(Self)
    }

    /// Returns `true` if the mask is [`EMPTY`](Self::EMPTY).
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if the mask is [`FULL`](Self::FULL).
    #[must_use]
    pub const fn is_full(&self) -> bool {
        self.0 == u64::MAX
    }

    /// Returns an iterator over all cells of the mask.
    ///
    /// Iterates from the top-left cell (`(0, 0)`) to the bottom-right cell
    /// (`(7, 7)`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::GridMask;
    /// let mask = GridMask(0b101);
    ///
    /// let mut cells = mask.cells();
    ///
    /// assert_eq!(cells.next(), Some(true));
    /// assert_eq!(cells.next(), Some(false));
    /// assert_eq!(cells.next(), Some(true));
    /// assert_eq!(cells.nth(60), Some(false));
    /// ```
    #[must_use]
    pub const fn cells(&self) -> Cells<'_> {
        Cells::new(self)
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
    pub fn contiguous<A: Adjacency>(self, seed: impl Into<BitIndexU64>) -> Self {
        match seed.into().conv::<Self>() & self {
            connected if connected.is_empty() => Self::EMPTY,
            mut connected => loop {
                match A::connected(connected) & self {
                    grown if grown == connected => break connected,
                    grown => connected = grown,
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
        A::connected(self)
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
    /// let mask = GridMask(0b101);
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
    /// let mask = GridMask::FULL.with(GridPoint::ORIGIN, false);
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
    /// assert_eq!(GridMask(1 | 1 << 63).occupied_cols(), 0b1000_0001);
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
    /// assert_eq!(GridMask(1 | 1 << 63).occupied_rows(), 0b1000_0001);
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
    /// assert_eq!(GridMask(1 | 1 << 63).bounds(), Some(GridRect::MAX));
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

// impl From<GridMask> for u64 {
//     fn from(value: GridMask) -> Self {
//         value.0
//     }
// }

impl<I: Into<Self>> FromIterator<I> for GridMask {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        iter.into_iter().map_into().fold(Self::EMPTY, |mask, item| mask | item)
    }
}

impl IntoIterator for GridMask {
    type Item = GridPoint;
    type IntoIter = Points;

    fn into_iter(self) -> Self::IntoIter {
        self.points()
    }
}

impl From<GridRect> for GridMask {
    fn from(rect: GridRect) -> Self {
        let (x2, y2): (BitIndexU8, GridPos) = rect.bottom_right().into();
        let (x1, y1): (BitIndexU8, GridPos) = rect.point().into();

        let col_mask = u8::from_bit_range(x1..=x2).conv::<u64>() * Self::COL_FIRST;

        let start = BitIndexU64::at(GridPos::MIN, y1);
        let end = BitIndexU64::at(GridPos::MAX, y2);
        let row_mask = u64::from_bit_range(start..=end);

        Self(col_mask & row_mask)
    }
}

impl From<BitIndexU64> for GridMask {
    fn from(idx: BitIndexU64) -> Self {
        Self(1u64 << idx.get())
    }
}

impl From<GridPoint> for GridMask {
    fn from(pos: GridPoint) -> Self {
        Self(1u64 << pos.0.get())
    }
}

impl From<[bool; 64]> for GridMask {
    fn from(bools: [bool; 64]) -> Self {
        std::iter::zip(bools, BitIndexU64::all_values())
            .filter_map(|(set, i)| set.then_some(i))
            .map_into()
            .fold(Self::EMPTY, |mask, i| mask | i)
    }
}

impl FromStr for GridMask {
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
        s.chars()
            .filter(NotWhitespace::is_not_whitespace)
            .take(65)
            .enumerate()
            .map(|(i, c)| (BitIndexU64::try_from(i), c))
            .try_fold((Self::EMPTY, None), |(mask, _), (i, c)| match (i, c) {
                (Err(_), _) => Err(PatternError::TooLong),
                (Ok(i), '#') => (mask | i.into(), Some(i)).into_ok(),
                (Ok(i), '.') => (mask, Some(i)).into_ok(),
                (_, c) => PatternError::InvalidChar(c).into_err(),
            })
            .and_then(|(mask, index)| match index.map_or(0, |i| i.get() + 1) {
                64 => Ok(mask),
                index => index.conv::<u32>().pipe(PatternError::TooShort).into_err(),
            })
    }
}
