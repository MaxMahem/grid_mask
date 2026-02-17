/// Adaptor trait for types that can be used to get a value from a grid `T`.
pub trait GridGetIndex<T: ?Sized> {
    /// Return type for a get operation.
    type GetOutput<'a>
    where
        T: 'a;

    /// Gets the value at this index in the grid.
    fn get(self, target: &T) -> Self::GetOutput<'_>;
}

/// Adaptor trait for types that can be used to set a value in a grid `T`.
pub trait GridSetIndex<T: ?Sized>: GridGetIndex<T> {
    /// Return type for a set operation.
    type SetOutput;

    /// Sets the value at this index in the grid.
    fn set(self, target: &mut T, value: bool) -> Self::SetOutput;
}
