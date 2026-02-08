use std::hash::Hash;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

use tap::Conv;

use crate::err::Discontiguous;
use crate::ext::bits::FromBitRange;
use crate::num::{GridIndexU64, GridLen, GridPos};
use crate::{Adjacency, GridShape, GridVector};

#[sealed::sealed]
pub trait GridData:
    Eq + PartialEq + Hash + BitAnd<Output = Self> + BitAndAssign + BitOr<Output = Self> + BitOrAssign + Not + Sized
{
    const EMPTY: Self;
    const FULL: Self;

    type RowLen;
    type ColLen;

    const ROWS: Self::RowLen;
    const COLS: Self::ColLen;

    type Index;

    /// Gets the state of the cell at `index`
    fn index(&self, index: Self::Index) -> bool;

    /// Sets the state of the cell at `index`
    fn set(&mut self, index: Self::Index);

    /// Unsets the state of the cell at `index`
    fn unset(&mut self, index: Self::Index);

    /// Translates the data by `delta`
    fn translate(&mut self, delta: GridVector);

    type Shape<A: Adjacency>;
    fn contiguous<A: Adjacency>(&self) -> Result<Self::Shape<A>, Discontiguous>;
}

#[sealed::sealed]
impl GridData for u64 {
    const EMPTY: Self = 0;
    const FULL: Self = Self::MAX;

    type RowLen = GridLen;
    type ColLen = GridLen;

    const ROWS: Self::RowLen = GridLen::const_new::<8>();
    const COLS: Self::ColLen = GridLen::const_new::<8>();

    type Index = GridIndexU64;

    fn index(&self, index: Self::Index) -> bool {
        (self & (1 << index.get())) != 0
    }

    fn set(&mut self, index: Self::Index) {
        let bit = 1 << index.get();
        *self |= bit;
    }

    fn unset(&mut self, index: Self::Index) {
        let bit = 1 << index.get();
        *self &= !bit;
    }

    fn translate(&mut self, delta: GridVector) {
        const COLS_U32: u32 = u64::COLS.get() as u32;
        const COL_FIRST: u64 = 0x0101_0101_0101_0101;

        let mask = *self;

        let mask_shifted_y = match delta.y {
            dy @ 1..=7 => mask << (dy.unsigned_abs().conv::<u32>() * COLS_U32),
            dy @ -7..=-1 => mask >> (dy.unsigned_abs().conv::<u32>() * COLS_U32),
            0 => mask,
            _ => return,
        };

        let mask_shifted_x_y = match delta.x {
            dx @ 1..=7 => {
                let shift = dx.unsigned_abs();
                let mask_shifted_x_y = mask_shifted_y << shift;

                // Safety: shift is 1..=7, so it is a valid GridPos
                let shift_pos = unsafe { GridPos::new_unchecked(shift) };

                let col_mask = Self::from_bit_range(..shift_pos) * COL_FIRST;

                mask_shifted_x_y & !col_mask
            }
            dx @ -7..=-1 => {
                let shift = dx.unsigned_abs();
                let mask_shifted_x_y = mask_shifted_y >> shift;

                // Safety: shift is 1..=7, so 8 - shift is 1..=7, which is a valid GridPos
                let start_pos = unsafe { GridPos::new_unchecked(8 - shift) };

                let col_mask = Self::from_bit_range(start_pos..) * COL_FIRST;

                mask_shifted_x_y & !col_mask
            }
            0 => mask_shifted_y,
            _ => return,
        };

        *self = mask_shifted_x_y;
    }

    type Shape<A: Adjacency> = GridShape<A>;

    fn contiguous<A: Adjacency>(&self) -> Result<Self::Shape<A>, Discontiguous> {
        GridShape::try_from(*self)
    }
}
