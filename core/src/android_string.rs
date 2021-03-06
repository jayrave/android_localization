use std::fmt;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct AndroidString {
    name: String,
    value: String,
    is_localizable: bool,
}

impl AndroidString {
    pub fn new(name: String, value: String, is_localizable: bool) -> AndroidString {
        AndroidString {
            name,
            value,
            is_localizable,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn is_localizable(&self) -> bool {
        self.is_localizable
    }
}

impl fmt::Display for AndroidString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Localizable: {}; Name: {}; Value: {}",
            self.is_localizable(),
            self.name(),
            self.value()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::AndroidString;

    /// To expose a convenient way to build for tests
    impl AndroidString {
        pub fn localizable<N: Into<String>, V: Into<String>>(name: N, value: V) -> AndroidString {
            AndroidString::new(name.into(), value.into(), true)
        }

        pub fn unlocalizable<N: Into<String>, V: Into<String>>(name: N, value: V) -> AndroidString {
            AndroidString::new(name.into(), value.into(), false)
        }
    }
}
