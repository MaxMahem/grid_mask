use crate::array::{ArrayPoint, ArraySize};
use crate::err::OutOfBounds;

/// A bounded rectangle for an [`ArrayGrid`](crate::array::ArrayGrid).
///
/// The rectangle is guaranteed to be entirely within the grid.
///
/// # Type Parameters
///
/// * `W` - The width of the grid.
/// * `H` - The height of the grid.
#[readonly::make]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
#[display("{point} {size}")]
pub struct ArrayRect<const W: u16, const H: u16> {
    /// Top-left point.
    pub point: ArrayPoint<W, H>,
    /// Rectangle size.
    pub size: ArraySize<W, H>,
}

impl<const W: u16, const H: u16> ArrayRect<W, H> {
    /// Creates a new [`ArrayRect`].
    ///
    /// # Errors
    ///
    /// Returns [`OutOfBounds`] when the rectangle extends beyond the grid.
    pub fn new<P: TryInto<ArrayPoint<W, H>>, S: TryInto<ArraySize<W, H>>>(
        point: P,
        size: S,
    ) -> Result<Self, OutOfBounds> {
        let point = point.try_into().map_err(OutOfBounds::from)?;
        let size = size.try_into().map_err(OutOfBounds::from)?;

        if u32::from(point.x()) + u32::from(size.width().get()) > u32::from(W)
            || u32::from(point.y()) + u32::from(size.height().get()) > u32::from(H)
        {
            return Err(OutOfBounds);
        }

        Ok(Self { point, size })
    }

    /// Creates a new [`ArrayRect`] from constants.
    ///
    /// # Panics
    ///
    /// Panics at compile time if the rectangle is invalid or out of bounds.
    #[must_use]
    pub const fn const_new<const X: u16, const Y: u16, const WIDTH: u16, const HEIGHT: u16>() -> Self {
        assert!(X < W && Y < H, "point out of bounds");
        assert!(WIDTH > 0 && HEIGHT > 0, "size must be non-zero");
        assert!(WIDTH <= W && HEIGHT <= H, "size out of bounds");
        assert!(X + WIDTH <= W && Y + HEIGHT <= H, "rectangle extends beyond grid");

        Self { point: ArrayPoint::const_new::<X, Y>(), size: ArraySize::const_new::<WIDTH, HEIGHT>() }
    }

    /// Returns the top-left point.
    #[must_use]
    pub const fn point(&self) -> ArrayPoint<W, H> {
        self.point
    }

    /// Returns the rectangle size.
    #[must_use]
    pub const fn size(&self) -> ArraySize<W, H> {
        self.size
    }

    /// Returns `true` when `point` lies within this rectangle.
    #[must_use]
    pub const fn contains(&self, point: ArrayPoint<W, H>) -> bool {
        point.x() >= self.point.x()
            && point.x() < self.point.x() + self.size.width().get()
            && point.y() >= self.point.y()
            && point.y() < self.point.y() + self.size.height().get()
    }
}

// impl<const W: u16, const H: u16, P: TryInto<ArrayPoint<W, H>>, S: TryInto<ArraySize<W, H>>> TryFrom<(P, S)>
//     for ArrayRect<W, H>
// where
//     OutOfBounds: From<P::Error> + From<S::Error>,
// {
//     type Error = OutOfBounds;

//     fn try_from((point, size): (P, S)) -> Result<Self, Self::Error> {
//         Self::new(point, size)
//     }
// }

// impl<const W: u16, const H: u16> From<ArrayRect<W, H>> for (ArrayPoint<W, H>, ArraySize<W, H>) {
//     fn from(rect: ArrayRect<W, H>) -> Self {
//         (rect.point, rect.size)
//     }
// }
