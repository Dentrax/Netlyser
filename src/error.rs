use crate::clap;
use std::{error::Error as StdError, fmt, io, sync::PoisonError};

pub type Result<T> = ::std::result::Result<T, Error>;

pub enum Error {
    Canonicalization(String, io::Error),
    Clap(clap::Error),
    Io(io::Error),
    PoisonedLock,
}

impl StdError for Error {
    fn description(&self) -> &str {
        "a netlyser error"
    }
}


impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Self {
        Error::Clap(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}


impl<'a, T> From<PoisonError<T>> for Error {
    fn from(_err: PoisonError<T>) -> Self {
        Error::PoisonedLock
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (error_type, error) = match self {
            Error::Canonicalization(path, err) => {
                ("Path", format!("couldn't canonicalize '{}':\n{}", path, err))
            }
            Error::Clap(err) => ("Argument", err.to_string()),
            Error::Io(err) => ("I/O", err.to_string()),
            Error::PoisonedLock => ("Internal", "poisoned lock".to_string()),
        };

        write!(f, "{} error: {}", error_type, error)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
