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
