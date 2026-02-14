use std::fmt::Display;
use std::ops::{Range, RangeFrom, RangeInclusive, RangeTo};

use tap::{Pipe, Tap};

use crate::ext::MapTuple;
use crate::num::{BitIndexU8, BitIndexU64};

/// Trait for creating a value/bitmask from a range of indices.
pub trait FromBitRange<R> {
    /// Returns a value/bitmask with the bits in the given range set.
    fn from_bit_range(range: R) -> Self;
}

const fn generate_mask_u64(range: Range<u32>) -> u64 {
    (u64::MAX << range.start) & (u64::MAX.unbounded_shr(u64::BITS - range.end))
}

impl<T> FromBitRange<RangeInclusive<T>> for u64
where
    T: Into<BitIndexU64> + Display + PartialOrd + Ord,
{
    fn from_bit_range(range: RangeInclusive<T>) -> Self {
        range
            .into_inner()
            .tap(|(start, end)| assert!(start <= end, "start ({start}) should be <= end ({end})"))
            .map_into::<BitIndexU64, BitIndexU64>()
            .map_into::<u32, u32>()
            .pipe(|(start, end)| generate_mask_u64(start..end + 1))
    }
}

impl<T> FromBitRange<RangeFrom<T>> for u64
where
    T: Into<BitIndexU64>,
{
    fn from_bit_range(range: RangeFrom<T>) -> Self {
        const EXCLUSIVE_MAX: u32 = (BitIndexU64::MAX.get() + 1) as u32;
        generate_mask_u64(range.start.into().into()..EXCLUSIVE_MAX)
    }
}

impl<T> FromBitRange<RangeTo<T>> for u64
where
    T: Into<BitIndexU64>,
{
    fn from_bit_range(range: RangeTo<T>) -> Self {
        const INCLUSIVE_MIN: u32 = BitIndexU64::MIN.get() as u32;
        generate_mask_u64(INCLUSIVE_MIN..range.end.into().into())
    }
}

const fn generate_mask_u8(range: Range<u32>) -> u8 {
    (u8::MAX << range.start) & (u8::MAX.unbounded_shr(u8::BITS - range.end))
}

impl<T> FromBitRange<RangeInclusive<T>> for u8
where
    T: Into<BitIndexU8> + Display + PartialOrd + Ord,
{
    fn from_bit_range(range: RangeInclusive<T>) -> Self {
        range
            .into_inner()
            .tap(|(start, end)| assert!(start <= end, "start ({start}) should be <= end ({end})"))
            .map_into::<BitIndexU8, BitIndexU8>()
            .map_into::<u32, u32>()
            .pipe(|(start, end)| generate_mask_u8(start..end + 1))
    }
}

// impl<T> FromBitRange<RangeFrom<T>> for u8
// where
//     T: Into<BitIndexU8>,
// {
//     fn from_bit_range(range: RangeFrom<T>) -> Self {
//         const EXCLUSIVE_MAX: u32 = (BitIndexU8::MAX.get() + 1) as u32;
//         generate_mask_u8(range.start.into().into()..EXCLUSIVE_MAX)
//     }
// }

impl<T> FromBitRange<RangeTo<T>> for u8
where
    T: Into<BitIndexU8>,
{
    fn from_bit_range(range: RangeTo<T>) -> Self {
        const INCLUSIVE_MIN: u32 = BitIndexU8::MIN.get() as u32;
        generate_mask_u8(INCLUSIVE_MIN..range.end.into().into())
    }
}
