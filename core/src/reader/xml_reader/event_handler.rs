use xml::attribute::OwnedAttribute;

use crate::android_string::AndroidString;
use crate::error::Error;

/// One instance of `EventHandler` is only expected to ever build one `AndroidString`
pub trait EventHandler {
    fn build_handler(
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

    // It would be great if a way can be found to make this consume self instead of
    // just take in a reference. Compiler complains if this is made a consumer as
    // `EventHandler` is used as a trait object & boxed, un-sized objects can't be
    // moved out :(
    fn built_string(&self) -> Option<AndroidString> {
        None
    }
}
