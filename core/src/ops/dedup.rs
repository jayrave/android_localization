use crate::android_string::AndroidString;

/// This methods assumes that all the strings that have the same name are grouped
/// together. If such groups are present, only the first string from those groups
/// will be let through
pub fn dedup_grouped_strings(mut android_strings: Vec<AndroidString>) -> Vec<AndroidString> {
    let mut indices_to_remove_in_asc_order = vec![];

    // Find indices of all repeated strings. Starting a new scope since
    // `last_string_name_let_through` needs to be dropped to mutate
    // `android_strings`
    {
        let mut last_string_name_let_through: Option<&str> = None;
        for (index, string) in android_strings.iter().enumerate() {
            match last_string_name_let_through {
                None => last_string_name_let_through = Some(string.name()),
                Some(let_through_name) => {
                    if let_through_name == string.name() {
                        indices_to_remove_in_asc_order.push(index)
                    } else {
                        last_string_name_let_through = Some(string.name())
                    }
                }
            }
        }
    }

    // Mutate list in place
    for (remove_count, index_to_remove) in indices_to_remove_in_asc_order.into_iter().enumerate() {
        android_strings.remove(index_to_remove - remove_count);
    }

    android_strings
}

#[cfg(test)]
mod tests {
    use crate::android_string::AndroidString;

    #[test]
    fn list_with_uniq_items_returned_as_is() {
        let original_list = vec![
            AndroidString::new(
                String::from("string_1"),
                String::from("string 1 value"),
                true,
            ),
            AndroidString::new(
                String::from("string_2"),
                String::from("string 2 value"),
                false,
            ),
            AndroidString::new(
                String::from("string_3"),
                String::from("string 3 value"),
                true,
            ),
            AndroidString::new(
                String::from("string_4"),
                String::from("string 4 value"),
                false,
            ),
        ];

        let deduplicated_list = super::dedup_grouped_strings(original_list.clone());
        assert_eq!(deduplicated_list, original_list);
    }

    #[test]
    fn list_with_identical_items_is_deduplicated() {
        let mut deduplicated_items = super::dedup_grouped_strings(vec![
            AndroidString::new(
                String::from("translatable_string"),
                String::from("translatable string value"),
                true,
            ),
            AndroidString::new(
                String::from("translatable_string"),
                String::from("translatable string value"),
                true,
            ),
            AndroidString::new(
                String::from("non_transltable_string"),
                String::from("non translatable string value"),
                false,
            ),
            AndroidString::new(
                String::from("non_transltable_string"),
                String::from("non translatable string value"),
                false,
            ),
        ])
        .into_iter();

        assert_eq!(
            deduplicated_items.next().unwrap(),
            AndroidString::new(
                String::from("translatable_string"),
                String::from("translatable string value"),
                true
            )
        );
        assert_eq!(
            deduplicated_items.next().unwrap(),
            AndroidString::new(
                String::from("non_transltable_string"),
                String::from("non translatable string value"),
                false
            )
        );
        assert_eq!(deduplicated_items.next(), None);
    }

    #[test]
    fn list_with_items_having_same_name_is_deduplicated() {
        let mut deduplicated_items = super::dedup_grouped_strings(vec![
            AndroidString::new(
                String::from("string_1"),
                String::from("string 1 value 1"),
                true,
            ),
            AndroidString::new(
                String::from("string_1"),
                String::from("string 1 value 2"),
                true,
            ),
            AndroidString::new(
                String::from("string_1"),
                String::from("string 1 value 3"),
                false,
            ),
            AndroidString::new(
                String::from("string_2"),
                String::from("string 2 value 1"),
                false,
            ),
            AndroidString::new(
                String::from("string_2"),
                String::from("string 2 value 2"),
                false,
            ),
            AndroidString::new(
                String::from("string_2"),
                String::from("string 2 value 3"),
                true,
            ),
        ])
        .into_iter();

        assert_eq!(
            deduplicated_items.next().unwrap(),
            AndroidString::new(
                String::from("string_1"),
                String::from("string 1 value 1"),
                true
            )
        );
        assert_eq!(
            deduplicated_items.next().unwrap(),
            AndroidString::new(
                String::from("string_2"),
                String::from("string 2 value 1"),
                false
            )
        );
        assert_eq!(deduplicated_items.next(), None);
    }
}
