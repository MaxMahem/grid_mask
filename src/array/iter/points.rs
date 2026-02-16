use bitvec::prelude::Lsb0;
use bitvec::slice::IterOnes;

use crate::ArrayPoint;
use crate::array::ArrayGrid;

/// An iterator over all set cells of an [`ArrayGrid`].
#[derive(Debug, Clone)]
pub struct Points<'a, const W: u16, const H: u16, const WORDS: usize> {
    iter: IterOnes<'a, u64, Lsb0>,
}

impl<'a, const W: u16, const H: u16, const WORDS: usize> Points<'a, W, H, WORDS> {
    pub(crate) fn new(grid: &'a ArrayGrid<W, H, WORDS>) -> Self {
        Self { iter: grid.data.as_bitslice().iter_ones() }
    }

    const W_USIZE: usize = W as usize;

    #[inline]
    fn to_point(i: usize) -> ArrayPoint<W, H> {
        let x = i % Self::W_USIZE;
        let y = i / Self::W_USIZE;
        (x, y).try_into().expect("index must be within bounds")
    }
}

impl<const W: u16, const H: u16, const WORDS: usize> Iterator for Points<'_, W, H, WORDS> {
    type Item = ArrayPoint<W, H>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Self::to_point)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<const W: u16, const H: u16, const WORDS: usize> DoubleEndedIterator for Points<'_, W, H, WORDS> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Self::to_point)
    }
}

impl<const W: u16, const H: u16, const WORDS: usize> ExactSizeIterator for Points<'_, W, H, WORDS> {}
impl<const W: u16, const H: u16, const WORDS: usize> std::iter::FusedIterator for Points<'_, W, H, WORDS> {}
