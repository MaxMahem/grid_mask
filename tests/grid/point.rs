use grid_mask::err::OutOfBounds;
use grid_mask::num::GridPos;
use grid_mask::{GridMask, GridPoint, GridSize};

#[test]
fn test_partial_eq_tuple() {
    let p = GridPoint::new(GridPos::new(2).unwrap(), GridPos::new(3).unwrap());
    assert_eq!(p, (2u8, 3u8));
    assert_eq!(p, (2u16, 3u16));
    assert_eq!(p, (2, 3));
    assert_ne!(p, (5u8, 5u8));
}

#[test]
fn test_from_iter() {
    let coords = vec![
        GridPoint::new(GridPos::new(0).unwrap(), GridPos::new(0).unwrap()),
        GridPoint::new(GridPos::new(1).unwrap(), GridPos::new(1).unwrap()),
        GridPoint::new(GridPos::new(7).unwrap(), GridPos::new(7).unwrap()),
    ];

    let mask: GridMask = coords.into_iter().collect();

    assert!(mask.get(GridPoint::new(GridPos::new(0).unwrap(), GridPos::new(0).unwrap())));
    assert!(mask.get(GridPoint::new(GridPos::new(1).unwrap(), GridPos::new(1).unwrap())));
    assert!(mask.get(GridPoint::new(GridPos::new(7).unwrap(), GridPos::new(7).unwrap())));
    assert!(!mask.get(GridPoint::new(GridPos::new(0).unwrap(), GridPos::new(1).unwrap())));
}

mod extent {
    use super::*;
    use grid_mask::GridRect;

    macro_rules! test_offset {
        ($name:ident, pos: $coord:expr, size: $size:expr, expected: $expected:expr) => {
            #[test]
            fn $name() {
                // If expected is Ok, we check that GridRect::new returns Ok and bottom_right matches.
                // If expected is Err, we check that GridRect::new returns Err.
                let result = GridRect::new($coord, $size);
                match $expected {
                    Ok(point) => {
                        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
                        assert_eq!(result.unwrap().bottom_right(), point);
                    }
                    Err(e) => {
                        assert_eq!(result.err(), Some(e));
                    }
                }
            }
        };
    }

    const ONE_BY_ONE: GridSize = GridSize::const_new::<1, 1>();
    const TWO_BY_TWO: GridSize = GridSize::const_new::<2, 2>();

    const ONE_BY_TWO: GridSize = GridSize::const_new::<1, 2>();
    const TWO_BY_ONE: GridSize = GridSize::const_new::<2, 1>();

    test_offset!(
        edge,
        pos: GridPoint::MAX,
        size: ONE_BY_ONE,
        expected: Ok(GridPoint::MAX)
    );

    test_offset!(
        oob_both,
        pos: GridPoint::MAX,
        size: TWO_BY_TWO,
        expected: Result::<GridPoint, _>::Err(OutOfBounds)
    );

    test_offset!(
        oob_x,
        pos: GridPoint::MAX,
        size: TWO_BY_ONE,
        expected: Result::<GridPoint, _>::Err(OutOfBounds)
    );

    test_offset!(
        oob_y,
        pos: GridPoint::MAX,
        size: ONE_BY_TWO,
        expected: Result::<GridPoint, _>::Err(OutOfBounds)
    );
}

#[test]
fn test_const_new() {
    const P1: GridPoint = GridPoint::const_new::<0, 0>();
    assert_eq!(P1, (0, 0));
    const P2: GridPoint = GridPoint::const_new::<7, 7>();
    assert_eq!(P2, (7, 7));
}
