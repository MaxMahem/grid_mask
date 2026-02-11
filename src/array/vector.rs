/// A 2D displacement vector for translating an [`ArrayGrid`](super::ArrayGrid).
///
/// Components are `i32` to cover the full range of `u16` grid dimensions.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    derive_more::Constructor,
    derive_more::Display,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::SubAssign,
)]
#[display("({dx:+}, {dy:+})")]
pub struct ArrayVector {
    /// The horizontal displacement (positive = east).
    pub dx: i32,
    /// The vertical displacement (positive = south).
    pub dy: i32,
}

impl ArrayVector {
    /// The zero vector.
    pub const ZERO: Self = Self::new(0, 0);

    /// The north unit vector.
    pub const NORTH: Self = Self::new(0, -1);

    /// The south unit vector.
    pub const SOUTH: Self = Self::new(0, 1);

    /// The east unit vector.
    pub const EAST: Self = Self::new(1, 0);

    /// The west unit vector.
    pub const WEST: Self = Self::new(-1, 0);
}

impl From<(i32, i32)> for ArrayVector {
    fn from((dx, dy): (i32, i32)) -> Self {
        Self::new(dx, dy)
    }
}

impl From<ArrayVector> for (i32, i32) {
    fn from(v: ArrayVector) -> Self {
        (v.dx, v.dy)
    }
}
