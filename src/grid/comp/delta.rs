use fluent_result::into::IntoResult;
use tap::TryConv;

use crate::GridVector;
use crate::err::OutOfBounds;
use crate::num::{SignedMag, VecDimU64, VecMagU64};

/// A 2D delta of a grid, representing a valid shift or displacement.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    derive_more::Constructor,
    derive_more::Display,
    // derive_more::Add,
    // derive_more::AddAssign,
    // derive_more::Sub,
    // derive_more::SubAssign,
)]
#[display("({x:+}, {y:+})")]
pub struct GridDelta<T> {
    /// The x-component of the delta.
    pub x: SignedMag<T>,
    /// The y-component of the delta.
    pub y: SignedMag<T>,
}

impl TryFrom<GridVector> for GridDelta<VecMagU64> {
    type Error = OutOfBounds;

    fn try_from(value: GridVector) -> Result<Self, Self::Error> {
        let x = value.x.try_conv::<VecDimU64>().map(Into::into).map_err(OutOfBounds::from)?;
        let y = value.y.try_conv::<VecDimU64>().map(Into::into).map_err(OutOfBounds::from)?;
        Self { x, y }.into_ok()
    }
}
