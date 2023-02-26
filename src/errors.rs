use std::fmt;

/// Result type for the library
pub type Result<T> = std::result::Result<T, Error>;

/// Error enum, used to return errors from the library.
#[derive(Debug)]
pub enum Error {
    /// Error from the hound crate
    Hound(hound::Error),
    /// IO error, such as file not found
    Io(std::io::Error),
    /// Larg file size error (maxnimum file size is 4 GB)
    LargeFileSize,
    /// Invalid wav file error
    InvalidWavFile(String),
}

impl From<hound::Error> for Error {
    fn from(err: hound::Error) -> Self {
        Error::Hound(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Hound(err) => write!(f, "Hound error: {}", err),
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::LargeFileSize => write!(f, "File size is too large, maximum file size is 4 GB"),
            Error::InvalidWavFile(msg) => write!(f, "Invalid wav file, {}", msg),
        }
    }
}
