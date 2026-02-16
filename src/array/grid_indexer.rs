use crate::ArrayGrid;
use crate::err::OutOfBounds;
use crate::num::Point;
use crate::{ArrayIndex, ArrayPoint};

/// Adaptor trait for types that can be used to index a grid `T`.
///
/// #Type parameters
///
/// * `T` - The type of the grid to index.
pub trait GridIndex<T: ?Sized> {
    /// Return type for a get operation.
    type GetOutput;
    /// Return type for a set operation.
    type SetOutput;

    /// Gets the value at this index in the grid.
    fn get(self, target: &T) -> Self::GetOutput;

    /// Sets the value at this index in the grid.
    fn set(self, target: &mut T, value: bool) -> Self::SetOutput;
}

/// Implementation of [`GridIndex`] for [`usize`] (fallible)
impl<const W: u16, const H: u16, const WORDS: usize> GridIndex<ArrayGrid<W, H, WORDS>> for usize {
    type GetOutput = Result<bool, OutOfBounds>;
    type SetOutput = Result<(), OutOfBounds>;

    fn get(self, grid: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
        ArrayIndex::<W, H>::try_new(self).map(|i| grid.const_get(i))
    }

    fn set(self, grid: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
        ArrayIndex::<W, H>::try_new(self).map(|i| grid.const_set(i, value))
    }
}

// /// Implementation of [`GridIndex`] for [`Result<I, E>`] (fallible input)
// impl<T, I, E> GridIndex<T> for Result<I, E>
// where
//     T: ?Sized,
//     I: GridIndex<T>,
// {
//     type GetOutput = Result<I::GetOutput, E>;
//     type SetOutput = Result<I::SetOutput, E>;

//     fn get(self, grid: &T) -> Self::GetOutput {
//         self.map(|i| i.get(grid))
//     }

//     fn set(self, grid: &mut T, value: bool) -> Self::SetOutput {
//         self.map(|i| i.set(grid, value))
//     }
// }

/// Implementation of [`GridIndex`] for [`ArrayPoint`] (infallible)
impl<const W: u16, const H: u16, const WORDS: usize> GridIndex<ArrayGrid<W, H, WORDS>> for ArrayPoint<W, H> {
    type GetOutput = bool;
    type SetOutput = ();

    fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
        target.const_get(self.to_index())
    }

    fn set(self, target: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
        target.const_set(self.to_index(), value);
    }
}

/// Implementation of [`GridIndex`] for [`ArrayIndex`] (infallible)
impl<const W: u16, const H: u16, const WORDS: usize> GridIndex<ArrayGrid<W, H, WORDS>> for ArrayIndex<W, H> {
    type GetOutput = bool;
    type SetOutput = ();

    fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
        target.const_get(self)
    }

    fn set(self, target: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
        target.const_set(self, value);
    }
}

/// Implementation of [`GridIndex`] for `(N1, N2)` (fallible)
impl<N1, N2, const W: u16, const H: u16, const WORDS: usize> GridIndex<ArrayGrid<W, H, WORDS>> for (N1, N2)
where
    N1: TryInto<u16> + Copy,
    N2: TryInto<u16> + Copy,
{
    type GetOutput = Result<bool, OutOfBounds>;
    type SetOutput = Result<(), OutOfBounds>;

    fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
        try_into_array_point::<N1, N2, W, H>(self.0, self.1).map(|p| target.const_get(p.to_index()))
    }

    fn set(self, target: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
        try_into_array_point::<N1, N2, W, H>(self.0, self.1).map(|p| target.const_set(p.to_index(), value))
    }
}

/// Implementation of [`GridIndex`] for [`Point<N1, N2>`] (fallible)
impl<N1, N2, const W: u16, const H: u16, const WORDS: usize> GridIndex<ArrayGrid<W, H, WORDS>> for Point<N1, N2>
where
    N1: TryInto<u16> + Copy,
    N2: TryInto<u16> + Copy,
{
    type GetOutput = Result<bool, OutOfBounds>;
    type SetOutput = Result<(), OutOfBounds>;

    fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
        try_into_array_point::<N1, N2, W, H>(self.x, self.y).map(|p| target.const_get(p.to_index()))
    }

    fn set(self, target: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
        try_into_array_point::<N1, N2, W, H>(self.x, self.y).map(|p| target.const_set(p.to_index(), value))
    }
}

macro_rules! impl_grid_indexer_for_int {
    ($($t:ty),*) => {
        $(
            impl<const W: u16, const H: u16, const WORDS: usize> GridIndex<ArrayGrid<W, H, WORDS>> for $t {
                type GetOutput = Result<bool, OutOfBounds>;
                type SetOutput = Result<(), OutOfBounds>;

                fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput {
                    ArrayIndex::<W, H>::try_new(self)
                        .map(|i| target.const_get(i))
                }

                fn set(self, target: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
                    ArrayIndex::<W, H>::try_new(self)
                        .map(|i| target.const_set(i, value))
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
