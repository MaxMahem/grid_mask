use crate::macros::test_transform;

use grid_mask::ext::range::RangeLength;

test_transform!(range_10_20: 10u8..20u8 => length() => 10);
test_transform!(range_0_10: 0u8..10u8 => length() => 10);
test_transform!(range_0_0: 0u8..0u8 => length() => 0);
