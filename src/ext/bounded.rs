use fluent_result::into::IntoOption;
use size_hinter::SizeHint;

/// A trait for types with defined minimum and maximum bounds.
pub trait Bound: Sized + Copy + PartialEq + PartialOrd + 'static {
    /// The minimum value of the type.
    const MIN: Self;

    /// The maximum value of the type.
    const MAX: Self;

    /// The total number of values in the range [`Self::MIN`] to [`Self::MAX`].
    ///
    /// This should be implemented such that it fails at compile time if `MIN > MAX`.
    const COUNT: usize;

    /// Returns the next value after `self`, or `None` if `self == Self::MAX`.
    #[must_use]
    fn increment(&self) -> Option<Self>;

    /// Returns the value before `self`, or `None` if `self == Self::MIN`.
    #[must_use]
    fn decrement(&self) -> Option<Self>;

    /// Returns the number of values remaining after `self` until [`Self::MAX`].
    ///
    /// If `self == Self::MAX`, this returns 0.
    #[must_use]
    fn remaining(&self) -> usize;
}

/// An iterator over values of a type that implements [`Bound`].
#[derive(Debug, Clone)]
pub struct BoundedIter<T>(Option<RangeInc<T>>);

#[derive(Debug, Clone)]
struct RangeInc<T> {
    start: T,
    end: T,
}

impl<T: Bound> BoundedIter<T> {
    /// Creates a new [`BoundedIter`] starting at [`Bound::MIN`] and ending at [`Bound::MAX`].
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self(Some(RangeInc { start: T::MIN, end: T::MAX }))
    }
}

impl<T: Bound> RangeInc<T> {
    fn len(&self) -> SizeHint {
        SizeHint::exact(self.start.remaining() - self.end.remaining() + 1)
    }
}

impl<T: Bound> Iterator for BoundedIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take()? {
            RangeInc { start, end } if start >= end => Some(start),
            RangeInc { start, end } => match start.increment() {
                Some(next) => {
                    self.0 = RangeInc { start: next, end }.into_some();
                    Some(start)
                }
                None => Some(start),
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.as_ref().map_or(SizeHint::ZERO, RangeInc::len).into()
    }
}

impl<T: Bound> DoubleEndedIterator for BoundedIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.0.take()? {
            RangeInc { start, end } if start >= end => Some(end),
            RangeInc { start, end } => match end.decrement() {
                Some(prev) => {
                    self.0 = RangeInc { start, end: prev }.into_some();
                    Some(end)
                }
                None => Some(end),
            },
        }
    }
}

impl<T: Bound> ExactSizeIterator for BoundedIter<T> {}

impl<T: Bound> std::iter::FusedIterator for BoundedIter<T> {}
