use tap::Pipe;

use crate::ext::debug_check_then;
use crate::num::SignedMag;

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
