use std::hash::Hash;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use tap::{Conv, TryConv};

use crate::err::Discontiguous;
use crate::ext::Bound;
use crate::ext::bits::FromBitRange;
use crate::grid::GridDelta;
use crate::num::{BitIndexU64, GridLen, SignedMag, VecMagU64};
use crate::{Adjacency, GridIndex, GridShape, GridVector};

/// A [`GridData`] that can be read.
#[sealed::sealed]
pub trait GridData: Default + Eq + PartialEq + Hash + Sized {
    /// An empty grid
    const EMPTY: Self;
    /// A full grid
    const FULL: Self;

    /// The type used to represent row lengths
    type RowLen: Bound;
    /// The type used to represent column lengths
    type ColLen: Bound;

    /// The number of rows in the grid
    const ROWS: Self::RowLen;
    /// The number of columns in the grid
    const COLS: Self::ColLen;
    /// The number of cells in the grid
    const CELLS: usize;

    /// The type of index used to access cells in the grid.
    type Index: GridIndex<Self> + Bound;

    /// The type used to represent valid (in bounds) translations
    type Delta;

    /// Gets the state of the cell at `index`
    fn index<Idx: GridIndex<Self>>(&self, index: Idx) -> bool;

    /// Returns the number of set cells in the grid.
    #[must_use]
    fn count(&self) -> usize;

    type Shape<A: Adjacency>;

    fn contiguous<A: Adjacency>(&self) -> Result<Self::Shape<A>, Discontiguous>;
}

/// A [`GridData`] that can be modified.
#[sealed::sealed]
pub trait GridDataMut: GridData + Clone + BitAndAssign + BitOrAssign + BitXorAssign {
    /// Sets the cell at `index`
    fn set<Idx: GridIndex<Self>>(&mut self, index: Idx);

    /// Unsets the cell at `index`
    fn unset<Idx: GridIndex<Self>>(&mut self, index: Idx);

    /// Translates the grid by `delta`
    fn translate_mut(&mut self, delta: GridVector);

    /// Flips all bits in the grid
    fn negate(&mut self);
}

/// An [`GridData`] that can be copied.
#[sealed::sealed]
pub trait GridDataValue:
    GridData //
    + Copy
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
{
    /// Returns a new grid with the cell at `index` set.
    fn with_set<Idx: GridIndex<Self>>(&self, index: Idx) -> Self;

    /// Returns a new grid with the cell at `index` unset.
    fn with_unset<Idx: GridIndex<Self>>(&self, index: Idx) -> Self;

    /// Returns a new grid with the data translated by `delta`.
    fn translate(&self, delta: GridVector) -> Self;
}

struct GridDataU64(u64);

#[sealed::sealed]
impl GridData for u64 {
    const EMPTY: Self = 0;
    const FULL: Self = Self::MAX;

    type RowLen = GridLen;
    type ColLen = GridLen;

    const ROWS: Self::RowLen = GridLen::const_new::<8>();
    const COLS: Self::ColLen = GridLen::const_new::<8>();
    const CELLS: usize = (Self::ROWS.get() * Self::COLS.get()) as usize;

    type Index = BitIndexU64;
    type Delta = GridDelta<VecMagU64>;

    type Shape<A: Adjacency> = GridShape<A>;

    fn index<Idx: GridIndex<Self>>(&self, index: Idx) -> bool {
        (self & (1 << index.to_index().get())) != 0
    }

    fn count(&self) -> usize {
        self.count_ones() as usize
    }

    fn contiguous<A: Adjacency>(&self) -> Result<Self::Shape<A>, Discontiguous> {
        GridShape::try_from(*self)
    }
}

fn translate(data: u64, delta: GridDelta<VecMagU64>) -> u64 {
    const COLS_U32: u32 = u64::COLS.get() as u32;
    const FIRST_COL: u64 = 0x0101_0101_0101_0101;

    let data_shifted_y = match delta.y {
        SignedMag::Positive(dy) => data << (dy.get().conv::<u32>() * COLS_U32),
        SignedMag::Negative(dy) => data >> (dy.get().conv::<u32>() * COLS_U32),
        SignedMag::Zero => data,
    };

    match delta.x {
        SignedMag::Positive(dx) => {
            let mask_shifted_x_y = data_shifted_y << dx.get();

            let col_mask = u8::from_bit_range(..dx).conv::<u64>() * FIRST_COL;

            mask_shifted_x_y & !col_mask
        }
        SignedMag::Negative(dx) => {
            let col_mask = u8::from_bit_range(..dx).conv::<u64>() * FIRST_COL;
            (data_shifted_y & !col_mask) >> dx.get()
        }
        SignedMag::Zero => data_shifted_y,
    }
}

#[sealed::sealed]
impl GridDataMut for u64 {
    fn set<Idx: GridIndex<Self>>(&mut self, index: Idx) {
        *self |= 1 << index.to_index().get();
    }

    fn unset<Idx: GridIndex<Self>>(&mut self, index: Idx) {
        *self &= !(1 << index.to_index().get());
    }

    fn translate_mut(&mut self, delta: GridVector) {
        *self = delta
            .try_conv::<Self::Delta>() // rustfmt
            .map_or(Self::EMPTY, |delta| translate(*self, delta));
    }

    fn negate(&mut self) {
        *self = !*self;
    }
}

#[sealed::sealed]
impl<T> GridDataValue for T
where
    T: GridDataMut // col align
        + Copy
        + BitAnd<Output = Self>
        + BitOr<Output = Self>
        + BitXor<Output = Self>
        + Not<Output = Self>,
{
    fn with_set<Idx: GridIndex<Self>>(&self, index: Idx) -> Self {
        copy_mutate(*self, |value| value.set(index))
    }

    fn with_unset<Idx: GridIndex<Self>>(&self, index: Idx) -> Self {
        copy_mutate(*self, |value| value.unset(index))
    }

    fn translate(&self, delta: GridVector) -> Self {
        copy_mutate(*self, |value| value.translate_mut(delta))
    }
}

#[inline]
/// Applies the mutation function `f` to a copy of `value` and returns the result.
fn copy_mutate<T: Copy>(value: T, f: impl FnOnce(&mut T)) -> T {
    let mut copy = value;
    f(&mut copy);
    copy
}
