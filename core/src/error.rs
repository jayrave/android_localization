use crate::reader::xml_reader;
use crate::util::xml_helper;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct Error {
    context: Option<String>,
    kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    Csv(csv::Error),
    Io(io::Error),
    Message(String),
    XmlRead(xml::reader::Error),
    XmlWrite(xml::writer::Error),
}

impl Error {
    pub fn context(&self) -> &Option<String> {
        &self.context
    }
}

/// To easily add context to errors
pub trait ResultExt<T> {
    fn with_context(self, context: String) -> Result<T, Error>;
}

impl<T, E: Into<Error>> ResultExt<T> for Result<T, E> {
    fn with_context(self, context: String) -> Result<T, Error> {
        self.map_err(|err| Into::into(err)).map_err(|err| Error {
            context: Some(context),
            ..err
        })
    }
}

impl From<csv::Error> for Error {
    fn from(error: csv::Error) -> Self {
        Error {
            context: None,
            kind: ErrorKind::Csv(error),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error {
            context: None,
            kind: ErrorKind::Io(error),
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error {
            context: None,
            kind: ErrorKind::Message(message),
        }
    }
}

impl From<xml::reader::Error> for Error {
    fn from(error: xml::reader::Error) -> Self {
        Error {
            context: None,
            kind: ErrorKind::XmlRead(error),
        }
    }
}

impl From<xml::writer::Error> for Error {
    fn from(error: xml::writer::Error) -> Self {
        Error {
            context: None,
            kind: ErrorKind::XmlWrite(error),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match &self.kind {
            ErrorKind::Csv(error) => Some(error),
            ErrorKind::Io(error) => Some(error),
            ErrorKind::Message(_message) => None,
            ErrorKind::XmlRead(error) => Some(error),
            ErrorKind::XmlWrite(error) => Some(error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(context) = &self.context {
            write!(f, "{}: ", context)?;
        }

        match &self.kind {
            ErrorKind::Csv(error) => fmt::Display::fmt(error, f),
            ErrorKind::Io(error) => fmt::Display::fmt(error, f),
            ErrorKind::Message(message) => fmt::Display::fmt(message, f),
            ErrorKind::XmlRead(error) => fmt::Display::fmt(error, f),
            ErrorKind::XmlWrite(error) => fmt::Display::fmt(error, f),
        }
    }
}
