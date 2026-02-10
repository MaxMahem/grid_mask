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
    const COUNT: usize = (Self::MAX.get() - Self::MIN.get()) as usize + 1;

    fn increment(&self) -> Option<Self> {
        self.get().checked_add(1).and_then(Self::new)
    }

    fn decrement(&self) -> Option<Self> {
        self.get().checked_sub(1).and_then(Self::new)
    }

    fn remaining(&self) -> usize {
        (Self::MAX.get() - self.get()) as usize
    }
}
