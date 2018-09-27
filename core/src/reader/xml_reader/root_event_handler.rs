use android_string::AndroidString;
use constants;
use reader::xml_reader::error::Error;
use reader::xml_reader::event_handler::EventHandler;
use reader::xml_reader::resources_event_handler::ResourcesEventHandler;
use reader::xml_reader::sinking_event_handler::SinkingEventHandler;
use xml::attribute::OwnedAttribute;

pub struct RootEventHandler {}

impl RootEventHandler {
    pub fn new() -> RootEventHandler {
        RootEventHandler {}
    }
}

impl EventHandler for RootEventHandler {
    fn handler_for_start_element_event(
        &self,
        tag_name: String,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Box<EventHandler>, Error> {
        match tag_name.as_str() {
            constants::elements::RESOURCES => Ok(Box::new(ResourcesEventHandler::new())),
            _ => Ok(Box::new(SinkingEventHandler::new())),
        }
    }

    fn handle_characters_event(&mut self, _text: String) {
        // No op
    }

    fn built_string(&self) -> Option<AndroidString> {
        None
    }
}
