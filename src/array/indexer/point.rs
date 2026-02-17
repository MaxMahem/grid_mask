use fluent_result::into::IntoResult;

use crate::array::indexer::traits::{GridGetIndex, GridGetMutIndex, GridSetIndex};
use crate::err::OutOfBounds;
use crate::num::Point;
use crate::{ArrayGrid, ArrayIndex, ArrayPoint, GridView, GridViewMut};

pub fn try_into_point<X, Y>(x: X, y: Y) -> Result<Point<u16, u16>, OutOfBounds>
where
    X: TryInto<u16>,
    Y: TryInto<u16>,
{
    let x = x.try_into().map_err(OutOfBounds::from)?;
    let y = y.try_into().map_err(OutOfBounds::from)?;

    Point::new(x, y).into_ok()
}

pub mod array_grid_array_point {
    use crate::array::indexer::traits::{GridGetIndex, GridGetMutIndex, GridSetIndex};
    use crate::{ArrayGrid, ArrayPoint};
    use bitvec::ptr::{BitRef, Mut};
    use tap::{Conv, Pipe};

    impl<const W: u16, const H: u16, const WORDS: usize> GridGetIndex<ArrayGrid<W, H, WORDS>> for ArrayPoint<W, H> {
        type GetOutput<'a>
            = bool
        where
            ArrayGrid<W, H, WORDS>: 'a;

        fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput<'_> {
            target.const_get(self.to_index())
        }
    }

    impl<const W: u16, const H: u16, const WORDS: usize> GridSetIndex<ArrayGrid<W, H, WORDS>> for ArrayPoint<W, H> {
        type SetOutput = ();

        fn set(self, target: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
            target.const_set(self.to_index(), value);
        }
    }

    impl<const W: u16, const H: u16, const WORDS: usize> GridGetMutIndex<ArrayGrid<W, H, WORDS>> for ArrayPoint<W, H> {
        type GetMutOutput<'a>
            = BitRef<'a, Mut, u64>
        where
            ArrayGrid<W, H, WORDS>: 'a;

        fn get_mut(self, target: &mut ArrayGrid<W, H, WORDS>) -> Self::GetMutOutput<'_> {
            self.to_index().conv::<usize>().pipe(|i: usize| unsafe { target.data.get_unchecked_mut(i) })
        }
    }
}

pub mod array_grid_array_index {
    use tap::{Conv, Pipe};

    use crate::array::indexer::traits::{GridGetIndex, GridGetMutIndex, GridSetIndex};
    use crate::{ArrayGrid, ArrayIndex};

    impl<const W: u16, const H: u16, const WORDS: usize> GridGetIndex<ArrayGrid<W, H, WORDS>> for ArrayIndex<W, H> {
        type GetOutput<'a>
            = bool
        where
            ArrayGrid<W, H, WORDS>: 'a;

        fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput<'_> {
            target.const_get(self)
        }
    }

    impl<const W: u16, const H: u16, const WORDS: usize> GridSetIndex<ArrayGrid<W, H, WORDS>> for ArrayIndex<W, H> {
        type SetOutput = ();

        fn set(self, target: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
            target.const_set(self, value);
        }
    }

    impl<const W: u16, const H: u16, const WORDS: usize> GridGetMutIndex<ArrayGrid<W, H, WORDS>> for ArrayIndex<W, H> {
        type GetMutOutput<'a>
            = bitvec::ptr::BitRef<'a, bitvec::ptr::Mut, u64>
        where
            ArrayGrid<W, H, WORDS>: 'a;

        fn get_mut(self, target: &mut ArrayGrid<W, H, WORDS>) -> Self::GetMutOutput<'_> {
            self.conv::<usize>().pipe(|i: usize| unsafe { target.data.get_unchecked_mut(i) })
        }
    }
}

pub mod generic_point {

    pub mod array_grid {
        use bitvec::ptr::{BitRef, Mut};
        use tap::TryConv;

