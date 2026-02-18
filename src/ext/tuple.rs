#[sealed::sealed]
pub trait MapTuple<T1, T2> {
    /// Maps a tuple into a new tuple of different types using [`Into`].
    ///
    /// # Example
    /// ```
    /// # use grid_mask::ext::MapTuple;
    /// let (a, b): (i64, i64) = (1i32, 2i32).map_into();
    ///
    /// assert_eq!((a, b), (1i64, 2i64));
    /// ```
    fn map_into<U1, U2>(self) -> (U1, U2)
    where
        T1: Into<U1>,
        T2: Into<U2>;
}

#[sealed::sealed]
impl<T1, T2> MapTuple<T1, T2> for (T1, T2) {
    #[inline]
    fn map_into<U1, U2>(self) -> (U1, U2)
    where
        T1: Into<U1>,
        T2: Into<U2>,
    {
        (self.0.into(), self.1.into())
    }
}

#[sealed::sealed]
pub trait SwapTuple<A, B> {
    /// Swaps the elements of a tuple.
    ///
    /// # Example
    /// ```
    /// # use grid_mask::ext::SwapTuple;
    /// let t = (1, 2);
    /// assert_eq!(t.swap(), (2, 1));
    /// ```
    fn swap(self) -> (B, A);
}

#[sealed::sealed]
impl<A, B> SwapTuple<A, B> for (A, B) {
    #[inline]
    fn swap(self) -> (B, A) {
        (self.1, self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_into() {
        let (a, b): (i64, i64) = (1i32, 2i32).map_into();
        assert_eq!(a, 1i64);
        assert_eq!(b, 2i64);

        let (c, d): (String, String) = ("hello", "world").map_into();
        assert_eq!(c, "hello");
        assert_eq!(d, "world");
    }

    #[test]
    fn test_swap() {
        let t = (1, 2);
        assert_eq!(t.swap(), (2, 1));

        let t = ("hello", 42);
        assert_eq!(t.swap(), (42, "hello"));
    }
}
