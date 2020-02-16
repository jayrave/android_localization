use std::fmt::Error;
use std::fmt::Write;

use android_localization_utilities::DevExpt;

use crate::validate::validator::InvalidStringsFile;

pub fn format_to_string(invalid_strings_files: Vec<InvalidStringsFile>) -> Result<String, Error> {
    let files_count = invalid_strings_files.len();
    let mut issues_count = 0;
    let mut output = String::new();

    for (index, invalid_strings_file) in invalid_strings_files.into_iter().enumerate() {
        if index > 0 {
            writeln!(&mut output)?;
        }

        issues_count += format_errors_from_one_file(invalid_strings_file, &mut output)?
    }

    let pluralized_issue = if issues_count <= 1 { "issue" } else { "issues" };
    let pluralized_file = if files_count <= 1 { "file" } else { "files" };

    write!(
        &mut output,
        "\nFound {} {} across {} {}!",
        issues_count, pluralized_issue, files_count, pluralized_file
    )?;

    Ok(output)
}

fn format_errors_from_one_file(
    invalid_strings_file: InvalidStringsFile,
    mut output: &mut String,
) -> Result<usize, Error> {
    let mut file_output = String::new();
    let mut issues_count_in_file = 0;
    if invalid_strings_file.apostrophe_error.is_some() {
        for invalid_string in invalid_strings_file
            .apostrophe_error
            .expt("Apostrophe error without invalid strings!")
            .invalid_strings
        {
            issues_count_in_file += 1;
            writeln!(
                &mut file_output,
                "Error {} (unescaped apostrophe): {}",
                issues_count_in_file,
                invalid_string.value()
            )?;
        }
    }

    if invalid_strings_file.format_string_error.is_some() {
        for mismatch in invalid_strings_file
            .format_string_error
            .expt("Format string error without mismatches!")
            .mismatches
        {
            issues_count_in_file += 1;

            // To make sure that the errors array line up as much as possible
            let number_placeholder = match issues_count_in_file {
                0..=9 => " ",
                10..=99 => "  ",
                100..=999 => "   ",
                _ => "    ",
            };

            writeln!(
                &mut file_output,
                "Error {} (mismatched format string(s)): Found [{}] in {}",
                issues_count_in_file,
                mismatch
                    .foreign_parsed_data
                    .sorted_format_strings
                    .join(", "),
                mismatch.foreign_parsed_data.android_string.value()
            )?;
            writeln!(
                &mut file_output,
                "      {}                                Found [{}] in {}",
                number_placeholder,
                mismatch
                    .default_parsed_data
                    .sorted_format_strings
                    .join(", "),
                mismatch.default_parsed_data.android_string.value()
            )?;
        }
    }

    if let Some(missing_strings) = invalid_strings_file.missing_strings_error {
        if !missing_strings.extra_in_default_locale.is_empty() {
            issues_count_in_file += 1;
            writeln!(
                &mut file_output,
                "Error {} (unlocalized string(s)): {}",
                issues_count_in_file,
                missing_strings
                    .extra_in_default_locale
                    .iter()
                    .map(|s| s.value())
                    .collect::<Vec<&str>>()
                    .join(", ")
            )?;
        }

        if !missing_strings.extra_in_foreign_locale.is_empty() {
            issues_count_in_file += 1;
            writeln!(
                &mut file_output,
                "Error {} (string(s) not in defaut locale): {}",
                issues_count_in_file,
                missing_strings
                    .extra_in_foreign_locale
                    .iter()
                    .map(|s| s.value())
                    .collect::<Vec<&str>>()
                    .join(", ")
            )?;
        }
    }

    let pluralized_issue = if issues_count_in_file <= 1 {
        "issue"
    } else {
        "issues"
    };

    writeln!(
        &mut output,
        "Path: {} ({} {})",
        invalid_strings_file.file_path, issues_count_in_file, pluralized_issue
    )?;
    write!(&mut output, "{}", file_output)?;

    Ok(issues_count_in_file)
}

#[cfg(test)]
mod tests {
    use crate::android_string::AndroidString;
    use crate::validate::apostrophe;
    use crate::validate::format_string;
    use crate::validate::missing_strings;
    use crate::validate::validator::InvalidStringsFile;

    #[test]
    fn formats() {
        let default_s1 = AndroidString::localizable("s1", "default_value1");
        let default_s2 = AndroidString::localizable("s2", "default_value2");
        let french_s1 = AndroidString::localizable("s1", "french_value1");
        let spanish_s1 = AndroidString::localizable("s1", "spanish_value1");
        let spanish_s2 = AndroidString::localizable("s2", "spanish_value2");

        let invalid_strings_file = vec![
            InvalidStringsFile {
                file_path: String::from("default"),
                apostrophe_error: Some(apostrophe::InvalidStrings {
                    invalid_strings: vec![default_s1.clone()],
                }),
                format_string_error: None,
                missing_strings_error: None,
            },
            InvalidStringsFile {
                file_path: String::from("french"),
                apostrophe_error: None,
                missing_strings_error: None,
                format_string_error: Some(format_string::Mismatches {
                    mismatches: vec![format_string::Mismatch {
                        default_parsed_data: format_string::ParsedData {
                            android_string: default_s1.clone(),
                            sorted_format_strings: vec![],
                        },
                        foreign_parsed_data: format_string::ParsedData {
                            android_string: french_s1,
                            sorted_format_strings: vec![String::from("asdf"), String::from("qwer")],
                        },
                    }],
                }),
            },
            InvalidStringsFile {
                file_path: String::from("spanish"),
                apostrophe_error: Some(apostrophe::InvalidStrings {
                    invalid_strings: vec![spanish_s1.clone()],
                }),
                format_string_error: Some(format_string::Mismatches {
                    mismatches: vec![format_string::Mismatch {
                        default_parsed_data: format_string::ParsedData {
                            android_string: default_s1.clone(),
                            sorted_format_strings: vec![String::from("%1$s"), String::from("%1$d")],
                        },
                        foreign_parsed_data: format_string::ParsedData {
                            android_string: spanish_s1.clone(),
                            sorted_format_strings: vec![String::from("%1$d")],
                        },
                    }],
                }),
                missing_strings_error: Some(missing_strings::MissingStrings {
                    extra_in_default_locale: vec![default_s1, default_s2],
                    extra_in_foreign_locale: vec![spanish_s1, spanish_s2],
                }),
            },
        ];

        assert_eq!(
            super::format_to_string(invalid_strings_file).unwrap(),
            String::from(
                r#"Path: default (1 issue)
Error 1 (unescaped apostrophe): default_value1

Path: french (1 issue)
Error 1 (mismatched format string(s)): Found [asdf, qwer] in french_value1
                                       Found [] in default_value1

Path: spanish (4 issues)
Error 1 (unescaped apostrophe): spanish_value1
Error 2 (mismatched format string(s)): Found [%1$d] in spanish_value1
                                       Found [%1$s, %1$d] in default_value1
Error 3 (unlocalized string(s)): default_value1, default_value2
Error 4 (string(s) not in defaut locale): spanish_value1, spanish_value2

Found 6 issues across 3 files!"#
            )
        );
    }
}
