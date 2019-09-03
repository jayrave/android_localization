use std::error::Error;

/// To easily add context to panic messages
pub trait DevExpt<T, S: AsRef<str>> {
    fn expt(self, msg: S) -> T;
}

impl<T, S: AsRef<str>> DevExpt<T, S> for Option<T> {
    fn expt(self, msg: S) -> T {
        self.unwrap_or_else(|| panic!(build_message_to_contact_dev(msg)))
    }
}

impl<T, S: AsRef<str>, E: Error> DevExpt<T, S> for Result<T, E> {
    fn expt(self, msg: S) -> T {
        self.unwrap_or_else(|error| {
            panic!(build_message_to_contact_dev(format!(
                "{}: {}",
                error,
                msg.as_ref()
            )))
        })
    }
}

fn build_message_to_contact_dev<T: AsRef<str>>(msg: T) -> String {
    format!(
        r#"{}
This is not supposed to happen!
Please file an issue at https://github.com/jayrave/android_localization"#,
        msg.as_ref()
    )
}
