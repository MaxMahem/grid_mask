use grid_mask::ext::range::RangeCast;

use crate::macros::{test_ctor, test_transform};

mod range {
    use super::*;

    test_transform!(cast: 0u32..10u32 => cast::<u64> => 0u64..10u64);
    test_transform!(try_cast_ok: 0u8..10u8 => try_cast::<u16> => Ok(0u16..10u16));
    test_transform!(try_cast_fail: 0u16..300u16 => try_cast::<u8> => matches Err(_));
}

mod range_inclusive {
    use super::*;

    test_transform!(cast: 0u32..=10u32 => cast::<u64> => 0u64..=10u64);
    test_transform!(try_cast_ok: 0u8..=10u8 => try_cast::<u16> => Ok(0u16..=10u16));
    test_transform!(try_cast_fail: 0u16..=300u16 => try_cast::<u8> => matches Err(_));
}

mod range_from {
    use super::*;

    test_transform!(cast: 0u32.. => cast::<u64> => 0u64..);
    test_transform!(try_cast_ok: 0u8.. => try_cast::<u16> => Ok(0u16..));
    test_transform!(try_cast_fail: 300u16.. => try_cast::<u8> => matches Err(_));
}

mod range_to {
    use super::*;

    test_transform!(cast: ..10u32 => cast::<u64> => ..10u64);
    test_transform!(try_cast_ok: ..10u8 => try_cast::<u16> => Ok(..10u16));
    test_transform!(try_cast_fail: ..300u16 => try_cast::<u8> => matches Err(_));
}

mod range_to_inclusive {
    use super::*;

    test_transform!(cast: ..=10u32 => cast::<u64> => ..=10u64);
    test_transform!(try_cast_ok: ..=10u8 => try_cast::<u16> => Ok(..=10u16));
    test_transform!(try_cast_fail: ..=300u16 => try_cast::<u8> => matches Err(_));
}

mod range_full {
    use super::*;

    test_ctor!(cast: RangeCast::<u32>::cast::<u64>(..) => ..);
    test_ctor!(try_cast: RangeCast::<u8>::try_cast::<u16>(..) => Ok(..));
}
