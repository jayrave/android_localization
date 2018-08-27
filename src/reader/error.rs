use std::error;
use std::fmt;
use std::io;
use xml::reader;

#[derive(Debug)]
pub enum Error {
	IoError(io::Error),
    LogicError(String),
	SyntaxError(String),
    XmlError(reader::Error)
}

impl error::Error for Error {
	fn cause(&self) -> Option<&error::Error> {
		match self {
			Error::IoError(error) => Some(error),
            Error::LogicError(_message) => None,
			Error::SyntaxError(_message) => None,
			Error::XmlError(error) => Some(error)
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::IoError(error) => fmt::Display::fmt(error, f),
            Error::LogicError(message) => fmt::Display::fmt(message, f),
			Error::SyntaxError(message) => fmt::Display::fmt(message, f),
			Error::XmlError(error) => fmt::Display::fmt(error, f),
		}
	}
}
