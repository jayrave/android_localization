use crate::android_string::AndroidString;
use crate::ops::sort;
use android_localization_helpers::DevExpt;

/// While grouping strings, strings from `strings_1` take precedence over `strings_2` in case both
/// strings have the same name
pub fn merge_and_group_strings(
    strings_1: &mut [AndroidString],
    strings_2: &mut [AndroidString],
) -> Vec<AndroidString> {
    // Sort both the list to group list-wise
    sort::sort_android_strings_by_name(strings_1);
    sort::sort_android_strings_by_name(strings_2);

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
            && (string_2.is_none()
                || string_1
                    .expt("Already checked for is_some but still fails!")
                    .name()
                    <= string_2
                        .expt("Already checked for is_some but still fails!")
                        .name())
        {
            result.push(
                string_1
                    .expt("Already checked for is_some but still fails!")
                    .clone(),
            );
            strings_1_index += 1;
        } else {
            result.push(
                string_2
                    .expt("Already checked for is_some but still fails!")
                    .clone(),
            );
            strings_2_index += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::android_string::AndroidString;

    #[test]
    fn merges_and_groups() {
        let mut strings = super::merge_and_group_strings(
            &mut vec![
                AndroidString::localizable("string_1", "string value"),
                AndroidString::unlocalizable("string_4", "string value"),
            ],
            &mut vec![
                AndroidString::localizable("string_3", "string value"),
                AndroidString::unlocalizable("string_2", "string value"),
            ],
        )
        .into_iter();

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::localizable("string_1", "string value")
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::unlocalizable("string_2", "string value")
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::localizable("string_3", "string value")
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::unlocalizable("string_4", "string value")
        );

        assert_eq!(strings.next(), None);
    }

    #[test]
    fn list_1_strings_takes_precedence_over_list_2_strings_in_case_of_same_name() {
        let mut strings = super::merge_and_group_strings(
            &mut vec![
                AndroidString::localizable("string_1", "from list 1"),
                AndroidString::unlocalizable("string_3", "from list 1"),
                AndroidString::unlocalizable("string_1", "from list 1 again"),
            ],
            &mut vec![
                AndroidString::unlocalizable("string_1", "from list 2"),
                AndroidString::localizable("string_2", "from list 2"),
            ],
        )
        .into_iter();

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::localizable("string_1", "from list 1")
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::unlocalizable("string_1", "from list 1 again")
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::unlocalizable("string_1", "from list 2")
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::localizable("string_2", "from list 2")
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::unlocalizable("string_3", "from list 1")
        );

        assert_eq!(strings.next(), None);
    }
}
