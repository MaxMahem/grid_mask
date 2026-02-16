use std::num::NonZeroU32;
use std::num::NonZeroU64;

use size_hinter::SizeHint;
use tap::Pipe;

use crate::ext::bits::UnsetBit;
use crate::ext::{Bound, BoundedIter};
use crate::num::GridPos;

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
    pub const fn all_values() -> BoundedIter<Self> {
        BoundedIter::new()
    }

    /// Returns the position of the first set bit in `data`, if any.
    #[must_use]
    pub const fn from_first_set(data: u64) -> Option<Self> {
        match data.trailing_zeros() {
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

    const COUNT: usize = (Self::MAX.get() - Self::MIN.get()) as usize + 1;

    fn increment(&self) -> Option<Self> {
        self.get().checked_add(1).and_then(Self::new)
    }

    fn decrement(&self) -> Option<Self> {
        self.get().checked_sub(1).and_then(Self::new)
    }

    fn remaining(&self) -> usize {
        (Self::MAX.get() - self.get()) as usize
    }
}

/// An iterator over all possible [`BitIndexU64`] values in a range.
pub type BitIndexIter = BoundedIter<BitIndexU64>;

// impl From<GridPos> for BitIndexU64 {
//     fn from(val: GridPos) -> Self {
//         let index = val.get();
//         debug_assert_then!(
//             // Safety: GridPos is always <= 7, so it is always a valid GridIndexU64
//             index <= 7 => unsafe { Self::new_unchecked(index) },
//             "index ({index}) must be <= 7"
//         )
//     }
// }

impl BitIndexU64 {
    /// Creates a new [`BitIndexU64`] from grid coordinates `(x, y)`.
    ///
    /// This is equivalent to `y * 8 + x`.
    #[must_use]
    pub const fn at(x: GridPos, y: GridPos) -> Self {
        let index = y.get() * 8 + x.get();
        // Safety: The coordinates `x` and `y` are guaranteed to be in `0..=7`,
        // so the resulting index `y * 8 + x` is always in `0..=63`.
        unsafe { Self::new_unchecked(index) }
    }
}

// impl From<GridLen> for BitIndexU64 {
//     fn from(value: GridLen) -> Self {
//         let index = value.get();
//         debug_assert_then!(
//             // Safety: GridLen is always <= 8, so it is always a valid GridIndexU64
//             index <= 8 => unsafe { Self::new_unchecked(index) },
//             "index ({index}) must be <= 8"
//         )
//     }
// }
