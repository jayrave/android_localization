use android_string::AndroidString;
use constants;
use reader::error::Error;
use reader::xml_reader::event_handler::EventHandler;
use reader::xml_reader::resources_event_handler::ResourcesEventHandler;
use reader::xml_reader::sinking_event_handler::SinkingEventHandler;
use std::cell::RefCell;
use std::rc::Rc;
use xml::attribute::OwnedAttribute;

pub struct RootEventHandler {
    strings: Rc<RefCell<Vec<AndroidString>>>,
}

impl RootEventHandler {
    pub fn new(strings: Rc<RefCell<Vec<AndroidString>>>) -> RootEventHandler {
        RootEventHandler { strings }
    }
}

impl EventHandler for RootEventHandler {
    fn handler_for_start_element_event(
        &self,
        tag_name: String,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Box<EventHandler>, Error> {
        match tag_name.as_str() {
            constants::elements::RESOURCES => Ok(Box::new(ResourcesEventHandler::new(Rc::clone(
                &self.strings,
            )))),
            _ => Ok(Box::new(SinkingEventHandler::new())),
        }
    }

    fn handle_characters_event(&self, _text: String) {
        // No op
    }
}
