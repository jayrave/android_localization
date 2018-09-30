use android_string::AndroidString;
use constants;
use reader::xml_reader;
use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::ops::Add;
use std::path::Path;

pub fn read_default_strings(res_dir_path: &Path) -> Result<Vec<AndroidString>, Error> {
    let file = open_default_strings_file(res_dir_path).map_err(Error::IoError)?;
    match xml_reader::reader::from(file) {
        Err(error) => Err(Error::XmlError(error)),
        Ok(strings) => Ok(strings),
    }
}

pub fn read_foreign_strings(
    res_dir_path: &Path,
    lang_id: &str,
) -> Result<Vec<AndroidString>, Error> {
    let file = open_foreign_strings_file(res_dir_path, lang_id).map_err(Error::IoError)?;
    match xml_reader::reader::from(file) {
        Err(error) => Err(Error::XmlError(error)),
        Ok(strings) => Ok(strings),
    }
}

fn open_default_strings_file(res_dir_path: &Path) -> Result<File, io::Error> {
    open_strings_file(res_dir_path, constants::fs::BASE_VALUES_DIR_NAME)
}

fn open_foreign_strings_file(res_dir_path: &Path, lang_id: &str) -> Result<File, io::Error> {
    let values_dir_name = String::from(constants::fs::BASE_VALUES_DIR_NAME);
    let values_dir_name = values_dir_name.add(&format!("-{}", lang_id));
    open_strings_file(res_dir_path, &values_dir_name)
}

fn open_strings_file(res_dir_path: &Path, values_dir_name: &str) -> Result<File, io::Error> {
    let mut strings_file_path = res_dir_path.to_path_buf();
    strings_file_path.push(values_dir_name);
    strings_file_path.push(constants::fs::STRING_FILE_NAME);
    File::open(strings_file_path)
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

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};

    #[test]
    fn open_strings_file_errors_if_values_dir_is_missing() {
        let res_dir = tempfile::tempdir().unwrap();
        let error = super::open_strings_file(res_dir.path(), "values");
        assert!(
            error
                .unwrap_err()
                .to_string()
                .starts_with("No such file or directory")
        )
    }

    #[test]
    fn open_strings_file_errors_if_strings_file_is_missing() {
        let res_dir = tempfile::tempdir().unwrap();

        let mut values_dir_path = res_dir.path().to_path_buf();
        values_dir_path.push("values");
        fs::create_dir(values_dir_path).unwrap();

        let error = super::open_strings_file(res_dir.path(), "values");
        assert!(
            error
                .unwrap_err()
                .to_string()
                .starts_with("No such file or directory")
        )
    }

    #[test]
    fn open_default_strings_file() {
        let res_dir = tempfile::tempdir().unwrap();

        let mut values_dir_path = res_dir.path().to_path_buf();
        values_dir_path.push("values");

        let mut strings_file_path = values_dir_path.clone();
        strings_file_path.push("strings.xml");

        fs::create_dir(values_dir_path).unwrap();
        let mut tmpfile: File = File::create(strings_file_path).unwrap();
        tmpfile.write("example content".as_bytes()).unwrap();

        let mut file_contents = String::new();
        super::open_default_strings_file(res_dir.path())
            .unwrap()
            .read_to_string(&mut file_contents)
            .unwrap();

        assert_eq!(file_contents, "example content");
    }

    #[test]
    fn open_foreign_strings_file() {
        let res_dir = tempfile::tempdir().unwrap();

        let mut values_dir_path = res_dir.path().to_path_buf();
        values_dir_path.push("values-fr");

        let mut strings_file_path = values_dir_path.clone();
        strings_file_path.push("strings.xml");

        fs::create_dir(values_dir_path).unwrap();
        let mut tmpfile: File = File::create(strings_file_path).unwrap();
        tmpfile.write("example content".as_bytes()).unwrap();

        let mut file_contents = String::new();
        super::open_foreign_strings_file(res_dir.path(), "fr")
            .unwrap()
            .read_to_string(&mut file_contents)
            .unwrap();

        assert_eq!(file_contents, "example content");
    }
}
