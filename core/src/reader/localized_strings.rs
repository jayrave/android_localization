use reader::translated_string::TranslatedString;

#[derive(Clone, Debug, PartialEq)]
pub struct LocalizedStrings {
    locale: String,
    strings: Vec<TranslatedString>,
}

impl LocalizedStrings {
    pub fn new(locale: String, strings: Vec<TranslatedString>) -> LocalizedStrings {
        LocalizedStrings { locale, strings }
    }

    pub fn locale(&self) -> &str {
        &self.locale
    }

    pub fn strings(&self) -> &Vec<TranslatedString> {
        &self.strings
    }
}
