use std::collections::HashMap;
use std::fs;
use std::path::Path;

use regex::Regex;

use crate::constants;
use crate::error::Error;
use crate::error::ResultExt;

lazy_static::lazy_static! {
    static ref LOCALE_ID_REGEX: Regex = Regex::new("-([a-zA-z]+)$").unwrap();
}

/// Finds language IDs from the folder names. Only folders whose name are of the
/// format `values-...` that have a file with the name `strings.xml` are considered.
/// What is after the last `-` in the folder name is returned as the lang id
pub fn find(res_dir_path: &str) -> Result<Vec<String>, Error> {
    if !Path::new(res_dir_path).is_dir() {
        return Err(From::from(format!(
            "Res dir({}) doesn't exist",
            res_dir_path
        )));
    }

    let locale_ids = fs::read_dir(res_dir_path)
        .with_context(String::from(res_dir_path))?
        .filter_map(|dir_entry| match dir_entry {
            Err(_) => None,
            Ok(dir_entry) => match dir_entry.file_type() {
                Err(_) => None,
                Ok(file_type) => {
                    if !file_type.is_dir() {
                        None
                    } else {
                        let mut strings_file_path = dir_entry.path();
                        strings_file_path.push(constants::fs::STRING_FILE_NAME);
                        if !strings_file_path.is_file() {
                            None
                        } else {
                            dir_entry.file_name().to_str().map(String::from)
                        }
                    }
                }
            },
        })
        .filter_map(|file_name| match LOCALE_ID_REGEX.captures(&file_name) {
            None => None,
            Some(capture) => match capture.get(1) {
                None => None,
                Some(m) => Some(String::from(m.as_str())),
            },
        })
        .collect();

    Ok(locale_ids)
}

/// Look @ `find`'s doc to figure out how the lang IDs are figured out
pub fn build_map_if_empty_or_return<S: ::std::hash::BuildHasher>(
    mut map: HashMap<String, String, S>,
    res_dir_path: &str,
) -> Result<HashMap<String, String, S>, Error> {
    if map.is_empty() {
        for locale_id in find(res_dir_path)? {
            map.insert(locale_id.clone(), locale_id);
        }
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::path::PathBuf;

    #[test]
    fn find_errors_if_res_dir_does_not_exist() {
        let tempdir = tempfile::tempdir().unwrap();
        let mut res_dir_path = tempdir.path().to_path_buf();
        res_dir_path.push("res");

        let error = super::find(res_dir_path.to_str().unwrap()).unwrap_err();
        assert_eq!(
            error.to_string(),
            format!("Res dir({}) doesn't exist", res_dir_path.to_str().unwrap())
        )
    }

    #[test]
    fn find_finds_foreign_locale_ids() {
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

        let mut locale_ids = super::find(res_dir_path.to_str().unwrap())
            .unwrap()
            .into_iter();

        let locale_id_1 = locale_ids.next().unwrap();
        let locale_id_2 = locale_ids.next().unwrap();
        assert_eq!(locale_ids.next(), None);
        assert!(locale_id_1 == "fr" || locale_id_1 == "it");
        assert!(locale_id_2 == "fr" || locale_id_2 == "it");
    }

    #[test]
    fn build_map_if_empty_or_return() {
        let mut map = HashMap::new();
        map.insert(String::from("a"), String::from("a"));
        assert_eq!(
            super::build_map_if_empty_or_return(map.clone(), "").unwrap(),
            map
        )
    }

    #[test]
    fn build_map_if_empty_or_return_builds_map() {
        let tempdir = tempfile::tempdir().unwrap();
        let mut res_dir_path = tempdir.path().to_path_buf();
        res_dir_path.push("res");

        let mut french_values_dir_path = res_dir_path.clone();
        french_values_dir_path.push("values-fr");
        fs::create_dir_all(&french_values_dir_path).unwrap();
        create_strings_file_in(&french_values_dir_path);

        let mut italian_values_dir_path = res_dir_path.clone();
        italian_values_dir_path.push("values-it");
        fs::create_dir_all(&italian_values_dir_path).unwrap();
        create_strings_file_in(&italian_values_dir_path);

        let mut map = HashMap::new();
        map.insert(String::from("fr"), String::from("fr"));
        map.insert(String::from("it"), String::from("it"));
        assert_eq!(
            super::build_map_if_empty_or_return(
                HashMap::new(),
                res_dir_path.to_str().unwrap()
            )
            .unwrap(),
            map
        )
    }

    fn create_strings_file_in(dir_path: &PathBuf) {
        let mut strings_file_path = dir_path.clone();
        strings_file_path.push("strings.xml");
        File::create(strings_file_path).unwrap();
    }
}
