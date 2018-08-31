use csv;
use csv::ReaderBuilder;
use reader::csv_reader::error::Error;
use std::io::Read;
use translatable_android_string::TranslatableAndroidString;

pub fn from<R: Read>(read: R) -> Result<Vec<TranslatableAndroidString>, Error> {
    let mut strings = vec![];
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(read); // Read is automatically buffered

    for record_or_error in reader.records() {
        match record_or_error {
            Err(error) => return Err(Error::CsvError(error)),
            Ok(record) => strings.push(extract_string_from_record(record)?),
        }
    }

    Ok(strings)
}

fn extract_string_from_record(
    record: csv::StringRecord,
) -> Result<TranslatableAndroidString, Error> {
    let mut iterator = record.iter();
    let name = iterator.next();
    let value = iterator.next();
    let extra = iterator.next();

    if name.is_none() {
        return Err(Error::SyntaxError(format!("Empty record!")));
    }

    if value.is_none() {
        return Err(Error::SyntaxError(format!(
            "Too few values in record => \"{}\"",
            name.unwrap()
        )));
    }

    if extra.is_some() {
        return Err(Error::SyntaxError(format!(
            "Too many values in record. 3rd field => \"{}\"",
            extra.unwrap()
        )));
    }

    Ok(TranslatableAndroidString::new(
        String::from(name.unwrap()),
        String::from(value.unwrap()),
    ))
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use reader::csv_reader::error::Error;
    use std::fs::File;
    use std::io::{Seek, SeekFrom, Write};
    use translatable_android_string::TranslatableAndroidString;

    #[test]
    fn strings_are_read_from_valid_file() {
        let mut strings = read_strings_from_file(
            "string_1, string 1 value\nstring_2, string 2 value",
        ).unwrap()
            .into_iter();

        assert_eq!(
            strings.next(),
            Some(TranslatableAndroidString::new(
                String::from("string_1"),
                String::from("string 1 value")
            ))
        );

        assert_eq!(
            strings.next(),
            Some(TranslatableAndroidString::new(
                String::from("string_2"),
                String::from("string 2 value")
            ))
        );

        assert_eq!(strings.next(), None);
    }

    #[test]
    fn errors_for_file_with_record_having_too_few_values() {
        let error = read_strings_from_file("string_1");
        assert_eq!(
            error.unwrap_err().to_string(),
            format!("Too few values in record => \"{}\"", "string_1")
        )
    }

    #[test]
    fn errors_for_file_with_record_having_too_many_values() {
        let error = read_strings_from_file("string_1, string 1 value, useless value");
        assert_eq!(
            error.unwrap_err().to_string(),
            format!(
                "Too many values in record. 3rd field => \"{}\"",
                "useless value"
            )
        )
    }

    fn read_strings_from_file(file_content: &str) -> Result<Vec<TranslatableAndroidString>, Error> {
        // Write content to file
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        tmpfile.write(file_content.as_bytes()).unwrap();

        // Seek to start
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        // Read strings from file
        super::from(tmpfile.try_clone().unwrap())
    }
}
