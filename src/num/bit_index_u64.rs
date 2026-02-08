use std::num::NonZeroU32;
use std::num::NonZeroU64;
use std::ops::Range;

use size_hinter::SizeHint;
use tap::Pipe;

use crate::ext::Bound;
use crate::ext::bits::UnsetBit;
use crate::num::{GridLen, GridPos};

bounded_integer::bounded_integer! {
    /// A position in a u64 bitmask.
    ///
    /// The valid range is 0 to 63.
    #[repr(u8)]
    pub struct BitIndexU64(0, 63);
}

impl BitIndexU64 {
    /// Returns an iterator over all possible values of [`BitIndexU64`].
    #[must_use]
    pub fn all_values() -> BitIndexIter {
        BitIndexIter::new()
    }

    /// Returns the position of the first set bit in `val`, if any.
    #[must_use]
    pub const fn from_first_set(val: u64) -> Option<Self> {
        match val.trailing_zeros() {
            #[expect(clippy::cast_possible_truncation, reason = "match guards valid range")]
            val @ 0..64 => Self::new(val as u8),
            _ => None,
        }
    }

    /// Returns an iterator of all set indexes in a u64.
    #[must_use]
    pub fn iter_set_bits(val: u64) -> SetBitsIter {
        NonZeroU64::new(val).pipe(SetBitsIter)
    }
}

/// An iterator over all set bits in a `u64`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetBitsIter(Option<NonZeroU64>);

impl Iterator for SetBitsIter {
    type Item = BitIndexU64;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.0.map(NonZeroU64::trailing_zeros)?;
        self.0 = self.0.unset_low_bit();

        index.try_into().ok()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0
            .map(NonZeroU64::count_ones)
            .map(NonZeroU32::get)
            .map(usize::try_from)
            .transpose()
            .ok()
            .flatten()
            .unwrap_or(0)
            .pipe(SizeHint::exact)
            .into()
    }
}

impl DoubleEndedIterator for SetBitsIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.0.map(NonZeroU64::ilog2)?.try_into().ok()?;
        self.0 = self.0.unset_bit(index);

        Some(index)
    }
}

impl std::iter::ExactSizeIterator for SetBitsIter {}

impl std::iter::FusedIterator for SetBitsIter {}

impl Bound for BitIndexU64 {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

/// An iterator over all possible [`BitIndexU64`] values in a range.
#[derive(Debug, Clone)]
pub struct BitIndexIter(Range<u8>);

impl BitIndexIter {
    /// Creates a new [`BitIndexIter`] starting at `0` and ending at [`BitIndexU64::MAX`].
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self(0..64)
    }
}

impl Iterator for BitIndexIter {
    type Item = BitIndexU64;

    fn next(&mut self) -> Option<Self::Item> {
        // Safety: The range is always 0..64, so the values are always valid BitIndexU64
        self.0.next().map(|val| unsafe { BitIndexU64::new_unchecked(val) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for BitIndexIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        // Safety: The range is always 0..64, so the values are always valid BitIndexU64
        self.0.next_back().map(|val| unsafe { BitIndexU64::new_unchecked(val) })
    }
}

impl std::iter::ExactSizeIterator for BitIndexIter {}
impl std::iter::FusedIterator for BitIndexIter {}

impl From<GridPos> for BitIndexU64 {
    fn from(val: GridPos) -> Self {
        // Safety: GridPos is always <= 8, so it is always a valid BitIndexU64
        unsafe { Self::new_unchecked(val.get()) }
    }
}

impl From<(GridPos, GridPos)> for BitIndexU64 {
    fn from((x, y): (GridPos, GridPos)) -> Self {
        let index = y.get() * 8 + x.get();
        // Safety: index is always <= 63, so it is always a valid BitIndexU64
        unsafe { Self::new_unchecked(index) }
    }
}

impl From<GridLen> for BitIndexU64 {
    fn from(value: GridLen) -> Self {
        // Safety: GridLen is always <= 8, so it is always a valid BitIndexU64
        unsafe { Self::new_unchecked(value.get()) }
    }
}
