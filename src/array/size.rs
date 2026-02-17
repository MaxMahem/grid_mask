use std::num::NonZeroU16;

use crate::err::OutOfBounds;
use crate::num::{ArrayGridLen, Size};

/// A bounded width/height pair for an [`ArrayGrid`](crate::array::ArrayGrid).
///
/// Both dimensions must be non-zero and no larger than the grid dimensions.
///
/// # Type Parameters
///
/// - `W`: The width of the grid.
/// - `H`: The height of the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display, derive_more::Deref)]
#[display("({width}x{height})", width = self.0.width, height = self.0.height)]
pub struct ArraySize<const W: u16, const H: u16>(pub Size<ArrayGridLen<W>, ArrayGridLen<H>>);

impl<const W: u16, const H: u16> ArraySize<W, H> {
    /// The minimum valid size.
    pub const MIN: Self = Self(Size::new(ArrayGridLen::MIN, ArrayGridLen::MIN));

    /// The maximum valid size.
    pub const MAX: Self = Self(Size::new(ArrayGridLen::MAX, ArrayGridLen::MAX));

    /// Creates a new [`ArraySize`] from raw [`u16`] dimensions.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfBounds`] when `width`/`height` are zero or exceed `W`/`H`.
    pub const fn new(width: u16, height: u16) -> Result<Self, OutOfBounds> {
        let width = match ArrayGridLen::new(width) {
            Ok(w) => w,
            Err(e) => return Err(e),
        };
        let height = match ArrayGridLen::new(height) {
            Ok(h) => h,
            Err(e) => return Err(e),
        };
        Ok(Self(Size::new(width, height)))
    }

    /// Creates a new [`ArraySize`] from constants.
    ///
    /// # Panics
    ///
    /// Panics at compile time if either dimension is out of bounds.
    #[must_use]
    pub const fn const_new<const WIDTH: u16, const HEIGHT: u16>() -> Self {
        let width = ArrayGridLen::const_new::<WIDTH>();
        let height = ArrayGridLen::const_new::<HEIGHT>();
        Self(Size::new(width, height))
    }

    /// Returns the width.
    #[must_use]
    pub const fn width(&self) -> NonZeroU16 {
        self.0.width.get()
    }

    /// Returns the height.
    #[must_use]
    pub const fn height(&self) -> NonZeroU16 {
        self.0.height.get()
    }

    /// Returns `true` if the given coordinates are within the bounds of this size.
    #[must_use]
    pub const fn contains(&self, x: u16, y: u16) -> bool {
        x < self.0.width.get().get() && y < self.0.height.get().get()
    }
}

impl<N1, N2, const W: u16, const H: u16> TryFrom<(N1, N2)> for ArraySize<W, H>
where
    N1: TryInto<u16>,
    N2: TryInto<u16>,
{
    type Error = OutOfBounds;

    fn try_from((width, height): (N1, N2)) -> Result<Self, Self::Error> {
        let width = width.try_into().map_err(OutOfBounds::from)?;
        let height = height.try_into().map_err(OutOfBounds::from)?;
        Self::new(width, height)
    }
}

impl<W1, H1, const W: u16, const H: u16> TryFrom<Size<W1, H1>> for ArraySize<W, H>
where
    W1: TryInto<u16>,
    H1: TryInto<u16>,
{
    type Error = OutOfBounds;

    fn try_from(size: Size<W1, H1>) -> Result<Self, Self::Error> {
        let width = size.width.try_into().map_err(OutOfBounds::from)?;
        let height = size.height.try_into().map_err(OutOfBounds::from)?;
        Self::new(width, height)
    }
}

// impl<const W: u16, const H: u16> From<ArraySize<W, H>> for (NonZeroU16, NonZeroU16) {
//     fn from(size: ArraySize<W, H>) -> Self {
//         (size.width, size.height)
//     }
// }
