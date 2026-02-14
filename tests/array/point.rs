use tap::Pipe;

use crate::macros::{test_ctor, test_property, test_transform};

use grid_mask::{ArrayIndex, ArrayPoint, err::OutOfBounds};

type Point8 = ArrayPoint<8, 8>;
type Index8 = ArrayIndex<8, 8>;

const POINT_3_5: Point8 = Point8::const_new::<3, 5>();

mod new {
    use super::*;

    test_ctor!(valid_zero: Point8::new(0, 0) => Ok(Point8::ORIGIN));
    test_ctor!(valid_max: Point8::new(7, 7) => Ok(Point8::MAX));
    test_ctor!(oob_x: Point8::new(8, 0) => Err(OutOfBounds));
    test_ctor!(oob_y: Point8::new(0, 8) => Err(OutOfBounds));
    test_ctor!(oob_x_y: Point8::new(8, 8) => Err(OutOfBounds));
}

mod properties {
    use super::*;

    test_property!(x: POINT_3_5 => x() => 3);
    test_property!(y: POINT_3_5 => y() => 5);
}

mod into_index {
    use super::*;

    // Index from Point is row-major: y * W + x
    // Width = 8.
    // (0, 1) -> 1 * 8 + 0 = 8.
    // (1, 0) -> 0 * 8 + 1 = 1.

    test_transform!(from_0_1: Point8::const_new::<0,1>() => pipe(Index8::from) => ArrayIndex::<8,8>::const_new::<8>());
    test_transform!(from_1_0: Point8::const_new::<1,0>() => pipe(Index8::from) => ArrayIndex::<8,8>::const_new::<1>());

    test_transform!(from_origin: Point8::ORIGIN => pipe(Index8::from) => Index8::MIN);
    test_transform!(from_min: Point8::MIN => pipe(Index8::from) => Index8::MIN);
    test_transform!(from_max: Point8::MAX => pipe(Index8::from) => Index8::MAX);
}

mod into_tuple {
    use super::*;

    test_transform!(origin: Point8::ORIGIN => pipe(<(u16, u16)>::from) => (0u16, 0u16));
    test_transform!(max: Point8::MAX => pipe(<(u16, u16)>::from) => (7u16, 7u16));
}

mod try_from_tuple {
    use super::*;

    test_ctor!(valid: Point8::try_from((3, 5)) => Ok(POINT_3_5));
    test_ctor!(oob_x: Point8::try_from((8, 0)) => Err(OutOfBounds));
    test_ctor!(oob_y: Point8::try_from((0, 8)) => Err(OutOfBounds));
    test_ctor!(oob_x_y: Point8::try_from((8, 8)) => Err(OutOfBounds));

    test_ctor!(oob_x_fail_cast: Point8::try_from((u32::MAX, 0)) => Err(OutOfBounds));
    test_ctor!(oob_y_fail_cast: Point8::try_from((0, u32::MAX)) => Err(OutOfBounds));
    test_ctor!(oob_x_y_fail_cast: Point8::try_from((u32::MAX, u32::MAX)) => Err(OutOfBounds));
}

mod tuple_eq {
    use super::*;

    test_property!(eq_point: POINT_3_5 => eq(&(3u16, 5u16)) => true);
    test_property!(eq_origin: Point8::ORIGIN => eq(&(0u16, 0u16)) => true);
    test_property!(eq_max: Point8::MAX => eq(&(7u16, 7u16)) => true);
    test_property!(ne_point_x: POINT_3_5 => eq(&(3u16, 4u16)) => false);
    test_property!(ne_point_y: POINT_3_5 => eq(&(5u16, 3u16)) => false);
}
