use grid_mask::num::GridPos;
use grid_mask::{GridMask, GridPoint, GridShape};

#[test]
fn test_contiguous() {
    let p = GridPoint::new(GridPos::new(0).unwrap(), GridPos::new(0).unwrap());
    let mask = GridMask::from(p);
    // GridShape defaults to GridShape<Cardinal>
    let shape: Result<GridShape, _> = GridShape::try_from(mask);
    assert!(shape.is_ok());
}

#[test]
fn test_discontiguous() {
    let p1 = GridPoint::new(GridPos::new(0).unwrap(), GridPos::new(0).unwrap());
    let p2 = GridPoint::new(GridPos::new(7).unwrap(), GridPos::new(7).unwrap());
    let mut mask = GridMask::from(p1);
    mask.set(p2);
    let shape: Result<GridShape, _> = GridShape::try_from(mask);
    assert!(shape.is_err());
}
