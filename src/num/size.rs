/// A width/height pair for a 2D grid.
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
#[display("({width}x{height})")]
pub struct Size<W, H = W> {
    /// The width.
    pub width: W,
    /// The height.
    pub height: H,
}

impl<W, H, UW, UH> PartialEq<(UW, UH)> for Size<W, H>
where
    W: PartialEq<UW>,
    H: PartialEq<UH>,
{
    fn eq(&self, other: &(UW, UH)) -> bool {
        self.width == other.0 && self.height == other.1
    }
}
