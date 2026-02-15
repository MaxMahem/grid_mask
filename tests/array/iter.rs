use crate::macros::{test_ctor, test_property};

use grid_mask::{ArrayGrid, ArrayPoint};

type Grid8 = ArrayGrid<8, 8, 1>;
type Point8 = ArrayPoint<8, 8>;

mod cells {
    use super::*;

    const EMPTY_CELLS: [bool; 64] = [false; 64];
    const FULL_CELLS: [bool; 64] = [true; 64];

    test_property!(empty: Grid8::EMPTY.cells() => collect::<Vec<_>>() => EMPTY_CELLS);
    test_property!(full: Grid8::FULL.cells() => collect::<Vec<_>>() => FULL_CELLS);

    test_property!(empty_rev: Grid8::EMPTY.cells().rev() => collect::<Vec<_>>() => EMPTY_CELLS);
    test_property!(full_rev: Grid8::FULL.cells().rev() => collect::<Vec<_>>() => FULL_CELLS);

    test_property!(size_hint: Grid8::EMPTY.cells() => size_hint() => (64, Some(64)));
}

const P1: Point8 = Point8::const_new::<0, 1>();
const P2: Point8 = Point8::const_new::<0, 3>();

const GRID8_MIXED: Grid8 = {
    let mut grid = Grid8::EMPTY;
    grid.const_update(P1, true);
    grid.const_update(P2, true);
    grid
};

mod points {
    use super::*;

    test_property!(empty: Grid8::EMPTY.points() => collect::<Vec<_>>() => Vec::<Point8>::new());
    test_property!(mixed: GRID8_MIXED.points() => collect::<Vec<_>>() => [P1, P2]);
    test_property!(mixed_rev: GRID8_MIXED.points().rev() => collect::<Vec<_>>() => [P2, P1]);
    test_property!(iter: GRID8_MIXED.iter() => collect::<Vec<_>>() => [P1, P2]);
    test_ctor!(into_iter: GRID8_MIXED.into_iter().collect::<Vec<_>>() => [P1, P2]);
}

mod spaces {
    use super::*;

    const GRID8_SPARSE: Grid8 = {
        let mut grid = Grid8::FULL;
        grid.const_update(P1, false);
        grid.const_update(P2, false);
        grid
    };

    test_property!(empty: Grid8::FULL.spaces() => collect::<Vec<_>>() => Vec::<Point8>::new());
    test_property!(sparse: GRID8_SPARSE.spaces() => collect::<Vec<_>>() => [P1, P2]);
    test_property!(sparse_rev: GRID8_SPARSE.spaces().rev() => collect::<Vec<_>>() => [P2, P1]);
}
