use crate::localized_string::LocalizedString;

#[derive(Clone, Debug, PartialEq)]
pub struct LocalizedStrings {
    locale: String,
    strings: Vec<LocalizedString>,
}

impl LocalizedStrings {
    pub fn new(locale: String, strings: Vec<LocalizedString>) -> LocalizedStrings {
        LocalizedStrings { locale, strings }
    }

    pub fn locale(&self) -> &str {
        &self.locale
    }

    pub fn into_strings(self) -> Vec<LocalizedString> {
        self.strings
    }
}