        use crate::array::indexer::traits::{GridGetIndex, GridGetMutIndex, GridSetIndex};
        use crate::err::OutOfBounds;
        use crate::num::Point;
        use crate::{ArrayGrid, ArrayPoint};

        impl<N1, N2, const W: u16, const H: u16, const WORDS: usize> GridGetIndex<ArrayGrid<W, H, WORDS>> for Point<N1, N2>
        where
            N1: TryInto<u16> + Copy,
            N2: TryInto<u16> + Copy,
        {
            type GetOutput<'a>
                = Result<bool, OutOfBounds>
            where
                ArrayGrid<W, H, WORDS>: 'a;

            fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput<'_> {
                self.try_into().map(|p: ArrayPoint<W, H>| target.const_get(p.to_index()))
            }
        }

        impl<N1, N2, const W: u16, const H: u16, const WORDS: usize> GridSetIndex<ArrayGrid<W, H, WORDS>> for Point<N1, N2>
        where
            N1: TryInto<u16> + Copy,
            N2: TryInto<u16> + Copy,
        {
            type SetOutput = Result<(), OutOfBounds>;

            fn set(self, target: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
                self.try_into().map(|p: ArrayPoint<W, H>| target.const_set(p.to_index(), value))
            }
        }

        impl<N1, N2, const W: u16, const H: u16, const WORDS: usize> GridGetMutIndex<ArrayGrid<W, H, WORDS>> for Point<N1, N2>
        where
            N1: TryInto<u16> + Copy,
            N2: TryInto<u16> + Copy,
        {
            type GetMutOutput<'a>
                = Result<BitRef<'a, Mut, u64>, OutOfBounds>
            where
                ArrayGrid<W, H, WORDS>: 'a;

            fn get_mut(self, target: &mut ArrayGrid<W, H, WORDS>) -> Self::GetMutOutput<'_> {
                self.try_conv::<ArrayPoint<W, H>>()
                    .map(ArrayPoint::<W, H>::to_index)
                    .map(Into::into)
                    .map(|index: usize| unsafe { target.data.get_unchecked_mut(index) })
            }
        }
    }

    pub mod grid_view {
        use super::super::{GridGetIndex, GridView, OutOfBounds, Point, try_into_point};

        impl<'a, N1, N2> GridGetIndex<GridView<'a>> for Point<N1, N2>
        where
            N1: TryInto<u16> + Copy,
            N2: TryInto<u16> + Copy,
        {
            type GetOutput<'b>
                = Result<bool, OutOfBounds>
            where
                GridView<'a>: 'b;

            fn get<'b>(self, target: &'b GridView<'a>) -> Self::GetOutput<'b> {
                try_into_point(self.x, self.y)
                    .and_then(|p| target.translate_point_to_index(p))
                    .map(|idx| target.data[idx])
            }
        }
    }

    pub mod grid_view_mut {
        use super::super::{GridGetIndex, GridSetIndex, GridViewMut, OutOfBounds, Point, try_into_point};

        impl<'a, N1, N2> GridGetIndex<GridViewMut<'a>> for Point<N1, N2>
        where
            N1: TryInto<u16> + Copy,
            N2: TryInto<u16> + Copy,
        {
            type GetOutput<'b>
                = Result<bool, OutOfBounds>
            where
                GridViewMut<'a>: 'b;

            fn get<'b>(self, target: &'b GridViewMut<'a>) -> Self::GetOutput<'b> {
                try_into_point(self.x, self.y)
                    .and_then(|p| target.translate_point_to_index(p))
                    .map(|idx| target.data[idx])
            }
        }

        impl<'a, N1, N2> GridSetIndex<GridViewMut<'a>> for Point<N1, N2>
        where
            N1: TryInto<u16> + Copy,
            N2: TryInto<u16> + Copy,
        {
            type SetOutput = Result<(), OutOfBounds>;

            fn set(self, target: &mut GridViewMut<'a>, value: bool) -> Self::SetOutput {
                try_into_point(self.x, self.y)
                    .and_then(|p| target.translate_point_to_index(p))
                    .map(|idx| target.data.set(idx, value))
            }
        }
    }
}

