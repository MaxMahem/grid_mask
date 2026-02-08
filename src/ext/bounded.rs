/// A trait for types with defined minimum and maximum bounds.
pub trait Bound: Sized + Copy + PartialEq + 'static {
    /// The minimum value of the type.
    const MIN: Self;

    /// The maximum value of the type.
    const MAX: Self;
}
