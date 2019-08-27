use regex::Regex;

use crate::android_string::AndroidString;
use android_localization_helpers::DevExpt;

lazy_static::lazy_static! {
    static ref APOSTROPHE: Regex = Regex::new("(')").expt("Invalid regex!");
    static ref ESCAPED_APOSTROPHE: Regex = Regex::new(r"(\\')").expt("Invalid regex!");
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
            AndroidString::localizable("s1", "value"),
            AndroidString::localizable("s2", r"val\'ue"),
        ])
        .is_ok())
    }

    #[test]
    fn errors_in_presence_of_unescaped_apostrophes() {
        let invalid_strings = super::validate(&vec![
            AndroidString::localizable("s1", "val'ue"),
            AndroidString::localizable("s2", "value"),
            AndroidString::localizable("s3", r"val\'ue"),
            AndroidString::localizable("s4", "value'"),
            AndroidString::localizable("s5", r"\'va\l\ue\'"),
        ])
        .unwrap_err();

        assert_eq!(
            invalid_strings.invalid_strings,
            vec![
                AndroidString::localizable("s1", "val'ue"),
                AndroidString::localizable("s4", "value'"),
            ]
        )
    }
}
