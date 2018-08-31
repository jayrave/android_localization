#[derive(Clone, Debug, PartialEq)]
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

    pub fn translatable(name: String, value: String) -> AndroidString {
        AndroidString {
            name,
            value,
            is_translatable: true,
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
