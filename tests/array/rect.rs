use crate::macros::{test_ctor, test_self_method, test_transform};

use grid_mask::err::OutOfBounds;
use grid_mask::{ArrayPoint, ArrayRect, ArraySize};

type Point8 = ArrayPoint<8, 8>;
type Size8 = ArraySize<8, 8>;
type Rect8 = ArrayRect<8, 8>;

const POINT_1_2: Point8 = Point8::const_new::<1, 2>();
const SIZE_3_4: Size8 = Size8::const_new::<3, 4>();
const RECT_1_2_3_4: Rect8 = Rect8::const_new::<1, 2, 3, 4>();

mod new {
    use super::*;

    test_ctor!(ok: Rect8::new((1, 2), (3, 4)) => Ok(RECT_1_2_3_4));
    test_ctor!(ok_edge: Rect8::new((7, 7), (1, 1)) => Ok(Rect8::const_new::<7, 7, 1, 1>()));

    test_ctor!(err_width: Rect8::new((7, 7), (2, 1)) => Err(OutOfBounds));
    test_ctor!(err_height: Rect8::new((7, 7), (1, 2)) => Err(OutOfBounds));
    test_ctor!(err_zero_size: Rect8::new((0, 0), (0, 1)) => Err(OutOfBounds));
}

mod const_new {
    use super::*;

    test_ctor!(ok: Rect8::const_new::<1, 2, 3, 4>() => RECT_1_2_3_4);

    // These tests fail compilation due to const_assert_then in const_new
    // test_panic!(panic_zero_w: Rect8::const_new::<1, 2, 0, 1>() => "size must be non-zero");
    // test_panic!(panic_zero_h: Rect8::const_new::<1, 2, 1, 0>() => "size must be non-zero");
    // test_panic!(panic_oob_point: Rect8::const_new::<8, 0, 1, 1>() => "point out of bounds");
    // test_panic!(panic_oob_rect_w: Rect8::const_new::<7, 7, 2, 1>() => "rectangle extends beyond grid");
    // test_panic!(panic_oob_rect_h: Rect8::const_new::<7, 7, 1, 2>() => "rectangle extends beyond grid");
}

mod properties {
    use super::*;

    test_self_method!(point: RECT_1_2_3_4 => point() => POINT_1_2);
    test_self_method!(size: RECT_1_2_3_4 => size() => SIZE_3_4);

    test_self_method!(contains_origin: RECT_1_2_3_4 => contains(Point8::const_new::<1, 2>()) => true);
    test_self_method!(contains_bottom_right: RECT_1_2_3_4 => contains(Point8::const_new::<3, 5>()) => true);
    test_self_method!(contains_left_out: RECT_1_2_3_4 => contains(Point8::const_new::<0, 2>()) => false);
    test_self_method!(contains_right_out: RECT_1_2_3_4 => contains(Point8::const_new::<4, 2>()) => false);
    test_self_method!(contains_bottom_out: RECT_1_2_3_4 => contains(Point8::const_new::<1, 6>()) => false);
}

mod conversions {
    use super::*;

    test_ctor!(try_from_ok: Rect8::try_from(((1, 2), (3, 4))) => Ok(RECT_1_2_3_4));
    test_ctor!(try_from_err: Rect8::try_from(((7, 7), (2, 1))) => Err(OutOfBounds));

    test_transform!(display: RECT_1_2_3_4 => to_string() => "(1, 2) (3x4)");
}
