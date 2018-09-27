use android_string::AndroidString;
use constants;
use reader::xml_reader::error::Error;
use reader::xml_reader::event_handler::EventHandler;
use reader::xml_reader::sinking_event_handler::SinkingEventHandler;
use xml::attribute::OwnedAttribute;

pub struct StringEventHandler {
    name: String,
    is_translatable: bool,
    built_android_string: Option<AndroidString>,
}

impl StringEventHandler {
    pub fn new(attributes: Vec<OwnedAttribute>) -> Result<StringEventHandler, Error> {
        let mut string_name = None;
        let mut is_translatable = true;
        for attribute in attributes {
            match attribute.name.local_name.as_str() {
                constants::attributes::NAME => string_name = Some(attribute.value),
                constants::attributes::TRANSLATABLE => {
                    if let constants::flags::FALSE = attribute.value.as_str() {
                        is_translatable = false
                    }
                }
                _ => {}
            }
        }

        match string_name {
            None => Err(Error::SyntaxError(String::from(
                "string element is missing required name attribute",
            ))),
            Some(name) => Ok(StringEventHandler {
                name,
                is_translatable,
                built_android_string: None,
            }),
        }
    }
}

impl EventHandler for StringEventHandler {
    fn handler_for_start_element_event(
        &self,
        _tag_name: String,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Box<EventHandler>, Error> {
        Ok(Box::new(SinkingEventHandler::new()))
    }

    fn handle_characters_event(&mut self, text: String) {
        self.built_android_string = Some(AndroidString::new(
            self.name.clone(),
            text,
            self.is_translatable,
        ));
    }

    fn built_string(&self) -> Option<AndroidString> {
        self.built_android_string.clone()
    }
}
