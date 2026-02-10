use super::{GridDataValue, GridVector};

/// Defines how a mask grows to include adjacent cells.
#[sealed::sealed]
pub trait Adjacency: Sized {
    /// Returns a shape containing the original data plus their adjacent neighbors.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to grow.
    ///
    /// # Type Parameters
    ///
    /// * `T` - A type that implements [`GridDataValue`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{Adjacency, Cardinal};
    /// let mask: u64 = 0b101;
    ///
    /// let grown = Cardinal::connected(mask);
    ///
    /// assert_eq!(grown.count_ones(), 6);
    /// ```
    fn connected<T: GridDataValue>(data: T) -> T;
}

/// Cardinal adjacency (North, South, East, West).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Cardinal;

#[sealed::sealed]
impl Adjacency for Cardinal {
    fn connected<T: GridDataValue>(mask: T) -> T {
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
    fn connected<T: GridDataValue>(mask: T) -> T {
        let n = mask.translate(GridVector::NORTH);
        let s = mask.translate(GridVector::SOUTH);

        let vertical = mask | n | s;

        let east = vertical.translate(GridVector::EAST);
        let west = vertical.translate(GridVector::WEST);

        vertical | east | west
    }
}
