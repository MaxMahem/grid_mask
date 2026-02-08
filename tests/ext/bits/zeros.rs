use crate::macros::test_property;

use grid_mask::ext::bits::BitZeros;

mod u64 {
    use super::*;

    mod trailing {
        use super::*;

        test_property!(zero: 0u64 => trailing_zeros_u8() => 64);
        test_property!(one: 1u64 => trailing_zeros_u8() => 0);
        test_property!(two: 2u64 => trailing_zeros_u8() => 1);
        test_property!(max: 0x8000_0000_0000_0000u64 => trailing_zeros_u8() => 63);
    }

    mod leading {
        use super::*;

        test_property!(zero: 0u64 => leading_zeros_u8() => 64);
        test_property!(one: 1u64 => leading_zeros_u8() => 63);
        test_property!(max: 0x8000_0000_0000_0000u64 => leading_zeros_u8() => 0);
    }
}

mod u8 {
    use super::*;

    mod trailing {
        use super::*;

        test_property!(zero: 0u8 => trailing_zeros_u8() => 8);
        test_property!(one: 1u8 => trailing_zeros_u8() => 0);
        test_property!(two: 2u8 => trailing_zeros_u8() => 1);
        test_property!(max: 0x80u8 => trailing_zeros_u8() => 7);
    }

    mod leading {
        use super::*;

        test_property!(zero: 0u8 => leading_zeros_u8() => 8);
        test_property!(one: 1u8 => leading_zeros_u8() => 7);
        test_property!(max: 0x80u8 => leading_zeros_u8() => 0);
    }
}
