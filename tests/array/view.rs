use crate::macros::test_self_method;

use grid_mask::err::OutOfBounds;
use grid_mask::num::{Point, Size};
use grid_mask::{ArrayIndex, ArrayPoint, ArrayRect, array_grid};

type Grid8 = array_grid!(8, 8);
type Point8 = ArrayPoint<8, 8>;
type Index8 = ArrayIndex<8, 8>;
type Rect8 = ArrayRect<8, 8>;

const RECT_1_1_2_2: Rect8 = ArrayRect::const_new::<1, 1, 2, 2>();

const SAMPLE_GRID: Grid8 = {
    let mut grid = Grid8::EMPTY;
    // (1, 1) -> 9
    grid.const_set(Index8::const_new::<9>(), true);
    // (2, 1) -> 10
    grid.const_set(Index8::const_new::<10>(), true);
    // (2, 2) -> 18
    grid.const_set(Index8::const_new::<18>(), true);
    // (4, 4) -> 36
    grid.const_set(Index8::const_new::<36>(), true);
    grid
};

mod properties {
    use super::*;

    test_self_method!(rect: SAMPLE_GRID.view(RECT_1_1_2_2) => size() => Size::new(2, 2));

    test_self_method!(local_0_0: SAMPLE_GRID.view(RECT_1_1_2_2) => get(Point::new(0, 0)) => Ok(true));
    test_self_method!(local_1_0: SAMPLE_GRID.view(RECT_1_1_2_2) => get(Point::new(1, 0)) => Ok(true));
    test_self_method!(local_1_1: SAMPLE_GRID.view(RECT_1_1_2_2) => get(Point::new(1, 1)) => Ok(true));
    test_self_method!(local_0_1: SAMPLE_GRID.view(RECT_1_1_2_2) => get(Point::new(0, 1)) => Ok(false));
    test_self_method!(local_oob: SAMPLE_GRID.view(RECT_1_1_2_2) => get(Point::new(2, 0)) => Err(OutOfBounds));
}

mod mutation {
    use super::*;

    #[test]
    fn update_local() {
        let mut grid = Grid8::EMPTY;
        let mut view = grid.view_mut(RECT_1_1_2_2);

        // Update local (0, 0) -> global (1, 1)
        assert_eq!(view.set(Point::new(0, 0), true), Ok(()));
        assert_eq!(view.get(Point::new(0, 0)), Ok(true));
    }

    #[test]
    fn update_local_propagates() {
        let mut grid = Grid8::EMPTY;
        {
            let mut view = grid.view_mut(RECT_1_1_2_2);
            assert_eq!(view.set(Point::new(1, 1), true), Ok(()));
        }
        // Global (1, 1) + (1, 1) = (2, 2)
        assert!(grid.get(Point8::const_new::<2, 2>()));
    }

    #[test]
    fn update_oob() {
        let mut grid = Grid8::EMPTY;
        let mut view = grid.view_mut(RECT_1_1_2_2);
        assert_eq!(view.set(Point::new(2, 0), true), Err(OutOfBounds));
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
