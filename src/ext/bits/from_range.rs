use std::ops::{RangeFrom, RangeInclusive, RangeTo};

use crate::ext::Bound;
use crate::num::BitIndexU64;

/// Returns a bitmask with the bits in the given range set.
pub trait FromBitRange<R> {
    fn from_bit_range(range: R) -> Self;
}

/// Returns a bitmask with the bits in the given range set.
const fn generate_mask(start: u32, end: u32) -> u64 {
    match (start, end) {
        (start, end) if start > end => panic!("Invalid range"),
        (_, 0) => 0,
        (start, end) => (u64::MAX << start) & (u64::MAX >> (64 - end)),
    }
}

// impl<T> FromBitRange<Range<T>> for u64
// where
//     T: Into<BitIndexU64> + BoundedValue,
// {
//     fn from_bit_range(range: Range<T>) -> Self {
//         let start: BitIndexU64 = range.start.into();
//         let end: BitIndexU64 = range.end.into();

//         generate_mask(start.into(), end.into())
//     }
// }

impl<T> FromBitRange<RangeInclusive<T>> for u64
where
    T: Into<BitIndexU64> + Bound,
{
    fn from_bit_range(range: RangeInclusive<T>) -> Self {
        let (start, end) = range.into_inner();
        let start: BitIndexU64 = start.into();
        let end: BitIndexU64 = end.into();

        generate_mask(start.into(), u32::from(end) + 1)
    }
}

impl<T> FromBitRange<RangeFrom<T>> for u64
where
    T: Into<BitIndexU64> + Bound,
{
    fn from_bit_range(range: RangeFrom<T>) -> Self {
        let start: BitIndexU64 = range.start.into();
        let end: BitIndexU64 = T::MAX.into();

        generate_mask(start.into(), u32::from(end) + 1)
    }
}

impl<T> FromBitRange<RangeTo<T>> for u64
where
    T: Into<BitIndexU64> + Bound,
{
    fn from_bit_range(range: RangeTo<T>) -> Self {
        let start: BitIndexU64 = T::MIN.into();
        let end: BitIndexU64 = range.end.into();

        generate_mask(start.into(), end.into())
    }
}

// impl<T> FromBitRange<RangeToInclusive<T>> for u64
// where
//     T: Into<BitIndexU64> + BoundedValue,
// {
//     fn from_bit_range(range: RangeToInclusive<T>) -> Self {
//         let start: BitIndexU64 = T::MIN.into();
//         let end: BitIndexU64 = range.end.into();

//         generate_mask(start.into(), u32::from(end) + 1)
//     }
// }

// impl FromBitRange<RangeFull> for u64 {
//     fn from_bit_range(_: RangeFull) -> Self {
//         Self::MAX
//     }
// }
