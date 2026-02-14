use crate::GridPoint;
use crate::grid::GridMask;
use crate::num::{BitIndexU64, SetBitsIter};

/// An iterator over all unset cells of a [`GridMask`].
#[derive(Debug, Clone)]
pub struct Spaces(SetBitsIter);

impl Spaces {
    pub(crate) fn new(mask: GridMask) -> Self {
        Self(BitIndexU64::iter_set_bits(!mask.0))
    }
}

impl Iterator for Spaces {
    type Item = GridPoint;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(GridPoint::from)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for Spaces {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(GridPoint::from)
    }
}

impl ExactSizeIterator for Spaces {}
impl std::iter::FusedIterator for Spaces {}
