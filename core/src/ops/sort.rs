use crate::android_string::AndroidString;
use crate::localized_string::LocalizedString;
use std::cmp::Ordering;

/// In place, stable sorting
pub fn sort_android_strings_by_name(strings: &mut Vec<AndroidString>) {
    // Not using `sort_by_key` as I can't figure out how to specify
    // lifetime for closure's return :(
    strings.sort_by(|s1, s2| s1.name().cmp(s2.name()));
}

/// In place, stable sorting
pub fn sort_localized_strings_by_name(strings: &mut Vec<LocalizedString>) {
    // Not using `sort_by_key` as I can't figure out how to specify
    // lifetime for closure's return :(
    strings.sort_by(|s1, s2| s1.name().cmp(s2.name()));
}

pub fn compare_android_strings(s1: &AndroidString, s2: &AndroidString) -> Ordering {
    s1.name().cmp(s2.name())
}

#[cfg(test)]
mod tests {
    use crate::android_string::AndroidString;
    use crate::localized_string::LocalizedString;

    #[test]
    fn android_sorted_by_name() {
        let mut strings = vec![
            AndroidString::new(String::from("string_2"), String::from("string value"), true),
            AndroidString::new(
                String::from("string_3"),
                String::from("string 3 value 1"),
                true,
            ),
            AndroidString::new(
                String::from("string_3"),
                String::from("string 3 value 2"),
                true,
            ),
            AndroidString::new(String::from("string_1"), String::from("string value"), true),
        ];

        super::sort_android_strings_by_name(&mut strings);
        let mut strings = strings.into_iter();

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(String::from("string_1"), String::from("string value"), true)
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(String::from("string_2"), String::from("string value"), true)
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(
                String::from("string_3"),
                String::from("string 3 value 1"),
                true
            )
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(
                String::from("string_3"),
                String::from("string 3 value 2"),
                true
            )
        );

        assert_eq!(strings.next(), None);
    }

    #[test]
    fn localized_sorted_by_name() {
        let mut strings = vec![
            LocalizedString::new(
                String::from("string_2"),
                String::from("string value"),
                String::from("string value"),
            ),
            LocalizedString::new(
                String::from("string_3"),
                String::from("string 3 value 1"),
                String::from("string 3 value 1"),
            ),
            LocalizedString::new(
                String::from("string_3"),
                String::from("string 3 value 2"),
                String::from("string 3 value 2"),
            ),
            LocalizedString::new(
                String::from("string_1"),
                String::from("string value"),
                String::from("string value"),
            ),
        ];

        super::sort_localized_strings_by_name(&mut strings);
        let mut strings = strings.into_iter();

        assert_eq!(
            strings.next().unwrap(),
            LocalizedString::new(
                String::from("string_1"),
                String::from("string value"),
                String::from("string value")
            )
        );

        assert_eq!(
            strings.next().unwrap(),
            LocalizedString::new(
                String::from("string_2"),
                String::from("string value"),
                String::from("string value")
            )
        );

        assert_eq!(
            strings.next().unwrap(),
            LocalizedString::new(
                String::from("string_3"),
                String::from("string 3 value 1"),
                String::from("string 3 value 1")
            )
        );

        assert_eq!(
            strings.next().unwrap(),
            LocalizedString::new(
                String::from("string_3"),
                String::from("string 3 value 2"),
                String::from("string 3 value 2")
            )
        );

        assert_eq!(strings.next(), None);
    }
}
