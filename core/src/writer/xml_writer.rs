use android_string::AndroidString;
use constants;
use std::error;
use std::fmt;
use std::io::BufWriter;
use std::io::Write;
use xml::writer;
use xml::writer::XmlEvent;
use xml::EmitterConfig;

pub fn to<W: Write>(sink: &mut W, android_strings: Vec<AndroidString>) -> Result<(), Error> {
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .indent_string("    ") // 4 spaces
        .write_document_declaration(true)
        .create_writer(BufWriter::new(sink));

    // Start resources element
    writer
        .write(XmlEvent::start_element(constants::elements::RESOURCES))
        .map_err(|e| Error { error: e })?;

    // Write all string elements
    for android_string in android_strings {
        let mut string_element = XmlEvent::start_element(constants::elements::STRING)
            .attr(constants::attributes::NAME, android_string.name());

        if !android_string.is_translatable() {
            string_element =
                string_element.attr(constants::attributes::TRANSLATABLE, constants::flags::FALSE);
        }

        writer
            .write(string_element)
            .map_err(|e| Error { error: e })?;

        writer
            .write(XmlEvent::characters(android_string.value()))
            .map_err(|e| Error { error: e })?;

        writer
            .write(XmlEvent::end_element())
            .map_err(|e| Error { error: e })?;
    }

    // Ending resources
    writer
        .write(XmlEvent::end_element())
        .map_err(|e| Error { error: e })?;

    Ok(())
}

#[derive(Debug)]
pub struct Error {
    error: writer::Error,
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        Some(&self.error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.error, f)
    }
}

#[cfg(test)]
mod tests {
    use android_string::AndroidString;

    #[test]
    fn strings_are_written_to_file() {
        let android_strings = vec![
            AndroidString::new(
                String::from("translatable_string"),
                String::from("translatable string value"),
                true,
            ),
            AndroidString::new(
                String::from("non_translatable_string"),
                String::from("non translatable string value"),
                false,
            ),
        ];

        // Write strings to a vector & split o/p into lines
        let mut sink: Vec<u8> = vec![];
        super::to(&mut sink, android_strings).unwrap();
        let written_content = String::from_utf8(sink).unwrap();
        let mut written_lines = written_content.lines();

        assert_eq!(
            written_lines.next().unwrap(),
            r##"<?xml version="1.0" encoding="utf-8"?>"##
        );
        assert_eq!(written_lines.next().unwrap(), r##"<resources>"##);
        assert_eq!(
            written_lines.next().unwrap(),
            r##"    <string name="translatable_string">translatable string value</string>"##
        );
        assert_eq!(written_lines.next().unwrap(), r##"    <string name="non_translatable_string" translatable="false">non translatable string value</string>"##);
        assert_eq!(written_lines.next().unwrap(), r##"</resources>"##);
        assert_eq!(written_lines.next(), None);
    }
}
