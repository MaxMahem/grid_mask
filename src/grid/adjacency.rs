use super::{GridMask, GridVector};

/// Defines how a mask grows to include adjacent cells.
#[sealed::sealed]
pub trait Adjacency: Sized {
    /// Returns a mask of all cells adjacent to `data` (including `data` itself).
    ///
    /// # Arguments
    ///
    /// * `data` - The data to grow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{Adjacency, Cardinal, GridMask, GridPoint};
    /// let center = GridMask::from(GridPoint::try_new(1, 1).unwrap());
    /// let grown = Cardinal::connected(center);
    ///
    /// assert_eq!(grown.count(), 5);
    /// ```
    #[must_use]
    fn connected(data: GridMask) -> GridMask;
}

/// Cardinal adjacency (North, South, East, West).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Cardinal;

#[sealed::sealed]
impl Adjacency for Cardinal {
    fn connected(mask: GridMask) -> GridMask {
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
    fn connected(mask: GridMask) -> GridMask {
        let n = mask.translate(GridVector::NORTH);
        let s = mask.translate(GridVector::SOUTH);

        let vertical = mask | n | s;

        let east = vertical.translate(GridVector::EAST);
        let west = vertical.translate(GridVector::WEST);

        vertical | east | west
    }
}
