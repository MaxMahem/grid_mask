use crate::err::OutOfBounds;

/// A bounded width/height pair for an [`ArrayGrid`](crate::array::ArrayGrid).
///
/// Both dimensions must be non-zero and no larger than the grid dimensions.
#[readonly::make]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
#[display("({width}x{height})")]
pub struct ArraySize<const W: u16, const H: u16> {
    /// Width of the rectangle.
    pub width: u16,
    /// Height of the rectangle.
    pub height: u16,
}

impl<const W: u16, const H: u16> ArraySize<W, H> {
    /// The minimum valid size.
    pub const MIN: Self = Self { width: 1, height: 1 };

    /// The maximum valid size.
    pub const MAX: Self = Self { width: W, height: H };

    /// Creates a new [`ArraySize`].
    ///
    /// # Errors
    ///
    /// Returns [`OutOfBounds`] when `width`/`height` are zero or exceed `W`/`H`.
    pub const fn new(width: u16, height: u16) -> Result<Self, OutOfBounds> {
        match width == 0 || height == 0 || width > W || height > H {
            true => Err(OutOfBounds),
            false => Ok(Self { width, height }),
        }
    }

    /// Creates a new [`ArraySize`] from constants.
    ///
    /// # Panics
    ///
    /// Panics at compile time if either dimension is out of bounds.
    #[must_use]
    pub const fn const_new<const WIDTH: u16, const HEIGHT: u16>() -> Self {
        assert!(WIDTH > 0 && WIDTH <= W, "width out of bounds");
        assert!(HEIGHT > 0 && HEIGHT <= H, "height out of bounds");
        Self { width: WIDTH, height: HEIGHT }
    }

    /// Returns the width.
    #[must_use]
    pub const fn width(&self) -> u16 {
        self.width
    }

    /// Returns the height.
    #[must_use]
    pub const fn height(&self) -> u16 {
        self.height
    }
}

impl<const W: u16, const H: u16> TryFrom<(u16, u16)> for ArraySize<W, H> {
    type Error = OutOfBounds;

    fn try_from((width, height): (u16, u16)) -> Result<Self, Self::Error> {
        Self::new(width, height)
    }
}

impl<const W: u16, const H: u16> From<ArraySize<W, H>> for (u16, u16) {
    fn from(size: ArraySize<W, H>) -> Self {
        (size.width, size.height)
    }
}
