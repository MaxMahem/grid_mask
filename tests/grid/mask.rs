use grid_mask::{Cardinal, GridMask, GridPoint, GridVector, Octile};
use std::str::FromStr;

use crate::macros::{test_ctor, test_foreach, test_iter, test_panic, test_property, test_transform};

test_ctor!(grid_mask_new: GridMask::new(12345).0 => 12345);

const fn mask_from_coords(x: u8, y: u8) -> GridMask {
    assert!(x < 8);
    assert!(y < 8);
    GridMask::new(1u64 << (x + y * 8))
}

const fn mask_from_point(point: GridPoint) -> GridMask {
    GridMask::new(1u64 << point.0.get())
}

const POINT_4_4: GridPoint = GridPoint::const_new::<4, 4>();
const MASK_4_4: GridMask = mask_from_point(POINT_4_4);

const ORIGIN_POINT_MASK: GridMask = mask_from_point(GridPoint::ORIGIN);
const MAX_POINT_MASK: GridMask = mask_from_point(GridPoint::MAX);

mod pattern_data {
    use super::*;
    use grid_mask::err::PatternError;

    pub const CHECKERBOARD: &str = "
        # . # . # . # .
        . # . # . # . #
        # . # . # . # .
        . # . # . # . #
        # . # . # . # .
        . # . # . # . #
        # . # . # . # .
        . # . # . # . #
    ";

    pub const SPIRAL: &str = "
        # # # # # # # #
        . . . . . . . #
        # # # # # # . #
        # . . . . # . #
        # . # . . # . #
        # . # # # # . #
        # . . . . . . #
        # # # # # # # #
    ";

    pub const CROSS: &str = "
        . . . . # . . .
        . . . . # . . .
        . . . . # . . .
        . . . . # . . .
        # # # # # # # #
        . . . . # . . .
        . . . . # . . .
        . . . . # . . .
    ";

    pub const DISCONNECTED_MASK: GridMask = GridMask::new(1 | (1 << 63));

    pub const TOO_LONG: &str = ".................................................................";

    pub const TOO_SHORT: &str = "...............................................................";
    pub const PATTERN_TOO_SHORT: PatternError = PatternError::TooShort(63);

    pub const INVALID: &str = "...............................................................?";
    pub const PATTERN_INVALID: PatternError = PatternError::InvalidChar('?');

    pub const EVEN_ROWS_COLS: &str = "
        # . # . # . # .
        . . . . . . . .
        # . # . # . # .
        . . . . . . . .
        # . # . # . # .
        . . . . . . . .
        # . # . # . # .
        . . . . . . . .
    ";
}

mod set_unset {
    use super::*;

    test_transform!(set: GridMask::EMPTY => set(POINT_4_4) => MASK_4_4);
    test_transform!(unset: MASK_4_4 => unset(POINT_4_4) => GridMask::EMPTY);
}

mod index {
    use super::*;

    test_property!(empty: GridMask::EMPTY => index(POINT_4_4) => false);
    test_property!(set: GridMask::new(1u64 << 36) => index(POINT_4_4) => true);
}

mod count {
    use super::*;

    test_property!(empty: GridMask::EMPTY => count() => 0);
    test_property!(set: MASK_4_4 => count() => 1);
    test_property!(full: GridMask::FULL => count() => 64);
}

mod empty_full {
    use super::*;

    test_property!(empty_is_empty: GridMask::EMPTY => is_empty() => true);
    test_property!(empty_is_not_full: GridMask::EMPTY => is_full() => false);
    test_property!(full_is_not_empty: GridMask::FULL => is_empty() => false);
    test_property!(full_is_full: GridMask::FULL => is_full() => true);
    test_property!(mixed_is_not_empty: MASK_4_4 => is_empty() => false);
    test_property!(mixed_is_not_full: MASK_4_4 => is_full() => false);
}

mod cell_arrays {
    use super::*;

    pub const MIXED_MASK: GridMask = GridMask::new(2 | (1 << 10) | (1 << 63));

    pub const MIXED_CELLS: [bool; 64] = {
        let mut v = [false; 64];
        v[1] = true;
        v[10] = true;
        v[63] = true;
        v
    };

    pub const FULL_CELLS: [bool; 64] = [true; 64];
    pub const EMPTY_CELLS: [bool; 64] = [false; 64];
}

mod cells {
    use super::cell_arrays::*;
    use super::*;

    test_iter!(empty: GridMask::EMPTY => cells() => EMPTY_CELLS);
    test_iter!(full: GridMask::FULL => cells() => FULL_CELLS);
    test_iter!(mixed: MIXED_MASK => cells() => MIXED_CELLS);

    #[test]
    fn test_double_ended() {
        let mask = GridMask::new(1 | 1 << 63);
        let mut cells = mask.cells();
        assert_eq!(cells.next(), Some(true));
        assert_eq!(cells.next_back(), Some(true));
        assert_eq!(cells.next(), Some(false));
    }
}

