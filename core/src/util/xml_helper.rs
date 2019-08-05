use crate::android_string::AndroidString;
use crate::constants;
use crate::reader::xml_reader;
use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::ops::Add;
use std::path::Path;

type FileWithPath = (File, String);

pub fn read_default_strings(res_dir_path: &Path) -> Result<Vec<AndroidString>, Error> {
    let (file, path) = open_default_strings_file(res_dir_path)?;
    match xml_reader::read(file) {
        Err(error) => Err(Error {
            path,
            kind: ErrorKind::XmlError(error),
        }),
        Ok(strings) => Ok(strings),
    }
}

pub fn read_foreign_strings(res_dir_path: &Path, lang_id: &str) -> Result<StringsWithPath, Error> {
    let (file, path) = open_foreign_strings_file(res_dir_path, lang_id)?;
    match xml_reader::read(file) {
        Err(error) => Err(Error {
            path,
            kind: ErrorKind::XmlError(error),
        }),
        Ok(strings) => Ok(StringsWithPath { path, strings }),
    }
}

fn open_default_strings_file(res_dir_path: &Path) -> Result<FileWithPath, Error> {
    open_strings_file(res_dir_path, constants::fs::BASE_VALUES_DIR_NAME)
}

fn open_foreign_strings_file(res_dir_path: &Path, lang_id: &str) -> Result<FileWithPath, Error> {
    let values_dir_name = String::from(constants::fs::BASE_VALUES_DIR_NAME);
    let values_dir_name = values_dir_name.add(&format!("-{}", lang_id));
    open_strings_file(res_dir_path, &values_dir_name)
}

fn open_strings_file(res_dir_path: &Path, values_dir_name: &str) -> Result<FileWithPath, Error> {
    let mut strings_file_path = res_dir_path.to_path_buf();
    strings_file_path.push(values_dir_name);
    strings_file_path.push(constants::fs::STRING_FILE_NAME);

    let path = String::from(strings_file_path.to_string_lossy());
    Ok((
        File::open(strings_file_path).map_err(|e| Error {
            path: path.clone(),
            kind: ErrorKind::IoError(e),
        })?,
        path,
    ))
}

pub struct StringsWithPath {
    path: String,
    strings: Vec<AndroidString>,
}

impl StringsWithPath {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn into_strings(self) -> Vec<AndroidString> {
        self.strings
    }
}

#[derive(Debug)]
pub struct Error {
    path: String,
    kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    IoError(io::Error),
    XmlError(xml_reader::Error),
}

impl Error {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn into_kind(self) -> ErrorKind {
        self.kind
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match &self.kind {
            ErrorKind::IoError(error) => Some(error),
            ErrorKind::XmlError(error) => Some(error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Path: {}; Error: ", self.path)?;
        match &self.kind {
            ErrorKind::IoError(error) => fmt::Display::fmt(error, f),
            ErrorKind::XmlError(error) => fmt::Display::fmt(error, f),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};

    #[test]
    fn open_strings_file_errors_if_values_dir_is_missing() {
        let res_dir = tempfile::tempdir().unwrap();
        let error = super::open_strings_file(res_dir.path(), "values");
        assert!(error
            .unwrap_err()
            .to_string()
            .contains("No such file or directory"))
    }

    #[test]
    fn open_strings_file_errors_if_strings_file_is_missing() {
        let res_dir = tempfile::tempdir().unwrap();

        let mut values_dir_path = res_dir.path().to_path_buf();
        values_dir_path.push("values");
        fs::create_dir(values_dir_path).unwrap();

        let error = super::open_strings_file(res_dir.path(), "values");
        assert!(error
            .unwrap_err()
            .to_string()
            .contains("No such file or directory"))
    }

    #[test]
    fn open_default_strings_file() {
        let res_dir = tempfile::tempdir().unwrap();

        let mut values_dir_path = res_dir.path().to_path_buf();
        values_dir_path.push("values");

        let mut strings_file_path = values_dir_path.clone();
        strings_file_path.push("strings.xml");

        fs::create_dir(values_dir_path).unwrap();
        let mut tmpfile: File = File::create(strings_file_path.clone()).unwrap();
        tmpfile.write("example content".as_bytes()).unwrap();

        let mut file_contents = String::new();
        let (mut file, file_path) = super::open_default_strings_file(res_dir.path()).unwrap();
        file.read_to_string(&mut file_contents).unwrap();

        assert_eq!(file_contents, "example content");
        assert_eq!(file_path, strings_file_path.to_str().unwrap());
    }

    #[test]
    fn open_foreign_strings_file() {
        let res_dir = tempfile::tempdir().unwrap();

        let mut values_dir_path = res_dir.path().to_path_buf();
        values_dir_path.push("values-fr");

        let mut strings_file_path = values_dir_path.clone();
        strings_file_path.push("strings.xml");

        fs::create_dir(values_dir_path).unwrap();
        let mut tmpfile: File = File::create(strings_file_path.clone()).unwrap();
        tmpfile.write("example content".as_bytes()).unwrap();

        let mut file_contents = String::new();
        let (mut file, file_path) = super::open_foreign_strings_file(res_dir.path(), "fr").unwrap();
        file.read_to_string(&mut file_contents).unwrap();

        assert_eq!(file_contents, "example content");
        assert_eq!(file_path, strings_file_path.to_str().unwrap());
    }
}
