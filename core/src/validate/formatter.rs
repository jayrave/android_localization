use crate::validate::validator::InvalidStringsFile;
use std::fmt::Error;
use std::fmt::Write;

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
            .unwrap()
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
        for mismatch in invalid_strings_file.format_string_error.unwrap().mismatches {
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
                "Error {} (mismatched format string): Found [{}] in {}",
                issues_count_in_file,
                mismatch
                    .foreign_parsed_data
                    .sorted_format_strings
                    .join(", "),
                mismatch.foreign_parsed_data.android_string.value()
            )?;
            writeln!(
                &mut file_output,
                "      {}                             Found [{}] in {}",
                number_placeholder,
                mismatch
                    .default_parsed_data
                    .sorted_format_strings
                    .join(", "),
                mismatch.default_parsed_data.android_string.value()
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

mod tests {
    use crate::android_string::AndroidString;
    use crate::validate::apostrophe;
    use crate::validate::format_string;
    use crate::validate::validator::InvalidStringsFile;

    #[test]
    fn formats() {
        let default_s1 =
            AndroidString::new(String::from("s1"), String::from("default_value"), true);
        let french_s1 = AndroidString::new(String::from("s1"), String::from("french_value"), true);
        let spanish_s1 =
            AndroidString::new(String::from("s1"), String::from("spanish_value"), true);

        let invalid_strings_file = vec![
            InvalidStringsFile {
                file_path: String::from("default"),
                apostrophe_error: Some(apostrophe::InvalidStrings {
                    invalid_strings: vec![default_s1.clone()],
                }),
                format_string_error: None,
            },
            InvalidStringsFile {
                file_path: String::from("french"),
                apostrophe_error: None,
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
                            android_string: default_s1,
                            sorted_format_strings: vec![String::from("%1$s"), String::from("%1$d")],
                        },
                        foreign_parsed_data: format_string::ParsedData {
                            android_string: spanish_s1,
                            sorted_format_strings: vec![String::from("%1$d")],
                        },
                    }],
                }),
            },
        ];

        assert_eq!(
            super::format_to_string(invalid_strings_file).unwrap(),
            String::from(
                r#"Path: default (1 issue)
Error 1 (unescaped apostrophe): default_value

Path: french (1 issue)
Error 1 (mismatched format string): Found [asdf, qwer] in french_value
                                    Found [] in default_value

Path: spanish (2 issues)
Error 1 (unescaped apostrophe): spanish_value
Error 2 (mismatched format string): Found [%1$d] in spanish_value
                                    Found [%1$s, %1$d] in default_value

Found 4 issues across 3 files!"#
            )
        );
    }
}
