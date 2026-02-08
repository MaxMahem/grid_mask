use crate::macros::test_transform;

use grid_mask::ext::range::Len32;

test_transform!(range_10_20: 10u32..20u32 => len_32() => 10);
test_transform!(range_0_10: 0u32..10u32 => len_32() => 10);
test_transform!(range_0_0: 0u32..0u32 => len_32() => 0);
