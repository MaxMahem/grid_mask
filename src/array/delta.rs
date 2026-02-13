use std::num::{NonZeroU16, NonZeroU32};

use fluent_result::bool::Then;
use fluent_result::into::IntoResult;

use crate::ArrayVector;
use crate::err::OutOfBounds;
use crate::num::SignedMag;

/// A validated translation delta for [`ArrayGrid<W, H>`](super::ArrayGrid).
#[readonly::make]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ArrayDelta<const W: u16, const H: u16> {
    /// The total linear offset.
    /// Guranteed to be in the range `-(CELL_COUNT - 1)..=(CELL_COUNT - 1)`.
    pub(crate) linear_offset: SignedMag<NonZeroU32>,
    /// The horizontal displacement, guaranteed to be in `0..W`.
    pub(crate) dx: SignedMag<NonZeroU16>,
}

impl<const W: u16, const H: u16> ArrayDelta<W, H> {
    const W_I32: i32 = W as i32;
    const W_U32: u32 = W as u32;
    const H_U32: u32 = H as u32;
}

impl<const W: u16, const H: u16> TryFrom<ArrayVector> for ArrayDelta<W, H> {
    type Error = OutOfBounds;

    fn try_from(vec: ArrayVector) -> Result<Self, Self::Error> {
        // Validation: Magnitude must be strictly less than dimension to be a valid shift within grid logic

        (vec.dx.unsigned_abs() >= Self::W_U32 || vec.dy.unsigned_abs() >= Self::H_U32).then_err(OutOfBounds)?;
        let dx = vec.dx.try_into().expect("bounds should be guaranteed by check above");
        let linear_offset = (vec.dy * Self::W_I32 + vec.dx).into();
        Self { linear_offset, dx }.into_ok()
    }
}