mod points {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(GridMask::EMPTY.points().count(), 0);
    }

    #[test]
    fn full() {
        assert_eq!(GridMask::FULL.points().count(), 64);
    }

    #[test]
    fn mixed() {
        let mask = GridMask::new(1 | 1 << 36 | 1 << 63);
        let points: Vec<_> = mask.points().collect();
        assert_eq!(points, vec![GridPoint::ORIGIN, POINT_4_4, GridPoint::MAX]);
    }

    #[test]
    fn double_ended() {
        let mask = GridMask::new(1 | 1 << 63);
        let mut points = mask.points();
        assert_eq!(points.next(), Some(GridPoint::ORIGIN));
        assert_eq!(points.next_back(), Some(GridPoint::MAX));
        assert_eq!(points.next(), None);
    }
}

mod from_bool_array {
    use super::cell_arrays::*;
    use super::*;

    test_ctor!(empty: GridMask::from(EMPTY_CELLS) => GridMask::EMPTY);
    test_ctor!(full: GridMask::from(FULL_CELLS) => GridMask::FULL);
    test_ctor!(mixed: GridMask::from(MIXED_CELLS) => MIXED_MASK);
}

mod from_bit_index_u64 {
    use super::*;
    use grid_mask::num::BitIndexU64;

    test_ctor!(zero: GridMask::from(BitIndexU64::new(0).unwrap()) => ORIGIN_POINT_MASK);
    test_ctor!(max: GridMask::from(BitIndexU64::new(63).unwrap()) => MAX_POINT_MASK);
    test_ctor!(val: GridMask::from(BitIndexU64::new(36).unwrap()) => GridMask::new(1 << 36));
}

mod from_grid_point {
    use super::*;

    test_ctor!(zero: GridMask::from(GridPoint::ORIGIN) => GridMask::new(1));
    test_ctor!(max: GridMask::from(GridPoint::MAX) => GridMask::new(1 << 63));
    test_ctor!(val: GridMask::from(POINT_4_4) => MASK_4_4);
}

mod from_grid_rect {
    use super::*;
    use grid_mask::GridRect;

    test_ctor!(single_point: GridMask::from(GridRect::const_new::<4, 4, 1, 1>()) => MASK_4_4);
    test_ctor!(full_rect: GridMask::from(GridRect::const_new::<0, 0, 8, 8>()) => GridMask::FULL);
}

const POINT_4_4_MASK: GridMask = GridMask::new(1u64 << 36);

const PLUS_4_4: &str = "
    . . . . . . . .
    . . . . . . . .
    . . . . . . . .
    . . . . # . . .
    . . . # # # . .
    . . . . # . . .
    . . . . . . . .
    . . . . . . . .
";

const POINT_4_4_PATTERN: &str = "
    . . . . . . . .
    . . . . . . . .
    . . . . . . . .
    . . . . . . . .
    . . . . # . . .
    . . . . . . . .
    . . . . . . . .
    . . . . . . . .
";

const SQUARE_4_4: &str = "
    . . . . . . . .
    . . . . . . . .
    . . . . . . . .
    . . . # # # . .
    . . . # # # . .
    . . . # # # . .
    . . . . . . . .
    . . . . . . . .
";

const ZERO_POINT_PLUS: &str = "
    # # . . . . . .
    # . . . . . . .
    . . . . . . . .
    . . . . . . . .
    . . . . . . . .
    . . . . . . . .
    . . . . . . . .
    . . . . . . . .
";

const ZERO_POINT_SQUARE: &str = "
    # # . . . . . .
    # # . . . . . .
    . . . . . . . .
    . . . . . . . .
    . . . . . . . .
    . . . . . . . .
    . . . . . . . .
    . . . . . . . .
";

const SPARSE_CORNERS: &str = "
    . . # . . # . .
    . . . . . . . .
    . . . . . . . .
    . . # . . # . .
    . . . . . . . .
    . . . . . . . .
    . . . . . . . .
    . . . . . . . .
";

mod grow {

    macro_rules! test_grow {
        ($direction:ty> $name:ident: $mask:expr => $expected:expr) => {
            test_property!($name: $mask => grow::<$direction>() => $expected);
        };
    }

    mod cardinal {
        use super::super::*;

        test_grow!(Cardinal> empty: GridMask::EMPTY => GridMask::EMPTY);
        test_grow!(Cardinal> full: GridMask::FULL => GridMask::FULL);
        test_grow!(Cardinal> center: POINT_4_4_MASK => GridMask::from_str(PLUS_4_4)?);
        test_grow!(Cardinal> top_left: ORIGIN_POINT_MASK => GridMask::from_str(ZERO_POINT_PLUS)?);
    }

    mod octile {
        use super::super::*;

