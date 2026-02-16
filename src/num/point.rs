/// A point in a 2D grid.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    derive_more::From,
    derive_more::Into,
    derive_more::Display,
    derive_more::Constructor,
)]
#[display("({x}, {y})")]
pub struct Point<X, Y = X> {
    /// The x-coordinate.
    pub x: X,
    /// The y-coordinate.
    pub y: Y,
}

impl<X, Y, UX, UY> PartialEq<(UX, UY)> for Point<X, Y>
where
    X: PartialEq<UX>,
    Y: PartialEq<UY>,
{
    fn eq(&self, other: &(UX, UY)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}