pub mod tuple {

    pub mod array_grid {
        use bitvec::ptr::{BitRef, Mut};

        use super::super::{ArrayGrid, ArrayPoint, GridGetIndex, GridGetMutIndex, GridSetIndex, OutOfBounds};

        impl<N1, N2, const W: u16, const H: u16, const WORDS: usize> GridGetIndex<ArrayGrid<W, H, WORDS>> for (N1, N2)
        where
            N1: TryInto<u16> + Copy,
            N2: TryInto<u16> + Copy,
        {
            type GetOutput<'a>
                = Result<bool, OutOfBounds>
            where
                ArrayGrid<W, H, WORDS>: 'a;

            fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput<'_> {
                self.try_into().map(|p: ArrayPoint<W, H>| target.const_get(p.to_index()))
            }
        }

        impl<N1, N2, const W: u16, const H: u16, const WORDS: usize> GridSetIndex<ArrayGrid<W, H, WORDS>> for (N1, N2)
        where
            N1: TryInto<u16> + Copy,
            N2: TryInto<u16> + Copy,
        {
            type SetOutput = Result<(), OutOfBounds>;

            fn set(self, target: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
                self.try_into().map(|p: ArrayPoint<W, H>| target.const_set(p.to_index(), value))
            }
        }

        impl<N1, N2, const W: u16, const H: u16, const WORDS: usize> GridGetMutIndex<ArrayGrid<W, H, WORDS>> for (N1, N2)
        where
            N1: TryInto<u16> + Copy,
            N2: TryInto<u16> + Copy,
        {
            type GetMutOutput<'a>
                = Result<BitRef<'a, Mut, u64>, OutOfBounds>
            where
                ArrayGrid<W, H, WORDS>: 'a;

            fn get_mut(self, target: &mut ArrayGrid<W, H, WORDS>) -> Self::GetMutOutput<'_> {
                self.try_into().map(|p: ArrayPoint<W, H>| target.get_mut_ref(p.to_index()))
            }
        }
    }

    pub mod grid_view {
        use super::super::{GridGetIndex, GridView, OutOfBounds, try_into_point};

        impl<'a, N1, N2> GridGetIndex<GridView<'a>> for (N1, N2)
        where
            N1: TryInto<u16> + Copy,
            N2: TryInto<u16> + Copy,
        {
            type GetOutput<'b>
                = Result<bool, OutOfBounds>
            where
                GridView<'a>: 'b;

            fn get<'b>(self, target: &'b GridView<'a>) -> Self::GetOutput<'b> {
                try_into_point(self.0, self.1)
                    .and_then(|p| target.translate_point_to_index(p))
                    .map(|idx| target.data[idx])
            }
        }
    }

    pub mod grid_view_mut {
        use super::super::{GridGetIndex, GridSetIndex, GridViewMut, OutOfBounds, try_into_point};

        impl<'a, N1, N2> GridGetIndex<GridViewMut<'a>> for (N1, N2)
        where
            N1: TryInto<u16> + Copy,
            N2: TryInto<u16> + Copy,
        {
            type GetOutput<'b>
                = Result<bool, OutOfBounds>
            where
                GridViewMut<'a>: 'b;

            fn get<'b>(self, target: &'b GridViewMut<'a>) -> Self::GetOutput<'b> {
                try_into_point(self.0, self.1)
                    .and_then(|p| target.translate_point_to_index(p))
                    .map(|idx| target.data[idx])
            }
        }

        impl<'a, N1, N2> GridSetIndex<GridViewMut<'a>> for (N1, N2)
        where
            N1: TryInto<u16> + Copy,
            N2: TryInto<u16> + Copy,
        {
            type SetOutput = Result<(), OutOfBounds>;

            fn set(self, target: &mut GridViewMut<'a>, value: bool) -> Self::SetOutput {
                try_into_point(self.0, self.1)
                    .and_then(|p| target.translate_point_to_index(p))
                    .map(|idx| target.data.set(idx, value))
            }
        }
    }
}
