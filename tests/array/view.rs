use crate::macros::{test_ctor, test_property, test_transform};

use grid_mask::err::OutOfBounds;
use grid_mask::{ArrayGrid, ArrayPoint, ArrayRect};

type Grid8 = ArrayGrid<8, 8, 1>;
type Point8 = ArrayPoint<8, 8>;
type Rect8 = ArrayRect<8, 8>;

const RECT_1_1_2_2: Rect8 = Rect8::const_new::<1, 1, 2, 2>();

const SAMPLE_GRID: Grid8 = {
    let mut grid = Grid8::EMPTY;
    grid.const_update(Point8::const_new::<1, 1>(), true);
    grid.const_update(Point8::const_new::<2, 1>(), true);
    grid.const_update(Point8::const_new::<2, 2>(), true);
    grid.const_update(Point8::const_new::<4, 4>(), true);
    grid
};

mod grid_view {
    use super::*;

    test_transform!(create_with_rect: Grid8::EMPTY => view(RECT_1_1_2_2) => matches Ok(_));
    test_transform!(create_with_tuple: Grid8::EMPTY => view(((1, 1), (2, 2))) => matches Ok(_));

    test_property!(rect: SAMPLE_GRID.view(RECT_1_1_2_2)? => rect() => RECT_1_1_2_2);
    test_property!(origin: SAMPLE_GRID.view(RECT_1_1_2_2)? => origin() => Point8::const_new::<1, 1>());

    test_property!(local_0_0: SAMPLE_GRID.view(RECT_1_1_2_2)? => get(0, 0) => Ok(true));
    test_property!(local_1_0: SAMPLE_GRID.view(RECT_1_1_2_2)? => get(1, 0) => Ok(true));
    test_property!(local_1_1: SAMPLE_GRID.view(RECT_1_1_2_2)? => get(1, 1) => Ok(true));
    test_property!(local_0_1: SAMPLE_GRID.view(RECT_1_1_2_2)? => get(0, 1) => Ok(false));
    test_property!(local_oob: SAMPLE_GRID.view(RECT_1_1_2_2)? => get(2, 0) => Err(OutOfBounds));

    test_ctor!(invalid_rect_w: Grid8::EMPTY.view(((7, 7), (2, 1))) => Err(OutOfBounds));
    test_ctor!(invalid_rect_h: Grid8::EMPTY.view(((7, 7), (1, 2))) => Err(OutOfBounds));
    test_ctor!(invalid_rect_zero_size: Grid8::EMPTY.view(((0, 0), (0, 1))) => Err(OutOfBounds));
}
