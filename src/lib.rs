#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic, clippy::cargo, clippy::nursery)]
#![warn(missing_docs, missing_debug_implementations)]
#![allow(clippy::match_bool, clippy::single_match_else)]

#[doc(hidden)]
pub mod ext;

mod grid;

/// crate Number types.
pub mod num;

/// crate Error types.
pub mod err;

/// An iterator over the cells of a [`GridMask`].
pub use grid::Cells;
/// An iterator over the points of a [`GridMask`].
pub use grid::Points;
pub use grid::{
    Adjacency, Cardinal, Grid, GridIndex, GridMask, GridPoint, GridRect, GridShape, GridSize, GridVector, Octile,
};
