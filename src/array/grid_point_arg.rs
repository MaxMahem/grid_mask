use crate::ArrayGrid;
use crate::err::OutOfBounds;
use crate::num::Point;
use crate::{ArrayIndex, ArrayPoint};

/// Argument adapter for [`ArrayGrid::get`] and [`ArrayGrid::set`].
///
/// This trait enables dual-mode point conversion:
/// - infallible inputs return plain values (`bool` for `get`, `()` for `set`)
/// - fallible inputs return [`Result`] values with [`OutOfBounds`] errors
pub trait ArrayGridPointArg<const W: u16, const H: u16> {
    /// Return type for [`ArrayGrid::get`].
    type GetOutput;
    /// Return type for [`ArrayGrid::set`].
    type SetOutput;

    #[doc(hidden)]
    fn get<const WORDS: usize>(self, grid: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput;
    #[doc(hidden)]
    fn set<const WORDS: usize>(self, grid: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput;
}

impl<const W: u16, const H: u16> ArrayGridPointArg<W, H> for ArrayPoint<W, H> {
    type GetOutput = bool;
    type SetOutput = ();

    fn get<const WORDS: usize>(self, grid: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
        grid.const_get(self)
    }

    fn set<const WORDS: usize>(self, grid: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
        grid.const_set(self, value);
    }
}

impl<const W: u16, const H: u16> ArrayGridPointArg<W, H> for ArrayIndex<W, H> {
    type GetOutput = bool;
    type SetOutput = ();

    fn get<const WORDS: usize>(self, grid: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
        grid.const_get(self.into())
    }

    fn set<const WORDS: usize>(self, grid: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
        grid.const_set(self.into(), value);
    }
}

impl<N1, N2, const W: u16, const H: u16> ArrayGridPointArg<W, H> for (N1, N2)
where
    N1: TryInto<u32>,
    N2: TryInto<u32>,
{
    type GetOutput = Result<bool, OutOfBounds>;
    type SetOutput = Result<(), OutOfBounds>;

    fn get<const WORDS: usize>(self, grid: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
        try_into_array_point(self.0, self.1).map(|point| grid.const_get(point))
    }

    fn set<const WORDS: usize>(self, grid: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
        try_into_array_point(self.0, self.1).map(|point| grid.const_set(point, value))
    }
}

impl<N1, N2, const W: u16, const H: u16> ArrayGridPointArg<W, H> for Point<N1, N2>
where
    N1: TryInto<u32>,
    N2: TryInto<u32>,
{
    type GetOutput = Result<bool, OutOfBounds>;
    type SetOutput = Result<(), OutOfBounds>;

    fn get<const WORDS: usize>(self, grid: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
        try_into_array_point(self.x, self.y).map(|point| grid.const_get(point))
    }

    fn set<const WORDS: usize>(self, grid: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
        try_into_array_point(self.x, self.y).map(|point| grid.const_set(point, value))
    }
}

fn try_into_array_point<N1, N2, const W: u16, const H: u16>(x: N1, y: N2) -> Result<ArrayPoint<W, H>, OutOfBounds>
where
    N1: TryInto<u32>,
    N2: TryInto<u32>,
{
    let x = x.try_into().map_err(OutOfBounds::new_from)?;
    let y = y.try_into().map_err(OutOfBounds::new_from)?;

    let x = x.try_into().map_err(OutOfBounds::new_from)?;
    let y = y.try_into().map_err(OutOfBounds::new_from)?;

    ArrayPoint::new(x, y)
}
