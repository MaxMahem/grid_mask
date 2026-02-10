use crate::ext::BoundedIter;
use crate::grid::{GridData, GridMask};

/// An iterator over all cells of a [`GridMask`].
#[derive(Debug, Clone)]
pub struct Cells<'a, T: GridData = u64> {
    mask: &'a GridMask<T>,
    iter: BoundedIter<T::Index>,
}

impl<'a, T: GridData> Cells<'a, T> {
    pub(crate) const fn new(mask: &'a GridMask<T>) -> Self {
        Self { mask, iter: BoundedIter::new() }
    }
}

impl<T: GridData> Iterator for Cells<'_, T> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|i| self.mask.index(i))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T: GridData> DoubleEndedIterator for Cells<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|i| self.mask.index(i))
    }
}

impl<T: GridData> ExactSizeIterator for Cells<'_, T> {}
impl<T: GridData> std::iter::FusedIterator for Cells<'_, T> {}
