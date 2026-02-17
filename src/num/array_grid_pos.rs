use crate::err::OutOfBounds;

/// A position in a grid dimension with a fixed maximum size.
///
/// The value is guaranteed to be in the range `0..MAX`.
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
    derive_more::Into,
    derive_more::Display,
    derive_more::Deref,
    derive_more::AsRef,
)]
#[into(owned(u16), owned(i32), owned(u32), owned(i64), owned(u64), owned(usize), ref(u16))]
pub struct ArrayGridPos<const MAX: u16>(pub(crate) u16);

impl<const MAX: u16> ArrayGridPos<MAX> {
    /// The zero position.
    pub const ZERO: Self = Self(0);
    /// The maximum position.
    pub const MAX: Self = Self(MAX - 1);

    // /// Creates a new [`ArrayGridPos`] without checking bounds.
    // ///
    // /// # Safety
    // ///
    // /// The caller must ensure that `val < MAX`.
    // #[must_use]
    // pub const unsafe fn new_unchecked(val: u16) -> Self {
    //     debug_check_then!(val < MAX => Self(val), "value out of bounds")
    // }

    /// Creates a new [`ArrayGridPos`].
    ///
    /// # Errors
    ///
    /// [`OutOfBounds`] if `val >= MAX`.
    pub const fn new(val: u16) -> Result<Self, OutOfBounds> {
        if val < MAX { Ok(Self(val)) } else { Err(OutOfBounds) }
    }

    /// Creates a new [`ArrayGridPos`] from a constant value.
    ///
    /// # Panics
    ///
    /// Panics at compile time if `VAL >= MAX`.
    #[must_use]
    pub const fn const_new<const VAL: u16>() -> Self {
        assert!(VAL < MAX, "value out of bounds");
        Self(VAL)
    }

    /// Returns the raw value.
    #[must_use]
    pub const fn get(&self) -> u16 {
        self.0
    }
}

impl<const MAX: u16, U> PartialEq<U> for ArrayGridPos<MAX>
where
    u16: PartialEq<U>,
{
    fn eq(&self, other: &U) -> bool {
        self.0 == *other
    }
}

// macro_rules! impl_try_from {
//     ($($t:ty),*) => {
//         $(
//             impl<const MAX: u16> TryFrom<$t> for ArrayGridPos<MAX> {
//                 type Error = OutOfBounds;
//                 fn try_from(value: $t) -> Result<Self, Self::Error> {
//                     u16::try_from(value).map_err(OutOfBounds::new_from).and_then(Self::new)
//                 }
//             }
//         )*
//     };
// }

// impl_try_from!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
