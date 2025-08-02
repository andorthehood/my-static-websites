use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    /// IO operation error
    Io(io::Error),
    /// Error that occurs during liquid processing
    Liquid(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error: {err}"),
            Error::Liquid(msg) => write!(f, "Liquid error: {msg}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::Liquid(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Liquid(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
