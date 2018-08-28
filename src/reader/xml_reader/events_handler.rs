use android_string::AndroidString;
use reader::error::Error;
use reader::xml_reader::event_handler::EventHandler;
use reader::xml_reader::root_event_handler::RootEventHandler;
use std::cell::RefCell;
use std::rc::Rc;
use xml::attribute::OwnedAttribute;

pub struct EventsHandler {
    android_strings: Rc<RefCell<Vec<AndroidString>>>,
    event_handlers: Vec<Box<EventHandler>>,
}

impl EventsHandler {
    pub fn new() -> EventsHandler {
        let strings = Rc::new(RefCell::new(vec![]));
        let strings_clone = Rc::clone(&strings);
        EventsHandler {
            android_strings: strings,
            event_handlers: vec![Box::new(RootEventHandler::new(strings_clone))],
        }
    }

    pub fn handle_start_element_event(
        &mut self,
        tag_name: String,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<(), Error> {
        let event_handler = self
            .event_handlers
            .last_mut()
            .unwrap()
            .handler_for_start_element_event(tag_name, attributes)?;
        self.event_handlers.push(event_handler);
        Ok(())
    }

    pub fn handle_characters_event(&mut self, text: String) {
        self.event_handlers
            .last_mut()
            .unwrap()
            .handle_characters_event(text);
    }

    pub fn handle_end_element_event(&mut self) {
        self.event_handlers.pop();
    }

    pub fn strings(mut self) -> Result<Vec<AndroidString>, Error> {
        self.event_handlers.clear();
        match Rc::try_unwrap(self.android_strings) {
            Err(ref_cell) => Err(Error::LogicError(format!(
                "Rc has {} strong references!",
                Rc::strong_count(&ref_cell)
            ))),
            Ok(ref_cell) => Ok(ref_cell.into_inner()),
        }
    }
}
