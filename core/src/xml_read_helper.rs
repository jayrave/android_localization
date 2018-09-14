use android_string::AndroidString;
use file_helper;
use reader::xml_reader;
use std::error;
use std::fmt;
use std::io;
use std::path::Path;

pub fn read_default_strings(res_dir_path: &Path) -> Result<Vec<AndroidString>, Error> {
    let file = file_helper::open_default_strings_file(res_dir_path).map_err(|e| Error::IoError(e))?;
    match xml_reader::reader::from(file) {
        Err(error) => Err(Error::XmlError(error)),
        Ok(strings) => Ok(strings),
    }
}

pub fn read_foreign_strings(
    res_dir_path: &Path,
    lang_id: &str,
) -> Result<Vec<AndroidString>, Error> {
    let file = file_helper::open_foreign_strings_file(res_dir_path, lang_id)
        .map_err(|e| Error::IoError(e))?;
    match xml_reader::reader::from(file) {
        Err(error) => Err(Error::XmlError(error)),
        Ok(strings) => Ok(strings),
    }
}

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    XmlError(xml_reader::Error),
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::IoError(error) => Some(error),
            Error::XmlError(error) => Some(error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IoError(error) => fmt::Display::fmt(error, f),
            Error::XmlError(error) => fmt::Display::fmt(error, f),
        }
    }
}
