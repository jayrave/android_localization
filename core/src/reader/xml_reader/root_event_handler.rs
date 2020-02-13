use xml::attribute::OwnedAttribute;

use crate::constants;
use crate::error::InnerError;
use crate::reader::xml_reader::event_handler::EventHandler;
use crate::reader::xml_reader::resources_event_handler::ResourcesEventHandler;
use crate::reader::xml_reader::sinking_event_handler::SinkingEventHandler;

pub struct RootEventHandler {}

impl RootEventHandler {
    pub fn new() -> RootEventHandler {
        RootEventHandler {}
    }
}

impl EventHandler for RootEventHandler {
    fn build_handler(
        &self,
        tag_name: String,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Box<dyn EventHandler>, InnerError> {
        match tag_name.as_str() {
            constants::elements::RESOURCES => Ok(Box::new(ResourcesEventHandler::new())),
            _ => Ok(Box::new(SinkingEventHandler::new())),
        }
    }
}
