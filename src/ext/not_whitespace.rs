//pub trait FoldMut: Iterator {
//    fn fold_mut<Accumulate, F>(self, mut init: Accumulate, mut f: F) -> Accumulate
//    where
//        Self: Sized,
//        F: FnMut(&mut Accumulate, Self::Item),
//    {
//        for item in self {
//            f(&mut init, item);
//        }
//        init
//    }
//}
//
//impl<T: Iterator + ?Sized> FoldMut for T {}

#[sealed::sealed]
pub trait NotWhitespace {
    /// Returns `true` if the [`char`] is not whitespace.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grid_mask::ext::NotWhitespace;
    /// assert_eq!('a'.is_not_whitespace(), true);
    /// assert_eq!(' '.is_not_whitespace(), false);
    /// ```
    fn is_not_whitespace(&self) -> bool;
}

#[sealed::sealed]
impl NotWhitespace for char {
    fn is_not_whitespace(&self) -> bool {
        !self.is_whitespace()
    }
}
