#![cfg(feature = "serde")]

#[path = "common/macros.rs"]
#[macro_use]
mod macros;

use grid_mask::{GridMask, GridPoint, GridSize};

mod point {
    use super::*;

    test_self_method!(ser: this = GridPoint::const_new::<3, 4>() => serde_json::to_string(&this)? => "[3,4]");
    test_ctor!(de_array: serde_json::from_str::<GridPoint>("[3,4]")? => GridPoint::const_new::<3, 4>());
    test_ctor!(de_object: serde_json::from_str::<GridPoint>(r#"{"x":3,"y":4}"#)? => GridPoint::const_new::<3, 4>());
}

mod mask {
    use super::*;

    test_self_method!(ser: this = GridMask::from(GridPoint::ORIGIN) => serde_json::to_string(&this)? => "[[0,0]]");

    test_ctor!(de_points: serde_json::from_str::<GridMask>("[[0,0],[7,7]]")? => GridMask::from_iter([GridPoint::ORIGIN, GridPoint::MAX]));
    test_ctor!(de_bitmask: serde_json::from_str::<GridMask>("1")? => GridMask(1));
}

mod size {
    use super::*;

    test_self_method!(ser: this = GridSize::const_new::<2, 3>() => serde_json::to_string(&this)? => "[2,3]");
    test_ctor!(de_array: serde_json::from_str::<GridSize>("[2,3]")? => GridSize::const_new::<2, 3>());
    test_ctor!(de_object: serde_json::from_str::<GridSize>(r#"{"w":2,"h":3}"#)? => GridSize::const_new::<2, 3>());
}

mod fail {
    use super::*;

    #[test]
    fn point_oob() {
        // x=8 is out of bounds for GridPos (0..=7)
        let res = serde_json::from_str::<GridPoint>("[8,0]");
        assert!(res.is_err());
    }

    #[test]
    fn size_oob() {
        // 0 is out of bounds for GridLen (1..=8)
        let res = serde_json::from_str::<GridSize>("[0,5]");
        assert!(res.is_err());
    }

    #[test]
    fn mask_invalid_type() {
        // Mask expects list of points or u64
        let res = serde_json::from_str::<GridMask>(r#"{"invalid": true}"#);
        assert!(res.is_err());
    }
}
