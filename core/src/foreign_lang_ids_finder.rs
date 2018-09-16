use constants;
use regex::Regex;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

lazy_static! {
    static ref LANG_ID_REGEX: Regex = Regex::new("-([a-zA-z]+)$").unwrap();
}

/// Finds language IDs from the folder names. Only folders whose name are of the
/// format `values-...` that have a file with the name `strings.xml` are considered.
/// What is after the last `-` in the folder name is returned as the lang id
pub fn find(res_dir_path: &str) -> Result<Vec<String>, Error> {
    if !Path::new(res_dir_path).is_dir() {
        return Err(Error {
            error: io::Error::new(
                io::ErrorKind::NotFound,
                format!("Res dir ({}) doesn't exist!", res_dir_path),
            ),
        });
    }

    let lang_ids = fs::read_dir(res_dir_path)?
        .filter_map(|dir_entry| match dir_entry {
            Err(_) => None,
            Ok(dir_entry) => match dir_entry.file_type() {
                Err(_) => None,
                Ok(file_type) => if !file_type.is_dir() {
                    None
                } else {
                    let mut strings_file_path = dir_entry.path();
                    strings_file_path.push(constants::fs::STRING_FILE_NAME);
                    if !strings_file_path.is_file() {
                        None
                    } else {
                        dir_entry.file_name().to_str().map(|s| String::from(s))
                    }
                },
            },
        })
        .filter_map(|file_name| match LANG_ID_REGEX.captures(&file_name) {
            None => None,
            Some(capture) => match capture.get(1) {
                None => None,
                Some(m) => Some(String::from(m.as_str())),
            },
        })
        .collect();

    Ok(lang_ids)
}

#[derive(Debug)]
pub struct Error {
    pub error: io::Error,
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error { error }
    }
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use std::fs;
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::path::Path;
    use std::path::PathBuf;

    #[test]
    fn errors_if_res_dir_does_not_exist() {
        let tempdir = tempfile::tempdir().unwrap();
        let mut res_dir_path = tempdir.path().to_path_buf();
        res_dir_path.push("res");

        let error = super::find(res_dir_path.to_str().unwrap());
        assert_eq!(
            error.unwrap_err().error.to_string(),
            format!(
                "Res dir ({}) doesn't exist!",
                res_dir_path.to_str().unwrap()
            )
        )
    }

    #[test]
    fn finds_foreign_lang_ids() {
        let tempdir = tempfile::tempdir().unwrap();
        let mut res_dir_path = tempdir.path().to_path_buf();
        res_dir_path.push("res");

        let mut default_values_dir_path = res_dir_path.clone();
        default_values_dir_path.push("values");
        fs::create_dir_all(&default_values_dir_path).unwrap();
        create_strings_file_in(&default_values_dir_path);

        let mut french_values_dir_path = res_dir_path.clone();
        french_values_dir_path.push("values-fr");
        fs::create_dir_all(&french_values_dir_path).unwrap();
        create_strings_file_in(&french_values_dir_path);

        let mut spanish_values_dir_path = res_dir_path.clone();
        spanish_values_dir_path.push("values-es");
        fs::create_dir_all(&spanish_values_dir_path).unwrap();

        let mut italian_values_dir_path = res_dir_path.clone();
        italian_values_dir_path.push("values-it");
        fs::create_dir_all(&italian_values_dir_path).unwrap();
        create_strings_file_in(&italian_values_dir_path);

        let mut lang_ids = super::find(res_dir_path.to_str().unwrap())
            .unwrap()
            .into_iter();

        let lang_id_1 = lang_ids.next().unwrap();
        let lang_id_2 = lang_ids.next().unwrap();
        assert_eq!(lang_ids.next(), None);
        assert!(lang_id_1 == "fr" || lang_id_1 == "it");
        assert!(lang_id_2 == "fr" || lang_id_2 == "it");
    }

    fn create_strings_file_in(dir_path: &PathBuf) {
        let mut strings_file_path = dir_path.clone();
        strings_file_path.push("strings.xml");
        File::create(strings_file_path).unwrap();
    }
}
