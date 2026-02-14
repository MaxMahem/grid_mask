use crate::ext::BoundedIter;
use crate::grid::GridMask;
use crate::num::BitIndexU64;

/// An iterator over all cells of a [`GridMask`].
#[derive(Debug, Clone)]
pub struct Cells<'a> {
    mask: &'a GridMask,
    iter: BoundedIter<BitIndexU64>,
}

impl<'a> Cells<'a> {
    pub(crate) const fn new(mask: &'a GridMask) -> Self {
        Self { mask, iter: BoundedIter::new() }
    }
}

impl Iterator for Cells<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|i| self.mask.get(i))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl DoubleEndedIterator for Cells<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|i| self.mask.get(i))
    }
}

impl ExactSizeIterator for Cells<'_> {}
impl std::iter::FusedIterator for Cells<'_> {}
