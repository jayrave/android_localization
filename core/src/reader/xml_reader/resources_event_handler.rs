use constants;
use reader::xml_reader::error::Error;
use reader::xml_reader::event_handler::EventHandler;
use reader::xml_reader::sinking_event_handler::SinkingEventHandler;
use reader::xml_reader::string_event_handler::StringEventHandler;
use xml::attribute::OwnedAttribute;

pub struct ResourcesEventHandler {}

impl ResourcesEventHandler {
    pub fn new() -> ResourcesEventHandler {
        ResourcesEventHandler {}
    }
}

impl EventHandler for ResourcesEventHandler {
    fn handler_for_start_element_event(
        &self,
        tag_name: String,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Box<EventHandler>, Error> {
        match tag_name.as_str() {
            constants::elements::STRING => Ok(Box::new(StringEventHandler::build(attributes)?)),
            _ => Ok(Box::new(SinkingEventHandler::new())),
        }
    }
}
