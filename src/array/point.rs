use tap::Pipe;

use crate::ArrayIndex;
use crate::err::OutOfBounds;
use crate::ext::{MapTuple, const_assert_then};

/// A point in an `ArrayGrid` of width `W` and height `H`.
///
/// Both coordinates are bounded, so invalid points cannot be expressed.
#[readonly::make]
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
)]
#[display("({x}, {y})")]
pub struct ArrayPoint<const W: u16, const H: u16> {
    /// The x-coordinate of the point.
    x: u16,
    /// The y-coordinate of the point.
    y: u16,
}

impl<const W: u16, const H: u16> ArrayPoint<W, H> {
    /// The origin point.
    pub const ORIGIN: Self = Self { x: 0, y: 0 };
    /// The minimum valid point.
    pub const MIN: Self = Self::ORIGIN;
    /// The maximum valid point.
    pub const MAX: Self = Self {
        x: W.checked_sub(1).expect("width must be > 0"), //
        y: H.checked_sub(1).expect("height must be > 0"),
    };

    const W_U32: u32 = W as u32;

    /// Creates a new [`ArrayPoint`] from raw [`u16`] coordinates.
    ///
    /// # Errors
    ///
    /// [`OutOfBounds`] if `x >= W` or `y >= H`.
    pub const fn new(x: u16, y: u16) -> Result<Self, OutOfBounds> {
        match x < W && y < H {
            true => Ok(Self { x, y }),
            false => Err(OutOfBounds),
        }
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
        let x = const_assert_then!(X < W => X, "x out of bounds");
        let y = const_assert_then!(Y < H => Y, "y out of bounds");
        Self { x, y }
    }

    /// Returns the x-coordinate of the point.
    #[must_use]
    pub const fn x(&self) -> u16 {
        self.x
    }

    /// Returns the y-coordinate of the point.
    #[must_use]
    pub const fn y(&self) -> u16 {
        self.y
    }
}

impl<const W: u16, const H: u16> From<ArrayPoint<W, H>> for ArrayIndex<W, H> {
    fn from(point: ArrayPoint<W, H>) -> Self {
        (point.x, point.y) //
            .map_into()
            .pipe(|(x, y): (u32, u32)| y * ArrayPoint::<W, H>::W_U32 + x)
            .pipe(Self)
    }
}

impl<const W: u16, const H: u16, WOut, HOut> From<ArrayPoint<W, H>> for (WOut, HOut)
where
    WOut: From<u16>,
    HOut: From<u16>,
{
    fn from(point: ArrayPoint<W, H>) -> Self {
        (point.x.into(), point.y.into())
    }
}

impl<const W: u16, const H: u16> TryFrom<(u16, u16)> for ArrayPoint<W, H> {
    type Error = OutOfBounds;

    fn try_from(value: (u16, u16)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1)
    }
}

impl<const W: u16, const H: u16> PartialEq<(u16, u16)> for ArrayPoint<W, H> {
    fn eq(&self, other: &(u16, u16)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}
