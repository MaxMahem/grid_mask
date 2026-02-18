use std::str::FromStr;

use grid_mask::err::OutOfBounds;
use grid_mask::num::{Point, Rect, Size};
use grid_mask::{ArrayIndex, ArrayPoint, ArrayVector};

use crate::macros::{test_ctor, test_mutation, test_self_method, test_try_mutation};

type Grid8 = grid_mask::array_grid!(8, 8);
type Point8 = ArrayPoint<8, 8>;
type Index8 = ArrayIndex<8, 8>;

const GRID8_1_1: Grid8 = {
    let mut g = Grid8::EMPTY;
    g.const_set(Index8::const_new::<9>(), true); // (1, 1) -> 9
    g
};

// 10x10 grid needs 100 bits. 100 / 64 = 2 words (ceil).
type Grid10 = grid_mask::array_grid!(10, 10);
type Point10 = ArrayPoint<10, 10>;

mod consts {
    use super::*;

    #[test]
    fn dimensions_8() {
        assert_eq!(Grid8::CELLS, 64);
    }

    #[test]
    fn dimensions_10() {
        assert_eq!(Grid10::CELLS, 100);
    }
}

mod from {
    use super::*;

    test_ctor!(from_array: Grid10::from([u64::MAX, u64::MAX]) => Grid10::FULL);
}

mod properties {
    use super::*;

    test_self_method!(empty_count: Grid8::EMPTY => count() => 0);
    test_self_method!(full_count: Grid8::FULL => count() => 64);

    test_self_method!(full_10_count: Grid10::FULL => count() => 100);

    // Grid10 FULL data check:
    // Word 0: u64::MAX (64 bits)
    // Word 1: 36 bits set (100 - 64). (1 << 36) - 1.
    const EXPECTED_FULL_10: [u64; 2] = [u64::MAX, (1u64 << 36) - 1];
    test_self_method!(full_10_data: Grid10::FULL => data() => &EXPECTED_FULL_10);
}

mod mutation {
    use super::*;

    test_mutation!(set_0_0: Grid8::EMPTY => set(Point8::ORIGIN, true) => Grid8::from([1]));
    test_mutation!(unset_1_1: GRID8_1_1 => set(Point8::new(1, 1)?, false) => Grid8::EMPTY);

    test_mutation!(clear: Grid8::FULL => clear() => Grid8::EMPTY);
    test_mutation!(fill_true: Grid8::EMPTY => fill(true) => Grid8::FULL);
    test_mutation!(fill_false: Grid8::FULL => fill(false) => Grid8::EMPTY);

    test_mutation!(fill_true_10: Grid10::EMPTY => fill(true) => Grid10::FULL);

    test_mutation!(
        mutate_data: Grid10::EMPTY
        => mutate_data(|data| data.fill(u64::MAX))
        => Grid10::FULL
    );

    test_mutation!(negate_empty: Grid8::EMPTY => negate() => Grid8::FULL);
    test_mutation!(negate_full: Grid8::FULL => negate() => Grid8::EMPTY);
    test_mutation!(negate_10: Grid10::EMPTY => negate() => Grid10::FULL);
}

mod get {
    use super::*;

    test_self_method!(get_point: Grid8::FULL => get(Point8::MIN) => true);
    test_self_method!(get_index: Grid8::FULL => get(Index8::MIN) => true);
    test_self_method!(get_empty: Grid8::EMPTY => get(Index8::MIN) => false);

    test_self_method!(get_tuple_ok: Grid8::FULL => get((0u16, 0u16)) => Ok(true));
    test_self_method!(get_tuple_err: Grid8::FULL => get((8u16, 0u16)) => Err(OutOfBounds));

    test_self_method!(get_tuple_u32_ok: Grid8::FULL => get((0u32, 0u32)) => Ok(true));
    test_self_method!(get_tuple_u32_err: Grid8::FULL => get((u32::MAX, 0u32)) => Err(OutOfBounds));

