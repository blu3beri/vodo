use std::fmt::{Display, Formatter};

/// User-level errors that can be thrown at runtime
#[derive(Debug)]
pub enum Error {
    /// Unable to create a file
    UnableToCreateFile,
    /// Unable to write the output of the note to the file
    UnableToSaveFile,
}

impl std::error::Error for Error {}

/// Generic result type which binds the error to be an instance of the `Error` enum
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnableToCreateFile => write!(f, "Unable to create file"),
            Error::UnableToSaveFile => write!(f, "Unable to save file"),
        }
    }
}
