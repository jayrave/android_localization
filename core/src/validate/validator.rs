use std::path::Path;

use crate::android_string::AndroidString;
use crate::error::Error;
use crate::util::foreign_locale_ids_finder;
use crate::util::xml_utilities;
use crate::util::xml_utilities::StringsWithPath;
use crate::validate::apostrophe;
use crate::validate::format_string;
use crate::validate::format_string::ParsedData;
use crate::validate::missing_strings;

/// Runs all validations for default & all foreign strings & returns a collection
/// of file names on which the validations were run
pub fn validate(
    res_dir_path: &str,
    fail_on_unlocalized: bool,
) -> Result<Result<Vec<String>, Vec<InvalidStringsFile>>, Error> {
    let mut path_of_validated_files = vec![];
    let mut invalid_strings_files = vec![];

    let default_strings_with_path = xml_utilities::read_default_strings(Path::new(res_dir_path))?;
    let mut default_parsed_data =
        format_string::parse_and_build_data(&default_strings_with_path.strings());

    validate_default_strings(
        &default_strings_with_path,
        &mut path_of_validated_files,
        &mut invalid_strings_files,
    );

    let mut default_strings = default_strings_with_path.into_strings();
    let res_dir_path_string = res_dir_path;
    let locale_ids = foreign_locale_ids_finder::find(res_dir_path_string)?;
    for locale_id in locale_ids {
        validate_foreign_strings(
            xml_utilities::read_foreign_strings(Path::new(res_dir_path), &locale_id)?,
            &mut default_strings,
            &mut default_parsed_data,
            fail_on_unlocalized,
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
    strings_with_path: &StringsWithPath,
    path_of_validated_files: &mut Vec<String>,
    invalid_strings_files: &mut Vec<InvalidStringsFile>,
) {
    let default_strings_file_path = String::from(strings_with_path.path());
    let apos_result = apostrophe::validate(strings_with_path.strings());
    if let Err(apos_error) = apos_result {
        invalid_strings_files.push(InvalidStringsFile {
            file_path: default_strings_file_path,
            apostrophe_error: Some(apos_error),
            format_string_error: None,
            missing_strings_error: None,
        })
    } else {
        path_of_validated_files.push(default_strings_file_path)
    }
}

fn validate_foreign_strings(
    strings_with_path: StringsWithPath,
    mut default_strings: &mut [AndroidString],
    mut default_parsed_data: &mut [ParsedData],
    fail_on_unlocalized: bool,
    path_of_validated_files: &mut Vec<String>,
    invalid_strings_files: &mut Vec<InvalidStringsFile>,
) {
    let foreign_strings_file_path = String::from(strings_with_path.path());
    let mut foreign_strings = strings_with_path.into_strings();

    let apos_result = apostrophe::validate(&foreign_strings);
    let fs_result = format_string::validate(&mut default_parsed_data, &mut foreign_strings);
    let ms_result = missing_strings::validate(&mut default_strings, &mut foreign_strings);

    let mut potential_invalid_file = InvalidStringsFile::new(foreign_strings_file_path.clone());

    if let Err(apos_error) = apos_result {
        potential_invalid_file.apostrophe_error = Some(apos_error);
    }

    if let Err(fs_error) = fs_result {
        potential_invalid_file.format_string_error = Some(fs_error);
    }

    if fail_on_unlocalized {
        if let Err(ms_error) = ms_result {
            potential_invalid_file.missing_strings_error = Some(ms_error);
        }
    }

    if potential_invalid_file.has_errors() {
        invalid_strings_files.push(potential_invalid_file)
    } else {
        path_of_validated_files.push(foreign_strings_file_path)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct InvalidStringsFile {
    pub file_path: String,
    pub apostrophe_error: Option<apostrophe::InvalidStrings>,
    pub format_string_error: Option<format_string::Mismatches>,
    pub missing_strings_error: Option<missing_strings::MissingStrings>,
}

impl InvalidStringsFile {
    fn new(file_path: String) -> InvalidStringsFile {
        InvalidStringsFile {
            file_path,
            ..Default::default()
        }
    }

    fn has_errors(&self) -> bool {
        self.apostrophe_error.is_some()
            || self.format_string_error.is_some()
            || self.missing_strings_error.is_some()
    }
}

#[cfg(test)]
mod tests {
    use test_utilities;

    use crate::android_string::AndroidString;
    use crate::validate::apostrophe;
    use crate::validate::format_string;
    use crate::validate::missing_strings;
    use crate::validate::validator::InvalidStringsFile;
    use crate::writer::xml_writer;

    #[test]
    fn validates() {
        let tempdir = tempfile::tempdir().unwrap();
        let mut res_path = tempdir.path().to_path_buf();
        res_path.push("res");

        let mut default_strings =
            test_utilities::res::setup_empty_strings_for_default_locale(res_path.clone());
        let mut french_strings =
            test_utilities::res::setup_empty_strings_for_locale(res_path.clone(), "fr");
        let mut spanish_strings =
            test_utilities::res::setup_empty_strings_for_locale(res_path.clone(), "es");

        xml_writer::write(
            &mut default_strings.file,
            vec![AndroidString::localizable("s1", "value")],
        )
        .unwrap();

        xml_writer::write(
            &mut french_strings.file,
            vec![AndroidString::localizable("s1", "value")],
        )
        .unwrap();

        xml_writer::write(
            &mut spanish_strings.file,
            vec![AndroidString::localizable("s1", "value")],
        )
        .unwrap();

        let mut actual_output = super::validate(res_path.to_str().unwrap(), true)
            .unwrap()
            .unwrap();

        // This is to make sure that `fs` iteration order doesn't matter
        actual_output.sort();

        test_utilities::list::assert_strict_list_eq(
            actual_output,
            vec![
                spanish_strings.path,
                french_strings.path,
                default_strings.path,
            ],
        )
    }

    #[test]
    fn errors_without_skipping_missing_errors() {
        test_errors(true)
    }

    #[test]
    fn errors_skipping_missing_errors() {
        test_errors(false)
    }

    fn test_errors(fail_on_unlocalized: bool) {
        let tempdir = tempfile::tempdir().unwrap();
        let mut res_path = tempdir.path().to_path_buf();
        res_path.push("res");

        let mut default_strings =
            test_utilities::res::setup_empty_strings_for_default_locale(res_path.clone());
        let mut french_strings =
            test_utilities::res::setup_empty_strings_for_locale(res_path.clone(), "fr");
        let mut spanish_strings =
            test_utilities::res::setup_empty_strings_for_locale(res_path.clone(), "es");

        let default_s1 = AndroidString::localizable("s1", "value");
        let default_s2 = AndroidString::localizable("s2", "v'alue");
        xml_writer::write(
            &mut default_strings.file,
            vec![default_s1.clone(), default_s2.clone()],
        )
        .unwrap();

        let french_s1 = AndroidString::localizable("s1", "v'alue");
        xml_writer::write(&mut french_strings.file, vec![french_s1.clone()]).unwrap();

        let spanish_s2 = AndroidString::localizable("s2", "v'alue %1$d");
        xml_writer::write(&mut spanish_strings.file, vec![spanish_s2.clone()]).unwrap();

        let mut invalid_strings_files =
            super::validate(res_path.to_str().unwrap(), fail_on_unlocalized)
                .unwrap()
                .unwrap_err();

        let missing_strings_error_for_fr: Option<missing_strings::MissingStrings>;
        let missing_strings_error_for_es = if fail_on_unlocalized {
            missing_strings_error_for_fr = Some(missing_strings::MissingStrings {
                extra_in_foreign_locale: vec![],
                extra_in_default_locale: vec![default_s2.clone()],
            });

            Some(missing_strings::MissingStrings {
                extra_in_foreign_locale: vec![],
                extra_in_default_locale: vec![default_s1],
            })
        } else {
            missing_strings_error_for_fr = None;
            None
        };

        // This is to make sure that `fs` iteration order doesn't matter
        invalid_strings_files.sort_by(|a, b| a.file_path.cmp(&b.file_path));

        test_utilities::list::assert_strict_list_eq(
            invalid_strings_files,
            vec![
                InvalidStringsFile {
                    file_path: spanish_strings.path,
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
                    missing_strings_error: missing_strings_error_for_es,
                },
                InvalidStringsFile {
                    file_path: french_strings.path,
                    apostrophe_error: Some(apostrophe::InvalidStrings {
                        invalid_strings: vec![french_s1],
                    }),
                    format_string_error: None,
                    missing_strings_error: missing_strings_error_for_fr,
                },
                InvalidStringsFile {
                    file_path: default_strings.path,
                    apostrophe_error: Some(apostrophe::InvalidStrings {
                        invalid_strings: vec![default_s2],
                    }),
                    format_string_error: None,
                    missing_strings_error: None,
                },
            ],
        )
    }
}
