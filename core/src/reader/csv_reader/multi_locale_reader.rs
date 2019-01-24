use csv;
use csv::ReaderBuilder;
use reader::csv_reader::error::Error;
use reader::translated_string::TranslatedString;
use std::collections::HashMap;
use std::io::Read;

pub fn read<S: Read>(source: S) -> Result<HashMap<String, Vec<TranslatedString>>, Error> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true) // To treat first row specially
        .flexible(false) // Takes care of making sure that all records are of the same size
        .trim(csv::Trim::All) // To skip whitespace around commas
        .from_reader(source); // Read is automatically buffered

    // Get foreign_lang_names
    let foreign_lang_names = extract_foreign_lang_names(reader.headers())?;
    let mut localized_strings_list: Vec<Vec<TranslatedString>> =
        vec![Vec::new(); foreign_lang_names.len()];

    // Extract localized record
    for record in reader.records() {
        let localized_record = extract_localized_record(&record?)?;
        let string_name = localized_record.string_name.clone();
        let default_value = localized_record.default_value.clone();

        // Go through all localized values & add them to respective lists
        for (index, foreign_value) in localized_record.foreign_values.into_iter().enumerate() {
            if !foreign_value.is_empty() {
                let mut localized_strings = localized_strings_list
                    .get_mut(index)
                    .expect("Oops! Something is wrong");
                localized_strings.push(TranslatedString::new(
                    string_name.clone(),
                    default_value.clone(),
                    foreign_value,
                ))
            }
        }
    }

    Ok(foreign_lang_names
        .into_iter()
        .zip(localized_strings_list)
        .collect())
}

fn extract_foreign_lang_names(
    record: csv::Result<&csv::StringRecord>,
) -> Result<Vec<String>, Error> {
    let record = record?;
    if record.len() < 3 {
        return Err(Error::SyntaxError(String::from(
            "Too few values in header (at least 3 required)",
        )));
    }

    let mut iterator = record.into_iter();
    let header1 = iterator.next().unwrap(); // Safe to unwrap since size is at least 3
    let header2 = iterator.next().unwrap(); // Safe to unwrap since size is at least 3
    let foreign_lang_names: Vec<String> = iterator.map(String::from).collect();

    if header1 != "string_name" {
        return Err(Error::SyntaxError(String::from(
            "First header should be named string_name",
        )));
    }

    if header2 != "default_locale" {
        return Err(Error::SyntaxError(String::from(
            "Second header should be named default_locale",
        )));
    }

    if foreign_lang_names.iter().any(|header| header.is_empty()) {
        return Err(Error::SyntaxError(String::from(
            "Headers can't be empty strings",
        )));
    }

    Ok(foreign_lang_names)
}

fn extract_localized_record(record: &csv::StringRecord) -> Result<LocalizedRecord, Error> {
    // Since `ReaderBuilder` is set to be not flexible, we can be sure
    // that the this record is going to be as long as the headers record
    let mut iterator = record.into_iter();
    let string_name = iterator.next().unwrap_or("");
    let default_value = iterator.next().unwrap_or("");
    let foreign_values = iterator.map(String::from).collect();

    if string_name.is_empty() {
        return Err(Error::SyntaxError(String::from(
            "string_name can't be empty for any record",
        )));
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
    extern crate tempfile;

    use reader::csv_reader::Error;
    use reader::translated_string::TranslatedString;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{Seek, SeekFrom, Write};

    #[test]
    fn strings_are_read_from_valid_file() {
        let mut strings = read_strings_from_file(
            r#"string_name, default_locale, french, spanish
            string_1, english 1, french 1, spanish 1
            string_2, english 2, , spanish 2"#,
        )
        .unwrap();

        let mut french_strings = strings.remove("french").unwrap().into_iter();
        let mut spanish_strings = strings.remove("spanish").unwrap().into_iter();
        assert!(
            strings.is_empty(),
            "Map was expected to be empty but isn't!"
        );

        assert_eq!(
            french_strings.next(),
            Some(TranslatedString::new(
                String::from("string_1"),
                String::from("english 1"),
                String::from("french 1")
            ))
        );

        assert_eq!(french_strings.next(), None);

        assert_eq!(
            spanish_strings.next(),
            Some(TranslatedString::new(
                String::from("string_1"),
                String::from("english 1"),
                String::from("spanish 1")
            ))
        );

        assert_eq!(
            spanish_strings.next(),
            Some(TranslatedString::new(
                String::from("string_2"),
                String::from("english 2"),
                String::from("spanish 2")
            ))
        );

        assert_eq!(spanish_strings.next(), None);
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

    fn read_strings_from_file(
        file_content: &str,
    ) -> Result<HashMap<String, Vec<TranslatedString>>, Error> {
        // Write content to file
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        tmpfile.write(file_content.as_bytes()).unwrap();

        // Seek to start
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        // Read strings from file
        super::read(tmpfile.try_clone().unwrap())
    }
}
