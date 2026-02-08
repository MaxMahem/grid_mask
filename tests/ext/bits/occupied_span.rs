use crate::macros::test_property;

use grid_mask::ext::bits::OccupiedBitSpan;

mod u64 {
    use super::*;

    test_property!(zero: 0u64 => occupied_span() => 0..0);
    test_property!(max: u64::MAX => occupied_span() => 0..64);
    test_property!(middle: 0x00F0_0F00u64 => occupied_span() => 8..24);
}

mod u8 {
    use super::*;

    test_property!(zero: 0u8 => occupied_span() => 0..0);
    test_property!(max: u8::MAX => occupied_span() => 0..8);
    test_property!(middle: 0b0100_0010u8 => occupied_span() => 1..7);
}
