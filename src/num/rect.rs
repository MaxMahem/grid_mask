use std::num::NonZeroU16;

use crate::ArrayRect;
use crate::num::{Point, Size};

/// A rectangle in a 2D grid.
#[derive(
    Debug, //
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    derive_more::Display,
    derive_more::Constructor,
)]
#[display("{point} {size}")]
pub struct Rect<P, S = P> {
    /// Top-left point.
    pub point: P,
    /// Rectangle size.
    pub size: S,
}

impl<const W: u16, const H: u16> From<ArrayRect<W, H>> for Rect<Point<u16>, Size<NonZeroU16>> {
    fn from(value: ArrayRect<W, H>) -> Self {
        Self {
            point: Point::new(value.point.x(), value.point.y()),
            size: Size::new(value.size.width(), value.size.height()),
        }
    }
}
