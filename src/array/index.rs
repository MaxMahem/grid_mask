use tap::Pipe;

use crate::ArrayPoint;
use crate::err::OutOfBounds;
use crate::ext::{MapTuple, const_assert};

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
    derive_more::Into,
)]
#[into(ref(u32), owned(u32), owned(u64))]
pub struct ArrayIndex<const W: u16, const H: u16>(u32);

impl<const W: u16, const H: u16> ArrayIndex<W, H> {
    /// The minimum valid index.
    pub const MIN: Self = Self(0);

    /// The maximum valid index.
    pub const MAX: Self = Self(W as u32 * H as u32 - 1);

    const W_U32: u32 = W as u32;

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
    /// # Errors
    ///
    /// [`OutOfBounds`] if the index is out of bounds (>= W * H).
    pub fn try_new<T: TryInto<u32>>(index: T) -> Result<Self, OutOfBounds> {
        index.try_into().map_err(OutOfBounds::from).and_then(Self::new)
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
    pub const fn get(self) -> u32 {
        self.0
    }

    pub(crate) const fn word_and_bit(self) -> (usize, u16) {
        (self.0 as usize / u64::BITS as usize, (self.0 % u64::BITS) as u16)
    }

    pub(crate) const fn from_point(point: ArrayPoint<W, H>) -> Self {
        Self(point.y() as u32 * W as u32 + point.x() as u32)
    }
}

impl<const W: u16, const H: u16> PartialEq<u32> for ArrayIndex<W, H> {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl<const W: u16, const H: u16> From<ArrayPoint<W, H>> for ArrayIndex<W, H> {
    fn from(point: ArrayPoint<W, H>) -> Self {
        (point.x, point.y) //
            .map_into::<u32, u32>()
            .pipe(|(x, y)| y * Self::W_U32 + x)
            .pipe(Self)
    }
}

impl<const W: u16, const H: u16> From<ArrayIndex<W, H>> for usize {
    fn from(index: ArrayIndex<W, H>) -> Self {
        index.0 as Self
    }
}

impl<const W: u16, const H: u16> crate::ext::Bound for ArrayIndex<W, H> {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;

    const COUNT: usize = W as usize * H as usize;

    fn increment(&self) -> Option<Self> {
        self.0.checked_add(1).and_then(|i| Self::new(i).ok())
    }

    fn decrement(&self) -> Option<Self> {
        self.0.checked_sub(1).and_then(|i| Self::new(i).ok())
    }

    fn remaining(&self) -> usize {
        (Self::MAX.0 - self.0) as usize
    }
}
