use std::fmt::{Display, Formatter};

/// User-level errors that can be thrown at runtime
#[derive(Debug)]
pub enum Error {
    /// unused error for testing
    _Sample,
}

impl std::error::Error for Error {}

/// Generic result type which binds the error to be an instance of the `Error` enum
pub type _Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::_Sample => write!(f, "sample error"),
        }
    }
}
