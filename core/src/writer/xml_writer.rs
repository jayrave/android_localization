use std::io::BufWriter;
use std::io::Write;

use xml::reader::XmlEvent as ReadXmlEvent;
use xml::writer;
use xml::writer::XmlEvent as WriteXmlEvent;
use xml::EmitterConfig;
use xml::ParserConfig;

use crate::android_string::AndroidString;
use crate::constants;
use crate::error::InnerError;

pub fn write<S: Write>(
    sink: &mut S,
    android_strings: Vec<AndroidString>,
) -> Result<(), InnerError> {
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .indent_string("    ") // 4 spaces
        .write_document_declaration(true)
        .create_writer(BufWriter::new(sink));

    // Start resources element
    writer.write(WriteXmlEvent::start_element(constants::elements::RESOURCES))?;

    // Write all string elements
    for android_string in android_strings {
        // String tag with name attribute
        let mut string_element = WriteXmlEvent::start_element(constants::elements::STRING)
            .attr(constants::attributes::NAME, android_string.name());

        // Include `localizable` attribute if required
        if !android_string.is_localizable() {
            string_element =
                string_element.attr(constants::attributes::LOCALIZABLE, constants::flags::FALSE);
        }

        writer.write(string_element)?;
        write_string(&mut writer, android_string.value())?;
        writer.write(WriteXmlEvent::end_element())?;
    }

    // Ending resources
    writer.write(WriteXmlEvent::end_element())?;

    Ok(())
}

fn write_string<W: Write>(
    writer: &mut writer::EventWriter<W>,
    value: &str,
) -> Result<(), InnerError> {
    // Right now, to write CDATA sections in strings properly out to the file,
    // we are creating a reader & then piping the required read events to the
    // writer. This feels wasteful! There has got to a better, more efficient
    // way to do this

    // Artificially inject tags to create valid XML out of the passed in string
    let value = format!("<a>{}</a>", value);
    let reader = ParserConfig::new().create_reader(value.as_bytes());
    for element_or_error in reader {
        match element_or_error {
            Err(error) => return Err::<_, InnerError>(From::from(error)),
            Ok(ref element) => match element {
                ReadXmlEvent::Characters(_) => {
                    writer.write(element.as_writer_event().ok_or_else(|| {
                        InnerError::from(format!("Can't build writer event from {}", &value))
                    })?)
                }

                ReadXmlEvent::CData(_) => {
                    writer.write(element.as_writer_event().ok_or_else(|| {
                        InnerError::from(format!("Can't build writer event from {}", &value))
                    })?)
                }

                _ => Ok(()), // No op for other events
            },
        }?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::android_string::AndroidString;

    #[test]
    fn writes_strings_to_file() {
        let android_strings = vec![
            AndroidString::localizable("localizable_string", "localizable string value"),
            AndroidString::unlocalizable("non_localizable_string", "non localizable string value"),
        ];

        // Write strings to a vector & split o/p into lines
        let mut sink: Vec<u8> = vec![];
        super::write(&mut sink, android_strings).unwrap();
        let written_content = String::from_utf8(sink).unwrap();
        let mut written_lines = written_content.lines();

        assert_eq!(
            written_lines.next().unwrap(),
            r##"<?xml version="1.0" encoding="utf-8"?>"##
        );
        assert_eq!(written_lines.next().unwrap(), r##"<resources>"##);
        assert_eq!(
            written_lines.next().unwrap(),
            r##"    <string name="localizable_string">localizable string value</string>"##
        );
        assert_eq!(written_lines.next().unwrap(), r##"    <string name="non_localizable_string" translatable="false">non localizable string value</string>"##);
        assert_eq!(written_lines.next().unwrap(), r##"</resources>"##);
        assert_eq!(written_lines.next(), None);
    }

    #[test]
    fn writes_string_with_one_cdata_event() {
        test_cdata_handling("<![CDATA[this is a test]]>")
    }

    #[test]
    fn writes_string_with_character_followed_by_cdata_event() {
        test_cdata_handling("character event <![CDATA[cdata event]]>")
    }

    #[test]
    fn writes_string_with_cdata_followed_by_character_event() {
        test_cdata_handling("<![CDATA[cdata event]]> character event")
    }

    #[test]
    fn writes_string_with_multiple_character_and_cdata_events() {
        test_cdata_handling("character event 1 <![CDATA[cdata event 1]]> character event 2 <![CDATA[cdata event 2]]> <![CDATA[cdata event 3]]> character event 3")
    }

    fn test_cdata_handling(value: &str) {
        // Write string to a vector & split o/p into lines
        let mut sink: Vec<u8> = vec![];
        super::write(&mut sink, vec![AndroidString::localizable("s1", value)]).unwrap();

        let written_content = String::from_utf8(sink).unwrap();
        let mut written_lines = written_content.lines();

        written_lines.next().unwrap(); // XML header
        written_lines.next().unwrap(); // Resources opening
        assert_eq!(
            written_lines.next().unwrap(),
            format!("    <string name=\"s1\">{}</string>", value)
        );
        written_lines.next().unwrap(); // Resources closing
        assert_eq!(written_lines.next(), None);
    }
}
