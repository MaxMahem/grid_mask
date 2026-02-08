use crate::ext::Bound;
use crate::num::{BitIndexU64, GridLen};

/// A trait representing the topology of a grid.
pub trait Grid {
    /// The type used for grid dimensions.
    type Len: Copy + PartialOrd + Ord + PartialEq + Eq;
    /// The type used for grid positions.
    type Pos: Bound;

    /// The width of the grid.
    const WIDTH: Self::Len;
    /// The height of the grid.
    const HEIGHT: Self::Len;
}

/// A 8x8 grid.
#[derive(Debug)]
pub struct GridU64;

impl Grid for GridU64 {
    type Len = GridLen;
    type Pos = BitIndexU64;

    const WIDTH: GridLen = GridLen::const_new::<8>();
    const HEIGHT: GridLen = GridLen::const_new::<8>();
}
