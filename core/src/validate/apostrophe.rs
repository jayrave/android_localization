use android_string::AndroidString;
use regex::Regex;
use std::error;
use std::fmt;

lazy_static! {
    static ref APOSTROPHE: Regex = Regex::new("(')").unwrap();
    static ref ESCAPED_APOSTROPHE: Regex = Regex::new(r"(\\')").unwrap();
}

pub fn validate(strings: &Vec<AndroidString>) -> Result<(), Error> {
    let invalid_strings: Vec<AndroidString> = strings
        .iter()
        .filter(|s| !is_valid_value(s.value()))
        .map(|s| s.clone())
        .collect();

    match invalid_strings.is_empty() {
        true => Ok(()),
        false => Err(Error { invalid_strings }),
    }
}

fn is_valid_value(value: &str) -> bool {
    // Could use look behind/look ahead, but this is easier to understand & implement
    APOSTROPHE.captures_iter(value).count() == ESCAPED_APOSTROPHE.captures_iter(value).count()
}

#[derive(Debug)]
pub struct Error {
    invalid_strings: Vec<AndroidString>,
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.invalid_strings
            .iter()
            .map(|s| {
                write!(f, "(");
                fmt::Display::fmt(s, f);
                write!(f, ")")
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use android_string::AndroidString;

    #[test]
    fn passes_in_absence_of_unescaped_apostrophes() {
        assert!(
            super::validate(&vec![
                AndroidString::new(String::from("s1"), String::from("value"), true),
                AndroidString::new(String::from("s2"), String::from(r"val\'ue"), true),
            ]).is_ok()
        )
    }

    #[test]
    fn errors_in_presence_of_unescaped_apostrophes() {
        let error = super::validate(&vec![
            AndroidString::new(String::from("s1"), String::from("val'ue"), true),
            AndroidString::new(String::from("s2"), String::from("value"), true),
            AndroidString::new(String::from("s3"), String::from(r"val\'ue"), true),
            AndroidString::new(String::from("s4"), String::from("value'"), true),
            AndroidString::new(String::from("s5"), String::from(r"\'va\l\ue\'"), true),
        ]).unwrap_err();

        assert_eq!(
            error.invalid_strings,
            vec![
                AndroidString::new(String::from("s1"), String::from("val'ue"), true),
                AndroidString::new(String::from("s4"), String::from("value'"), true),
            ]
        )
    }
}
