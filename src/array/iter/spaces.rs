use bitvec::prelude::Lsb0;
use bitvec::slice::IterZeros;

use crate::ArrayPoint;
use crate::array::ArrayGrid;

/// An iterator over all unset cells of an [`ArrayGrid`].
#[derive(Debug, Clone)]
pub struct Spaces<'a, const W: u16, const H: u16, const WORDS: usize> {
    iter: IterZeros<'a, u64, Lsb0>,
}

impl<'a, const W: u16, const H: u16, const WORDS: usize> Spaces<'a, W, H, WORDS> {
    pub(crate) fn new(grid: &'a ArrayGrid<W, H, WORDS>) -> Self {
        Self { iter: grid.words.as_bitslice()[..ArrayGrid::<W, H, WORDS>::CELL_COUNT_USZ].iter_zeros() }
    }

    fn to_point(i: usize) -> ArrayPoint<W, H> {
        let x = i % W as usize;
        let y = i / W as usize;
        (x, y).try_into().expect("index must be within bounds")
    }
}

impl<const W: u16, const H: u16, const WORDS: usize> Iterator for Spaces<'_, W, H, WORDS> {
    type Item = ArrayPoint<W, H>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Self::to_point)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<const W: u16, const H: u16, const WORDS: usize> DoubleEndedIterator for Spaces<'_, W, H, WORDS> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Self::to_point)
    }
}

impl<const W: u16, const H: u16, const WORDS: usize> ExactSizeIterator for Spaces<'_, W, H, WORDS> {}
impl<const W: u16, const H: u16, const WORDS: usize> std::iter::FusedIterator for Spaces<'_, W, H, WORDS> {}
