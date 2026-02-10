use tap::Pipe;

use crate::ext::debug_check_then;

bounded_integer::bounded_integer! {
    /// The dimension of a vector component.
    ///
    /// The valid range is -7 to 7.
    pub struct VecDimU64(-7, 7);
}

bounded_integer::bounded_integer! {
    /// The magnitude of a vector component for a [`u64`] based grid.
    ///
    /// The valid range is `1..=7`.
    pub struct VecMagU64(1, 7);
}

/// A signed magnitude.
///
/// # Type Parameters
///
/// - `T`: The type of the magnitude
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SignedMag<T> {
    /// The magnitude is positive
    Positive(T),
    /// The magnitude is zero
    #[default]
    Zero,
    /// The magnitude is negative
    Negative(T),
}

impl From<VecDimU64> for SignedMag<VecMagU64> {
    fn from(value: VecDimU64) -> Self {
        match value.get() {
            pos @ 1..=7 => pos // rustfmt col
                .cast_unsigned()
                .pipe(|pos| unsafe { VecMagU64::new_unchecked(pos) })
                .pipe(Self::Positive),
            neg @ -7..=-1 => neg // rustfmt col
                .unsigned_abs()
                .pipe(|neg| unsafe { VecMagU64::new_unchecked(neg) })
                .pipe(Self::Negative),
            _ => debug_check_then!(
                value.get() == 0 => Self::Zero,
                "value ({value}) should be 0 (-7..=7)"
            ),
        }
    }
}
