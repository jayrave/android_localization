use regex::Regex;

use crate::android_string::AndroidString;

lazy_static::lazy_static! {
    static ref APOSTROPHE: Regex = Regex::new("(')").unwrap();
    static ref ESCAPED_APOSTROPHE: Regex = Regex::new(r"(\\')").unwrap();
}

pub fn validate(strings: &[AndroidString]) -> Result<(), InvalidStrings> {
    let invalid_strings: Vec<AndroidString> = strings
        .iter()
        .filter(|s| is_invalid_value(s.value()))
        .cloned()
        .collect();

    if invalid_strings.is_empty() {
        Ok(())
    } else {
        Err(InvalidStrings { invalid_strings })
    }
}

fn is_invalid_value(value: &str) -> bool {
    // Could use look behind/look ahead, but this is easier to understand & implement
    APOSTROPHE.captures_iter(value).count() != ESCAPED_APOSTROPHE.captures_iter(value).count()
}

#[derive(Debug, PartialEq)]
pub struct InvalidStrings {
    pub invalid_strings: Vec<AndroidString>,
}

#[cfg(test)]
mod tests {
    use crate::android_string::AndroidString;

    #[test]
    fn passes_in_absence_of_unescaped_apostrophes() {
        assert!(super::validate(&vec![
            AndroidString::new(String::from("s1"), String::from("value"), true),
            AndroidString::new(String::from("s2"), String::from(r"val\'ue"), true),
        ])
        .is_ok())
    }

    #[test]
    fn errors_in_presence_of_unescaped_apostrophes() {
        let invalid_strings = super::validate(&vec![
            AndroidString::new(String::from("s1"), String::from("val'ue"), true),
            AndroidString::new(String::from("s2"), String::from("value"), true),
            AndroidString::new(String::from("s3"), String::from(r"val\'ue"), true),
            AndroidString::new(String::from("s4"), String::from("value'"), true),
            AndroidString::new(String::from("s5"), String::from(r"\'va\l\ue\'"), true),
        ])
        .unwrap_err();

        assert_eq!(
            invalid_strings.invalid_strings,
            vec![
                AndroidString::new(String::from("s1"), String::from("val'ue"), true),
                AndroidString::new(String::from("s4"), String::from("value'"), true),
            ]
        )
    }
}
