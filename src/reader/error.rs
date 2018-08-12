use std::error;
use std::fmt;
use std::io;
use xml::reader;

#[derive(Debug)]
pub enum Error {
	IoError(io::Error), 
	XmlError(reader::Error), 
	SyntaxError(String)
}

impl error::Error for Error {
	fn cause(&self) -> Option<&error::Error> {
		match self {
			Error::XmlError(error) => Some(error),
			Error::IoError(error) => Some(error), 
			Error::SyntaxError(message) => None
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::XmlError(error) => fmt::Display::fmt(error, f),
			Error::IoError(error) => fmt::Display::fmt(error, f), 
			Error::SyntaxError(message) => fmt::Display::fmt(message, f)
		}
	}
}
