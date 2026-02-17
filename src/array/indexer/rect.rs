use crate::ArrayGrid;
use crate::err::OutOfBounds;
use crate::num::{Point, Rect, Size};
use crate::{ArrayPoint, ArrayRect, ArraySize, GridView, GridViewMut};

use crate::array::indexer::traits::{GridGetIndex, GridGetMutIndex};

// --- ArrayRect ---

/// Implementation of [`GridGetIndex`] for [`ArrayRect`] (infallible view)
impl<const W: u16, const H: u16, const WORDS: usize> GridGetIndex<ArrayGrid<W, H, WORDS>> for ArrayRect<W, H> {
    type GetOutput<'a>
        = GridView<'a>
    where
        ArrayGrid<W, H, WORDS>: 'a;

    fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput<'_> {
        target.view(self)
    }
}

/// Implementation of [`GridGetMutIndex`] for [`ArrayRect`] (infallible mutable view)
impl<const W: u16, const H: u16, const WORDS: usize> GridGetMutIndex<ArrayGrid<W, H, WORDS>> for ArrayRect<W, H> {
    type GetMutOutput<'a>
        = GridViewMut<'a>
    where
        ArrayGrid<W, H, WORDS>: 'a;

    fn get_mut(self, target: &mut ArrayGrid<W, H, WORDS>) -> Self::GetMutOutput<'_> {
        target.view_mut(self)
    }
}

// --- Rect ---

/// Implementation of [`GridGetIndex`] for [`Rect<P, S>`] on [`ArrayGrid`] (fallible view)
impl<PX, PY, SW, SH, const W: u16, const H: u16, const WORDS: usize> GridGetIndex<ArrayGrid<W, H, WORDS>>
    for Rect<Point<PX, PY>, Size<SW, SH>>
where
    PX: TryInto<u16> + Copy,
    PY: TryInto<u16> + Copy,
    SW: TryInto<u16> + Copy,
    SH: TryInto<u16> + Copy,
{
    type GetOutput<'a>
        = Result<GridView<'a>, OutOfBounds>
    where
        ArrayGrid<W, H, WORDS>: 'a;

    fn get(self, target: &ArrayGrid<W, H, WORDS>) -> Self::GetOutput<'_> {
        let x = self.point.x.try_into().map_err(OutOfBounds::from)?;
        let y = self.point.y.try_into().map_err(OutOfBounds::from)?;
        let width = self.size.width.try_into().map_err(OutOfBounds::from)?;
        let height = self.size.height.try_into().map_err(OutOfBounds::from)?;

        let point = ArrayPoint::new(x, y)?;
        let size = ArraySize::new(width, height)?;

        ArrayRect::new(point, size).map(|rect| target.view(rect))
    }
}

/// Implementation of [`GridGetMutIndex`] for [`Rect<P, S>`] on [`ArrayGrid`] (fallible mutable view)
impl<PX, PY, SW, SH, const W: u16, const H: u16, const WORDS: usize> GridGetMutIndex<ArrayGrid<W, H, WORDS>>
    for Rect<Point<PX, PY>, Size<SW, SH>>
where
    PX: TryInto<u16> + Copy,
    PY: TryInto<u16> + Copy,
    SW: TryInto<u16> + Copy,
    SH: TryInto<u16> + Copy,
{
    type GetMutOutput<'a>
        = Result<GridViewMut<'a>, OutOfBounds>
    where
        ArrayGrid<W, H, WORDS>: 'a;

    fn get_mut(self, target: &mut ArrayGrid<W, H, WORDS>) -> Self::GetMutOutput<'_> {
        let x = self.point.x.try_into().map_err(OutOfBounds::from)?;
        let y = self.point.y.try_into().map_err(OutOfBounds::from)?;
        let width = self.size.width.try_into().map_err(OutOfBounds::from)?;
        let height = self.size.height.try_into().map_err(OutOfBounds::from)?;

        let point = ArrayPoint::new(x, y)?;
        let size = ArraySize::new(width, height)?;

        ArrayRect::new(point, size).map(|rect| target.view_mut(rect))
    }
}

/// Implementation of [`GridGetIndex`] for [`Rect<P, S>`] on [`GridView`] (fallible view)
impl<'a, PX, PY, SW, SH> GridGetIndex<GridView<'a>> for Rect<Point<PX, PY>, Size<SW, SH>>
where
    PX: TryInto<u16> + Copy,
    PY: TryInto<u16> + Copy,
    SW: TryInto<u16> + Copy,
    SH: TryInto<u16> + Copy,
{
    type GetOutput<'b>
        = Result<GridView<'b>, OutOfBounds>
    where
        GridView<'a>: 'b;

    fn get<'b>(self, target: &'b GridView<'a>) -> Self::GetOutput<'b> {
        let x = self.point.x.try_into().map_err(OutOfBounds::from)?;
        let y = self.point.y.try_into().map_err(OutOfBounds::from)?;
        let width = self.size.width.try_into().map_err(OutOfBounds::from)?;
        let height = self.size.height.try_into().map_err(OutOfBounds::from)?;

        let point = Point::new(x, y);
        let size = Size::new(width, height);
        let rect = Rect::new(point, size);

        target.try_view(rect)
    }
}
