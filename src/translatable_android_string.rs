#[derive(Debug, PartialEq)]
pub struct TranslatableAndroidString {
    name: String,
    value: String,
}

impl TranslatableAndroidString {
    pub fn new(name: String, value: String) -> TranslatableAndroidString {
        TranslatableAndroidString { name, value }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
