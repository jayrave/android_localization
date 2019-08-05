use std::error;
use std::fmt;
use xml::reader;
use xml::writer;

#[derive(Debug)]
pub enum Error {
    LogicError(String),
    XmlReadError(reader::Error),
    XmlWriteError(writer::Error),
}

impl From<writer::Error> for Error {
    fn from(error: writer::Error) -> Self {
        Error::XmlWriteError(error)
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::LogicError(_) => None,
            Error::XmlReadError(e) => Some(e),
            Error::XmlWriteError(e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::LogicError(message) => fmt::Display::fmt(message, f),
            Error::XmlReadError(e) => fmt::Display::fmt(e, f),
            Error::XmlWriteError(e) => fmt::Display::fmt(e, f),
        }
    }
}
