use std::num::NonZeroU16;

use crate::err::OutOfBounds;

/// A length in a grid dimension with a fixed maximum size.
///
/// The value is guaranteed to be in the range `1..=MAX`.
#[repr(transparent)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::Into,
    derive_more::Deref,
    derive_more::AsRef,
)]
#[into(owned(u16), owned(NonZeroU16), ref(NonZeroU16))]
pub struct ArrayGridLen<const MAX: u16>(pub(crate) NonZeroU16);

impl<const MAX: u16> ArrayGridLen<MAX> {
    /// The minimum length.
    pub const MIN: Self = Self(NonZeroU16::new(1).unwrap());
    /// The maximum length.
    pub const MAX: Self = Self(NonZeroU16::new(MAX).unwrap());

    // /// Creates a new [`ArrayGridLen`] without checking bounds.
    // ///
    // /// # Safety
    // ///
    // /// The caller must ensure that `1 <= val <= MAX`.
    // #[must_use]
    // pub const unsafe fn new_unchecked(val: u16) -> Self {
    //     debug_check_then!(val > 0 && val <= MAX => Self(unsafe { NonZeroU16::new_unchecked(val) }), "value out of bounds")
    // }

    /// Creates a new [`ArrayGridLen`].
    ///
    /// # Errors
    ///
    /// [`OutOfBounds`] if `val == 0` or `val > MAX`.
    pub const fn new(val: u16) -> Result<Self, OutOfBounds> {
        match NonZeroU16::new(val) {
            Some(nz) if val <= MAX => Ok(Self(nz)),
            _ => Err(OutOfBounds),
        }
    }

    /// Creates a new [`ArrayGridLen`] from a constant value.
    ///
    /// # Panics
    ///
    /// Panics at compile time if `VAL == 0` or `VAL > MAX`.
    #[must_use]
    pub const fn const_new<const VAL: u16>() -> Self {
        assert!(VAL <= MAX, "value out of bounds");
        Self(NonZeroU16::new(VAL).expect("value must be non-zero"))
    }

    /// Returns the raw value.
    #[must_use]
    pub const fn get(&self) -> NonZeroU16 {
        self.0
    }
}

impl<const MAX: u16, U> PartialEq<U> for ArrayGridLen<MAX>
where
    u16: PartialEq<U>,
{
    fn eq(&self, other: &U) -> bool {
        self.0.get() == *other
    }
}

impl<const MAX: u16> TryFrom<u16> for ArrayGridLen<MAX> {
    type Error = OutOfBounds;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<const MAX: u16> TryFrom<NonZeroU16> for ArrayGridLen<MAX> {
    type Error = OutOfBounds;

    fn try_from(value: NonZeroU16) -> Result<Self, Self::Error> {
        Self::new(value.get())
    }
}