        test_grow!(Octile> empty: GridMask::EMPTY => GridMask::EMPTY);
        test_grow!(Octile> full: GridMask::FULL => GridMask::FULL);
        test_grow!(Octile> center: POINT_4_4_MASK => GridMask::from_str(SQUARE_4_4)?);
        test_grow!(Octile> top_left: ORIGIN_POINT_MASK => GridMask::from_str(ZERO_POINT_SQUARE)?);
    }
}

mod connected {

    mod cardinal {
        use super::super::cell_arrays::*;
        use super::super::pattern_data::*;
        use super::super::*;

        test_property!(empty: GridMask::EMPTY => connected::<Cardinal>(GridPoint::ORIGIN) => GridMask::EMPTY);
        test_property!(single_point: ORIGIN_POINT_MASK => connected::<Cardinal>(GridPoint::ORIGIN) => ORIGIN_POINT_MASK);
        test_property!(full: GridMask::FULL => connected::<Cardinal>(GridPoint::ORIGIN) => GridMask::FULL);
        test_property!(empty_cell: MIXED_MASK => connected::<Cardinal>(GridPoint::ORIGIN) => GridMask::EMPTY);

        test_property!(
            spiral: GridMask::from_str(SPIRAL)?
            => connected::<Cardinal>(GridPoint::ORIGIN)
            => GridMask::from_str(SPIRAL)?
        );

        test_property!(
            cross: GridMask::from_str(CROSS)?
            => connected::<Cardinal>(POINT_4_4)
            => GridMask::from_str(CROSS)?
        );

        test_property!(
            disjoint: DISCONNECTED_MASK
            => connected::<Cardinal>(GridPoint::ORIGIN)
            => ORIGIN_POINT_MASK
        );

        test_property!(
            checkerboard: GridMask::from_str(CHECKERBOARD)?
            => connected::<Cardinal>(GridPoint::ORIGIN)
            => GridMask::from(GridPoint::ORIGIN)
        );
    }

    mod octile {
        use super::super::cell_arrays::*;
        use super::super::pattern_data::*;
        use super::super::*;

        test_property!(empty: GridMask::EMPTY => connected::<Octile>(GridPoint::ORIGIN) => GridMask::EMPTY);
        test_property!(full: GridMask::FULL => connected::<Octile>(GridPoint::ORIGIN) => GridMask::FULL);
        test_property!(empty_cell: MIXED_MASK => connected::<Octile>(GridPoint::ORIGIN) => GridMask::EMPTY);

        test_property!(
            spiral: GridMask::from_pattern(SPIRAL, '#', '.')?
            => connected::<Octile>(GridPoint::ORIGIN)
            => GridMask::from_pattern(SPIRAL, '#', '.')?
        );

        test_property!(
            cross: GridMask::from_str(CROSS)?
            => connected::<Octile>(POINT_4_4)
            => GridMask::from_str(CROSS)?
        );

        test_property!(
            disjoint: DISCONNECTED_MASK
            => connected::<Octile>(GridPoint::ORIGIN)
            => ORIGIN_POINT_MASK
        );

        test_property!(
            checkerboard: GridMask::from_pattern(CHECKERBOARD, '#', '.')?
            => connected::<Octile>(GridPoint::ORIGIN)
            => GridMask::from_pattern(CHECKERBOARD, '#', '.')?
        );
    }
}

mod is_contiguous {
    macro_rules! test_is_contiguous {
        ($direction:ty> $name:ident: $mask:expr => $expected:expr) => {
            test_property!($name: $mask => is_contiguous::<$direction>() => $expected);
        };
    }

    mod cardinal {
        use super::super::pattern_data::*;
        use super::super::*;

        test_is_contiguous!(Cardinal> empty: GridMask::EMPTY => false);
        test_is_contiguous!(Cardinal> full: GridMask::FULL => true);
        test_is_contiguous!(Cardinal> spiral: GridMask::from_str(SPIRAL)? => true);
        test_is_contiguous!(Cardinal> cross: GridMask::from_str(CROSS)? => true);
        test_is_contiguous!(Cardinal> disjoint: DISCONNECTED_MASK => false);
        test_is_contiguous!(Cardinal> checkerboard: GridMask::from_str(CHECKERBOARD)? => false);
    }

    mod octile {
        use super::super::pattern_data::*;
        use super::super::*;

        test_is_contiguous!(Octile> empty: GridMask::EMPTY => false);
        test_is_contiguous!(Octile> full: GridMask::FULL => true);
        test_is_contiguous!(Octile> spiral: GridMask::from_str(SPIRAL)? => true);
        test_is_contiguous!(Octile> cross: GridMask::from_str(CROSS)? => true);
        test_is_contiguous!(Octile> disjoint: DISCONNECTED_MASK => false);
        test_is_contiguous!(Octile> checkerboard: GridMask::from_str(CHECKERBOARD)? => true);
    }
}

