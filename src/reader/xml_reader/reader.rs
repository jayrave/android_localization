use android_string::AndroidString;
use reader::error::Error;
use reader::xml_reader::events_handler::EventsHandler;
use std::io::BufReader;
use std::io::Read;
use xml::reader::XmlEvent;
use xml::ParserConfig;

pub fn from<R: Read>(read: R) -> Result<Vec<AndroidString>, Error> {
    let mut events_handler = EventsHandler::new();
    let reader = ParserConfig::new()
        .cdata_to_characters(true)
        .create_reader(BufReader::new(read));

    for element_or_error in reader {
        match element_or_error {
            Err(error) => return Err(Error::XmlError(error)),
            Ok(element) => match element {
                XmlEvent::StartElement {
                    name, attributes, ..
                } => events_handler.handle_start_element_event(name.local_name, attributes)?,
                XmlEvent::Characters(text) => events_handler.handle_characters_event(text),
                XmlEvent::EndElement { .. } => events_handler.handle_end_element_event(),
                _ => {} // No op for other events
            },
        }
    }

    events_handler.strings()
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use android_string::AndroidString;
    use std::fs::File;
    use std::io::{Seek, SeekFrom, Write};

    #[test]
    fn strings_are_read_from_valid_clean_file() {
        perform_valid_file_test(r##"
			<?xml version="1.0" encoding="utf-8"?>
			<resources>
			    <string name="string_1">string 1 value</string>
			    <string name="string_2" translatable="true">string 2 value</string>
				<string name="non_translatable_string" translatable="false">non translatable string value</string>
			</resources>
		"##);
    }

    #[test]
    fn strings_are_read_from_valid_dirty_file() {
        perform_valid_file_test(r##"
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
				<string name="non_translatable_string" translatable="false">non translatable string value</string>
			</resources>
			<outside_container>
				<string name="dont_care_string_5">value</string>
				<string name="dont_care_string_6" translatable="false">value</string>
			</outside_container>
		"##);
    }

    fn perform_valid_file_test(file_content: &str) {
        // Write content to file
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        tmpfile.write(file_content.as_bytes()).unwrap();

        // Seek to start
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        // Read strings from file & assert
        let mut it = super::from(tmpfile.try_clone().unwrap())
            .unwrap()
            .into_iter();

        assert_eq!(
            it.next(),
            Some(AndroidString::new(
                String::from("string_1"),
                String::from("string 1 value"),
                true
            ))
        );

        assert_eq!(
            it.next(),
            Some(AndroidString::new(
                String::from("string_2"),
                String::from("string 2 value"),
                true
            ))
        );

        assert_eq!(
            it.next(),
            Some(AndroidString::new(
                String::from("non_translatable_string"),
                String::from("non translatable string value"),
                false
            ))
        );

        assert_eq!(it.next(), None);
    }
}
