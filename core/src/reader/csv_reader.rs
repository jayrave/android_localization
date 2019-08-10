use csv;
use csv::ReaderBuilder;
use crate::localized_string::LocalizedString;
use crate::localized_strings::LocalizedStrings;
use crate::error::Error;
use std::io::Read;

pub fn read<S: Read>(source: S) -> Result<Vec<LocalizedStrings>, Error> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true) // To treat first row specially
        .flexible(false) // Takes care of making sure that all records are of the same size
        .trim(csv::Trim::All) // To skip whitespace around commas
        .from_reader(source); // Read is automatically buffered

    // Get foreign_locales
    let foreign_locales = extract_foreign_locales(reader.headers())?;
    let mut localized_strings_list: Vec<Vec<LocalizedString>> =
        vec![Vec::new(); foreign_locales.len()];

    // Extract localized record
    for record in reader.records() {
        let localized_record = extract_localized_record(&record?)?;
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

    Ok(foreign_locales
        .into_iter()
        .zip(localized_strings_list)
        .map(|(locale, strings)| LocalizedStrings::new(locale, strings))
        .collect())
}

fn extract_foreign_locales(record: csv::Result<&csv::StringRecord>) -> Result<Vec<String>, Error> {
    let record = record?;
    if record.len() < 3 {
        Err(String::from("Too few values in header (at least 3 required)"))?;
    }

    let mut iterator = record.into_iter();
    let header1 = iterator.next().unwrap(); // Safe to unwrap since size is at least 3
    let header2 = iterator.next().unwrap(); // Safe to unwrap since size is at least 3
    let foreign_locales: Vec<String> = iterator.map(String::from).collect();

    if header1 != "string_name" {
        Err(String::from("First header should be named string_name"))?;
    }

    if header2 != "default_locale" {
        Err(String::from("Second header should be named default_locale"))?;
    }

    if foreign_locales.iter().any(|header| header.is_empty()) {
        Err(String::from("Headers can't be empty strings"))?;
    }

    Ok(foreign_locales)
}

fn extract_localized_record(record: &csv::StringRecord) -> Result<LocalizedRecord, Error> {
    // Since `ReaderBuilder` is set to be not flexible, we can be sure
    // that the this record is going to be as long as the headers record
    let mut iterator = record.into_iter();
    let string_name = iterator.next().unwrap_or("");
    let default_value = iterator.next().unwrap_or("");
    let foreign_values = iterator.map(String::from).collect();

    if string_name.is_empty() {
        Err(String::from("string_name can't be empty for any record"))?;
    }

    Ok(LocalizedRecord {
        string_name: String::from(string_name),
        default_value: String::from(default_value),
        foreign_values,
    })
}

#[derive(Debug)]
struct LocalizedRecord {
    string_name: String,
    default_value: String,
    foreign_values: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::localized_string::LocalizedString;
    use crate::localized_strings::LocalizedStrings;
    use std::fs::File;
    use std::io::{Seek, SeekFrom, Write};

    #[test]
    fn strings_are_read_from_valid_file() {
        let mut strings_list = read_strings_from_file(
            r#"string_name, default_locale, french, spanish
            string_1, english 1, french 1, spanish 1
            string_2, english 2, , spanish 2"#,
        )
            .unwrap()
            .into_iter();

        let french_strings = strings_list.next().unwrap();
        let spanish_strings = strings_list.next().unwrap();
        assert_eq!(strings_list.next(), None);

        let mut french_strings_iter = french_strings.strings().iter();
        assert_eq!(
            french_strings_iter.next(),
            Some(&LocalizedString::new(
                String::from("string_1"),
                String::from("english 1"),
                String::from("french 1")
            ))
        );
        assert_eq!(french_strings_iter.next(), None);

        let mut spanish_strings_iter = spanish_strings.strings().iter();
        assert_eq!(
            spanish_strings_iter.next(),
            Some(&LocalizedString::new(
                String::from("string_1"),
                String::from("english 1"),
                String::from("spanish 1")
            ))
        );
        assert_eq!(
            spanish_strings_iter.next(),
            Some(&LocalizedString::new(
                String::from("string_2"),
                String::from("english 2"),
                String::from("spanish 2")
            ))
        );
        assert_eq!(spanish_strings_iter.next(), None);
    }

    #[test]
    fn errors_if_header_values_are_not_as_expected() {
        let error = read_strings_from_file("").unwrap_err();
        assert_eq!(
            error.to_string(),
            "Too few values in header (at least 3 required)"
        );

        let error = read_strings_from_file("header_1, default_locale, french").unwrap_err();
        assert_eq!(
            error.to_string(),
            "First header should be named string_name"
        );

        let error = read_strings_from_file("string_name, header_2, french").unwrap_err();
        assert_eq!(
            error.to_string(),
            "Second header should be named default_locale"
        );

        let error = read_strings_from_file("string_name, default_locale, , lang").unwrap_err();
        assert_eq!(error.to_string(), "Headers can't be empty strings");
    }

    #[test]
    fn errors_if_string_name_is_empty() {
        let error =
            read_strings_from_file("string_name, default_locale, french\n, a, b").unwrap_err();
        assert_eq!(
            error.to_string(),
            "string_name can't be empty for any record"
        );
    }

    fn read_strings_from_file(file_content: &str) -> Result<Vec<LocalizedStrings>, Error> {
        // Write content to file
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        tmpfile.write(file_content.as_bytes()).unwrap();

        // Seek to start
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        // Read strings from file
        super::read(tmpfile.try_clone().unwrap())
    }
}
