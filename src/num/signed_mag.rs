use std::num::{NonZeroU16, NonZeroU32};

use crate::err::OutOfBounds;

/// A signed magnitude.
///
/// # Type Parameters
///
/// - `T`: The type of the magnitude
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, derive_more::Display)]
pub enum SignedMag<T> {
    /// The magnitude is positive
    Positive(T),
    /// The magnitude is zero
    #[default]
    Zero,
    /// The magnitude is negative
    Negative(T),
}

// impl TryFrom<i32> for SignedMag<u16> {
//     type Error = OutOfBounds;

//     fn try_from(value: i32) -> Result<Self, Self::Error> {
//         match value {
//             0 => Ok(Self::Zero),
//             1.. => value.try_into().map(Self::Positive).map_err(OutOfBounds::new_from),
//             ..0 => (-value).try_into().map(Self::Negative).map_err(OutOfBounds::new_from),
//         }
//     }
// }

impl TryFrom<i32> for SignedMag<NonZeroU16> {
    type Error = OutOfBounds;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Zero),
            1.. => value
                .try_into()
                .map_err(OutOfBounds::new_from)
                .and_then(|n| NonZeroU16::new(n).ok_or(OutOfBounds))
                .map(Self::Positive),
            ..0 => (-value)
                .try_into()
                .map_err(OutOfBounds::new_from)
                .and_then(|n| NonZeroU16::new(n).ok_or(OutOfBounds))
                .map(Self::Negative),
        }
    }
}

// impl TryFrom<i64> for SignedMag<u32> {
//     type Error = OutOfBounds;

//     fn try_from(value: i64) -> Result<Self, Self::Error> {
//         match value {
//             0 => Ok(Self::Zero),
//             1.. => value.try_into().map(Self::Positive).map_err(OutOfBounds::new_from),
//             ..0 => (-value).try_into().map(Self::Negative).map_err(OutOfBounds::new_from),
//         }
//     }
// }

impl From<i32> for SignedMag<NonZeroU32> {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Zero,
            1.. => {
                let n = u32::try_from(value).expect("positive");
                let nz = NonZeroU32::new(n).expect("non-zero");
                Self::Positive(nz)
            }
            ..0 => {
                let n = u32::try_from(-value).expect("positive");
                let nz = NonZeroU32::new(n).expect("non-zero");
                Self::Negative(nz)
            }
        }
    }
}

// impl From<SignedMag<NonZeroU16>> for i32 {
//     fn from(mag: SignedMag<NonZeroU16>) -> Self {
//         match mag {
//             SignedMag::Positive(n) => Self::from(n.get()),
//             SignedMag::Negative(n) => -Self::from(n.get()),
//             SignedMag::Zero => 0,
//         }
//     }
// }
