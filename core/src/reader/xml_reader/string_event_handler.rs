use xml::attribute::OwnedAttribute;

use crate::android_string::AndroidString;
use crate::constants;
use crate::error::InnerError;
use crate::reader::xml_reader::event_handler::EventHandler;
use crate::reader::xml_reader::sinking_event_handler::SinkingEventHandler;

pub struct StringEventHandler {
    name: String,
    is_localizable: bool,
    built_android_string: Option<AndroidString>,
}

impl StringEventHandler {
    pub fn build(attributes: Vec<OwnedAttribute>) -> Result<StringEventHandler, InnerError> {
        let mut string_name = None;
        let mut is_localizable = true;
        for attribute in attributes {
            match attribute.name.local_name.as_str() {
                constants::attributes::NAME => string_name = Some(attribute.value),
                constants::attributes::LOCALIZABLE => {
                    if let constants::flags::FALSE = attribute.value.as_str() {
                        is_localizable = false
                    }
                }
                _ => {}
            }
        }

        match string_name {
            None => Err("string element is missing required name attribute")?,
            Some(name) => Ok(StringEventHandler {
                name,
                is_localizable,
                built_android_string: None,
            }),
        }
    }

    fn append_or_create_string(&mut self, text: String) {
        let text = match &self.built_android_string {
            None => text,
            Some(s) => format!("{}{}", s.value(), text),
        };

        self.built_android_string = Some(AndroidString::new(
            self.name.clone(),
            text,
            self.is_localizable,
        ));
    }
}

impl EventHandler for StringEventHandler {
    fn build_handler(
        &self,
        _tag_name: String,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Box<EventHandler>, InnerError> {
        Ok(Box::new(SinkingEventHandler::new()))
    }

    fn handle_characters_event(&mut self, text: String) {
        self.append_or_create_string(text)
    }

    fn handle_cdata_event(&mut self, text: String) {
        self.append_or_create_string(format!("<![CDATA[{}]]>", text))
    }

    fn built_string(&self) -> Option<AndroidString> {
        self.built_android_string.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::xml_reader::event_handler::EventHandler;

    use super::StringEventHandler;

    #[test]
    fn builds_string_with_one_character_event() {
        let mut handler = build_event_handler();
        handler.handle_characters_event(String::from("this is a test"));
        assert_string(handler, "this is a test")
    }

    #[test]
    fn builds_string_with_one_cdata_event() {
        let mut handler = build_event_handler();
        handler.handle_cdata_event(String::from("this is a test"));
        assert_string(handler, "<![CDATA[this is a test]]>")
    }

    #[test]
    fn builds_string_with_character_followed_by_cdata_event() {
        let mut handler = build_event_handler();
        handler.handle_characters_event(String::from("character event "));
        handler.handle_cdata_event(String::from("cdata event"));
        assert_string(handler, "character event <![CDATA[cdata event]]>")
    }

    #[test]
    fn builds_string_with_cdata_followed_by_character_event() {
        let mut handler = build_event_handler();
        handler.handle_cdata_event(String::from("cdata event"));
        handler.handle_characters_event(String::from(" character event"));
        assert_string(handler, "<![CDATA[cdata event]]> character event")
    }

    #[test]
    fn builds_string_with_multiple_character_and_cdata_events() {
        let mut handler = build_event_handler();
        handler.handle_characters_event(String::from("character event 1 "));
        handler.handle_cdata_event(String::from("cdata event 1"));
        handler.handle_characters_event(String::from(" character event 2 "));
        handler.handle_cdata_event(String::from("cdata event 2"));
        handler.handle_characters_event(String::from(" "));
        handler.handle_cdata_event(String::from("cdata event 3"));
        handler.handle_characters_event(String::from(" character event 3"));
        assert_string(handler, "character event 1 <![CDATA[cdata event 1]]> character event 2 <![CDATA[cdata event 2]]> <![CDATA[cdata event 3]]> character event 3")
    }

    fn build_event_handler() -> StringEventHandler {
        StringEventHandler {
            name: String::from("test_string"),
            is_localizable: true,
            built_android_string: None,
        }
    }

    fn assert_string(handler: StringEventHandler, expected: &str) {
        assert_eq!(handler.built_string().unwrap().value(), expected)
    }
}
