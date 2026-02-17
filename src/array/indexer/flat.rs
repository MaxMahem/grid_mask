use crate::ArrayGrid;
use crate::err::OutOfBounds;
use crate::{ArrayIndex, GridView, GridViewMut};

use crate::array::indexer::traits::{GridGetIndex, GridSetIndex};

macro_rules! impl_grid_indexer_for_int {
    ($($t:ty),*) => {
        $(
            impl<const W: u16, const H: u16, const WORDS: usize> GridGetIndex<ArrayGrid<W, H, WORDS>> for $t {
                type GetOutput<'a> = Result<bool, OutOfBounds> where ArrayGrid<W, H, WORDS>: 'a;

                fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput<'_> {
                    ArrayIndex::<W, H>::try_new(self).map(|i| target.const_get(i))
                }
            }

            impl<const W: u16, const H: u16, const WORDS: usize> GridSetIndex<ArrayGrid<W, H, WORDS>> for $t {
                type SetOutput = Result<(), OutOfBounds>;

                fn set(self, target: &mut ArrayGrid<W, H, WORDS>, value: bool) -> Self::SetOutput {
                    ArrayIndex::<W, H>::try_new(self).map(|i| target.const_set(i, value))
                }
            }

            impl<'a> GridGetIndex<GridView<'a>> for $t {
                type GetOutput<'b> = Result<bool, OutOfBounds> where GridView<'a>: 'b;

                fn get<'b>(self, target: &'b GridView<'a>) -> Self::GetOutput<'b> {
                    let index = usize::try_from(self).map_err(OutOfBounds::from)?;
                    target.data.get(index).ok_or(OutOfBounds).map(|b| *b)
                }
            }

            impl<'a> GridGetIndex<GridViewMut<'a>> for $t {
                type GetOutput<'b> = Result<bool, OutOfBounds> where GridViewMut<'a>: 'b;

                fn get<'b>(self, target: &'b GridViewMut<'a>) -> Self::GetOutput<'b> {
                    let index = usize::try_from(self).map_err(OutOfBounds::from)?;
                    target.data.get(index).ok_or(OutOfBounds).map(|b| *b)
                }
            }

            impl<'a> GridSetIndex<GridViewMut<'a>> for $t {
                type SetOutput = Result<(), OutOfBounds>;

                fn set(self, target: &mut GridViewMut<'a>, value: bool) -> Self::SetOutput {
                    let index = usize::try_from(self).map_err(OutOfBounds::from)?;
                    (index < target.data.len()).then(|| target.data.set(index, value)).ok_or(OutOfBounds)
                }
            }
        )*
    };
}

impl_grid_indexer_for_int!(u32, i32, usize);
