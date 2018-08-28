use android_string::AndroidString;
use constants;
use reader::error::Error;
use reader::xml_reader::event_handler::EventHandler;
use reader::xml_reader::sinking_event_handler::SinkingEventHandler;
use std::cell::RefCell;
use std::rc::Rc;
use xml::attribute::OwnedAttribute;

pub struct StringEventHandler {
    strings: Rc<RefCell<Vec<AndroidString>>>,
    name: String,
    is_translatable: bool,
}

impl StringEventHandler {
    pub fn new(
        strings: Rc<RefCell<Vec<AndroidString>>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<StringEventHandler, Error> {
        let mut string_name = None;
        let mut is_translatable = true;
        for attribute in attributes {
            match attribute.name.local_name.as_str() {
                constants::attributes::NAME => string_name = Some(attribute.value),
                constants::attributes::TRANSLATABLE => match attribute.value.as_str() {
                    constants::flags::FALSE => is_translatable = false,
                    _ => {}
                },
                _ => {}
            }
        }

        match string_name {
            None => Err(Error::SyntaxError(String::from(
                "string element is missing required name attribute",
            ))),
            Some(name) => Ok(StringEventHandler {
                strings,
                name,
                is_translatable,
            }),
        }
    }
}

impl EventHandler for StringEventHandler {
    fn handler_for_start_element_event(
        &self,
        _tag_name: String,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Box<EventHandler>, Error> {
        Ok(Box::new(SinkingEventHandler::new()))
    }

    fn handle_characters_event(&self, text: String) {
        self.strings.borrow_mut().push(AndroidString::new(
            self.name.clone(),
            text,
            self.is_translatable,
        ));
    }
}
