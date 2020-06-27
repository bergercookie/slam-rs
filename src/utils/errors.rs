pub type SlamResult<T> = std::result::Result<T, SlamError>;

use std::error::Error;
use std::fmt;

// -------------------------------------------------------------------------------------------------
// SlamError
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct SlamError {
    kind: SlamErrorKind,
}

impl SlamError {
    pub fn new(kind: SlamErrorKind) -> SlamError {
        SlamError { kind }
    }

    pub fn kind(&self) -> &SlamErrorKind {
        &self.kind
    }
}

impl Error for SlamError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}

impl fmt::Display for SlamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("kalimera!")
    }
}

// -------------------------------------------------------------------------------------------------
// SlamErrorKind
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum SlamErrorKind {
    /// Error during CLI Parsing
    /// Provide the name of the flag and an exlanation string
    InvalidCLI(String, String),
    /// Catch-all error for invalid inputs
    InvalidInput(String),
    __Nonexhaustive,
}

impl Error for SlamErrorKind {}

impl fmt::Display for SlamErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlamErrorKind::InvalidCLI(flag, reason) => {
                write!(f, "Invalid Usage of flag '{}' - {}", flag, reason)
            }
            SlamErrorKind::InvalidInput(input) => write!(f, "Invalid input provided {}", input),
            SlamErrorKind::__Nonexhaustive => unreachable!(),
        }
    }
}
