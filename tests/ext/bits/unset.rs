use grid_mask::ext::bits::UnsetBit;
use grid_mask::num::BitIndexU64;
use std::num::NonZeroU64;

const VAL_3_BITS: Option<NonZeroU64> = NonZeroU64::new(0b0101010);
const VAL_NO_LOW_BIT: Option<NonZeroU64> = NonZeroU64::new(0b0101000);
const VAL_NO_HIGH_BIT: Option<NonZeroU64> = NonZeroU64::new(0b0001010);
const VAL_NO_MID_BIT: Option<NonZeroU64> = NonZeroU64::new(0b0100010);

const VAL_1_BIT: Option<NonZeroU64> = NonZeroU64::new(0b0001000);

const INDEX_MID: BitIndexU64 = BitIndexU64::const_new::<3>();
const INDEX_UNSET: BitIndexU64 = BitIndexU64::const_new::<4>();

use crate::macros::test_transform;

mod unset_low_bit {
    use super::*;

    test_transform!(three_bits: VAL_3_BITS => unset_low_bit => VAL_NO_LOW_BIT);
    test_transform!(one_bit: VAL_1_BIT => unset_low_bit => None);
    test_transform!(unset_none: None => unset_low_bit => None);
}

mod unset_high_bit {
    use super::*;

    test_transform!(three_bits: VAL_3_BITS => unset_high_bit => VAL_NO_HIGH_BIT);
    test_transform!(one_bit: VAL_1_BIT => unset_high_bit => None);
    test_transform!(unset_none: None => unset_high_bit => None);
}

mod unset_index {
    use super::*;

    test_transform!(three_bits: VAL_3_BITS => unset_bit(INDEX_MID) => VAL_NO_MID_BIT);
    test_transform!(one_bit: VAL_1_BIT => unset_bit(INDEX_MID) => None);
    test_transform!(unset_unset_index: VAL_1_BIT => unset_bit(INDEX_UNSET) => VAL_1_BIT);
    test_transform!(unset_none: None => unset_bit(INDEX_UNSET) => None);
}
