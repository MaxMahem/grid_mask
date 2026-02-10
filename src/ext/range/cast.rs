use std::ops::{Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use tap::Pipe;

use crate::ext::MapTuple;

/// Extension trait for casting ranges to a different type.
pub trait RangeCast<T>: RangeBounds<T> + Sized {
    type Output<U>;

    /// Casts the range bounds to type `U`.
    fn cast<U>(self) -> Self::Output<U>
    where
        U: From<T>;

    /// Try to cast the range bounds to type `U` and returns a `Result`.
    fn try_cast<U>(self) -> Result<Self::Output<U>, U::Error>
    where
        U: TryFrom<T>;
}

impl<T> RangeCast<T> for Range<T> {
    type Output<U> = Range<U>;

    fn cast<U: From<T>>(self) -> Range<U> {
        self.start.into()..self.end.into()
    }

    fn try_cast<U: TryFrom<T>>(self) -> Result<Range<U>, U::Error> {
        Ok(self.start.try_into()?..self.end.try_into()?)
    }
}

impl<T: Clone> RangeCast<T> for RangeInclusive<T> {
    type Output<U> = RangeInclusive<U>;

    fn cast<U: From<T>>(self) -> RangeInclusive<U> {
        self.into_inner() // fmt col
            .map_into::<T, T>()
            .pipe(|(start, end)| start.into()..=end.into())
    }

    fn try_cast<U: TryFrom<T>>(self) -> Result<RangeInclusive<U>, U::Error> {
        Ok(self.start().clone().try_into()?..=self.end().clone().try_into()?)
    }
}

impl<T> RangeCast<T> for RangeFull {
    type Output<U> = Self;

    fn cast<U: From<T>>(self) -> Self {
        self
    }

    fn try_cast<U: TryFrom<T>>(self) -> Result<Self, U::Error> {
        Ok(self)
    }
}

impl<T> RangeCast<T> for RangeFrom<T> {
    type Output<U> = RangeFrom<U>;

    fn cast<U: From<T>>(self) -> Self::Output<U> {
        self.start.into()..
    }

    fn try_cast<U: TryFrom<T>>(self) -> Result<Self::Output<U>, U::Error> {
        Ok(self.start.try_into()?..)
    }
}

impl<T> RangeCast<T> for RangeTo<T> {
    type Output<U> = RangeTo<U>;

    fn cast<U: From<T>>(self) -> Self::Output<U> {
        ..self.end.into()
    }

    fn try_cast<U: TryFrom<T>>(self) -> Result<Self::Output<U>, U::Error> {
        Ok(..self.end.try_into()?)
    }
}

impl<T> RangeCast<T> for RangeToInclusive<T> {
    type Output<U> = RangeToInclusive<U>;

    fn cast<U: From<T>>(self) -> Self::Output<U> {
        ..=self.end.into()
    }

    fn try_cast<U: TryFrom<T>>(self) -> Result<Self::Output<U>, U::Error> {
        Ok(..=self.end.try_into()?)
    }
}
