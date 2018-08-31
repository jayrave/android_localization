use android_string::AndroidString;
use ops::sort;

/// While grouping strings, strings from `strings_1` take precedence over `strings_2` in case both
/// strings have the same name
pub fn merge_and_group_strings(
    strings_1: &mut Vec<AndroidString>,
    strings_2: &mut Vec<AndroidString>,
) -> Vec<AndroidString> {
    // Sort both the list to group list-wise
    sort::sort_strings(strings_1);
    sort::sort_strings(strings_2);

    // Since both the lists are sorted, we can use a 2-index pointer method to merge them
    // & keep the strings with same name together
    let total_strings_count = strings_1.len() + strings_2.len();
    let mut result = Vec::with_capacity(total_strings_count);
    let mut strings_1_index = 0;
    let mut strings_2_index = 0;
    for _ in 0..total_strings_count {
        let string_1 = strings_1.get(strings_1_index);
        let string_2 = strings_2.get(strings_2_index);

        // Our iteration condition will make sure that either string_1 or string_2
        // will be a valid string always
        if string_1.is_some()
            && (string_2.is_none() || string_1.unwrap().name() <= string_2.unwrap().name())
        {
            result.push(string_1.unwrap().clone());
            strings_1_index += 1;
        } else {
            result.push(string_2.unwrap().clone());
            strings_2_index += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use android_string::AndroidString;

    #[test]
    fn merged_and_grouped() {
        let mut strings = super::merge_and_group_strings(
            &mut vec![
                AndroidString::new(String::from("string_1"), String::from("string value"), true),
                AndroidString::new(
                    String::from("string_4"),
                    String::from("string value"),
                    false,
                ),
            ],
            &mut vec![
                AndroidString::new(String::from("string_3"), String::from("string value"), true),
                AndroidString::new(
                    String::from("string_2"),
                    String::from("string value"),
                    false,
                ),
            ],
        ).into_iter();

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(String::from("string_1"), String::from("string value"), true)
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(
                String::from("string_2"),
                String::from("string value"),
                false
            )
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(String::from("string_3"), String::from("string value"), true)
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(
                String::from("string_4"),
                String::from("string value"),
                false
            )
        );

        assert_eq!(strings.next(), None);
    }

    #[test]
    fn merged_and_grouped_with_list_1_strings_taking_precedence_over_list_2_strings_in_case_of_same_name(
) {
        let mut strings = super::merge_and_group_strings(
            &mut vec![
                AndroidString::new(String::from("string_1"), String::from("from list 1"), true),
                AndroidString::new(String::from("string_3"), String::from("from list 1"), false),
                AndroidString::new(
                    String::from("string_1"),
                    String::from("from list 1 again"),
                    false,
                ),
            ],
            &mut vec![
                AndroidString::new(String::from("string_1"), String::from("from list 2"), false),
                AndroidString::new(String::from("string_2"), String::from("from list 2"), true),
            ],
        ).into_iter();

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(String::from("string_1"), String::from("from list 1"), true)
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(
                String::from("string_1"),
                String::from("from list 1 again"),
                false
            )
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(String::from("string_1"), String::from("from list 2"), false)
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(String::from("string_2"), String::from("from list 2"), true)
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(String::from("string_3"), String::from("from list 1"), false)
        );

        assert_eq!(strings.next(), None);
    }
}
