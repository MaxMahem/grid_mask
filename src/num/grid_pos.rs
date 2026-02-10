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
