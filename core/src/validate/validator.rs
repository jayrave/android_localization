use std::path::Path;

use crate::error::Error;
use crate::util::foreign_locale_ids_finder;
use crate::util::xml_utilities;
use crate::util::xml_utilities::StringsWithPath;
use crate::validate::apostrophe;
use crate::validate::format_string;
use crate::validate::format_string::ParsedData;

/// Runs all validations for default & all foreign strings & returns a collection
/// of file names on which the validations were run
pub fn validate(res_dir_path: &str) -> Result<Result<Vec<String>, Vec<InvalidStringsFile>>, Error> {
    let mut path_of_validated_files = vec![];
    let mut invalid_strings_files = vec![];

    let default_strings_with_path = xml_utilities::read_default_strings(Path::new(res_dir_path))?;
    let mut default_parsed_data =
        format_string::parse_and_build_data(&default_strings_with_path.strings());
    validate_default_strings(
        default_strings_with_path,
        &mut path_of_validated_files,
        &mut invalid_strings_files,
    );

    let res_dir_path_string = res_dir_path;
    let locale_ids = foreign_locale_ids_finder::find(res_dir_path_string)?;
    for locale_id in locale_ids {
        validate_foreign_strings(
            xml_utilities::read_foreign_strings(Path::new(res_dir_path), &locale_id)?,
            &mut default_parsed_data,
            &mut path_of_validated_files,
            &mut invalid_strings_files,
        )
    }

    if invalid_strings_files.is_empty() {
        Ok(Ok(path_of_validated_files))
    } else {
        Ok(Err(invalid_strings_files))
    }
}

fn validate_default_strings(
    strings_with_path: StringsWithPath,
    path_of_validated_files: &mut Vec<String>,
    invalid_strings_files: &mut Vec<InvalidStringsFile>,
) {
    let default_strings_file_path = String::from(strings_with_path.path());
    let apos_result = apostrophe::validate(&strings_with_path.into_strings());
    if apos_result.is_err() {
        invalid_strings_files.push(InvalidStringsFile {
            file_path: default_strings_file_path,
            apostrophe_error: Some(apos_result.unwrap_err()),
            format_string_error: None,
        })
    } else {
        path_of_validated_files.push(default_strings_file_path)
    }
}

fn validate_foreign_strings(
    strings_with_path: StringsWithPath,
    mut default_parsed_data: &mut [ParsedData],
    path_of_validated_files: &mut Vec<String>,
    invalid_strings_files: &mut Vec<InvalidStringsFile>,
) {
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

#[derive(Debug, PartialEq)]
pub struct InvalidStringsFile {
    pub file_path: String,
    pub apostrophe_error: Option<apostrophe::InvalidStrings>,
    pub format_string_error: Option<format_string::Mismatches>,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::path::PathBuf;

    use test_utilities;

    use crate::android_string::AndroidString;
    use crate::validate::apostrophe;
    use crate::validate::format_string;
    use crate::validate::validator::InvalidStringsFile;
    use crate::writer::xml_writer;

    #[test]
    fn validates() {
        let tempdir = tempfile::tempdir().unwrap();
        let mut res_dir_path = tempdir.path().to_path_buf();
        res_dir_path.push("res");

        let mut default_values_dir_path = res_dir_path.clone();
        default_values_dir_path.push("values");
        fs::create_dir_all(&default_values_dir_path).unwrap();
        let (mut default_strings_file, default_strings_file_path) =
            create_strings_file_in(&default_values_dir_path);

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
            vec![AndroidString::localizable("s1", "value")],
        )
        .unwrap();

        xml_writer::write(
            &mut french_strings_file,
            vec![AndroidString::localizable("s1", "value")],
        )
        .unwrap();

        xml_writer::write(
            &mut spanish_strings_file,
            vec![AndroidString::localizable("s1", "value")],
        )
        .unwrap();

        let mut actual_output = super::validate(res_dir_path.to_str().unwrap())
            .unwrap()
            .unwrap();

        // This is to make sure that `fs` iteration order doesn't matter
        actual_output.sort();

        test_utilities::assert_strict_list_eq(
            actual_output,
            vec![
                spanish_strings_file_path,
                french_strings_file_path,
                default_strings_file_path,
            ],
        )
    }

    #[test]
    fn errors() {
        let tempdir = tempfile::tempdir().unwrap();
        let mut res_dir_path = tempdir.path().to_path_buf();
        res_dir_path.push("res");

        let mut default_values_dir_path = res_dir_path.clone();
        default_values_dir_path.push("values");
        fs::create_dir_all(&default_values_dir_path).unwrap();
        let (mut default_strings_file, default_strings_file_path) =
            create_strings_file_in(&default_values_dir_path);

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

        let default_s1 = AndroidString::localizable("s1", "value");
        let default_s2 = AndroidString::localizable("s2", "v'alue");
        xml_writer::write(
            &mut default_strings_file,
            vec![default_s1.clone(), default_s2.clone()],
        )
        .unwrap();

        let french_s1 = AndroidString::localizable("s1", "v'alue");
        xml_writer::write(&mut french_strings_file, vec![french_s1.clone()]).unwrap();

        let spanish_s2 = AndroidString::localizable("s2", "v'alue %1$d");
        xml_writer::write(&mut spanish_strings_file, vec![spanish_s2.clone()]).unwrap();

        let mut invalid_strings_files = super::validate(res_dir_path.to_str().unwrap())
            .unwrap()
            .unwrap_err();

        // This is to make sure that `fs` iteration order doesn't matter
        invalid_strings_files.sort_by(|a, b| a.file_path.cmp(&b.file_path));

        test_utilities::assert_strict_list_eq(
            invalid_strings_files,
            vec![
                InvalidStringsFile {
                    file_path: spanish_strings_file_path,
                    apostrophe_error: Some(apostrophe::InvalidStrings {
                        invalid_strings: vec![spanish_s2.clone()],
                    }),
                    format_string_error: Some(format_string::Mismatches {
                        mismatches: vec![format_string::Mismatch {
                            default_parsed_data: format_string::ParsedData {
                                android_string: default_s2.clone(),
                                sorted_format_strings: vec![],
                            },
                            foreign_parsed_data: format_string::ParsedData {
                                android_string: spanish_s2,
                                sorted_format_strings: vec![String::from("%1$d")],
                            },
                        }],
                    }),
                },
                InvalidStringsFile {
                    file_path: french_strings_file_path,
                    apostrophe_error: Some(apostrophe::InvalidStrings {
                        invalid_strings: vec![french_s1],
                    }),
                    format_string_error: None,
                },
                InvalidStringsFile {
                    file_path: default_strings_file_path,
                    apostrophe_error: Some(apostrophe::InvalidStrings {
                        invalid_strings: vec![default_s2],
                    }),
                    format_string_error: None,
                },
            ],
        )
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
