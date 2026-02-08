use crate::macros::{test_ctor, test_panic};
use grid_mask::ext::bits::FromBitRange;
use grid_mask::num::GridIndexU64;

mod range {
    use super::*;

    const INDEX_0: GridIndexU64 = GridIndexU64::const_new::<0>();
    const INDEX_2: GridIndexU64 = GridIndexU64::const_new::<2>();
    const INDEX_5: GridIndexU64 = GridIndexU64::const_new::<5>();

    test_ctor!(inclusive_inclusive: u64::from_bit_range(INDEX_2..=INDEX_5) => 0b111100);
    // test_ctor!(inclusive_exclusive: u64::from_bit_range(INDEX_2..INDEX_5) => 0b011100);
    test_ctor!(inclusive_unbounded: u64::from_bit_range(INDEX_2..) => !0b11);
    test_ctor!(unbounded_exclusive: u64::from_bit_range(..INDEX_5) => 0b11111);
    // test_ctor!(range_full: u64::from_bit_range(..) => u64::MAX);
    // test_ctor!(empty_exclusive: u64::from_bit_range(INDEX_2..INDEX_2) => 0);
    test_ctor!(exclusive_end_0_empty: u64::from_bit_range(..INDEX_0) => 0);

    test_panic!(panic_reversed: u64::from_bit_range(INDEX_5..=INDEX_2) => "Invalid range");
}
