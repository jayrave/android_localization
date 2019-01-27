use std::fmt;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct AndroidString {
    name: String,
    value: String,
    is_translatable: bool,
}

impl AndroidString {
    pub fn new(name: String, value: String, is_translatable: bool) -> AndroidString {
        AndroidString {
            name,
            value,
            is_translatable,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn is_translatable(&self) -> bool {
        self.is_translatable
    }
}

impl fmt::Display for AndroidString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Translatable: {}; Name: {}; Value: {}",
            self.is_translatable(),
            self.name(),
            self.value()
        )
    }
}
