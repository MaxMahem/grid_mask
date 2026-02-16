use crate::macros::{test_ctor, test_property};

use grid_mask::err::OutOfBounds;
use grid_mask::{ArrayGrid, ArrayPoint, ArrayRect};

type Grid8 = ArrayGrid<8, 8, 1>;
type Point8 = ArrayPoint<8, 8>;
type Rect8 = ArrayRect<8, 8>;

const RECT_1_1_2_2: Rect8 = Rect8::const_new::<1, 1, 2, 2>();

const SAMPLE_GRID: Grid8 = {
    let mut grid = Grid8::EMPTY;
    grid.const_set(Point8::const_new::<1, 1>(), true);
    grid.const_set(Point8::const_new::<2, 1>(), true);
    grid.const_set(Point8::const_new::<2, 2>(), true);
    grid.const_set(Point8::const_new::<4, 4>(), true);
    grid
};

mod properties {
    use super::*;

    test_ctor!(create_with_tuple: Rect8::try_from(((1, 1), (2, 2))) => Ok(RECT_1_1_2_2));

    test_property!(rect: SAMPLE_GRID.view(RECT_1_1_2_2) => rect() => RECT_1_1_2_2);
    test_property!(origin: SAMPLE_GRID.view(RECT_1_1_2_2) => origin() => Point8::const_new::<1, 1>());

    test_property!(local_0_0: SAMPLE_GRID.view(RECT_1_1_2_2) => get(0, 0) => Ok(true));
    test_property!(local_1_0: SAMPLE_GRID.view(RECT_1_1_2_2) => get(1, 0) => Ok(true));
    test_property!(local_1_1: SAMPLE_GRID.view(RECT_1_1_2_2) => get(1, 1) => Ok(true));
    test_property!(local_0_1: SAMPLE_GRID.view(RECT_1_1_2_2) => get(0, 1) => Ok(false));
    test_property!(local_oob: SAMPLE_GRID.view(RECT_1_1_2_2) => get(2, 0) => Err(OutOfBounds));

    test_ctor!(invalid_rect_w: Rect8::new((7, 7), (2, 1)) => Err(OutOfBounds));
    test_ctor!(invalid_rect_h: Rect8::new((7, 7), (1, 2)) => Err(OutOfBounds));
    test_ctor!(invalid_rect_zero_size: Rect8::new((0, 0), (0, 1)) => Err(OutOfBounds));
}

mod mutation {
    use super::*;

    #[test]
    fn update_local() {
        let mut grid = Grid8::EMPTY;
        let mut view = grid.view_mut(RECT_1_1_2_2);

        // Update local (0, 0) -> global (1, 1)
        assert_eq!(view.set(0, 0, true), Ok(()));
        assert_eq!(view.get(0, 0), Ok(true));
    }

    #[test]
    fn update_local_propagates() {
        let mut grid = Grid8::EMPTY;
        {
            let mut view = grid.view_mut(RECT_1_1_2_2);
            assert_eq!(view.set(1, 1, true), Ok(()));
        }
        // Global (1, 1) + (1, 1) = (2, 2)
        assert_eq!(grid.get(Point8::const_new::<2, 2>()), true);
    }

    #[test]
    fn update_oob() {
        let mut grid = Grid8::EMPTY;
        let mut view = grid.view_mut(RECT_1_1_2_2);
        assert_eq!(view.set(2, 0, true), Err(OutOfBounds));
    }
}

mod iter {
    use crate::macros::test_ctor;

    use super::*;

    test_ctor!(
        cells: SAMPLE_GRID.view(RECT_1_1_2_2).cells().collect::<Vec<_>>()
        => [true, true, false, true]
    );
    test_ctor!(
        points: SAMPLE_GRID.view(RECT_1_1_2_2).points().collect::<Vec<_>>()
        => [(0, 0), (1, 0), (1, 1)]
    );
    test_ctor!(
        spaces: SAMPLE_GRID.view(RECT_1_1_2_2).spaces().collect::<Vec<_>>()
        => [(0, 1)]
    );
}
