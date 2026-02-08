/// A 2D vector with unsigned components, representing a shift or displacement.
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
    derive_more::Sub,
    derive_more::SubAssign,
)]
#[display("({x:+}, {y:+})")]
pub struct GridVector {
    /// The horizontal component.
    pub x: i8,
    /// The vertical component.
    pub y: i8,
}

impl GridVector {
    /// The zero vector.
    pub const ZERO: Self = Self::new(0, 0);

    /// The North unit vector.
    pub const NORTH: Self = Self::new(0, -1);
    /// The South unit vector.
    pub const SOUTH: Self = Self::new(0, 1);
    /// The East unit vector.
    pub const EAST: Self = Self::new(1, 0);
    /// The West unit vector.
    pub const WEST: Self = Self::new(-1, 0);
    /// The Northeast unit vector.
    pub const NORTH_EAST: Self = Self::new(1, -1);
    /// The Northwest unit vector.
    pub const NORTH_WEST: Self = Self::new(-1, -1);
    /// The Southeast unit vector.
    pub const SOUTH_EAST: Self = Self::new(1, 1);
    /// The Southwest unit vector.
    pub const SOUTH_WEST: Self = Self::new(-1, 1);
}

impl From<(i8, i8)> for GridVector {
    fn from((x, y): (i8, i8)) -> Self {
        Self::new(x, y)
    }
}

impl From<GridVector> for (i8, i8) {
    fn from(v: GridVector) -> Self {
        (v.x, v.y)
    }
}
