use std::marker::PhantomData;

// use collect_failable::TryFromIterator;
use fluent_result::into::IntoResult;
use tap::{Conv, Pipe};

use crate::err::Discontiguous;
use crate::num::BitIndexU64;
use crate::{Adjacency, Cardinal, GridMask, GridRect};

impl<Adj: Adjacency> From<GridRect> for GridShape<Adj> {
    fn from(rect: GridRect) -> Self {
        GridMask::from(rect).pipe(Self::new)
    }
}

/// A contiguous shape on an 8x8 grid.
///
/// A `GridShape` is a [`GridMask`] that guarantees that all set cells are
/// connected via the [`Adjacency`] strategy, `A`.
///
/// # Type Parameters
///
/// * `A` - The type of [`Adjacency`] strategy
///
/// # Examples
///
/// ```rust
/// # use grid_mask::{GridShape, GridMask, GridPoint, GridRect};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create a shape from a rectangle (always contiguous)
/// let rect = GridRect::new(GridPoint::ORIGIN, (2, 2))?;
/// let mask = GridMask::from(rect);
/// let shape: GridShape = mask.try_into()?;
///
/// // GridShape wraps a contiguous GridMask
/// assert_eq!(mask.count(), 4);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, derive_more::Into, derive_more::Deref, derive_more::AsRef)]
pub struct GridShape<A = Cardinal>(
    #[deref]
    #[as_ref]
    GridMask,
    #[into(skip)] PhantomData<A>,
);

impl<A: Adjacency> GridShape<A> {
    /// A shape that contains all cells.
    pub const FULL: Self = Self(GridMask::FULL, PhantomData);

    /// Creates a new mask
    pub(crate) const fn new(data: GridMask) -> Self {
        Self(data, PhantomData)
    }
}

impl<A: Adjacency> GridShape<A> {
    /// Creates a new [`GridShape`] from data if it is contiguous.
    ///
    /// A mask is contiguous if all set cells are connected via the adjacency rule `A`.
    ///
    /// # Errors
    ///
    /// [`Discontiguous`] if the mask is not contiguous.
    pub fn contiguous(grid: GridMask, seed: impl Into<BitIndexU64>) -> Result<Self, Discontiguous> {
        let seed: BitIndexU64 = seed.into();
        match grid.get(seed) {
            false => return grid.conv::<GridMask>().pipe(Discontiguous).into_err(),
            true => grid,
        }
        .pipe(|grid| GrowableSeed::<A>::new(seed, grid))
        .connect()
        .pipe(Self::new)
        .into_ok()
    }
}

/// A type that gurantees that `seed` is set in `mask`
#[derive(Debug)]
struct GrowableSeed<Adj> {
    seed: BitIndexU64,
    mask: GridMask,
    _adj: PhantomData<Adj>,
}

impl<Adj: Adjacency> GrowableSeed<Adj> {
    const fn new(seed: BitIndexU64, mask: GridMask) -> Self {
        Self { seed, mask, _adj: PhantomData }
    }

    fn connect(self) -> GridMask {
        let mut connected = self.mask & GridMask::from(self.seed);

        loop {
            match Adj::connected(connected) & self.mask {
                grown if grown == connected => break connected,
                grown => connected = grown,
            }
        }
    }
}

impl<A: Adjacency> TryFrom<GridMask> for GridShape<A> {
    type Error = Discontiguous;

    /// Creates a [`GridShape`] from a [`GridMask`] if `data` is contiguous.
    ///
    /// A mask is contiguous if all set cells are connected via the adjacency rule `Cardinal`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{GridShape, GridMask, GridPoint};
    /// // A single point is contiguous
    /// let mask = GridMask::from(GridPoint::ORIGIN);
    /// let shape: Result<GridShape, _> = mask.try_into();
    /// assert!(shape.is_ok());
    /// ```
    fn try_from(mask: GridMask) -> Result<Self, Self::Error> {
        let grid_u64 = mask.0;
        let connected = BitIndexU64::from_first_set(grid_u64)
            .ok_or(Discontiguous(mask))?
            .pipe(|seed| GrowableSeed::<A>::new(seed, mask))
            .connect();

        // Mask is contiguous iff connected region equals original mask
        (connected == mask).then_some(Self::new(mask)).ok_or(Discontiguous(mask))
    }
}

impl<A: Adjacency> TryFrom<u64> for GridShape<A> {
    type Error = Discontiguous;

    fn try_from(mask: u64) -> Result<Self, Self::Error> {
        GridMask::from(mask).try_into()
    }
}

impl TryFrom<[bool; 64]> for GridShape<Cardinal> {
    type Error = Discontiguous;

    fn try_from(bools: [bool; 64]) -> Result<Self, Self::Error> {
        GridMask::from(bools).try_into()
    }
}

// impl FromStr for GridShape {
//     type Err = ShapePatternError;
//
//     /// Parses a string pattern into a [`GridShape`].
//     ///
//     /// Uses `#` for set cells and `.` for unset cells. Whitespace is ignored.
//     ///
//     /// # Errors
//     ///
//     /// Errors if:
//     ///
//     /// * The pattern is empty or not contiguous ([`ShapePatternError::Discontiguous`])
//     /// * The pattern contains characters other than `#`, `.` and whitespace
//     ///   ([`ShapePatternError::Pattern`])
//     /// * The pattern is longer or shorter than 64 characters ([`ShapePatternError::Pattern`])
//     ///
//     /// # Examples
//     ///
//     /// ```rust
//     /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
//     /// # use grid_mask::GridShape;
//     /// let pattern = "
//     ///     . . . . . . . .
//     ///     . . . . . . . .
//     ///     . . # # . . . .
//     ///     . . # # . . . .
//     ///     . . . . . . . .
//     ///     . . . . . . . .
//     ///     . . . . . . . .
//     ///     . . . . . . . .
//     /// ";
//     ///
//     /// let shape: GridShape = pattern.parse()?;
//     ///
//     /// assert_eq!(shape.count(), 4);
//     /// # Ok(())
//     /// # }
//     /// ```
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Self::from_pattern(s, '#', '.')
//     }
// }

// impl<T: Into<GridMask>, I: IntoIterator<Item = T>, Adj: Adjacency> TryFromIterator<I> for GridShape<Adj> {
//     type Error = Discontiguous;

//     fn try_from_iter(iter: I) -> Result<Self, Self::Error> {
//         GridMask::from_iter(iter).try_into()
//     }
// }
