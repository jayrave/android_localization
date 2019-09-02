use xml::attribute::OwnedAttribute;

use android_localization_utilities::DevExpt;

use crate::android_string::AndroidString;
use crate::error::InnerError;
use crate::reader::xml_reader::event_handler::EventHandler;
use crate::reader::xml_reader::root_event_handler::RootEventHandler;

pub struct EventsHandler {
    android_strings: Vec<AndroidString>,
    event_handlers: Vec<Box<EventHandler>>,
}

impl EventsHandler {
    pub fn new() -> EventsHandler {
        EventsHandler {
            android_strings: vec![],
            event_handlers: vec![Box::new(RootEventHandler::new())],
        }
    }

    pub fn handle_start_element_event(
        &mut self,
        tag_name: String,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<(), InnerError> {
        let event_handler = self
            .event_handlers
            .last_mut()
            .expt("There must have been at least one event handler!")
            .build_handler(tag_name, attributes)?;
        self.event_handlers.push(event_handler);
        Ok(())
    }

    pub fn handle_characters_event(&mut self, text: String) {
        self.event_handlers
            .last_mut()
            .expt("There must have been at least one event handler!")
            .handle_characters_event(text);
    }

    pub fn handle_cdata_event(&mut self, text: String) {
        self.event_handlers
            .last_mut()
            .expt("There must have been at least one event handler!")
            .handle_cdata_event(text);
    }

    pub fn handle_end_element_event(&mut self) {
        if let Some(event_handler) = self.event_handlers.pop() {
            if let Some(android_string) = event_handler.built_string() {
                self.android_strings.push(android_string);
            }
        }
    }

    pub fn strings(mut self) -> Vec<AndroidString> {
        self.event_handlers.clear();
        self.android_strings
    }
}
