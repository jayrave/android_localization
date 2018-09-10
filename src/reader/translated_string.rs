#[derive(Clone, Debug, PartialEq)]
pub struct TranslatedString {
    name: String,
    default: String,
    translated: String,
}

impl TranslatedString {
    pub fn new(name: String, default: String, translated: String) -> TranslatedString {
        TranslatedString {
            name,
            default,
            translated,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn default(&self) -> &str {
        &self.default
    }

    pub fn translated(&self) -> &str {
        &self.translated
    }
}
