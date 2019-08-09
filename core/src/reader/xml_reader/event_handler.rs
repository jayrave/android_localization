use crate::error::Error;
use crate::android_string::AndroidString;
use xml::attribute::OwnedAttribute;

/// One instance of `EventHandler` is only expected to ever build one `AndroidString`
pub trait EventHandler {
    fn handler_for_start_element_event(
        &self,
        tag_name: String,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Box<EventHandler>, Error>;

    fn handle_characters_event(&mut self, _text: String) {
        // No op
    }

    fn handle_cdata_event(&mut self, _text: String) {
        // No op
    }

    // It would be great it a way can be found to make this consume self instead of
    // just take in a reference. Compiler complains if this is made a consumer as
    // `EventHandler` is used as a trait object & boxed, un-sized objects can't be
    // moved out :(
    fn built_string(&self) -> Option<AndroidString> {
        None
    }
}
