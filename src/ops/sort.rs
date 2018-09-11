use android_string::AndroidString;
use reader::translated_string::TranslatedString;
use std::cmp::Ordering;

/// In place, stable sorting
pub fn sort_android_strings_by_name(strings: &mut Vec<AndroidString>) {
    // Not using `sort_by_key` as I can't figure out how to specify
    // lifetime for closure's return :(
    strings.sort_by(|s1, s2| s1.name().cmp(s2.name()));
}

/// In place, stable sorting
pub fn sort_translated_strings_by_name(strings: &mut Vec<TranslatedString>) {
    // Not using `sort_by_key` as I can't figure out how to specify
    // lifetime for closure's return :(
    strings.sort_by(|s1, s2| s1.name().cmp(s2.name()));
}

pub fn compare_android_strings(s1: &AndroidString, s2: &AndroidString) -> Ordering {
    s1.name().cmp(s2.name())
}

#[cfg(test)]
mod tests {
    use android_string::AndroidString;
    use reader::translated_string::TranslatedString;

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
    fn translated_sorted_by_name() {
        let mut strings = vec![
            TranslatedString::new(
                String::from("string_2"),
                String::from("string value"),
                String::from("string value"),
            ),
            TranslatedString::new(
                String::from("string_3"),
                String::from("string 3 value 1"),
                String::from("string 3 value 1"),
            ),
            TranslatedString::new(
                String::from("string_3"),
                String::from("string 3 value 2"),
                String::from("string 3 value 2"),
            ),
            TranslatedString::new(
                String::from("string_1"),
                String::from("string value"),
                String::from("string value"),
            ),
        ];

        super::sort_translated_strings_by_name(&mut strings);
        let mut strings = strings.into_iter();

        assert_eq!(
            strings.next().unwrap(),
            TranslatedString::new(
                String::from("string_1"),
                String::from("string value"),
                String::from("string value")
            )
        );

        assert_eq!(
            strings.next().unwrap(),
            TranslatedString::new(
                String::from("string_2"),
                String::from("string value"),
                String::from("string value")
            )
        );

        assert_eq!(
            strings.next().unwrap(),
            TranslatedString::new(
                String::from("string_3"),
                String::from("string 3 value 1"),
                String::from("string 3 value 1")
            )
        );

        assert_eq!(
            strings.next().unwrap(),
            TranslatedString::new(
                String::from("string_3"),
                String::from("string 3 value 2"),
                String::from("string 3 value 2")
            )
        );

        assert_eq!(strings.next(), None);
    }
}
