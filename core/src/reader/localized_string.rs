#[derive(Clone, Debug, PartialEq)]
pub struct LocalizedString {
    name: String,
    default: String,
    localized: String,
}

impl LocalizedString {
    pub fn new(name: String, default: String, localized: String) -> LocalizedString {
        LocalizedString {
            name,
            default,
            localized,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn default(&self) -> &str {
        &self.default
    }

    pub fn localized(&self) -> &str {
        &self.localized
    }
}
