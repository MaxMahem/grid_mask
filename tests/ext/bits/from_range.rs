use crate::macros::{test_ctor, test_panic};
use grid_mask::ext::bits::FromBitRange;
use grid_mask::num::BitIndexU64;

mod range {
    use super::*;

    const INDEX_0: BitIndexU64 = BitIndexU64::const_new::<0>();
    const INDEX_2: BitIndexU64 = BitIndexU64::const_new::<2>();
    const INDEX_5: BitIndexU64 = BitIndexU64::const_new::<5>();

    test_ctor!(inclusive_inclusive: u64::from_bit_range(INDEX_2..=INDEX_5) => 0b111100);
    // test_ctor!(inclusive_exclusive: u64::from_bit_range(INDEX_2..INDEX_5) => 0b011100);
    test_ctor!(inclusive_unbounded: u64::from_bit_range(INDEX_2..) => !0b11);
    test_ctor!(unbounded_exclusive: u64::from_bit_range(..INDEX_5) => 0b11111);
    // test_ctor!(range_full: u64::from_bit_range(..) => u64::MAX);
    // test_ctor!(empty_exclusive: u64::from_bit_range(INDEX_2..INDEX_2) => 0);
    test_ctor!(exclusive_end_0_empty: u64::from_bit_range(..INDEX_0) => 0);

    test_panic!(panic_reversed: u64::from_bit_range(INDEX_5..=INDEX_2) => "start (5) should be <= end (2)");
}

mod u8_range {
    use super::*;
    use grid_mask::num::BitIndexU8;

    const INDEX_2: BitIndexU8 = BitIndexU8::const_new::<2>();
    const INDEX_5: BitIndexU8 = BitIndexU8::const_new::<5>();

    test_ctor!(inclusive_inclusive: u8::from_bit_range(INDEX_2..=INDEX_5) => 0b0011_1100);
    test_panic!(panic_reversed: u8::from_bit_range(INDEX_5..=INDEX_2) => "start (5) should be <= end (2)");
}
