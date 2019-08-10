use crate::android_string::AndroidString;

/// This methods assumes that all the strings that have the same name are grouped
/// together. If such groups are present, only the first string from those groups
/// will be let through
pub fn dedup_grouped_strings(mut android_strings: Vec<AndroidString>) -> Vec<AndroidString> {
    android_strings.dedup_by(|string1, string2| string1.name() == string2.name());
    android_strings
}

#[cfg(test)]
mod tests {
    use crate::android_string::AndroidString;

    #[test]
    fn deduplicated() {
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
