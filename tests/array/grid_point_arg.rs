use crate::macros::test_property;
use grid_mask::err::OutOfBounds;
use grid_mask::num::Point;
use grid_mask::{ArrayGrid, ArrayGridPointArg, ArrayIndex, ArrayPoint};

type Grid8 = ArrayGrid<8, 8, 1>;
type Point8 = ArrayPoint<8, 8>;
type Index8 = ArrayIndex<8, 8>;

mod adapter_get {
    use super::*;

    test_property!(infallible_point_get: Grid8::FULL => get(Point8::ORIGIN) => true);
    test_property!(infallible_index_get: Grid8::FULL => get(Index8::MIN) => true);
    test_property!(fallible_tuple_get_ok: Grid8::FULL => get((0u32, 0u32)) => Ok(true));
    test_property!(fallible_tuple_get_err: Grid8::FULL => get((u32::MAX, 0u32)) => Err(OutOfBounds));
    test_property!(fallible_point_get_ok: Grid8::FULL => get(Point::new(0u32, 0u32)) => Ok(true));
    test_property!(fallible_point_get_err: Grid8::FULL => get(Point::new(8u32, 0u32)) => Err(OutOfBounds));

    #[test]
    fn trait_get_dispatches_correctly() {
        let grid = Grid8::FULL;

        let point_get = <Point8 as ArrayGridPointArg<8, 8>>::get(Point8::ORIGIN, &grid);
        let index_get = <Index8 as ArrayGridPointArg<8, 8>>::get(Index8::MIN, &grid);
        let tuple_get = <(u32, u32) as ArrayGridPointArg<8, 8>>::get((0, 0), &grid);
        let num_point_get = <Point<u32, u32> as ArrayGridPointArg<8, 8>>::get(Point::new(0, 0), &grid);

        assert!(point_get);
        assert!(index_get);
        assert_eq!(tuple_get, Ok(true));
        assert_eq!(num_point_get, Ok(true));
    }
}

mod adapter_set {
    use super::*;

    #[test]
    fn trait_set_dispatches_correctly() {
        let mut grid = Grid8::EMPTY;

        <Point8 as ArrayGridPointArg<8, 8>>::set(Point8::ORIGIN, &mut grid, true);
        assert!(grid.get(Point8::ORIGIN));

        <Index8 as ArrayGridPointArg<8, 8>>::set(Index8::MIN, &mut grid, false);
        assert!(!grid.get(Index8::MIN));

        let tuple_set = <(u32, u32) as ArrayGridPointArg<8, 8>>::set((0, 0), &mut grid, true);
        assert_eq!(tuple_set, Ok(()));
        assert_eq!(grid.get((0u16, 0u16)), Ok(true));

        let num_point_set = <Point<u32, u32> as ArrayGridPointArg<8, 8>>::set(Point::new(u32::MAX, 0), &mut grid, true);
        assert_eq!(num_point_set, Err(OutOfBounds));
    }
}
