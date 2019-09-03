use std::io::BufReader;
use std::io::Read;

use xml::reader::XmlEvent;
use xml::ParserConfig;

use crate::android_string::AndroidString;
use crate::error::InnerError;
use crate::reader::xml_reader::events_handler::EventsHandler;

pub fn read<S: Read>(source: S) -> Result<Vec<AndroidString>, InnerError> {
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
    use std::fs::File;
    use std::io::{Seek, SeekFrom, Write};

    use test_utilities;

    use crate::android_string::AndroidString;

    #[test]
    fn reads_strings_from_valid_clean_file() {
        let strings = write_to_file_and_read_strings_out(
            r##"
			<?xml version="1.0" encoding="utf-8"?>
			<resources>
			    <string name="string_1">string 1 value</string>
			    <string name="string_2" translatable="true">string 2 value</string>
				<string name="non_localizable_string" translatable="false">non localizable string value</string>
			</resources>
		"##,
        );

        test_utilities::list::assert_strict_list_eq(
            strings,
            vec![
                AndroidString::localizable("string_1", "string 1 value"),
                AndroidString::localizable("string_2", "string 2 value"),
                AndroidString::unlocalizable(
                    "non_localizable_string",
                    "non localizable string value",
                ),
            ],
        )
    }

    #[test]
    fn reads_strings_from_valid_dirty_file() {
        let strings = write_to_file_and_read_strings_out(
            r##"
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
		"##,
        );

        test_utilities::list::assert_strict_list_eq(
            strings,
            vec![
                AndroidString::localizable("string_1", "string 1 value"),
                AndroidString::localizable("string_2", "string 2 value"),
                AndroidString::unlocalizable(
                    "non_localizable_string",
                    "non localizable string value",
                ),
            ],
        )
    }

    #[test]
    fn reads_cdata_correctly() {
        let strings = write_to_file_and_read_strings_out(r##"
            <?xml version="1.0" encoding="utf-8"?>
            <resources>
                <string name="s1">Hi there. <![CDATA[<a href=\"https://www.mozilla.com\">Mozilla</a>]]> is awesome</string>
            </resources>
        "##);

        test_utilities::list::assert_strict_list_eq(
            strings,
            vec![AndroidString::localizable("s1", r##"Hi there. <![CDATA[<a href=\"https://www.mozilla.com\">Mozilla</a>]]> is awesome"##)]
        )
    }

    #[test]
    fn reads_string_with_whitespace_between_cdata() {
        let strings = write_to_file_and_read_strings_out(r##"
            <?xml version="1.0" encoding="utf-8"?>
            <resources>
                <string name="s1"><![CDATA[<a href=\"https://www.mozilla.com\">Mozilla</a>]]> <![CDATA[<a href=\"https://www.firefox.com\">Firefox</a>]]></string>
            </resources>
        "##);

        test_utilities::list::assert_strict_list_eq(
            strings,
            vec![AndroidString::localizable("s1", r##"<![CDATA[<a href=\"https://www.mozilla.com\">Mozilla</a>]]> <![CDATA[<a href=\"https://www.firefox.com\">Firefox</a>]]>"##)]
        );
    }

    fn write_to_file_and_read_strings_out(file_content: &str) -> Vec<AndroidString> {
        // Write content to file
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        tmpfile.write(file_content.as_bytes()).unwrap();

        // Seek to start
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        // Read strings from file
        super::read(tmpfile.try_clone().unwrap()).unwrap()
    }
}
