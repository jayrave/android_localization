use crate::android_string::AndroidString;
use crate::ops::sort;
use crate::util::two_pointer_traversal;

pub fn validate(
    default_strings: &mut [AndroidString],
    foreign_strings: &mut [AndroidString],
) -> Result<(), MissingStrings> {
    // Sort both the lists
    sort::sort_android_strings_by_name(default_strings);
    sort::sort_android_strings_by_name(foreign_strings);

    let mut extra_in_default_locale = vec![];
    let mut extra_in_foreign_locale = vec![];
    two_pointer_traversal::diff(
        default_strings,
        foreign_strings,
        |default_string, foriegn_string| default_string.name().cmp(foriegn_string.name()),
        |default_string| {
            // It is ok for non-translatable strings to be present in default locale but
            // not the other way around
            if default_string.is_localizable() {
                extra_in_default_locale.push(default_string.clone())
            }
        },
        |foriegn_string| extra_in_foreign_locale.push(foriegn_string.clone()),
    );

    if extra_in_default_locale.is_empty() && extra_in_foreign_locale.is_empty() {
        Ok(())
    } else {
        Err(MissingStrings {
            extra_in_default_locale,
            extra_in_foreign_locale,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct MissingStrings {
    pub extra_in_default_locale: Vec<AndroidString>,
    pub extra_in_foreign_locale: Vec<AndroidString>,
}

#[cfg(test)]
mod tests {
    use crate::android_string::AndroidString;

    use super::MissingStrings;

    #[test]
    fn validate_passes_in_absence_of_missing_strings() {
        let mut default_strings = vec![
            AndroidString::localizable("s2", "d2"),
            AndroidString::localizable("s3", "d3"),
        ];

        let mut foreign_strings = vec![
            AndroidString::localizable("s2", "f2"),
            AndroidString::localizable("s3", "f3"),
        ];

        assert!(super::validate(&mut default_strings, &mut foreign_strings).is_ok())
    }

    #[test]
    fn validate_errors_in_presence_of_missing_strings() {
        let mut default_strings = vec![
            AndroidString::localizable("s3", "d3"),
            AndroidString::localizable("s2", "d2"),
            AndroidString::unlocalizable("s1", "d1"),
            AndroidString::localizable("s4", "d4"),
        ];

        let mut foreign_strings = vec![
            AndroidString::localizable("s5", "f3"),
            AndroidString::localizable("s2", "f2"),
            AndroidString::localizable("s8", "f4"),
            AndroidString::unlocalizable("s6", "f1"),
        ];

        assert_eq!(
            super::validate(&mut default_strings, &mut foreign_strings).unwrap_err(),
            MissingStrings {
                extra_in_default_locale: vec![
                    AndroidString::localizable("s3", "d3"),
                    AndroidString::localizable("s4", "d4"),
                ],
                extra_in_foreign_locale: vec![
                    AndroidString::localizable("s5", "f3"),
                    AndroidString::unlocalizable("s6", "f1"),
                    AndroidString::localizable("s8", "f4"),
                ],
            }
        )
    }
}
