use crate::{
    ArrayRect,
    num::{Point, Size},
};

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

impl<const X: u16, const Y: u16> From<ArrayRect<X, Y>> for Rect<Point<u16, u16>, Size<u16, u16>> {
    fn from(value: ArrayRect<X, Y>) -> Self {
        Self {
            point: Point::new(value.point.x(), value.point.y()),
            size: Size::new(value.size.width(), value.size.height()),
        }
    }
}
