use android_string::AndroidString;
use csv::Writer;
use std::io::Write;
use writer::csv_writer::error::Error;

pub fn write<S: Write>(
    sink: &mut S,
    translatable_android_strings: Vec<AndroidString>,
) -> Result<(), Error> {
    let mut writer = Writer::from_writer(sink); // Sink is automatically buffered
    for string in translatable_android_strings {
        if let Err(error) = writer.write_record(vec![string.name(), string.value()]) {
            return Err(Error::CsvError(error));
        }
    }

    match writer.flush() {
        Err(error) => Err(Error::IoError(error)),
        Ok(_) => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use android_string::AndroidString;

    #[test]
    fn strings_are_written_to_file() {
        let translatable_android_strings = vec![
            AndroidString::new(
                String::from("string_1"),
                String::from("string 1 value"),
                true,
            ),
            AndroidString::new(
                String::from("string_2"),
                String::from("string 2 value"),
                true,
            ),
        ];

        // Write strings to a vector & split o/p into lines
        let mut sink: Vec<u8> = vec![];
        super::write(&mut sink, translatable_android_strings).unwrap();
        let written_content = String::from_utf8(sink).unwrap();
        let mut written_lines = written_content.lines();

        assert_eq!(written_lines.next().unwrap(), "string_1,string 1 value");
        assert_eq!(written_lines.next().unwrap(), "string_2,string 2 value");
        assert_eq!(written_lines.next(), None);
    }
}