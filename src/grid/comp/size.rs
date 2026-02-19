use fluent_result::into::IntoResult;

use crate::err::OutOfBounds;
use crate::num::GridLen;

/// A size in a 8x8 grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(into = "GridSizeSerde"))]
#[display("({width}x{height})")]
pub struct GridSize {
    /// The width of the size
    pub width: GridLen,
    /// The height of the size
    pub height: GridLen,
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GridSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        GridSizeSerde::deserialize(deserializer).map(Self::from)
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum GridSizeSerde {
    Array(GridLen, GridLen),
    Object { w: GridLen, h: GridLen },
}

#[cfg(feature = "serde")]
impl From<GridSizeSerde> for GridSize {
    fn from(value: GridSizeSerde) -> Self {
        use GridSizeSerde::{Array, Object};
        match value {
            Array(width, height) | Object { w: width, h: height } => Self { width, height },
        }
    }
}

#[cfg(feature = "serde")]
impl From<GridSize> for GridSizeSerde {
    fn from(value: GridSize) -> Self {
        Self::Array(value.width, value.height)
    }
}

impl GridSize {
    /// A minimum size [`GridSize`].
    pub const MIN: Self = Self { width: GridLen::MIN, height: GridLen::MIN };

    /// A maximum size [`GridSize`].
    pub const MAX: Self = Self { width: GridLen::MAX, height: GridLen::MAX };

    /// Creates a new [`GridSize`] without bounds checking.
    ///
    /// The caller must ensure that `width` and `height` are within the range `1..8`
    #[must_use]
    pub(crate) const fn new_unchecked(width: u8, height: u8) -> Self {
        debug_assert!(width >= 1 && width <= 8, "Safety: width should be within 1..=8");
        debug_assert!(height >= 1 && height <= 8, "Safety: height should be within 1..=8");
        let width = unsafe { GridLen::new_unchecked(width) };
        let height = unsafe { GridLen::new_unchecked(height) };

        Self { width, height }
    }

    /// Creates a new [`GridSize`] from constant dimensions.
    ///
    /// This function enforces bounds checking at compile time.
    ///
    /// # Panics
    ///
    /// This function fails at compile time if `W` or `H` are not in the range 1..=8.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::GridSize;
    /// const SIZE: GridSize = GridSize::const_new::<1, 8>();
    ///
    /// assert_eq!(SIZE, (1, 8));
    /// ```
    ///
    /// Failing to provide valid dimensions will result in a compile error:
    ///
    /// ```rust,compile_fail
    /// # use grid_mask::GridSize;
    /// const SIZE: GridSize = GridSize::const_new::<0, 8>();
    /// ```
    #[must_use]
    pub const fn const_new<const W: u8, const H: u8>() -> Self {
        assert!(W >= 1 && W <= 8, "width is out of bounds (must be 1..=8)");
        assert!(H >= 1 && H <= 8, "height is out of bounds (must be 1..=8)");

        Self {
            width: GridLen::new(W).expect("width is out of bounds"),
            height: GridLen::new(H).expect("height is out of bounds"),
        }
    }

    /// Tries to creates a new [`GridSize`]
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the size
    /// * `height` - The height of the size
    ///
    /// # Errors
    ///
    /// [`OutOfBounds`] if any value is out of bounds for a [`GridLen`] (1..=8).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # use grid_mask::GridSize;
    /// let size = GridSize::new(1, 8)?;
    /// assert_eq!(size, (1, 8));
    ///
    /// GridSize::new(0, 8).expect_err("Should be out of bounds");
    /// GridSize::new(1, 9).expect_err("Should be out of bounds");
    /// GridSize::new(0, 9).expect_err("Should be out of bounds");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<W: TryInto<GridLen>, H: TryInto<GridLen>>(width: W, height: H) -> Result<Self, OutOfBounds> {
        let width = width.try_into().map_err(OutOfBounds::from)?;
        let height = height.try_into().map_err(OutOfBounds::from)?;
        Self { width, height }.into_ok()
    }
}

impl<W: From<GridLen>, H: From<GridLen>> From<GridSize> for (W, H) {
    fn from(size: GridSize) -> Self {
        (size.width.into(), size.height.into())
    }
}

impl<W: TryInto<GridLen>, H: TryInto<GridLen>> TryFrom<(W, H)> for GridSize {
    type Error = OutOfBounds;

    fn try_from(value: (W, H)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1)
    }
}

impl<X, Y> PartialEq<(X, Y)> for GridSize
where
    X: From<GridLen> + PartialEq,
    Y: From<GridLen> + PartialEq,
{
    fn eq(&self, other: &(X, Y)) -> bool {
        let x = X::from(self.width);
        let y = Y::from(self.height);
        x == other.0 && y == other.1
    }
}
