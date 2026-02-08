use crate::GridMask;

/// An error indicating that a mask is not contiguous.
///
/// This error is returned when attempting to create a [`GridShape`](crate::GridShape)
/// when the cells are not contiguous.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("Mask is not contiguous")]
pub struct Discontiguous(pub GridMask);
