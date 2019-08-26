use crate::android_string::AndroidString;

#[derive(Clone, Debug, PartialEq)]
pub struct LocalizableStrings {
    to_locale: String,
    default_locale_strings: Vec<AndroidString>,
}

impl LocalizableStrings {
    pub fn new(
        to_locale: String,
        default_locale_strings: Vec<AndroidString>,
    ) -> LocalizableStrings {
        LocalizableStrings {
            to_locale,
            default_locale_strings,
        }
    }

    pub fn to_locale(&self) -> &str {
        &self.to_locale
    }

    pub fn default_locale_strings(&self) -> &[AndroidString] {
        &self.default_locale_strings
    }
}
