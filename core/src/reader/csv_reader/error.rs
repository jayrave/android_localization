use csv;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    CsvError(csv::Error),
    SyntaxError(String),
}

impl From<csv::Error> for Error {
    fn from(error: csv::Error) -> Self {
        Error::CsvError(error)
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::CsvError(error) => Some(error),
            Error::SyntaxError(_message) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CsvError(error) => fmt::Display::fmt(error, f),
            Error::SyntaxError(message) => fmt::Display::fmt(message, f),
        }
    }
}
