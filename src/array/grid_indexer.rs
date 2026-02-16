use crate::ArrayGrid;
use crate::err::OutOfBounds;
use crate::num::Point;
use crate::{ArrayIndex, ArrayPoint};

/// Adaptor trait for types that can be used to get a value from a grid `T`.
///
/// # Type parameters
///
/// * `T` - The type of the grid to index.
pub trait GridGetIndex<T: ?Sized> {
    /// Return type for a get operation.
    type GetOutput;

    /// Gets the value at this index in the grid.
    fn get(self, target: &T) -> Self::GetOutput;
}

/// Adaptor trait for types that can be used to set a value in a grid `T`.
///
/// # Type parameters
///
/// * `T` - The type of the grid to index.
pub trait GridSetIndex<T: ?Sized>: GridGetIndex<T> {
    /// Return type for a set operation.
    type SetOutput;

    /// Sets the value at this index in the grid.
    fn set(self, target: &mut T, value: bool) -> Self::SetOutput;
}

/// Implementation of [`GridIndexGet`] for [`usize`] (fallible)
impl<const W: u16, const H: u16, const WORDS: usize> GridGetIndex<ArrayGrid<W, H, WORDS>> for usize {
    type GetOutput = Result<bool, OutOfBounds>;

    fn get(self, grid: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
        ArrayIndex::<W, H>::try_new(self).map(|i| grid.const_get(i))
    }
}

/// Implementation of [`GridIndexSet`] for [`usize`] (fallible)
impl<const W: u16, const H: u16, const WORDS: usize> GridSetIndex<ArrayGrid<W, H, WORDS>> for usize {
    type SetOutput = Result<(), OutOfBounds>;

    fn set(self, grid: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
        ArrayIndex::<W, H>::try_new(self).map(|i| grid.const_set(i, value))
    }
}

/// Implementation of [`GridIndexGet`] for [`ArrayPoint`] (infallible)
impl<const W: u16, const H: u16, const WORDS: usize> GridGetIndex<ArrayGrid<W, H, WORDS>> for ArrayPoint<W, H> {
    type GetOutput = bool;

    fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
        target.const_get(self.to_index())
    }
}

/// Implementation of [`GridIndexSet`] for [`ArrayPoint`] (infallible)
impl<const W: u16, const H: u16, const WORDS: usize> GridSetIndex<ArrayGrid<W, H, WORDS>> for ArrayPoint<W, H> {
    type SetOutput = ();

    fn set(self, target: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
        target.const_set(self.to_index(), value);
    }
}

/// Implementation of [`GridIndexGet`] for [`ArrayIndex`] (infallible)
impl<const W: u16, const H: u16, const WORDS: usize> GridGetIndex<ArrayGrid<W, H, WORDS>> for ArrayIndex<W, H> {
    type GetOutput = bool;

    fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
        target.const_get(self)
    }
}

/// Implementation of [`GridIndexSet`] for [`ArrayIndex`] (infallible)
impl<const W: u16, const H: u16, const WORDS: usize> GridSetIndex<ArrayGrid<W, H, WORDS>> for ArrayIndex<W, H> {
    type SetOutput = ();

    fn set(self, target: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
        target.const_set(self, value);
    }
}

/// Implementation of [`GridIndexGet`] for `(N1, N2)` (fallible)
impl<N1, N2, const W: u16, const H: u16, const WORDS: usize> GridGetIndex<ArrayGrid<W, H, WORDS>> for (N1, N2)
where
    N1: TryInto<u16> + Copy,
    N2: TryInto<u16> + Copy,
{
    type GetOutput = Result<bool, OutOfBounds>;

    fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
        try_into_array_point::<N1, N2, W, H>(self.0, self.1).map(|p| target.const_get(p.to_index()))
    }
}

/// Implementation of [`GridIndexSet`] for `(N1, N2)` (fallible)
impl<N1, N2, const W: u16, const H: u16, const WORDS: usize> GridSetIndex<ArrayGrid<W, H, WORDS>> for (N1, N2)
where
    N1: TryInto<u16> + Copy,
    N2: TryInto<u16> + Copy,
{
    type SetOutput = Result<(), OutOfBounds>;

    fn set(self, target: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
        try_into_array_point::<N1, N2, W, H>(self.0, self.1).map(|p| target.const_set(p.to_index(), value))
    }
}

/// Implementation of [`GridIndexGet`] for [`Point<N1, N2>`] (fallible)
impl<N1, N2, const W: u16, const H: u16, const WORDS: usize> GridGetIndex<ArrayGrid<W, H, WORDS>> for Point<N1, N2>
where
    N1: TryInto<u16> + Copy,
    N2: TryInto<u16> + Copy,
{
    type GetOutput = Result<bool, OutOfBounds>;

    fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
        try_into_array_point::<N1, N2, W, H>(self.x, self.y).map(|p| target.const_get(p.to_index()))
    }
}

/// Implementation of [`GridIndexSet`] for [`Point<N1, N2>`] (fallible)
impl<N1, N2, const W: u16, const H: u16, const WORDS: usize> GridSetIndex<ArrayGrid<W, H, WORDS>> for Point<N1, N2>
where
    N1: TryInto<u16> + Copy,
    N2: TryInto<u16> + Copy,
{
    type SetOutput = Result<(), OutOfBounds>;

    fn set(self, target: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
        try_into_array_point::<N1, N2, W, H>(self.x, self.y).map(|p| target.const_set(p.to_index(), value))
    }
}

macro_rules! impl_grid_indexer_for_int {
    ($($t:ty),*) => {
        $(
            impl<const W: u16, const H: u16, const WORDS: usize> GridGetIndex<ArrayGrid<W, H, WORDS>> for $t {
                type GetOutput = Result<bool, OutOfBounds>;

                fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
                    ArrayIndex::<W, H>::try_new(self).map(|i| target.const_get(i))
                }
            }

            impl<const W: u16, const H: u16, const WORDS: usize> GridSetIndex<ArrayGrid<W, H, WORDS>> for $t {
                type SetOutput = Result<(), OutOfBounds>;

                fn set(self, target: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
                    ArrayIndex::<W, H>::try_new(self).map(|i| target.const_set(i, value))
                }
            }
        )*
    };
}

impl_grid_indexer_for_int!(u32);

fn try_into_array_point<N1, N2, const W: u16, const H: u16>(x: N1, y: N2) -> Result<ArrayPoint<W, H>, OutOfBounds>
where
    N1: TryInto<u16>,
    N2: TryInto<u16>,
{
    let x = x.try_into().map_err(OutOfBounds::new_from)?;
    let y = y.try_into().map_err(OutOfBounds::new_from)?;

    ArrayPoint::new(x, y)
}
