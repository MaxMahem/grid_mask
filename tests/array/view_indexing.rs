use grid_mask::ArrayGrid;
use grid_mask::err::OutOfBounds;
use grid_mask::num::Point;

type Grid4 = ArrayGrid<4, 4, 1>;

#[test]
fn test_view_get() {
    let mut grid = Grid4::EMPTY;
    grid.set((1u16, 1u16), true).unwrap();
    let view = grid.as_view();

    // Point
    assert_eq!(view.get(Point::new(1u16, 1u16)), Ok(true));
    assert_eq!(view.get(Point::new(0u16, 0u16)), Ok(false));
    assert_eq!(view.get(Point::new(4u16, 0u16)), Err(OutOfBounds));

    // Tuple
    assert_eq!(view.get((1u16, 1u16)), Ok(true));
    assert_eq!(view.get((0u16, 0u16)), Ok(false));
    assert_eq!(view.get((4u16, 0u16)), Err(OutOfBounds));

    // usize (relative index)
    // 4x4 grid. (1,1) is index 1*4 + 1 = 5.
    assert_eq!(view.get(5usize), Ok(true));
    assert_eq!(view.get(0usize), Ok(false));
    assert_eq!(view.get(16usize), Err(OutOfBounds));
}

#[test]
fn test_view_mut_get_set() {
    let mut grid = Grid4::EMPTY;
    let mut view = grid.as_view_mut();

    // Set via Point
    view.set(Point::new(1u16, 1u16), true).unwrap();
    assert_eq!(view.get(Point::new(1u16, 1u16)), Ok(true));

    // Set via Tuple
    view.set((2u16, 2u16), true).unwrap();
    assert_eq!(view.get((2u16, 2u16)), Ok(true));

    // Set via usize
    // (1,1) is 5. (2,2) is 2*4 + 2 = 10.
    view.set(5usize, false).unwrap(); // Clear (1,1)
    assert_eq!(view.get(5usize), Ok(false));
    assert_eq!(view.get(Point::new(1u16, 1u16)), Ok(false));
}
