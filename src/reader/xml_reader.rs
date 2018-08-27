use android_string::AndroidString;
use reader::error::Error;
use std::io::BufReader;
use std::io::Read;
use std::rc::Rc;
use std::cell::RefCell;
use xml::attribute::OwnedAttribute;
use xml::ParserConfig;
use xml::reader::{XmlEvent};

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
	let mut events_handler = EventsHandler::new();
	let reader = ParserConfig::new()
		.cdata_to_characters(true)
		.create_reader(BufReader::new(read));

	for element_or_error in reader {
		match element_or_error {
			Err(error) => return Err(Error::XmlError(error)),
			Ok(element) => match element {
				XmlEvent::StartElement { name, attributes, .. } => events_handler.handle_start_element_event(name.local_name, attributes)?,
				XmlEvent::Characters(text) => events_handler.handle_characters_event(text),
				XmlEvent::EndElement { .. } => events_handler.handle_end_element_event(),
				_ => {} // No op for other events
			}
		}
	}

	events_handler.strings()
}

struct EventsHandler {
	android_strings: Rc<RefCell<Vec<AndroidString>>>,
	event_handlers: Vec<Box<EventHandler>>
}

impl EventsHandler {
	fn new() -> EventsHandler {
        let strings = Rc::new(RefCell::new(vec!()));
        let strings_clone = Rc::clone(&strings);
		EventsHandler {
			android_strings: strings,
			event_handlers: vec!(Box::new(RootEventHandler::new(strings_clone)))
		}
	}

	fn handle_start_element_event(&mut self, tag_name: String, attributes: Vec<OwnedAttribute>) -> Result<(), Error> {
		let event_handler = self.event_handlers.last_mut().unwrap().handler_for_start_element_event(tag_name, attributes)?;
		self.event_handlers.push(event_handler);
		Ok(())
	}

	fn handle_end_element_event(&mut self) {
		self.event_handlers.pop();
	}

	fn handle_characters_event(&mut self, text: String) {
		self.event_handlers.last_mut().unwrap().handle_characters_event(text);
	}

    fn strings(mut self) -> Result<Vec<AndroidString>, Error> {
        self.event_handlers.clear();
        match Rc::try_unwrap(self.android_strings) {
            Err(ref_cell) => Err(Error::LogicError(format!("Rc has {} strong references!", Rc::strong_count(&ref_cell)))),
            Ok(ref_cell) => Ok(ref_cell.into_inner())
        }
    }
}

trait EventHandler {
	fn handler_for_start_element_event(&self, tag_name: String, attributes: Vec<OwnedAttribute>) -> Result<Box<EventHandler>, Error>;
	fn handle_characters_event(&self, text: String);
}

struct RootEventHandler {
	strings: Rc<RefCell<Vec<AndroidString>>>
}

impl RootEventHandler {
	fn new(strings: Rc<RefCell<Vec<AndroidString>>>) -> RootEventHandler {
		RootEventHandler { 
			strings
		}
	}
}

impl EventHandler for RootEventHandler {
	fn handler_for_start_element_event(&self, tag_name: String, _attributes: Vec<OwnedAttribute>) -> Result<Box<EventHandler>, Error> {
		match tag_name.as_str() {
			RESOURCES_TEXT => Ok(Box::new(ResourcesEventHandler::new(Rc::clone(&self.strings)))),
			_ => Ok(Box::new(SinkingEventHandler::new()))
		}
	}

	fn handle_characters_event(&self, _text: String) {
        // No op
	}
}

struct ResourcesEventHandler {
	strings: Rc<RefCell<Vec<AndroidString>>>
}

impl ResourcesEventHandler {
	fn new(strings: Rc<RefCell<Vec<AndroidString>>>) -> ResourcesEventHandler {
		ResourcesEventHandler {
			strings
		}
	}
}

impl EventHandler for ResourcesEventHandler {
	fn handler_for_start_element_event(&self, tag_name: String, attributes: Vec<OwnedAttribute>) -> Result<Box<EventHandler>, Error> {
		match tag_name.as_str() {
			STRING_TEXT => Ok(Box::new(StringEventHandler::new(Rc::clone(&self.strings), attributes)?)),
			_ => Ok(Box::new(SinkingEventHandler::new()))
		}
	}

	fn handle_characters_event(&self, _text: String) {
        // No op
	}
}

struct StringEventHandler {
	strings: Rc<RefCell<Vec<AndroidString>>>,
	name: String,
	is_translatable: bool
}

impl StringEventHandler {
	fn new(strings: Rc<RefCell<Vec<AndroidString>>>, attributes: Vec<OwnedAttribute>) -> Result<StringEventHandler, Error> {
		let mut string_name = None;
		let mut is_translatable = true;
		for attribute in attributes {
			match attribute.name.local_name.as_str() {
				STRING_NAME_TEXT => string_name = Some(attribute.value),
				STRING_TRANSLATABLE_TEXT => match attribute.value.as_str() {
					TRUE_TEXT => is_translatable = true, 
					FALSE_TEXT => is_translatable = false, 
					_ => {}
				}, 
				_ => {}
			}
		}

		match string_name {
			None => Err(Error::SyntaxError(String::from("string element is missing required name attribute"))),
			Some(name) => Ok(StringEventHandler {
				strings,
				name,
				is_translatable
			})
		}
	}
}

impl EventHandler for StringEventHandler {
	fn handler_for_start_element_event(&self, _tag_name: String, _attributes: Vec<OwnedAttribute>) -> Result<Box<EventHandler>, Error> {
		Ok(Box::new(SinkingEventHandler::new()))
	}

	fn handle_characters_event(&self, text: String) {
		self.strings.borrow_mut().push(AndroidString::new(
			self.name.clone(),
			text, 
			self.is_translatable
		));
	}
}

struct SinkingEventHandler {

}

impl SinkingEventHandler {
	fn new() -> SinkingEventHandler {
		SinkingEventHandler {

		}
	}
}

impl EventHandler for SinkingEventHandler {
	fn handler_for_start_element_event(&self, _tag_name: String, _attributes: Vec<OwnedAttribute>) -> Result<Box<EventHandler>, Error> {
		Ok(Box::new(SinkingEventHandler::new()))
	}

	fn handle_characters_event(&self, _text: String) {
        // No op
	}
}

#[cfg(test)]
mod tests {
	extern crate tempfile;

	use ::android_string::AndroidString;
	use std::fs::File;
	use std::io::{Write, Seek, SeekFrom};

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
