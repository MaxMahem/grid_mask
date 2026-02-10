pub trait MapTuple<T1, T2> {
    /// Maps a tuple into a new tuple of different types using `Into`.
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
}
