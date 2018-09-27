use android_string::AndroidString;
use reader::xml_reader::error::Error;
use xml::attribute::OwnedAttribute;

pub trait EventHandler {
    fn handler_for_start_element_event(
        &self,
        tag_name: String,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Box<EventHandler>, Error>;

    fn handle_characters_event(&mut self, text: String);

    // It would be great it a way can be found to make this consume self instead of
    // just take in a reference. Compiler complains if this is made a consumer as
    // `EventHandler` is used as a trait object & boxed, un-sized objects can't be
    // moved out :(
    fn built_string(&self) -> Option<AndroidString>;
}
