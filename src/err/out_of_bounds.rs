/// An error indicating that a value is out of bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("Value out of bounds")]
pub struct OutOfBounds;

impl OutOfBounds {
    /// Creates a new [`OutOfBounds`] from any value.
    pub(crate) fn from<T>(_: T) -> Self {
        Self
    }
}
