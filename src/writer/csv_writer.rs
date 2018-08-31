use csv::Error;
use csv::Writer;
use std::io::Write;
use translatable_android_string::TranslatableAndroidString;

pub fn to<W: Write>(
    sink: &mut W,
    translatable_android_strings: Vec<TranslatableAndroidString>,
) -> Result<(), Error> {
    let mut writer = Writer::from_writer(sink); // Sink is automatically buffered
    for string in translatable_android_strings {
        writer.write_record(vec![string.name(), string.value()])?;
    }

    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use translatable_android_string::TranslatableAndroidString;

    #[test]
    fn strings_are_written_to_file() {
        let translatable_android_strings = vec![
            TranslatableAndroidString::new(
                String::from("string_1"),
                String::from("string 1 value"),
            ),
            TranslatableAndroidString::new(
                String::from("string_2"),
                String::from("string 2 value"),
            ),
        ];

        // Write strings to a vector & split o/p into lines
        let mut sink: Vec<u8> = vec![];
        super::to(&mut sink, translatable_android_strings).unwrap();
        let written_content = String::from_utf8(sink).unwrap();
        let mut written_lines = written_content.lines();

        assert_eq!(written_lines.next().unwrap(), "string_1,string 1 value");
        assert_eq!(written_lines.next().unwrap(), "string_2,string 2 value");
        assert_eq!(written_lines.next(), None);
    }
}
