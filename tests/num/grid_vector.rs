use grid_mask::GridVector;

#[test]
fn test_new() {
    let v = GridVector::new(1, 2);
    assert_eq!(v.x, 1);
    assert_eq!(v.y, 2);
}

#[test]
fn test_default() {
    let v = GridVector::default();
    assert_eq!(v.x, 0);
    assert_eq!(v.y, 0);
    assert_eq!(v, GridVector::ZERO);
}

#[test]
fn test_from_tuple() {
    let v: GridVector = (3, 4).into();
    assert_eq!(v.x, 3);
    assert_eq!(v.y, 4);
}

#[test]
fn test_add() {
    let v1 = GridVector::new(1, 2);
    let v2 = GridVector::new(3, 4);
    let sum = v1 + v2;
    assert_eq!(sum.x, 4);
    assert_eq!(sum.y, 6);
}

#[test]
fn test_add_assign() {
    let mut v = GridVector::new(1, 2);
    v += GridVector::new(3, 4);
    assert_eq!(v.x, 4);
    assert_eq!(v.y, 6);
}

#[test]
fn test_sub() {
    let v1 = GridVector::new(5, 6);
    let v2 = GridVector::new(2, 3);
    let diff = v1 - v2;
    assert_eq!(diff.x, 3);
    assert_eq!(diff.y, 3);
}

#[test]
fn test_sub_assign() {
    let mut v = GridVector::new(5, 6);
    v -= GridVector::new(2, 3);
    assert_eq!(v.x, 3);
    assert_eq!(v.y, 3);
}
