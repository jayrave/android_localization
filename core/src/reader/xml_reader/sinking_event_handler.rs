use android_string::AndroidString;
use reader::xml_reader::error::Error;
use reader::xml_reader::event_handler::EventHandler;
use xml::attribute::OwnedAttribute;

pub struct SinkingEventHandler {}

impl SinkingEventHandler {
    pub fn new() -> SinkingEventHandler {
        SinkingEventHandler {}
    }
}

impl EventHandler for SinkingEventHandler {
    fn handler_for_start_element_event(
        &self,
        _tag_name: String,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Box<EventHandler>, Error> {
        Ok(Box::new(SinkingEventHandler::new()))
    }

    fn handle_characters_event(&mut self, _text: String) {
        // No op
    }

    fn built_string(&self) -> Option<AndroidString> {
        None
    }
}
