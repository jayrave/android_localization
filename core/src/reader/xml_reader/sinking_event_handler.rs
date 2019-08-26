use xml::attribute::OwnedAttribute;

use crate::error::Error;
use crate::reader::xml_reader::event_handler::EventHandler;

pub struct SinkingEventHandler {}

impl SinkingEventHandler {
    pub fn new() -> SinkingEventHandler {
        SinkingEventHandler {}
    }
}

impl EventHandler for SinkingEventHandler {
    fn build_handler(
        &self,
        _tag_name: String,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Box<EventHandler>, Error> {
        Ok(Box::new(SinkingEventHandler::new()))
    }
}
