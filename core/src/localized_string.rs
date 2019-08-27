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

    pub fn build<N: Into<String>, D: Into<String>, L: Into<String>>(
        name: N,
        default: D,
        localized: L,
    ) -> LocalizedString {
        LocalizedString::new(name.into(), default.into(), localized.into())
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
