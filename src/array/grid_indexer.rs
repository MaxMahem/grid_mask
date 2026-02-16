use crate::ArrayGrid;
use crate::err::OutOfBounds;
use crate::num::Point;
use crate::{ArrayIndex, ArrayPoint};

/// Trait for types that can be used to index an [`ArrayGrid`].
pub trait GridIndexer<const W: u16, const H: u16> {
    /// Return type for get operations.
    type GetOutput;
    /// Return type for set operations.
    type SetOutput;

    #[doc(hidden)]
    fn get<const WORDS: usize>(self, grid: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput;
    #[doc(hidden)]
    fn set<const WORDS: usize>(self, grid: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput;
}

impl<const W: u16, const H: u16> GridIndexer<W, H> for ArrayPoint<W, H> {
    type GetOutput = bool;
    type SetOutput = ();

    fn get<const WORDS: usize>(self, grid: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
        grid.const_get(self)
    }

    fn set<const WORDS: usize>(self, grid: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
        grid.const_set(self, value);
    }
}

impl<const W: u16, const H: u16> GridIndexer<W, H> for ArrayIndex<W, H> {
    type GetOutput = bool;
    type SetOutput = ();

    fn get<const WORDS: usize>(self, grid: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
        grid.const_get(self.into())
    }

    fn set<const WORDS: usize>(self, grid: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
        grid.const_set(self.into(), value);
    }
}

impl<N1, N2, const W: u16, const H: u16> GridIndexer<W, H> for (N1, N2)
where
    N1: TryInto<u16>,
    N2: TryInto<u16>,
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

impl<N1, N2, const W: u16, const H: u16> GridIndexer<W, H> for Point<N1, N2>
where
    N1: TryInto<u16>,
    N2: TryInto<u16>,
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

macro_rules! impl_grid_indexer_for_int {
    ($($t:ty),*) => {
        $(
            impl<const W: u16, const H: u16> GridIndexer<W, H> for $t {
                type GetOutput = Result<bool, OutOfBounds>;
                type SetOutput = Result<(), OutOfBounds>;

                fn get<const WORDS: usize>(self, grid: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
                    ArrayIndex::try_new(self).map(|index| grid.const_get(index.into()))
                }

                fn set<const WORDS: usize>(self, grid: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
                    ArrayIndex::try_new(self).map(|index| grid.const_set(index.into(), value))
                }
            }
        )*
    };
}

impl_grid_indexer_for_int!(/*u8, u16*/ u32, /*u64, u128,*/ usize /*i8, i16, i32, i64, i128, isize*/);

fn try_into_array_point<N1, N2, const W: u16, const H: u16>(x: N1, y: N2) -> Result<ArrayPoint<W, H>, OutOfBounds>
where
    N1: TryInto<u16>,
    N2: TryInto<u16>,
{
    let x = x.try_into().map_err(OutOfBounds::new_from)?;
    let y = y.try_into().map_err(OutOfBounds::new_from)?;

    ArrayPoint::new(x, y)
}
