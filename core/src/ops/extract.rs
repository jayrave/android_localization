use android_string::AndroidString;
use ops::sort;
use reader::translated_string::TranslatedString;
use std::cmp::Ordering;

/// Translated strings will be converted into `AndroidString` only if both the name
/// & the default value from `TranslatedString` match up with whatever is in the
/// default string
pub fn extract_android_strings_from_translated(
    translated_strings: &mut Vec<TranslatedString>,
    default_strings: &mut Vec<AndroidString>,
) -> Vec<AndroidString> {
    // Sort both the incoming strings
    sort::sort_android_strings_by_name(default_strings);
    sort::sort_translated_strings_by_name(translated_strings);

    let total_strings_count = translated_strings.len() + default_strings.len();
    let mut result = Vec::with_capacity(translated_strings.len()); // Max number of expected strings
    let mut translated_strings_index = 0;
    let mut default_strings_index = 0;

    for _ in 0..total_strings_count {
        let translated_string = translated_strings.get(translated_strings_index);
        let default_string = default_strings.get(default_strings_index);

        // Can't compare if either of the strings have run out! This check is imperative as the
        // code flow in the else block increments both strings' pointers if there is a match
        if translated_string.is_none() || default_string.is_none() {
            break;
        } else {
            let translated_string = translated_string.unwrap();
            let default_string = default_string.unwrap();
            match translated_string.name().cmp(default_string.name()) {
                Ordering::Less => translated_strings_index += 1,
                Ordering::Greater => default_strings_index += 1,
                Ordering::Equal => {
                    if translated_string.default() == default_string.value() {
                        result.push(AndroidString::new(
                            String::from(translated_string.name()),
                            String::from(translated_string.translated()),
                            default_string.is_translatable(),
                        ));
                    }

                    // Feel free to increment both the indices as we have a `is_none` check
                    // for both the strings
                    translated_strings_index += 1;
                    default_strings_index += 1;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use android_string::AndroidString;
    use reader::translated_string::TranslatedString;

    #[test]
    fn extracted() {
        let mut default_strings = vec![
            AndroidString::new(
                String::from("string_2"),
                String::from("english 2 value"),
                false,
            ), // `false` to make sure the flag is carried over from here
            AndroidString::new(
                String::from("string_3"),
                String::from("english 3 value"),
                true,
            ),
            AndroidString::new(
                String::from("string_4"),
                String::from("english 4 new value"),
                true,
            ), // new value to make sure match is against both name & value
            AndroidString::new(
                String::from("string_1"),
                String::from("english 1 value"),
                true,
            ),
        ];

        let mut translated_strings = vec![
            TranslatedString::new(
                String::from("string_3"),
                String::from("english 3 value"),
                String::from("french 3 value"),
            ),
            TranslatedString::new(
                String::from("string_4"),
                String::from("english 4 value"),
                String::from("french 4 value"),
            ),
            TranslatedString::new(
                String::from("string_2"),
                String::from("english 2 value"),
                String::from("french 2 value"),
            ),
        ];

        let mut strings = super::extract_android_strings_from_translated(
            &mut translated_strings,
            &mut default_strings,
        ).into_iter();

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(
                String::from("string_2"),
                String::from("french 2 value"),
                false
            )
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(
                String::from("string_3"),
                String::from("french 3 value"),
                true
            )
        );

        assert_eq!(strings.next(), None);
    }
}
