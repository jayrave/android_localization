#[derive(Clone, Debug, PartialEq)]
pub struct LocalizedString {
    default: String,
    localized: String,
}

impl LocalizedString {
    pub fn new(default: String, localized: String) -> LocalizedString {
        LocalizedString { default, localized }
    }

    pub fn default(&self) -> &str {
        &self.default
    }

    pub fn localized(&self) -> &str {
        &self.localized
    }
}
