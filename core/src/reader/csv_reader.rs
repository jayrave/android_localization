use std::io::Read;

use csv;
use csv::ReaderBuilder;

use crate::error::Error;
use crate::localized_string::LocalizedString;
use crate::localized_strings::LocalizedStrings;
use android_localization_helpers::DevExpt;
use std::collections::HashSet;

pub fn read<S: Read>(
    source: S,
    allow_only_locales: HashSet<String>,
) -> Result<Vec<LocalizedStrings>, Error> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true) // To treat first row specially
        .flexible(false) // Takes care of making sure that all records are of the same size
        .trim(csv::Trim::All) // To skip whitespace around commas
        .from_reader(source); // Read is automatically buffered

    // Get foreign_locales
    let filtered_headers = extract_filtered_headers(reader.headers()?, allow_only_locales)?;
    let mut localized_strings_list: Vec<Vec<LocalizedString>> =
        vec![Vec::new(); filtered_headers.foreign_locales.len()];

    // Extract localized record
    for record in reader.records() {
        let localized_record =
            extract_localized_record(&record?, &filtered_headers.foreign_indices_allow_flags)?;
        let string_name = localized_record.string_name.clone();
        let default_value = localized_record.default_value.clone();

        // Go through all localized values & add them to respective lists
        for (index, foreign_value) in localized_record.foreign_values.into_iter().enumerate() {
            if !foreign_value.is_empty() {
                // Since we know all records should be of the same length
                // the following `expect` is safe
                let localized_strings = localized_strings_list
                    .get_mut(index)
                    .expect("Oops! Something is wrong");

                localized_strings.push(LocalizedString::new(
                    string_name.clone(),
                    default_value.clone(),
                    foreign_value,
                ))
            }
        }
    }

    Ok(filtered_headers
        .foreign_locales
        .into_iter()
        .zip(localized_strings_list)
        .map(|(locale, strings)| LocalizedStrings::new(locale, strings))
        .collect())
}

fn extract_filtered_headers(
    record: &csv::StringRecord,
    allow_only_locales: HashSet<String>,
) -> Result<FilteredHeaders, Error> {
    if record.len() < 3 {
        Err(String::from(
            "Too few values in header (at least 3 required)",
        ))?;
    }

    let mut iterator = record.into_iter();
    let header1 = iterator
        .next()
        .expt("Already checked the length but still fails!");
    let header2 = iterator
        .next()
        .expt("Already checked the length but still fails!");

    if header1 != "string_name" {
        Err(String::from("First header should be named string_name"))?;
    }

    if header2 != "default_locale" {
        Err(String::from("Second header should be named default_locale"))?;
    }

    let mut foreign_indices_allow_flags = vec![];
    let mut foreign_locales = vec![];
    for foriegn_locale in iterator {
        let foreign_locale = String::from(foriegn_locale);
        let allow_index = allow_only_locales.contains(&foreign_locale);
        foreign_indices_allow_flags.push(allow_index);
        if allow_index {
            foreign_locales.push(foreign_locale);
        }
    }

    Ok(FilteredHeaders {
        foreign_locales,
        foreign_indices_allow_flags,
    })
}

fn extract_localized_record(
    record: &csv::StringRecord,
    foreign_indices_allow_flags: &[bool],
) -> Result<LocalizedRecord, Error> {
    // Since `ReaderBuilder` is set to be not flexible, we can be sure
    // that the this record is going to be as long as the headers record
    let mut iterator = record.into_iter();
    let string_name = iterator.next().unwrap_or("");
    let default_value = iterator.next().unwrap_or("");

    if string_name.is_empty() {
        Err(String::from("string_name can't be empty for any record"))?;
    }

    let mut foreign_values = vec![];
    for (index, foreign_value) in iterator.enumerate() {
        if foreign_indices_allow_flags.get(index) == Some(&true) {
            foreign_values.push(String::from(foreign_value))
        }
    }

    Ok(LocalizedRecord {
        string_name: String::from(string_name),
        default_value: String::from(default_value),
        foreign_values,
    })
}

struct FilteredHeaders {
    foreign_locales: Vec<String>,
    foreign_indices_allow_flags: Vec<bool>,
}

#[derive(Debug)]
struct LocalizedRecord {
    string_name: String,
    default_value: String,
    foreign_values: Vec<String>,
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Seek, SeekFrom, Write};

    use crate::error::Error;
    use crate::localized_string::LocalizedString;
    use crate::localized_strings::LocalizedStrings;

    #[test]
    fn reads_strings_from_valid_file() {
        let mut strings_list = read_strings_from_file(
            r#"string_name, default_locale, french, german, spanish
            string_1, english 1, french 1, german 1, spanish 1
            string_2, english 2, , german 2, spanish 2"#,
            vec!["french", "some_random_thing", "spanish"],
        )
        .unwrap()
        .into_iter();

        let french_strings = strings_list.next().unwrap();
        let spanish_strings = strings_list.next().unwrap();
        assert_eq!(strings_list.next(), None);

        assert_eq!(french_strings.locale(), "french");
        let mut french_strings_iter = french_strings.strings().iter();
        assert_eq!(
            french_strings_iter.next(),
            Some(&LocalizedString::build("string_1", "english 1", "french 1"))
        );
        assert_eq!(french_strings_iter.next(), None);

        assert_eq!(spanish_strings.locale(), "spanish");
        let mut spanish_strings_iter = spanish_strings.strings().iter();
        assert_eq!(
            spanish_strings_iter.next(),
            Some(&LocalizedString::build(
                "string_1",
                "english 1",
                "spanish 1"
            ))
        );
        assert_eq!(
            spanish_strings_iter.next(),
            Some(&LocalizedString::build(
                "string_2",
                "english 2",
                "spanish 2"
            ))
        );
        assert_eq!(spanish_strings_iter.next(), None);
    }

    #[test]
    fn errors_if_enough_header_values_are_not_as_expected() {
        let error =
            read_strings_from_file("string_name, default_locale", vec!["french"]).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Too few values in header (at least 3 required)"
        );
    }

    #[test]
    fn errors_if_first_header_is_not_as_expected() {
        let error =
            read_strings_from_file("header_1, default_locale, french", vec!["french"]).unwrap_err();
        assert_eq!(
            error.to_string(),
            "First header should be named string_name"
        );
    }

    #[test]
    fn errors_if_second_header_is_not_as_expected() {
        let error =
            read_strings_from_file("string_name, header_2, french", vec!["french"]).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Second header should be named default_locale"
        );
    }

    #[test]
    fn errors_if_string_name_is_empty() {
        let error = read_strings_from_file(
            "string_name, default_locale, french\n, a, b",
            vec!["french"],
        )
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "string_name can't be empty for any record"
        );
    }

    fn read_strings_from_file(
        file_content: &str,
        allow_only_locales: Vec<&str>,
    ) -> Result<Vec<LocalizedStrings>, Error> {
        // Write content to file
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        tmpfile.write(file_content.as_bytes()).unwrap();

        // Seek to start
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        // Read strings from file
        super::read(
            tmpfile.try_clone().unwrap(),
            allow_only_locales.into_iter().map(String::from).collect(),
        )
    }
}
