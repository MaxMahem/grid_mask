#[sealed::sealed]
/// An extension trait for [`Iterator`] that provides a [`fold_mut`] method.
pub trait FoldMut: Iterator {
    /// Folds the iterator via the accumulator, `f` which should mutate the
    /// starting state, `acc`, then return the final `acc` state.
    fn fold_mut<Acc, F>(&mut self, mut acc: Acc, mut f: F) -> Acc
    where
        F: FnMut(&mut Acc, Self::Item),
    {
        self.for_each(|item| f(&mut acc, item));
        acc
    }
}

#[sealed::sealed]
impl<I: Iterator> FoldMut for I {}
