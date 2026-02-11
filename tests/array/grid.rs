use crate::macros::{test_ctor, test_mutation, test_property};
use grid_mask::{ArrayGrid, ArrayIndex, ArrayPoint, ArrayVector};

type Grid8 = ArrayGrid<8, 8, 1>;
type Point8 = ArrayPoint<8, 8>;
type Index8 = ArrayIndex<8, 8>;

// 10x10 grid needs 100 bits. 100 / 64 = 2 words (ceil).
type Grid10 = ArrayGrid<10, 10, 2>;

mod consts {
    use super::*;

    #[test]
    fn dimensions_8() {
        assert_eq!(Grid8::WIDTH.get(), 8);
        assert_eq!(Grid8::HEIGHT.get(), 8);
        assert_eq!(Grid8::CELL_COUNT, 64);
    }

    #[test]
    fn dimensions_10() {
        assert_eq!(Grid10::WIDTH.get(), 10);
        assert_eq!(Grid10::HEIGHT.get(), 10);
        assert_eq!(Grid10::CELL_COUNT, 100);
    }
}

mod from {
    use super::*;

    test_ctor!(from_array: Grid10::from([u64::MAX, u64::MAX]) => Grid10::FULL);
}

mod properties {
    use super::*;

    test_property!(empty_count: Grid8::EMPTY => count() => 0);
    test_property!(full_count: Grid8::FULL => count() => 64);

    test_property!(full_10_count: Grid10::FULL => count() => 100);

    // Grid10 FULL data check:
    // Word 0: u64::MAX (64 bits)
    // Word 1: 36 bits set (100 - 64). (1 << 36) - 1.
    const EXPECTED_FULL_10: [u64; 2] = [u64::MAX, (1u64 << 36) - 1];
    test_property!(full_10_data: Grid10::FULL => words() => &EXPECTED_FULL_10);
}

mod mutation {
    use super::*;

    test_mutation!(set_0_0: Grid8::EMPTY => set(Point8::ORIGIN, true) => Grid8::from([1]));
    test_mutation!(clear: Grid8::FULL => clear() => Grid8::EMPTY);
    test_mutation!(fill_true: Grid8::EMPTY => fill(true) => Grid8::FULL);
    test_mutation!(fill_false: Grid8::FULL => fill(false) => Grid8::EMPTY);

    test_mutation!(fill_true_10: Grid10::EMPTY => fill(true) => Grid10::FULL);

    test_mutation!(
        mutate_data: Grid10::EMPTY
        => mutate_words(|data| data.fill(u64::MAX))
        => Grid10::FULL
    );
}

mod access {
    use super::*;

    test_property!(get_point: Grid8::FULL => get(Point8::MIN) => true);
    test_property!(get_index: Grid8::FULL => get(Index8::MIN) => true);
    test_property!(get_empty: Grid8::EMPTY => get(Index8::MIN) => false);
}

mod translation {
    use super::*;

    test_mutation!(zero: Grid8::FULL => translate_mut(ArrayVector::ZERO) => Grid8::FULL);

    test_mutation!(east: Grid8::from([0x01]) => translate_mut(ArrayVector::EAST) => Grid8::from([0x02]));
    test_mutation!(west: Grid8::from([0x02]) => translate_mut(ArrayVector::WEST) => Grid8::from([0x01]));
    test_mutation!(south: Grid8::from([0x01]) => translate_mut(ArrayVector::SOUTH) => Grid8::from([0x01 << 8]));
    test_mutation!(north: Grid8::from([0x01 << 8]) => translate_mut(ArrayVector::NORTH) => Grid8::from([0x01]));

    test_mutation!(diagonal: Grid8::from([0x01]) => translate_mut(ArrayVector::new(1, 1)) => Grid8::from([0x01 << 9]));

    test_mutation!(wrap_east: Grid8::from([1 << 7]) => translate_mut(ArrayVector::new(1, 0)) => Grid8::EMPTY);
    test_mutation!(wrap_west: Grid8::from([1]) => translate_mut(ArrayVector::new(-1, 0)) => Grid8::EMPTY);

    test_mutation!(oob_x: Grid8::FULL => translate_mut(ArrayVector::new(8, 0)) => Grid8::EMPTY);
    test_mutation!(oob_y: Grid8::FULL => translate_mut(ArrayVector::new(0, 8)) => Grid8::EMPTY);

    #[test]
    fn multi_word_shift() {
        let mut grid = Grid10::EMPTY;
        // Set bit at (0, 0) - index 0, word 0
        grid.set(ArrayIndex::<10, 10>::new(0).unwrap(), true);
        // Translate to (0, 7) - index 70, word 1 (70/64 = 1, 70%64 = 6)
        grid.translate_mut(ArrayVector::new(0, 7));

        let mut expected = Grid10::EMPTY;
        expected.set(ArrayIndex::<10, 10>::new(70).unwrap(), true);
        assert_eq!(grid, expected);
        assert_ne!(grid.words()[1], 0);
    }
}
