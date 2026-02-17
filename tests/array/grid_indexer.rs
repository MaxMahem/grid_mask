use crate::macros::test_self_method;

use grid_mask::err::OutOfBounds;
use grid_mask::num::Point;
use grid_mask::{ArrayIndex, ArrayPoint, GridGetIndex, GridSetIndex};

type Grid8 = grid_mask::array_grid!(8, 8);
type Point8 = ArrayPoint<8, 8>;
type Index8 = ArrayIndex<8, 8>;

const GRID_0_0: Grid8 = {
    let mut grid = Grid8::EMPTY;
    grid.const_set(ArrayIndex::MIN, true);
    grid
};

mod get {
    use super::*;

    test_self_method!(array_point: Point8::ORIGIN => get(&Grid8::FULL) => true);
    test_self_method!(array_index: this = Index8::MIN => GridGetIndex::get(this, &Grid8::FULL) => true);
    test_self_method!(tuple: (0u32, 0u32) => get(&Grid8::FULL) => Ok(true));
    test_self_method!(tuple_err: (u32::MAX, 0u32) => get(&Grid8::FULL) => Err(OutOfBounds));
    test_self_method!(point: Point::new(0u32, 0u32) => get(&Grid8::FULL) => Ok(true));
    test_self_method!(point_err: Point::new(8u32, 0u32) => get(&Grid8::FULL) => Err(OutOfBounds));
    test_self_method!(index_u32: 0u32 => get(&Grid8::FULL) => Ok(true));
    test_self_method!(index_u32_err: u32::MAX => get(&Grid8::FULL) => Err(OutOfBounds));
    test_self_method!(index_usize: 0usize => get(&Grid8::FULL) => Ok(true));
    test_self_method!(index_usize_err: usize::MAX => get(&Grid8::FULL) => Err(OutOfBounds));
}

mod set {
    use super::*;

    macro_rules! test_set {
        ($id:ident: $ctor:expr => $result:expr, $expected:expr) => {
            #[test]
            #[allow(unused_variables)]
            fn $id() -> Result<(), Box<dyn std::error::Error>> {
                let this = $ctor;
                let mut grid = Grid8::EMPTY;
                let result = GridSetIndex::set(this, &mut grid, true);
                assert_eq!(result, $result);
                assert_eq!(grid, $expected);
                Ok(())
            }
        };
    }

    test_set!(array_point: Point8::ORIGIN => (), GRID_0_0);
    test_set!(array_index: Index8::MIN => (), GRID_0_0);
    test_set!(tuple: (0u32, 0u32) => Ok(()), GRID_0_0);
    test_set!(tuple_err: (u32::MAX, 0u32) => Err(OutOfBounds), Grid8::EMPTY);
    test_set!(point: Point::new(0u32, 0u32) => Ok(()), GRID_0_0);
    test_set!(point_err: Point::new(u32::MAX, 0u32) => Err(OutOfBounds), Grid8::EMPTY);
    test_set!(index_usize: 0usize => Ok(()), GRID_0_0);
    test_set!(index_usize_err: usize::MAX => Err(OutOfBounds), Grid8::EMPTY);
}
