use crate::android_string::AndroidString;
use crate::error::Error;
use crate::reader::xml_reader::events_handler::EventsHandler;
use std::io::BufReader;
use std::io::Read;
use xml::reader::XmlEvent;
use xml::ParserConfig;

pub fn read<S: Read>(source: S) -> Result<Vec<AndroidString>, Error> {
    let mut events_handler = EventsHandler::new();
    let reader = ParserConfig::new().create_reader(BufReader::new(source));

    for element_or_error in reader {
        match element_or_error {
            Err(error) => return Err(error)?,
            Ok(element) => match element {
                XmlEvent::StartElement {
                    name, attributes, ..
                } => events_handler.handle_start_element_event(name.local_name, attributes)?,
                XmlEvent::Characters(text) => events_handler.handle_characters_event(text),
                XmlEvent::CData(text) => events_handler.handle_cdata_event(text),
                XmlEvent::EndElement { .. } => events_handler.handle_end_element_event(),
                _ => {} // No op for other events
            },
        }
    }

    Ok(events_handler.strings())
}

#[cfg(test)]
mod tests {
    use crate::android_string::AndroidString;
    use std::fs::File;
    use std::io::{Seek, SeekFrom, Write};

    #[test]
    fn strings_are_read_from_valid_clean_file() {
        let mut strings = write_to_file_and_read_strings_out(r##"
			<?xml version="1.0" encoding="utf-8"?>
			<resources>
			    <string name="string_1">string 1 value</string>
			    <string name="string_2" translatable="true">string 2 value</string>
				<string name="non_localizable_string" translatable="false">non localizable string value</string>
			</resources>
		"##).into_iter();

        assert_eq!(
            strings.next(),
            Some(AndroidString::new(
                String::from("string_1"),
                String::from("string 1 value"),
                true
            ))
        );

        assert_eq!(
            strings.next(),
            Some(AndroidString::new(
                String::from("string_2"),
                String::from("string 2 value"),
                true
            ))
        );

        assert_eq!(
            strings.next(),
            Some(AndroidString::new(
                String::from("non_localizable_string"),
                String::from("non localizable string value"),
                false
            ))
        );

        assert_eq!(strings.next(), None);
    }

    #[test]
    fn strings_are_read_from_valid_dirty_file() {
        let mut strings = write_to_file_and_read_strings_out(r##"
			<?xml version="1.0" encoding="utf-8"?>
			<string name="dont_care_string_1">value</string>
			<string name="dont_care_string_2" translatable="false">value</string>
			<resources>
			    <string name="string_1">string 1 value</string>
			    <string name="string_2" translatable="true">string 2 value</string>
			    <inside_container>
					<string name="dont_care_string_3">value</string>
					<string name="dont_care_string_4" translatable="false">value</string>
				</inside_container>
				<string name="non_localizable_string" translatable="false">non localizable string value</string>
			</resources>
			<outside_container>
				<string name="dont_care_string_5">value</string>
				<string name="dont_care_string_6" translatable="false">value</string>
			</outside_container>
		"##).into_iter();

        assert_eq!(
            strings.next(),
            Some(AndroidString::new(
                String::from("string_1"),
                String::from("string 1 value"),
                true
            ))
        );

        assert_eq!(
            strings.next(),
            Some(AndroidString::new(
                String::from("string_2"),
                String::from("string 2 value"),
                true
            ))
        );

        assert_eq!(
            strings.next(),
            Some(AndroidString::new(
                String::from("non_localizable_string"),
                String::from("non localizable string value"),
                false
            ))
        );

        assert_eq!(strings.next(), None);
    }

    #[test]
    fn string_with_cdata_is_read_correctly() {
        let mut strings = write_to_file_and_read_strings_out(r##"
            <?xml version="1.0" encoding="utf-8"?>
            <resources>
                <string name="s1">Hi there. <![CDATA[<a href=\"https://www.mozilla.com\">Mozilla</a>]]> is awesome</string>
            </resources>
        "##).into_iter();

        assert_eq!(
            strings.next(),
            Some(AndroidString::new(
                String::from("s1"),
                String::from(r##"Hi there. <![CDATA[<a href=\"https://www.mozilla.com\">Mozilla</a>]]> is awesome"##),
                true
            ))
        );

        assert_eq!(strings.next(), None);
    }

    #[test]
    fn string_with_whitespace_between_cdata_is_read_correctly() {
        let mut strings = write_to_file_and_read_strings_out(r##"
            <?xml version="1.0" encoding="utf-8"?>
            <resources>
                <string name="s1"><![CDATA[<a href=\"https://www.mozilla.com\">Mozilla</a>]]> <![CDATA[<a href=\"https://www.firefox.com\">Firefox</a>]]></string>
            </resources>
        "##).into_iter();

        assert_eq!(
            strings.next(),
            Some(AndroidString::new(
                String::from("s1"),
                String::from(r##"<![CDATA[<a href=\"https://www.mozilla.com\">Mozilla</a>]]> <![CDATA[<a href=\"https://www.firefox.com\">Firefox</a>]]>"##),
                true
            ))
        );

        assert_eq!(strings.next(), None);
    }

    fn write_to_file_and_read_strings_out(file_content: &str) -> Vec<AndroidString> {
        // Write content to file
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        tmpfile.write(file_content.as_bytes()).unwrap();

        // Seek to start
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        // Read strings from file & assert
        super::read(tmpfile.try_clone().unwrap()).unwrap()
    }
}
