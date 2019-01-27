use android_string::AndroidString;

#[derive(Clone, Debug, PartialEq)]
pub struct LocalizableStrings {
    locale: String,
    strings: Vec<AndroidString>,
}

impl LocalizableStrings {
    pub fn new(locale: String, strings: Vec<AndroidString>) -> LocalizableStrings {
        LocalizableStrings { locale, strings }
    }

    pub fn locale(&self) -> &str {
        &self.locale
    }

    pub fn strings(&self) -> &Vec<AndroidString> {
        &self.strings
    }
}
