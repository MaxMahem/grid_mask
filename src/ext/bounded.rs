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
pub struct BoundedIter<T: Bound> {
    start: Option<T>,
    end: Option<T>,
}

impl<T: Bound> BoundedIter<T> {
    /// Creates a new [`BoundedIter`] starting at [`Bound::MIN`] and ending at [`Bound::MAX`].
    #[must_use]
    pub const fn new() -> Self {
        Self { start: Some(T::MIN), end: Some(T::MAX) }
    }
}

impl<T: Bound> Default for BoundedIter<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Bound> Iterator for BoundedIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.start?;

        if Some(start) == self.end {
            self.start = None;
            self.end = None;
        } else {
            self.start = start.increment();
        }

        Some(start)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = match (self.start, self.end) {
            (Some(start), Some(end)) => start.remaining() - end.remaining() + 1,
            _ => 0,
        };
        (len, Some(len))
    }
}

impl<T: Bound> DoubleEndedIterator for BoundedIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let end = self.end?;

        if Some(end) == self.start {
            self.start = None;
            self.end = None;
        } else {
            self.end = end.decrement();
        }

        Some(end)
    }
}

impl<T: Bound> ExactSizeIterator for BoundedIter<T> {}

impl<T: Bound> std::iter::FusedIterator for BoundedIter<T> {}
