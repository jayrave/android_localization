use crate::util::xml_helper;
use crate::error::Error;
use std::fmt;
use std::path::Path;
use crate::util::foreign_lang_ids_finder;
use crate::validate::apostrophe;
use crate::validate::format_string;

/// Runs all validations for all foreign strings & returns a collection
/// of file names on which the validations were run
pub fn do_the_thing(res_dir_path: &str) -> Result<Result<Vec<String>, Vec<InvalidStringsFile>>, Error> {
    let res_dir_path_string = res_dir_path;
    let res_dir_path = Path::new(res_dir_path);
    let default_strings = xml_helper::read_default_strings(res_dir_path)?;
    let lang_ids = foreign_lang_ids_finder::find(res_dir_path_string)?;

    let mut path_of_validated_files = vec![];
    let mut invalid_strings_files = vec![];
    let mut default_parsed_data = format_string::parse_and_build_data(&default_strings);

    for lang_id in lang_ids {
        let strings_with_path = xml_helper::read_foreign_strings(res_dir_path, &lang_id)?;
        let foreign_strings_file_path = String::from(strings_with_path.path());
        let mut foreign_strings = strings_with_path.into_strings();

        let apos_result = apostrophe::validate(&foreign_strings);
        let fs_result = format_string::validate(&mut default_parsed_data, &mut foreign_strings);

        let invalid = if apos_result.is_err() && fs_result.is_err() {
            Some(InvalidStringsFile {
                file_path: foreign_strings_file_path.clone(),
                apostrophe_error: Some(apos_result.unwrap_err()),
                format_string_error: Some(fs_result.unwrap_err()),
            })
        } else if apos_result.is_err() {
            Some(InvalidStringsFile {
                file_path: foreign_strings_file_path.clone(),
                apostrophe_error: Some(apos_result.unwrap_err()),
                format_string_error: None,
            })
        } else if fs_result.is_err() {
            Some(InvalidStringsFile {
                file_path: foreign_strings_file_path.clone(),
                apostrophe_error: None,
                format_string_error: Some(fs_result.unwrap_err()),
            })
        } else {
            None
        };

        if let Some(invalid_file) = invalid {
            invalid_strings_files.push(invalid_file)
        } else {
            path_of_validated_files.push(foreign_strings_file_path)
        }
    }

    if invalid_strings_files.is_empty() {
        Ok(Ok(path_of_validated_files))
    } else {
        Ok(Err(invalid_strings_files))
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidStringsFile {
    file_path: String,
    apostrophe_error: Option<apostrophe::InvalidStrings>,
    format_string_error: Option<format_string::Mismatches>,
}

#[cfg(test)]
mod tests {
    use crate::android_string::AndroidString;
    use std::cmp;
    use std::fmt;
    use std::fs;
    use std::fs::File;
    use std::path::PathBuf;
    use crate::writer::xml_writer;
    use crate::validate::apostrophe;
    use crate::validate::format_string;
    use crate::validate::validator::InvalidStringsFile;

    #[test]
    fn returns_list_of_file_names() {
        let tempdir = tempfile::tempdir().unwrap();
        let mut res_dir_path = tempdir.path().to_path_buf();
        res_dir_path.push("res");

        let mut default_values_dir_path = res_dir_path.clone();
        default_values_dir_path.push("values");
        fs::create_dir_all(&default_values_dir_path).unwrap();
        let mut default_strings_file = create_strings_file_in(&default_values_dir_path).0;

        let mut french_values_dir_path = res_dir_path.clone();
        french_values_dir_path.push("values-fr");
        fs::create_dir_all(&french_values_dir_path).unwrap();
        let (mut french_strings_file, french_strings_file_path) =
            create_strings_file_in(&french_values_dir_path);

        let mut spanish_values_dir_path = res_dir_path.clone();
        spanish_values_dir_path.push("values-es");
        fs::create_dir_all(&spanish_values_dir_path).unwrap();
        let (mut spanish_strings_file, spanish_strings_file_path) =
            create_strings_file_in(&spanish_values_dir_path);

        xml_writer::write(
            &mut default_strings_file,
            vec![AndroidString::new(
                String::from("s1"),
                String::from("value"),
                true,
            )],
        )
        .unwrap();

        xml_writer::write(
            &mut french_strings_file,
            vec![AndroidString::new(
                String::from("s1"),
                String::from("value"),
                true,
            )],
        )
        .unwrap();

        xml_writer::write(
            &mut spanish_strings_file,
            vec![AndroidString::new(
                String::from("s1"),
                String::from("value"),
                true,
            )],
        )
        .unwrap();

        assert_eq!(
            super::do_the_thing(res_dir_path.to_str().unwrap()).unwrap().unwrap(),
            vec![french_strings_file_path, spanish_strings_file_path],
        );
    }

    #[test]
    fn validations_issues_are_raised() {
        let tempdir = tempfile::tempdir().unwrap();
        let mut res_dir_path = tempdir.path().to_path_buf();
        res_dir_path.push("res");

        let mut default_values_dir_path = res_dir_path.clone();
        default_values_dir_path.push("values");
        fs::create_dir_all(&default_values_dir_path).unwrap();
        let mut default_strings_file = create_strings_file_in(&default_values_dir_path).0;

        let mut french_values_dir_path = res_dir_path.clone();
        french_values_dir_path.push("values-fr");
        fs::create_dir_all(&french_values_dir_path).unwrap();
        let (mut french_strings_file, french_strings_file_path) =
            create_strings_file_in(&french_values_dir_path);

        let mut spanish_values_dir_path = res_dir_path.clone();
        spanish_values_dir_path.push("values-es");
        fs::create_dir_all(&spanish_values_dir_path).unwrap();
        let (mut spanish_strings_file, spanish_strings_file_path) =
            create_strings_file_in(&spanish_values_dir_path);

        let default_s1 = AndroidString::new(String::from("s1"), String::from("value"), true);
        let default_s2 = AndroidString::new(String::from("s2"), String::from("v'alue"), true);
        xml_writer::write(
            &mut default_strings_file,
            vec![default_s1.clone(), default_s2.clone()]
        )
        .unwrap();

        let french_s1 = AndroidString::new(String::from("s1"), String::from("v'alue"), true);
        xml_writer::write(
            &mut french_strings_file,
            vec![french_s1.clone()],
        )
        .unwrap();

        let spanish_s2 = AndroidString::new(String::from("s2"), String::from("v'alue %1$d"), true);
        xml_writer::write(
            &mut spanish_strings_file,
            vec![spanish_s2.clone()],
        )
        .unwrap();

        let invalid_strings_file = super::do_the_thing(res_dir_path.to_str().unwrap()).unwrap().unwrap_err();
        let spanish_errors = format!("File path: {}; apostrophe errors: [(Localizable: true; Name: s1; Value: v'alue)]; format string errors: []\n", spanish_strings_file_path);
        let french_errors = format!("File path: {}; apostrophe errors: [(Localizable: true; Name: s1; Value: val'ue %1$s)(Localizable: true; Name: s2; Value: v'alue %1$d)]; format string errors: [(Format string mismatch. Found format strings () in default (value) & found format strings (%1$s) in foreign (val'ue %1$s))(Format string mismatch. Found format strings () in default (v'alue) & found format strings (%1$d) in foreign (v'alue %1$d))]\n", french_strings_file_path);
        assert_eq!(
            invalid_strings_file,
            vec![
                InvalidStringsFile {
                    file_path: french_strings_file_path,
                    apostrophe_error: Some(apostrophe::InvalidStrings {
                        invalid_strings: vec![french_s1]
                    }),
                    format_string_error: None
                },
                InvalidStringsFile {
                    file_path: spanish_strings_file_path,
                    apostrophe_error: Some(apostrophe::InvalidStrings {
                        invalid_strings: vec![spanish_s2.clone()]
                    }),
                    format_string_error: Some(format_string::Mismatches {
                        mismatches: vec![format_string::Mismatch {
                            default_parsed_data: format_string::ParsedData {
                                android_string: default_s2,
                                sorted_format_strings: vec![]
                            },
                            foreign_parsed_data: format_string::ParsedData {
                                android_string: spanish_s2,
                                sorted_format_strings: vec![String::from("%1$d")]
                            }
                        }]
                    })
                }
            ]
        );
    }

    fn create_strings_file_in(dir_path: &PathBuf) -> (File, String) {
        let mut strings_file_path = dir_path.clone();
        strings_file_path.push("strings.xml");
        (
            File::create(strings_file_path.clone()).unwrap(),
            String::from(strings_file_path.clone().to_str().unwrap()),
        )
    }
}