    test_self_method!(get_num_point_ok: Grid8::FULL => get(Point::new(0u32, 0u32)) => Ok(true));
    test_self_method!(get_num_point_err: Grid8::FULL => get(Point::new(8u32, 0u32)) => Err(OutOfBounds));

    test_self_method!(get_int_u32_ok: Grid8::FULL => get(0u32) => Ok(true));
    test_self_method!(get_int_u32_err: Grid8::FULL => get(64u32) => Err(OutOfBounds));
    test_self_method!(get_int_usize_ok: Grid8::FULL => get(0usize) => Ok(true));
    test_self_method!(get_int_usize_err: Grid8::FULL => get(64usize) => Err(OutOfBounds));

    #[test]
    fn get_array_rect_view_infallible() {
        let view = GRID8_1_1.get(grid_mask::ArrayRect::const_new::<1, 1, 2, 2>());
        assert_eq!(view.size(), Size::new(2, 2));
        assert_eq!(view.get((0u16, 0u16)), Ok(true));
    }

    #[test]
    fn get_rect_view_fallible() {
        let rect = Rect::new(Point::new(1, 1), Size::new(2, 2));
        let view = Grid8::FULL.get(rect).expect("rect should be valid");
        assert_eq!(view.size(), Size::new(2, 2));
        assert_eq!(view.get((1u16, 1u16)), Ok(true));

        let err = Grid8::FULL.get(Rect::new(Point::new(7u16, 7u16), Size::new(2u16, 2u16)));
        assert_eq!(err, Err(OutOfBounds));
    }
}

mod set {
    use super::*;

    test_try_mutation!(
        set_tuple_ok: Grid8::EMPTY
        => set((0u32, 0u32), true)
        => (Ok(()), Grid8::from([1]))
    );

    test_try_mutation!(
        set_tuple_err: Grid8::EMPTY
        => set((8u16, 0u16), true)
        => (Err(OutOfBounds), Grid8::EMPTY)
    );

    test_try_mutation!(
        set_num_point_ok: Grid8::EMPTY
        => set(Point::new(0u32, 0u32), true)
        => (Ok(()), Grid8::from([1]))
    );

    test_try_mutation!(
        set_num_point_err: Grid8::EMPTY
        => set(Point::new(u32::MAX, 0u32), true)
        => (Err(OutOfBounds), Grid8::EMPTY)
    );

    test_try_mutation!(
        set_int_u32_ok: Grid8::EMPTY
        => set(0u32, true)
        => (Ok(()), Grid8::from([1]))
    );

    test_try_mutation!(
        set_int_u32_err: Grid8::EMPTY
        => set(64u32, true)
        => (Err(OutOfBounds), Grid8::EMPTY)
    );
}

mod translation {
    use super::*;

    test_mutation!(zero_full: Grid8::FULL => translate(ArrayVector::ZERO) => Grid8::FULL);
    test_mutation!(zero_empty: Grid8::EMPTY => translate(ArrayVector::ZERO) => Grid8::EMPTY);

    test_mutation!(east: GRID8_1_1 => translate(ArrayVector::EAST) => Grid8::from_iter([Point8::new(2, 1)?]));
    test_mutation!(west: GRID8_1_1 => translate(ArrayVector::WEST) => Grid8::from_iter([Point8::new(0, 1)?]));
    test_mutation!(south: GRID8_1_1 => translate(ArrayVector::SOUTH) => Grid8::from_iter([Point8::new(1, 2)?]));
    test_mutation!(north: GRID8_1_1 => translate(ArrayVector::NORTH) => Grid8::from_iter([Point8::new(1, 0)?]));

