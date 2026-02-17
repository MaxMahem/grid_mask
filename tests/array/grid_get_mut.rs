use grid_mask::array_grid;
use grid_mask::num::{Point, Rect, Size};
use grid_mask::{ArrayGrid, ArrayIndex, ArrayPoint, ArrayRect, GridSetIndex};

type Grid8 = array_grid!(8, 8);

#[test]
fn test_get_mut_array_point() {
    let mut grid = Grid8::EMPTY;
    {
        let mut bit_ref = grid.get_mut(ArrayPoint::ORIGIN);
        *bit_ref = true;
    }
    assert!(grid.get(ArrayPoint::ORIGIN));
}

#[test]
fn test_get_mut_array_index() {
    let mut grid = Grid8::EMPTY;
    {
        let mut bit_ref = grid.get_mut(ArrayIndex::MIN);
        *bit_ref = true;
    }
    assert!(grid.get(ArrayIndex::MIN));
}

#[test]
fn test_get_mut_point() {
    let mut grid = Grid8::EMPTY;
    let point = Point::new(1u32, 1u32);
    {
        let mut bit_ref = grid.get_mut(point).expect("Point should be in bounds");
        *bit_ref = true;
    }
    assert!(grid.get(point).expect("Point should be in bounds"));
}

#[test]
fn test_get_mut_tuple() {
    let mut grid = Grid8::EMPTY;
    {
        let mut bit_ref = grid.get_mut((2u32, 2u32)).expect("Tuple should be in bounds");
        *bit_ref = true;
    }
    assert!(grid.get((2u32, 2u32)).expect("Tuple should be in bounds"));
}

#[test]
fn test_get_mut_usize() {
    let mut grid = Grid8::EMPTY;
    {
        let mut bit_ref = grid.get_mut(3usize).expect("Index should be in bounds");
        *bit_ref = true;
    }
    assert!(grid.get(3usize).expect("Index should be in bounds"));
}

#[test]
fn test_get_mut_array_rect() {
    let mut grid = Grid8::EMPTY;
    let rect = ArrayRect::new(ArrayPoint::ORIGIN, (2, 2)).expect("Rect should be valid");
    {
        let mut view = grid.get_mut(rect);
        view.fill(true);
    }
    assert_eq!(grid.count(), 4);
}

#[test]
fn test_get_mut_rect() {
    let mut grid = Grid8::EMPTY;
    let rect = Rect::new(Point::new(1, 1), Size::new(2, 2));
    {
        let mut view = grid.get_mut(rect).expect("Rect should be in bounds");
        view.fill(true);
    }

    let result_view = grid.get(rect).expect("Rect should be in bounds");
    assert_eq!(result_view.count(), 4);

    // Verify specific points are set
    assert!(grid.get((1, 1)).unwrap());
    assert!(grid.get((1, 2)).unwrap());
    assert!(grid.get((2, 1)).unwrap());
    assert!(grid.get((2, 2)).unwrap());
}
