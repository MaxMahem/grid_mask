mod adjacency;
mod base;
mod data;
mod index;
mod mask;
mod point;
mod rect;
mod shape;
mod size;
mod vector;

pub use adjacency::{Adjacency, Cardinal, Octile};
pub use index::{GridIndex, TryGridIndex};
pub use mask::{Cells, GridMask64, Points};
pub use point::GridPoint;
pub use rect::GridRect;
pub use shape::GridShape;
pub use size::GridSize;
pub use vector::GridVector;

pub use base::Grid;

/// Backward compatible alias for [`GridMask64`].
pub type GridMask = GridMask64;
