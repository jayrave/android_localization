use android_string::AndroidString;
use reader::error::Error;
use std::io::BufReader;
use std::io::Read;
use xml::ParserConfig;
use xml::reader::{EventReader, XmlEvent};

const RESOURCES_EVENT_DEPTH: u32 = 0;
const STRING_EVENT_DEPTH: u32 = 1;
const CHARACTER_EVENT_DEPTH: u32 = 2;

const RESOURCES_TEXT: &str = "resources";
const STRING_TEXT: &str = "string";
const STRING_NAME_TEXT: &str = "name";
const STRING_TRANSLATABLE_TEXT: &str = "translatable";
const TRUE_TEXT: &str = "true";
const FALSE_TEXT: &str = "false";

pub fn from<R : Read>(read: R) -> Result<Vec<AndroidString>, Error> {
	let mut android_strings = vec!();
	let reader = ParserConfig::new()
		.cdata_to_characters(true)
		.create_reader(BufReader::new(read));

	let mut depth = 0u32;
	let mut is_inside_resources_event = false;
	let mut is_inside_string_event = false;

	let mut string_name: Option<String> = None;
	let mut is_translatable_string = false;

	for element in reader {
		match element {
			Err(error) => return Err(Error::XmlError(error)),
			Ok(element) => match element {
				XmlEvent::StartElement { name, attributes, .. } => match name.local_name.as_str() {
					RESOURCES_TEXT => if depth == RESOURCES_EVENT_DEPTH {
						depth += 1;
						is_inside_resources_event = true;
					}, 

					STRING_TEXT => if depth == STRING_EVENT_DEPTH && is_inside_resources_event {
						depth += 1;
						is_inside_string_event = true;
						
						string_name = None;
						is_translatable_string = true;
						for attribute in attributes {
							match attribute.name.local_name.as_str() {
								STRING_NAME_TEXT => string_name = Some(attribute.value),
								STRING_TRANSLATABLE_TEXT => match attribute.value.as_str() {
									TRUE_TEXT => is_translatable_string = true, 
									FALSE_TEXT => is_translatable_string = false, 
									_ => {}
								}, 
								_ => {}
							}
						}
					}, 

					_ => depth += 1
				},

				XmlEvent::EndElement { name } => match name.local_name.as_str() {
					RESOURCES_TEXT => if depth == RESOURCES_EVENT_DEPTH + 1 && is_inside_resources_event {
						depth -= 1;
						is_inside_resources_event = false;
					}, 

					STRING_TEXT => if depth == STRING_EVENT_DEPTH + 1 && is_inside_string_event {
						depth -= 1;
						is_inside_string_event = false;
					}, 

					_ => depth -= 1
				},

				XmlEvent::Characters(c) => if depth == CHARACTER_EVENT_DEPTH && is_inside_string_event {
					match string_name {
						None => return Err(Error::SyntaxError(format!("Name is missing for text \"{}\"", &c))), 
						Some(ref name) => android_strings.push(AndroidString::new(
							String::from(name.as_str()),
							c, 
							is_translatable_string
						))
					}
				}

				_ => {}
			}
		}
	}

	Ok(android_strings)
}

#[cfg(test)]
mod tests {
	extern crate tempfile;

	use ::android_string::AndroidString;
	use std::fs::File;
	use std::io::{Write, Read, Seek, SeekFrom};

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
	    let mut it = super::from(tmpfile.try_clone().unwrap()).unwrap().into_iter();
	    assert_eq!(it.next(), Some(AndroidString::new( 
	    	String::from("string_1"), 
	    	String::from("string 1 value"), 
	    	true
	    )));

	    assert_eq!(it.next(), Some(AndroidString::new( 
	    	String::from("string_2"), 
	    	String::from("string 2 value"), 
	    	true
	    )));

	    assert_eq!(it.next(), Some(AndroidString::new( 
	    	String::from("non_translatable_string"), 
	    	String::from("non translatable string value"), 
	    	false
	    )));

	    assert_eq!(it.next(), None);
	}
}
