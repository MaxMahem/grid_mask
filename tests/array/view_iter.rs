use crate::macros::test_property;

use grid_mask::array::{ArrayGrid, ArrayPoint, ArrayRect};

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

// View of RECT_1_1_2_2 at (1, 1) size 2x2 covers:
// (1, 1) -> local (0, 0): true
// (2, 1) -> local (1, 0): true
// (1, 2) -> local (0, 1): unset (false)
// (2, 2) -> local (1, 1): true

#[test]
fn cells() {
    let view = SAMPLE_GRID.view(RECT_1_1_2_2);
    let cells: Vec<_> = view.cells().collect();
    // Order is row-major: (0,0), (1,0), (0,1), (1,1)
    assert_eq!(cells, vec![true, true, false, true]);
}

#[test]
fn points() {
    let view = SAMPLE_GRID.view(RECT_1_1_2_2);
    let points: Vec<_> = view.points().collect();
    assert_eq!(points, vec![(0, 0), (1, 0), (1, 1)]);
}

#[test]
fn spaces() {
    let view = SAMPLE_GRID.view(RECT_1_1_2_2);
    let spaces: Vec<_> = view.spaces().collect();
    assert_eq!(spaces, vec![(0, 1)]);
}
