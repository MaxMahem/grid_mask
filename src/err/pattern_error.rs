use crate::err::Discontiguous;

/// Errors parsing a [`str`] pattern into a [`GridMask`](crate::GridMask) or
/// [`ArrayGrid`](crate::ArrayGrid).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PatternError {
    /// The pattern contains more characters than expected.
    #[error("Pattern content too long")]
    TooLong,
    /// The pattern contains fewer characters than expected.
    #[error("Pattern content too short, found {0}")]
    TooShort(u32),
    /// The pattern contains an invalid character.
    #[error("Invalid character '{0}' in pattern")]
    InvalidChar(char),
}

/// Errors parsing a [`str`] pattern into a [`GridShape`](crate::GridShape).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ShapePatternError {
    /// An error that occurred while parsing the pattern.
    #[error(transparent)]
    Pattern(#[from] PatternError),
    /// The pattern contains disconnected cells.
    #[error(transparent)]
    Discontiguous(#[from] Discontiguous),
}
