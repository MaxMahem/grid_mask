use crate::{GridMask, GridVector};

/// Defines how a mask grows to include adjacent cells.
#[sealed::sealed]
pub trait Adjacency {
    /// Returns a mask containing the original cells plus their adjacent neighbors.
    fn grow(mask: GridMask) -> GridMask;
}

/// Cardinal adjacency (North, South, East, West).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Cardinal;

#[sealed::sealed]
impl Adjacency for Cardinal {
    fn grow(mask: GridMask) -> GridMask {
        let north = mask.translate(GridVector::NORTH);
        let south = mask.translate(GridVector::SOUTH);
        let east = mask.translate(GridVector::EAST);
        let west = mask.translate(GridVector::WEST);

        mask | north | south | east | west
    }
}

/// Octile adjacency (all 8 neighbors).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Octile;

#[sealed::sealed]
impl Adjacency for Octile {
    fn grow(mask: GridMask) -> GridMask {
        let n = mask.translate(GridVector::NORTH);
        let s = mask.translate(GridVector::SOUTH);

        let vertical = mask | n | s;

        let east = vertical.translate(GridVector::EAST);
        let west = vertical.translate(GridVector::WEST);

        vertical | east | west
    }
}
