use crate::ext::Bound;

bounded_integer::bounded_integer! {
    /// A length of a grid.
    ///
    /// The valid range is 1 to 8.
    pub struct GridLen(1, 8);
}

impl Bound for GridLen {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}
