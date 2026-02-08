use crate::{Grid, GridVector};

/// Defines how a mask grows to include adjacent cells.
#[sealed::sealed]
pub trait Adjacency {
    /// Returns a mask containing the original cells plus their adjacent neighbors.
    ///
    /// # Arguments
    ///
    /// * `mask` - The mask to grow.
    ///
    /// # Type Parameters
    ///
    /// * `G` - A type that implements [`Grid`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::{Adjacency, Grid, GridMask, GridVector, Cardinal};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mask = GridMask::new(0b101);
    ///
    /// let grown = Cardinal::grow(mask);
    ///
    /// assert_eq!(grown.count(), 6);
    /// # Ok(())
    /// # }
    /// ```
    fn grow<G: Grid>(mask: G) -> G;
}

/// Cardinal adjacency (North, South, East, West).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Cardinal;

#[sealed::sealed]
impl Adjacency for Cardinal {
    fn grow<G: Grid>(mask: G) -> G {
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
    fn grow<G: Grid>(mask: G) -> G {
        let n = mask.translate(GridVector::NORTH);
        let s = mask.translate(GridVector::SOUTH);

        let vertical = mask | n | s;

        let east = vertical.translate(GridVector::EAST);
        let west = vertical.translate(GridVector::WEST);

        vertical | east | west
    }
}
