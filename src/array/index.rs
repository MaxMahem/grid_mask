use crate::{err::OutOfBounds, ext::const_assert};

// /// A trait for types that can be converted into a flat array index.
// ///
// /// This is used to unify indexing into a grid using either a point (x, y)
// /// or a pre-validated flat index.
// pub trait IntoArrayIndex<const W: usize, const H: usize> {
//     /// Returns the flat index into the array.
//     fn into_index(self) -> ArrayIndex<W, H>;
// }

/// A flat index into an `ArrayGrid` of width `W` and height `H`.
///
/// The index is pre-validated to be within the bounds of the grid.
#[derive(
    Debug, //
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Display,
    derive_more::From,
    derive_more::Into,
)]
pub struct ArrayIndex<const W: u16, const H: u16>(pub(crate) u32);

impl<const W: u16, const H: u16> ArrayIndex<W, H> {
    /// The minimum valid index.
    pub const MIN: Self = Self(0);

    /// The maximum valid index.
    pub const MAX: Self = Self(W as u32 * H as u32 - 1);

    /// Creates a new [`ArrayIndex`] from a flat index.
    ///
    /// # Errors
    ///
    /// [`OutOfBounds`] if the index is out of bounds (>= W * H).
    pub const fn new(index: u32) -> Result<Self, OutOfBounds> {
        match index >= W as u32 * H as u32 {
            true => Err(OutOfBounds),
            false => Ok(Self(index)),
        }
    }

    /// Creates a new [`ArrayIndex`] from a flat index.
    ///
    /// # Panics
    ///
    /// Panics at compile time if the index is out of bounds (>= W * H).
    #[must_use]
    pub const fn const_new<const INDEX: u32>() -> Self {
        const_assert!(INDEX < W as u32 * H as u32, "index out of bounds W * H");
        Self(INDEX)
    }

    /// Returns the raw index value.
    #[must_use]
    pub const fn get(&self) -> u32 {
        self.0
    }

    pub(crate) const fn word_and_bit(self) -> (usize, u16) {
        (self.0 as usize / u64::BITS as usize, (self.0 % u64::BITS) as u16)
    }
}

impl<const W: u16, const H: u16> PartialEq<u32> for ArrayIndex<W, H> {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}