    test_mutation!(
        east_5: Grid10::FULL => translate(ArrayVector::new(5, 0)) => Grid10::from_str("
            . . . . . # # # # #
            . . . . . # # # # #
            . . . . . # # # # #
            . . . . . # # # # #
            . . . . . # # # # #
            . . . . . # # # # #
            . . . . . # # # # #
            . . . . . # # # # #
            . . . . . # # # # #
            . . . . . # # # # #
        ")?
    );
    test_mutation!(
        west_5: Grid10::FULL => translate(ArrayVector::new(-5, 0)) => Grid10::from_str("
            # # # # # . . . . .
            # # # # # . . . . .
            # # # # # . . . . .
            # # # # # . . . . .
            # # # # # . . . . .
            # # # # # . . . . .
            # # # # # . . . . .
            # # # # # . . . . .
            # # # # # . . . . .
            # # # # # . . . . .
        ")?
    );
    test_mutation!(
        south_5: Grid10::FULL => translate(ArrayVector::new(0, 5)) => Grid10::from_str("
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            # # # # # # # # # #
            # # # # # # # # # #
            # # # # # # # # # #
            # # # # # # # # # #
            # # # # # # # # # #
        ")?
    );

    // Diagonal translations (5 units each axis)
    test_mutation!(
        se_5: Grid10::FULL => translate(ArrayVector::new(5, 5)) => Grid10::from_str("
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . # # # # #
            . . . . . # # # # #
            . . . . . # # # # #
            . . . . . # # # # #
            . . . . . # # # # #
        ")?
    );
    test_mutation!(
        sw_5: Grid10::FULL => translate(ArrayVector::new(-5, 5)) => Grid10::from_str("
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            # # # # # . . . . .
            # # # # # . . . . .
            # # # # # . . . . .
            # # # # # . . . . .
            # # # # # . . . . .
        ")?
    );
    test_mutation!(
        ne_5: Grid10::FULL => translate(ArrayVector::new(5, -5)) => Grid10::from_str("
            . . . . . # # # # #
            . . . . . # # # # #
            . . . . . # # # # #
            . . . . . # # # # #
            . . . . . # # # # #
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
        ")?
    );
    test_mutation!(
        nw_5: Grid10::FULL => translate(ArrayVector::new(-5, -5)) => Grid10::from_str("
            # # # # # . . . . .
            # # # # # . . . . .
            # # # # # . . . . .
            # # # # # . . . . .
            # # # # # . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
        ")?
    );

    macro_rules! test_oob_empty {
        ( $( ($name:ident: $Grid:ty, $vector:expr) ),* $(,)? ) => {
            $(test_mutation!(
                $name: <$Grid>::FULL => translate($vector) => <$Grid>::EMPTY
            );)*
        };
    }

    test_oob_empty![
        (oob_8_x_pos: Grid8, ArrayVector::new(8, 0)),
        (oob_8_x_neg: Grid8, ArrayVector::new(-8, 0)),
        (oob_8_y_pos: Grid8, ArrayVector::new(0, 8)),
        (oob_8_y_neg: Grid8, ArrayVector::new(0, -8)),
    ];

    test_oob_empty![
        (oob_10_x_pos: Grid10, ArrayVector::new(10, 0)),
        (oob_10_x_neg: Grid10, ArrayVector::new(-10, 0)),
        (oob_10_y_pos: Grid10, ArrayVector::new(0, 10)),
        (oob_10_y_neg: Grid10, ArrayVector::new(0, -10)),
    ];

    macro_rules! test_max_shift {
        ( $( ($name:ident: $Grid:ty, $vector:expr => $expected:expr) ),* $(,)? ) => {
            $(test_mutation!(
                $name: <$Grid>::FULL => translate($vector) => <$Grid>::from_iter($expected)
            );)*
        };
    }

    test_max_shift![
        (max_nw: Grid8, ArrayVector::new(-7, -7) => [Point8::new(0, 0)?]),
        (max_ne: Grid8, ArrayVector::new(7, -7) => [Point8::new(7, 0)?]),
        (max_sw: Grid8, ArrayVector::new(-7, 7) => [Point8::new(0, 7)?]),
        (max_se: Grid8, ArrayVector::new(7, 7) => [Point8::new(7, 7)?]),
    ];

    test_max_shift![
        (max_nw_10: Grid10, ArrayVector::new(-9, -9) => [Point10::new(0, 0)?]),
        (max_ne_10: Grid10, ArrayVector::new(9, -9) => [Point10::new(9, 0)?]),
        (max_sw_10: Grid10, ArrayVector::new(-9, 9) => [Point10::new(0, 9)?]),
        (max_se_10: Grid10, ArrayVector::new(9, 9) => [Point10::new(9, 9)?]),
    ];
}

mod bitwise {
    use super::*;

    const POINT8_1_1: Point8 = Point8::const_new::<1, 1>();
    type Grid9 = grid_mask::array_grid!(9, 9);
    type Grid11 = grid_mask::array_grid!(11, 11);
    type Point11 = ArrayPoint<11, 11>;

    const POINT11_1_1: Point11 = Point11::const_new::<1, 1>();

    macro_rules! test_simple_bitwise_mut {
        ( $type:ty, $method:ident, [ $(( $name:ident: $ctor:ident $_:tt $arg:ident => $expected:ident)),* $(,)? ] ) => {
            $(test_try_mutation!(
                $name: <$type>::$ctor
                => $method(&<$type>::$arg, <$type>::ORIGIN)
                => (Ok(()), <$type>::$expected)
            );)*
        };
    }

    mod and {
        use super::*;

        test_simple_bitwise_mut!(Grid8, bitand_at, [
            (full_and_full: FULL & FULL => FULL),
            (empty_and_full: EMPTY & FULL => EMPTY),
            (full_and_empty: FULL & EMPTY => EMPTY),
            (empty_and_empty: EMPTY & EMPTY => EMPTY),
        ]);

        test_try_mutation!(
            oob: Grid8::FULL
            => bitand_at(&Grid8::FULL, POINT8_1_1)
            => (Err(OutOfBounds), Grid8::FULL)
        );

        test_try_mutation!(
            eleven_nine_full: Grid11::FULL
            => bitand_at(&Grid9::FULL, POINT11_1_1)
            => (Ok(()), Grid11::FULL)
        );

        test_try_mutation!(
            eleven_nine_empty: Grid11::FULL
            => bitand_at(&Grid9::EMPTY, POINT11_1_1)
            => (Ok(()), Grid11::from_str("
                # # # # # # # # # # #
                # . . . . . . . . . #
                # . . . . . . . . . #
                # . . . . . . . . . #
                # . . . . . . . . . #
                # . . . . . . . . . #
                # . . . . . . . . . #
                # . . . . . . . . . #
                # . . . . . . . . . #
                # . . . . . . . . . #
                # # # # # # # # # # #
            ")?)
        );
    }

    mod or {
        use super::*;

        test_simple_bitwise_mut!(Grid8, bitor_at, [
            (full_or_full: FULL | FULL => FULL),
            (empty_or_full: EMPTY | FULL => FULL),
            (full_or_empty: FULL | EMPTY => FULL),
            (empty_or_empty: EMPTY | EMPTY => EMPTY),
        ]);

        test_try_mutation!(
            oob: Grid8::FULL
            => bitor_at(&Grid8::FULL, POINT8_1_1)
            => (Err(OutOfBounds), Grid8::FULL)
        );

        test_try_mutation!(
            eleven_nine_full: Grid11::EMPTY
            => bitor_at(&Grid9::FULL, POINT11_1_1)
            => (Ok(()), Grid11::from_str("
                . . . . . . . . . . .
                . # # # # # # # # # .
                . # # # # # # # # # .
                . # # # # # # # # # .
                . # # # # # # # # # .
                . # # # # # # # # # .
                . # # # # # # # # # .
                . # # # # # # # # # .
                . # # # # # # # # # .
                . # # # # # # # # # .
                . . . . . . . . . . .
            ")?)
        );

        test_try_mutation!(
            eleven_nine_empty: Grid11::EMPTY
            => bitor_at(&Grid9::EMPTY, POINT11_1_1)
            => (Ok(()), Grid11::EMPTY)
        );
    }

    mod xor {
        use super::*;

        test_simple_bitwise_mut!(Grid8, bitxor_at, [
            (full_xor_full: FULL ^ FULL => EMPTY),
            (empty_xor_full: EMPTY ^ FULL => FULL),
            (full_xor_empty: FULL ^ EMPTY => FULL),
            (empty_xor_empty: EMPTY ^ EMPTY => EMPTY),
        ]);

        test_try_mutation!(
            oob: Grid8::FULL
            => bitxor_at(&Grid8::FULL, POINT8_1_1)
            => (Err(OutOfBounds), Grid8::FULL)
        );

        test_try_mutation!(
            eleven_nine_full: Grid11::FULL
            => bitxor_at(&Grid9::FULL, POINT11_1_1)
            => (Ok(()), Grid11::from_str("
                # # # # # # # # # # #
                # . . . . . . . . . #
                # . . . . . . . . . #
                # . . . . . . . . . #
                # . . . . . . . . . #
                # . . . . . . . . . #
                # . . . . . . . . . #
                # . . . . . . . . . #
                # . . . . . . . . . #
                # . . . . . . . . . #
                # # # # # # # # # # #
            ")?)
        );

        test_try_mutation!(
            eleven_nine_empty: Grid11::FULL
            => bitxor_at(&Grid9::EMPTY, POINT11_1_1)
            => (Ok(()), Grid11::FULL)
        );
    }
}

mod from_str {
    use super::*;
    use grid_mask::err::PatternError;

    const VALID_STR: &str = unsafe { std::str::from_utf8_unchecked(&[b'.'; 64]) };
    test_ctor!(valid: Grid8::from_str(VALID_STR) => Ok(Grid8::EMPTY));

    const TOO_LONG_STR: &str = unsafe { std::str::from_utf8_unchecked(&[b'.'; 65]) };
    test_ctor!(too_long: Grid8::from_str(TOO_LONG_STR) => Err(PatternError::TooLong));

    const TOO_SHORT_STR: &str = unsafe { std::str::from_utf8_unchecked(&[b'.'; 63]) };
    test_ctor!(too_short: Grid8::from_str(TOO_SHORT_STR) => Err(PatternError::TooShort(63)));
    test_ctor!(too_short_empty: Grid8::from_str("") => Err(PatternError::TooShort(0)));

    const INVALID_CHAR_STR: &str = unsafe { std::str::from_utf8_unchecked(&[b'?'; 64]) };
    test_ctor!(invalid: Grid8::from_str(INVALID_CHAR_STR) => Err(PatternError::InvalidChar('?')));
}

mod extend {
    use super::*;

    test_mutation!(
        empty_extend: Grid8::EMPTY
        => extend([Point8::new(0, 0)?, Point8::new(7, 7)?])
        => Grid8::from_iter([Point8::new(0, 0)?, Point8::new(7, 7)?])
    );

    test_mutation!(
        non_empty_extend: GRID8_1_1
        => extend([Point8::new(2, 2)?])
        => Grid8::from_iter([Point8::new(1, 1)?, Point8::new(2, 2)?])
    );

    test_mutation!(
        duplicate_extend: Grid8::EMPTY
        => extend([Point8::new(1, 1)?, Point8::new(1, 1)?])
        => GRID8_1_1
    );

    test_mutation!(
        empty_iterator: Grid8::FULL
        => extend(std::iter::empty::<Point8>())
        => Grid8::FULL
    );

    test_mutation!(
        index_extend: Grid8::EMPTY
        => extend([Index8::MIN, Index8::MAX])
        => Grid8::from_iter([Point8::MIN, Point8::new(7, 7)?])
    );
}
