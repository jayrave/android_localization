use csv;
use csv::ReaderBuilder;
use reader::csv_reader::error::Error;
use reader::localized_string::LocalizedString;
use std::io::Read;

pub fn from<R: Read>(read: R) -> Result<Vec<LocalizedString>, Error> {
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

fn extract_string_from_record(record: csv::StringRecord) -> Result<LocalizedString, Error> {
    let mut iterator = record.iter();
    let name = iterator.next();
    let default_value = iterator.next();
    let localized_value = iterator.next();
    let extra = iterator.next();

    if name.is_none() {
        return Err(Error::SyntaxError(format!("Empty record!")));
    }

    if default_value.is_none() {
        return Err(Error::SyntaxError(format!(
            "Too few values in record (exactly 3 required). 1st field => \"{}\"",
            name.unwrap()
        )));
    }

    if localized_value.is_none() {
        return Err(Error::SyntaxError(format!(
            "Too few values in record (exactly 3 required). 2nd field => \"{}\"",
            default_value.unwrap()
        )));
    }

    if extra.is_some() {
        return Err(Error::SyntaxError(format!(
            "Too many values in record (exactly 3 required). 4th field => \"{}\"",
            extra.unwrap()
        )));
    }

    Ok(LocalizedString::new(
        String::from(name.unwrap()),
        String::from(default_value.unwrap()),
        String::from(localized_value.unwrap()),
    ))
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use reader::csv_reader::error::Error;
    use reader::localized_string::LocalizedString;
    use std::fs::File;
    use std::io::{Seek, SeekFrom, Write};

    #[test]
    fn strings_are_read_from_valid_file() {
        let mut strings = read_strings_from_file(
            "string_1, english 1, french 1\nstring_2, english 2, french 2",
        ).unwrap()
            .into_iter();

        assert_eq!(
            strings.next(),
            Some(LocalizedString::new(
                String::from("string_1"),
                String::from("english 1"),
                String::from("french 1")
            ))
        );

        assert_eq!(
            strings.next(),
            Some(LocalizedString::new(
                String::from("string_2"),
                String::from("english 2"),
                String::from("french 2")
            ))
        );

        assert_eq!(strings.next(), None);
    }

    #[test]
    fn errors_for_file_with_record_having_only_1_value() {
        let error = read_strings_from_file("english 1");
        assert_eq!(
            error.unwrap_err().to_string(),
            "Too few values in record (exactly 3 required). 1st field => \"english 1\""
        )
    }

    #[test]
    fn errors_for_file_with_record_having_only_2_values() {
        let error = read_strings_from_file("english 1, french 1");
        assert_eq!(
            error.unwrap_err().to_string(),
            "Too few values in record (exactly 3 required). 2nd field => \"french 1\""
        )
    }

    #[test]
    fn errors_for_file_with_record_having_too_many_values() {
        let error = read_strings_from_file("string_1, english 1, french 1, useless value");
        assert_eq!(
            error.unwrap_err().to_string(),
            "Too many values in record (exactly 3 required). 4th field => \"useless value\""
        )
    }

    fn read_strings_from_file(file_content: &str) -> Result<Vec<LocalizedString>, Error> {
        // Write content to file
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        tmpfile.write(file_content.as_bytes()).unwrap();

        // Seek to start
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        // Read strings from file
        super::from(tmpfile.try_clone().unwrap())
    }
}
