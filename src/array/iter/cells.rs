use crate::ArrayIndex;
use crate::array::ArrayGrid;
use crate::ext::BoundedIter;

/// An iterator over all cells of an [`ArrayGrid`].
#[derive(Debug, Clone)]
pub struct Cells<'a, const W: u16, const H: u16, const WORDS: usize> {
    grid: &'a ArrayGrid<W, H, WORDS>,
    iter: BoundedIter<ArrayIndex<W, H>>,
}

impl<'a, const W: u16, const H: u16, const WORDS: usize> Cells<'a, W, H, WORDS> {
    pub(crate) const fn new(grid: &'a ArrayGrid<W, H, WORDS>) -> Self {
        Self { grid, iter: BoundedIter::new() }
    }
}

impl<const W: u16, const H: u16, const WORDS: usize> Iterator for Cells<'_, W, H, WORDS> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|i| self.grid.get(i))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<const W: u16, const H: u16, const WORDS: usize> DoubleEndedIterator for Cells<'_, W, H, WORDS> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|i| self.grid.get(i))
    }
}

impl<const W: u16, const H: u16, const WORDS: usize> ExactSizeIterator for Cells<'_, W, H, WORDS> {}
impl<const W: u16, const H: u16, const WORDS: usize> std::iter::FusedIterator for Cells<'_, W, H, WORDS> {}
