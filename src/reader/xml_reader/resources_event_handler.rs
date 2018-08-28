use android_string::AndroidString;
use reader::error::Error;
use reader::xml_reader::event_handler::EventHandler;
use reader::xml_reader::sinking_event_handler::SinkingEventHandler;
use reader::xml_reader::string_event_handler::StringEventHandler;
use std::cell::RefCell;
use std::rc::Rc;
use xml::attribute::OwnedAttribute;

pub struct ResourcesEventHandler {
    strings: Rc<RefCell<Vec<AndroidString>>>,
}

impl ResourcesEventHandler {
    const STRING_TEXT: &'static str = "string";

    pub fn new(strings: Rc<RefCell<Vec<AndroidString>>>) -> ResourcesEventHandler {
        ResourcesEventHandler { strings }
    }
}

impl EventHandler for ResourcesEventHandler {
    fn handler_for_start_element_event(
        &self,
        tag_name: String,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Box<EventHandler>, Error> {
        match tag_name.as_str() {
            ResourcesEventHandler::STRING_TEXT => Ok(Box::new(StringEventHandler::new(
                Rc::clone(&self.strings),
                attributes,
            )?)),
            _ => Ok(Box::new(SinkingEventHandler::new())),
        }
    }

    fn handle_characters_event(&self, _text: String) {
        // No op
    }
}
