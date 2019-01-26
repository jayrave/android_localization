use csv;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    CsvError(csv::Error),
    IoError(io::Error),
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::CsvError(error) => Some(error),
            Error::IoError(error) => Some(error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CsvError(error) => fmt::Display::fmt(error, f),
            Error::IoError(error) => fmt::Display::fmt(error, f),
        }
    }
}