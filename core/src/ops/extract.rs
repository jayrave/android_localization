use android_string::AndroidString;
use ops::sort;
use reader::localized_string::LocalizedString;
use std::cmp::Ordering;

/// Localized strings will be converted into `AndroidString` only if both the name
/// & the default value from `LocalizedString` match up with whatever is in the
/// default string
pub fn extract_android_strings_from_localized(
    localized_strings: &mut Vec<LocalizedString>,
    default_strings: &mut Vec<AndroidString>,
) -> Vec<AndroidString> {
    // Sort both the incoming strings
    sort::sort_android_strings_by_name(default_strings);
    sort::sort_localized_strings_by_name(localized_strings);

    let total_strings_count = localized_strings.len() + default_strings.len();
    let mut result = Vec::with_capacity(localized_strings.len()); // Max number of expected strings
    let mut localized_strings_index = 0;
    let mut default_strings_index = 0;

    for _ in 0..total_strings_count {
        let localized_string = localized_strings.get(localized_strings_index);
        let default_string = default_strings.get(default_strings_index);

        // Can't compare if either of the strings have run out! This check is imperative as the
        // code flow in the else block increments both strings' pointers if there is a match
        if localized_string.is_none() || default_string.is_none() {
            break;
        } else {
            let localized_string = localized_string.unwrap();
            let default_string = default_string.unwrap();
            match localized_string.name().cmp(default_string.name()) {
                Ordering::Less => localized_strings_index += 1,
                Ordering::Greater => default_strings_index += 1,
                Ordering::Equal => {
                    if localized_string.default() == default_string.value() {
                        result.push(AndroidString::new(
                            String::from(localized_string.name()),
                            String::from(localized_string.localized()),
                            default_string.is_translatable(),
                        ));
                    }

                    // Feel free to increment both the indices as we have a `is_none` check
                    // for both the strings
                    localized_strings_index += 1;
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
    use reader::localized_string::LocalizedString;

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

        let mut localized_strings = vec![
            LocalizedString::new(
                String::from("string_3"),
                String::from("english 3 value"),
                String::from("french 3 value"),
            ),
            LocalizedString::new(
                String::from("string_4"),
                String::from("english 4 value"),
                String::from("french 4 value"),
            ),
            LocalizedString::new(
                String::from("string_2"),
                String::from("english 2 value"),
                String::from("french 2 value"),
            ),
        ];

        let mut strings = super::extract_android_strings_from_localized(
            &mut localized_strings,
            &mut default_strings,
        )
        .into_iter();

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
