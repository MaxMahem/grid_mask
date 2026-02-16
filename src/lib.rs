#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic, clippy::cargo, clippy::nursery)]
#![warn(missing_docs, missing_debug_implementations)]
#![allow(clippy::match_bool, clippy::single_match_else)]
// TODO: remove this
#![allow(dead_code)]

#[doc(hidden)]
pub mod ext;

/// Fixed-size array-based grids.
pub mod array;
mod grid;

/// crate Number types.
pub mod num;

/// crate Error types.
pub mod err;

pub use array::{
    ArrayGrid, GridIndexer, GridView, GridViewMut, ArrayIndex, ArrayPoint, ArrayRect, ArraySize,
    ArrayVector,
};
pub use grid::{Adjacency, Cardinal, Octile};
pub use grid::{GridDelta, GridMask, GridPoint, GridRect, GridShape, GridSize, GridVector};