mod translate {
    use super::*;

    test_transform!(identity: MASK_4_4 => translate(GridVector::ZERO) => MASK_4_4);

    test_transform!(east: MASK_4_4 => translate(GridVector::EAST) => mask_from_coords(5, 4));
    test_transform!(west: MASK_4_4 => translate(GridVector::WEST) => mask_from_coords(3, 4));
    test_transform!(south: MASK_4_4 => translate(GridVector::SOUTH) => mask_from_coords(4, 5));
    test_transform!(north: MASK_4_4 => translate(GridVector::NORTH) => mask_from_coords(4, 3));

    test_transform!(wrap_prevention_east: MAX_POINT_MASK => translate(GridVector::EAST) => GridMask::EMPTY);
    test_transform!(wrap_prevention_west: ORIGIN_POINT_MASK => translate(GridVector::WEST) => GridMask::EMPTY);

    const OOB_SHIFTS: [GridVector; 4] = [
        // East
        GridVector::new(8, 0),
        // West
        GridVector::new(-8, 0),
        // South
        GridVector::new(0, 8),
        // North
        GridVector::new(0, -8),
    ];

    test_foreach!(oob_shifts: GridMask::FULL => translate(shift in OOB_SHIFTS) => GridMask::EMPTY);
}

mod from_pattern {
    use super::pattern_data::*;
    use super::*;

    use grid_mask::err::PatternError;

    test_panic!(set_eq_unset: GridMask::from_pattern("", '#', '#') => "set and unset must be different");
    test_panic!(set_whitespace: GridMask::from_pattern("", ' ', '.') => "set cannot be whitespace");
    test_panic!(unset_whitespace: GridMask::from_pattern("", '#', ' ') => "unset cannot be whitespace");

    test_ctor!(
        too_long: GridMask::from_pattern(TOO_LONG, '#', '.')
        => Err(PatternError::TooLong)
    );

    test_ctor!(
        too_short: GridMask::from_pattern(TOO_SHORT, '#', '.') 
        => Err(PATTERN_TOO_SHORT));
    test_ctor!(
        invalid_char: GridMask::from_pattern(INVALID, '#', '.')
        => Err(PATTERN_INVALID)
    );

    // valid construction tested elsewhere
}

mod from_str {
    use grid_mask::err::PatternError;

    use super::pattern_data::*;
    use super::*;

    test_ctor!(valid: GridMask::from_str(super::POINT_4_4_PATTERN) => Ok(super::POINT_4_4_MASK));
    test_ctor!(too_long: GridMask::from_str(TOO_LONG) => Err(PatternError::TooLong));
    test_ctor!(too_short: GridMask::from_str(TOO_SHORT) => Err(PATTERN_TOO_SHORT));
    test_ctor!(invalid: GridMask::from_str(INVALID) => Err(PATTERN_INVALID));
}

mod occupied {
    use super::pattern_data::*;
    use super::*;

    test_property!(empty_rows: GridMask::EMPTY => occupied_rows() => 0);
    test_property!(empty_cols: GridMask::EMPTY => occupied_cols() => 0);

    test_property!(full_rows: GridMask::FULL => occupied_rows() => 0xFF);
    test_property!(full_cols: GridMask::FULL => occupied_cols() => 0xFF);

    test_property!(even_rows: GridMask::from_str(EVEN_ROWS_COLS)? => occupied_rows() => 0b0101_0101);
    test_property!(even_cols: GridMask::from_str(EVEN_ROWS_COLS)? => occupied_cols() => 0b0101_0101);
}

mod bounds {
    use super::*;
    use grid_mask::GridRect;

    macro_rules! test_bounds {
        ($name:ident: $mask:expr => $expected:expr) => {
            test_property!($name: $mask => bounds() => $expected);
        };
    }

    test_bounds!(empty: GridMask::EMPTY => None);
    test_bounds!(full: GridMask::FULL => Some(GridRect::MAX));
    test_bounds!(origin_point: ORIGIN_POINT_MASK => Some(GridRect::const_new::<0, 0, 1, 1>()));
    test_bounds!(max_point: MAX_POINT_MASK => Some(GridRect::const_new::<7, 7, 1, 1>()));
    test_bounds!(center_plus: GridMask::from_str(PLUS_4_4)? => Some(GridRect::const_new::<3, 3, 3, 3>()));
    test_bounds!(nw_se_corners: GridMask::new(1 | 1 << 63) => Some(GridRect::MAX));
    test_bounds!(sw_ne_corners: GridMask::new(1 << 56 | 1 << 7) => Some(GridRect::MAX));
    test_bounds!(sparse_corners: GridMask::from_str(SPARSE_CORNERS)? => Some(GridRect::const_new::<2, 0, 4, 4>()));
}
