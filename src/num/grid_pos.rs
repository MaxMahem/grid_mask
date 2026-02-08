use crate::ext::Bound;

bounded_integer::bounded_integer! {
    /// A position in a grid.
    ///
    /// The valid range is 0 to 7.
    #[repr(u8)]
    pub struct GridPos(0, 7);
}

impl Bound for GridPos {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}
