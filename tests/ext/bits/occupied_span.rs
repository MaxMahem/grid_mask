use crate::macros::test_self_method;

use grid_mask::ext::bits::OccupiedBitSpan;

mod u64 {
    use super::*;

    test_self_method!(zero: 0u64 => occupied_span() => 0..0);
    test_self_method!(max: u64::MAX => occupied_span() => 0..64);
    test_self_method!(middle: 0x00F0_0F00u64 => occupied_span() => 8..24);
}

mod u8 {
    use super::*;

    test_self_method!(zero: 0u8 => occupied_span() => 0..0);
    test_self_method!(max: u8::MAX => occupied_span() => 0..8);
    test_self_method!(middle: 0b0100_0010u8 => occupied_span() => 1..7);
}
