use bitvec::ptr::{BitRef, Mut};

use crate::ArrayGrid;
use crate::array::indexer::traits::{GridGetIndex, GridGetMutIndex, GridSetIndex};
use crate::err::OutOfBounds;
use crate::{ArrayIndex, GridView, GridViewMut};

macro_rules! impl_grid_indexer_for_int {
    ($($t:ty),*) => {
        $(
            impl<const W: u16, const H: u16, const WORDS: usize> GridGetIndex<ArrayGrid<W, H, WORDS>> for $t {
                type GetOutput<'a> = Result<bool, OutOfBounds> where ArrayGrid<W, H, WORDS>: 'a;

                fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput<'_> {
                    ArrayIndex::<W, H>::try_new(self).map(Into::into).map(|i| target.get_at(i))
                }
            }

            impl<const W: u16, const H: u16, const WORDS: usize> GridGetMutIndex<ArrayGrid<W, H, WORDS>> for $t {
                type GetMutOutput<'a>
                    = Result<BitRef<'a, Mut, u64>, OutOfBounds>
                where
                    ArrayGrid<W, H, WORDS>: 'a;

                fn get_mut(self, target: &mut ArrayGrid<W, H, WORDS>) -> Self::GetMutOutput<'_> {
                    ArrayIndex::<W, H>::try_new(self).map(Into::into).map(|i| target.get_mut_at(i))
                }
            }

            impl<const W: u16, const H: u16, const WORDS: usize> GridSetIndex<ArrayGrid<W, H, WORDS>> for $t {
                type SetOutput = Result<(), OutOfBounds>;

                fn set(self, target: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
                    ArrayIndex::<W, H>::try_new(self).map(Into::into).map(|i| target.set_at(i, value))
                }
            }

            impl<'a> GridGetIndex<GridView<'a>> for $t {
                type GetOutput<'b> = Result<bool, OutOfBounds> where GridView<'a>: 'b;

                fn get<'b>(self, target: &'b GridView<'a>) -> Self::GetOutput<'b> {
                    usize::try_from(self).map_err(OutOfBounds::from).and_then(|i| target.get_at(i))
                }
            }

            impl<'a> GridGetIndex<GridViewMut<'a>> for $t {
                type GetOutput<'b> = Result<bool, OutOfBounds> where GridViewMut<'a>: 'b;

                fn get<'b>(self, target: &'b GridViewMut<'a>) -> Self::GetOutput<'b> {
                    usize::try_from(self).map_err(OutOfBounds::from).and_then(|i| target.get_at(i))
                }
            }

            impl<'a> GridSetIndex<GridViewMut<'a>> for $t {
                type SetOutput = Result<(), OutOfBounds>;

                fn set(self, target: &mut GridViewMut<'a>, value: bool) -> Self::SetOutput {
                    usize::try_from(self).map_err(OutOfBounds::from).and_then(|i| target.set_at(i, value))
                }
            }
        )*
    };
}

impl_grid_indexer_for_int!(u32, i32, usize);
