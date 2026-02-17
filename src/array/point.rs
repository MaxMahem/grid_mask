use num_integer::Integer;

use crate::ArrayIndex;
use crate::err::OutOfBounds;
use crate::ext::safe_into;
use crate::num::{ArrayGridPos, Point};

/// A point in an [`ArrayGrid`](struct@crate::ArrayGrid) of width `W` and height `H`.
///
/// The point is guaranteed to be in the range `(0..W, 0..H)`.
///
/// # Type Parameters
///
/// * `W`: The width of the grid.
/// * `H`: The height of the grid.
///
/// Both coordinates are bounded, so invalid points cannot be expressed.
#[derive(
    Debug, //
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::Into,
    derive_more::Deref,
)]
#[display("({x}, {y})", x = self.x(), y = self.y())]
pub struct ArrayPoint<const W: u16, const H: u16>(pub Point<ArrayGridPos<W>, ArrayGridPos<H>>);

impl<const W: u16, const H: u16> ArrayPoint<W, H> {
    /// The origin point.
    pub const ORIGIN: Self = Self(Point::new(ArrayGridPos::ZERO, ArrayGridPos::ZERO));
    /// The minimum valid point.
    pub const MIN: Self = Self::ORIGIN;
    /// The maximum valid point.
    pub const MAX: Self = Self(Point::new(ArrayGridPos::MAX, ArrayGridPos::MAX));

    pub(crate) const W_U32: u32 = W as u32;
    pub(crate) const H_U32: u32 = H as u32;

    /// Creates a new [`ArrayPoint`] from raw [`u16`] coordinates.
    ///
    /// # Errors
    ///
    /// [`OutOfBounds`] if `x >= W` or `y >= H`.
    pub const fn new(x: u16, y: u16) -> Result<Self, OutOfBounds> {
        let x = match ArrayGridPos::new(x) {
            Ok(x) => x,
            Err(e) => return Err(e),
        };
        let y = match ArrayGridPos::new(y) {
            Ok(y) => y,
            Err(e) => return Err(e),
        };
        Ok(Self(Point::new(x, y)))
    }

    /// Creates a new [`ArrayPoint`] from raw [`u16`] coordinates.
    ///
    /// # Panics
    ///
    /// Panics and fails at compile time if `x >= W` or `y >= H`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::array::ArrayPoint;
    /// const point: ArrayPoint<8, 8> = ArrayPoint::const_new::<3, 5>();
    /// assert_eq!(point.x(), 3);
    /// assert_eq!(point.y(), 5);
    /// ```
    #[must_use]
    pub const fn const_new<const X: u16, const Y: u16>() -> Self {
        let x = ArrayGridPos::const_new::<X>();
        let y = ArrayGridPos::const_new::<Y>();
        Self(Point::new(x, y))
    }

    /// Returns the x-coordinate of the point.
    #[must_use]
    pub const fn x(&self) -> u16 {
        self.0.x.get()
    }

    /// Returns the y-coordinate of the point.
    #[must_use]
    pub const fn y(&self) -> u16 {
        self.0.y.get()
    }

    /// Converts the point to an [`ArrayIndex`].
    #[must_use]
    pub(crate) const fn to_index(self) -> ArrayIndex<W, H> {
        ArrayIndex::from_point(self)
    }
}

impl<const W: u16, const H: u16> From<ArrayIndex<W, H>> for ArrayPoint<W, H> {
    fn from(index: ArrayIndex<W, H>) -> Self {
        let (y, x) = index.get().div_rem(&Self::W_U32);
        safe_into!((x, y) => Self)
    }
}

impl<const W: u16, const H: u16, WOut, HOut> From<ArrayPoint<W, H>> for (WOut, HOut)
where
    WOut: From<u16>,
    HOut: From<u16>,
{
    fn from(point: ArrayPoint<W, H>) -> Self {
        (point.x().into(), point.y().into())
    }
}

impl<N1, N2, const W: u16, const H: u16> TryFrom<(N1, N2)> for ArrayPoint<W, H>
where
    N1: TryInto<u16>,
    N2: TryInto<u16>,
{
    type Error = OutOfBounds;

    fn try_from(value: (N1, N2)) -> Result<Self, Self::Error> {
        let x = value.0.try_into().map_err(OutOfBounds::from)?;
        let y = value.1.try_into().map_err(OutOfBounds::from)?;
        Self::new(x, y)
    }
}

impl<N1, N2, const W: u16, const H: u16> TryFrom<Point<N1, N2>> for ArrayPoint<W, H>
where
    N1: TryInto<u16>,
    N2: TryInto<u16>,
{
    type Error = OutOfBounds;

    fn try_from(value: Point<N1, N2>) -> Result<Self, Self::Error> {
        let x = value.x.try_into().map_err(OutOfBounds::from)?;
        let y = value.y.try_into().map_err(OutOfBounds::from)?;
        Self::new(x, y)
    }
}

impl<const W: u16, const H: u16> PartialEq<(u16, u16)> for ArrayPoint<W, H> {
    fn eq(&self, other: &(u16, u16)) -> bool {
        self.0.x.get() == other.0 && self.0.y.get() == other.1
    }
}
