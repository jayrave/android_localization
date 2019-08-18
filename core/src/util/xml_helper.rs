use std::fs::File;
use std::ops::Add;
use std::path::Path;

use crate::android_string::AndroidString;
use crate::constants;
use crate::error::Error;
use crate::error::ResultExt;
use crate::reader::xml_reader;

type FileWithPath = (File, String);

pub fn read_default_strings(res_dir_path: &Path) -> Result<StringsWithPath, Error> {
    read_strings(open_default_strings_file(res_dir_path)?)
}

pub fn read_foreign_strings(
    res_dir_path: &Path,
    locale_id: &str,
) -> Result<StringsWithPath, Error> {
    read_strings(open_foreign_strings_file(res_dir_path, locale_id)?)
}

fn read_strings(file_with_path: FileWithPath) -> Result<StringsWithPath, Error> {
    let (file, path) = file_with_path;
    xml_reader::read(file)
        .with_context(path.clone())
        .map(|strings| StringsWithPath { path, strings })
}

fn open_default_strings_file(res_dir_path: &Path) -> Result<FileWithPath, Error> {
    open_strings_file(res_dir_path, constants::fs::BASE_VALUES_DIR_NAME)
}

fn open_foreign_strings_file(res_dir_path: &Path, locale_id: &str) -> Result<FileWithPath, Error> {
    let values_dir_name = String::from(constants::fs::BASE_VALUES_DIR_NAME);
    let values_dir_name = values_dir_name.add(&format!("-{}", locale_id));
    open_strings_file(res_dir_path, &values_dir_name)
}

fn open_strings_file(res_dir_path: &Path, values_dir_name: &str) -> Result<FileWithPath, Error> {
    let mut strings_file_path = res_dir_path.to_path_buf();
    strings_file_path.push(values_dir_name);
    strings_file_path.push(constants::fs::STRING_FILE_NAME);

    let path = String::from(strings_file_path.to_string_lossy());
    File::open(strings_file_path)
        .with_context(path.clone())
        .map(|file| (file, path))
}

pub struct StringsWithPath {
    path: String,
    strings: Vec<AndroidString>,
}

impl StringsWithPath {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn strings(&self) -> &[AndroidString] {
        &self.strings
    }

    pub fn into_strings(self) -> Vec<AndroidString> {
        self.strings
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
