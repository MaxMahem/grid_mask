use crate::macros::{test_ctor, test_panic, test_property, test_transform};
use tap::Pipe;

use grid_mask::ArraySize;
use grid_mask::err::OutOfBounds;

type Size8 = ArraySize<8, 8>;
const SIZE_3_5: Size8 = Size8::const_new::<3, 5>();

mod consts {
    use super::*;

    test_ctor!(min: Size8::MIN => Size8 { width: 1, height: 1 });
    test_ctor!(max: Size8::MAX => Size8 { width: 8, height: 8 });
}

mod new {
    use super::*;

    test_ctor!(ok_min: Size8::new(1, 1) => Ok(Size8::MIN));
    test_ctor!(ok_max: Size8::new(8, 8) => Ok(Size8::MAX));
    test_ctor!(ok_mid: Size8::new(3, 5) => Ok(SIZE_3_5));

    test_ctor!(err_zero_w: Size8::new(0, 1) => Err(OutOfBounds));
    test_ctor!(err_zero_h: Size8::new(1, 0) => Err(OutOfBounds));
    test_ctor!(err_big_w: Size8::new(9, 1) => Err(OutOfBounds));
    test_ctor!(err_big_h: Size8::new(1, 9) => Err(OutOfBounds));
}

mod const_new {
    use super::*;

    test_ctor!(ok: Size8::const_new::<3, 5>() => SIZE_3_5);

    test_panic!(panic_zero_w: Size8::const_new::<0, 1>() => "width out of bounds");
    test_panic!(panic_zero_h: Size8::const_new::<1, 0>() => "height out of bounds");
    test_panic!(panic_big_w: Size8::const_new::<9, 1>() => "width out of bounds");
    test_panic!(panic_big_h: Size8::const_new::<1, 9>() => "height out of bounds");
}

mod properties {
    use super::*;

    test_property!(width: SIZE_3_5 => width() => 3);
    test_property!(height: SIZE_3_5 => height() => 5);
}

mod conversions {
    use super::*;

    test_ctor!(try_from_tuple_ok: Size8::try_from((3, 5)) => Ok(SIZE_3_5));
    test_ctor!(try_from_tuple_err: Size8::try_from((0, 5)) => Err(OutOfBounds));

    test_transform!(into_tuple: SIZE_3_5 => pipe(<(u16, u16)>::from) => (3u16, 5u16));
}
