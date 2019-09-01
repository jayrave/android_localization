use xml::attribute::OwnedAttribute;

use crate::constants;
use crate::error::InnerError;
use crate::reader::xml_reader::event_handler::EventHandler;
use crate::reader::xml_reader::sinking_event_handler::SinkingEventHandler;
use crate::reader::xml_reader::string_event_handler::StringEventHandler;

pub struct ResourcesEventHandler {}

impl ResourcesEventHandler {
    pub fn new() -> ResourcesEventHandler {
        ResourcesEventHandler {}
    }
}

impl EventHandler for ResourcesEventHandler {
    fn build_handler(
        &self,
        tag_name: String,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Box<EventHandler>, InnerError> {
        match tag_name.as_str() {
            constants::elements::STRING => Ok(Box::new(StringEventHandler::build(attributes)?)),
            _ => Ok(Box::new(SinkingEventHandler::new())),
        }
    }
}
