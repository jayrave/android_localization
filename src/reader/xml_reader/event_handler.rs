use reader::error::Error;
use xml::attribute::OwnedAttribute;

pub trait EventHandler {
    fn handler_for_start_element_event(
        &self,
        tag_name: String,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Box<EventHandler>, Error>;

    fn handle_characters_event(&self, text: String);
}
