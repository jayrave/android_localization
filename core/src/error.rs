use std::error;
use std::fmt;
use std::io;

/// Do not implement any `From` for `Error` since we want all `Error`s to carry
/// some context (which most usually is the path of the file with issues)
#[derive(Debug)]
pub struct Error {
    pub(crate) context: String,
    pub(crate) kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    Csv(csv::Error),
    Io(io::Error),
    Message(String),
    XmlRead(xml::reader::Error),
    XmlWrite(xml::writer::Error),
}

/// Components that don't know the path, should return this which could be
/// converted into an `Error` with the appropriate context
#[derive(Debug)]
pub struct InnerError {
    kind: ErrorKind,
}

impl Error {
    pub fn new<S: Into<String>, E: Into<InnerError>>(context: S, error: E) -> Error {
        error.into().into_error(context)
    }

    pub fn context(&self) -> &str {
        &self.context
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
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
        write!(f, "{}: ", self.context)?;
        fmt::Display::fmt(&self.kind, f)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ErrorKind::Csv(error) => fmt::Display::fmt(error, f),
            ErrorKind::Io(error) => fmt::Display::fmt(error, f),
            ErrorKind::Message(message) => fmt::Display::fmt(message, f),
            ErrorKind::XmlRead(error) => fmt::Display::fmt(error, f),
            ErrorKind::XmlWrite(error) => fmt::Display::fmt(error, f),
        }
    }
}

impl fmt::Display for InnerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.kind, f)
    }
}

impl InnerError {
    pub fn into_error<S: Into<String>>(self, context: S) -> Error {
        Error {
            context: context.into(),
            kind: self.kind,
        }
    }
}

impl From<csv::Error> for InnerError {
    fn from(error: csv::Error) -> Self {
        InnerError {
            kind: ErrorKind::Csv(error),
        }
    }
}

impl From<io::Error> for InnerError {
    fn from(error: io::Error) -> Self {
        InnerError {
            kind: ErrorKind::Io(error),
        }
    }
}

impl From<String> for InnerError {
    fn from(message: String) -> Self {
        InnerError {
            kind: ErrorKind::Message(message),
        }
    }
}

impl From<&str> for InnerError {
    fn from(message: &str) -> Self {
        InnerError::from(String::from(message))
    }
}

impl From<xml::reader::Error> for InnerError {
    fn from(error: xml::reader::Error) -> Self {
        InnerError {
            kind: ErrorKind::XmlRead(error),
        }
    }
}

impl From<xml::writer::Error> for InnerError {
    fn from(error: xml::writer::Error) -> Self {
        InnerError {
            kind: ErrorKind::XmlWrite(error),
        }
    }
}

/// To easily add context to errors
pub trait ResultExt<T> {
    fn with_context<S: Into<String>>(self, context: S) -> Result<T, Error>;
}

impl<T, E: Into<InnerError>> ResultExt<T> for Result<T, E> {
    fn with_context<S: Into<String>>(self, context: S) -> Result<T, Error> {
        self.map_err(|error| error.into().into_error(context))
    }
}
