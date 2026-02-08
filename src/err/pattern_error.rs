use crate::err::Discontiguous;

/// Errors that can occur when parsing a [`GridMask`] from a `str` pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PatternError {
    /// The pattern contains more than 64 valid characters.
    #[error("Pattern content too long: expected exactly 64 characters")]
    TooLong,
    /// The pattern contains fewer than 64 valid characters.
    #[error("Pattern content too short: expected 64 characters, found {0}")]
    TooShort(usize),
    /// The pattern contains an invalid character.
    #[error("Invalid character '{0}' in pattern")]
    InvalidChar(char),
}

/// Errors that can occur when parsing a [`GridShape`] from a `str` pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ShapePatternError {
    /// An error that occurred while parsing the pattern.
    #[error(transparent)]
    Pattern(#[from] PatternError),
    /// The pattern contains disconnected cells.
    #[error(transparent)]
    Discontiguous(#[from] Discontiguous),
}
