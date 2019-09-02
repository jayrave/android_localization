use crate::android_string::AndroidString;

/// This methods assumes that all the strings that have the same name are grouped
/// together. If such groups are present, only the first string from those groups
/// will be let through
pub fn dedup_grouped_strings(android_strings: &mut Vec<AndroidString>) {
    android_strings.dedup_by(|string1, string2| string1.name() == string2.name());
}

#[cfg(test)]
mod tests {
    use test_utilities;

    use crate::android_string::AndroidString;

    #[test]
    fn dedupes() {
        let mut android_strings = vec![
            AndroidString::localizable("string_1", "string 1 value 1"),
            AndroidString::localizable("string_1", "string 1 value 2"),
            AndroidString::unlocalizable("string_1", "string 1 value 3"),
            AndroidString::unlocalizable("string_2", "string 2 value 1"),
            AndroidString::unlocalizable("string_2", "string 2 value 2"),
            AndroidString::localizable("string_2", "string 2 value 3"),
        ];

        super::dedup_grouped_strings(&mut android_strings);
        test_utilities::assert_strict_list_eq(
            android_strings,
            vec![
                AndroidString::localizable("string_1", "string 1 value 1"),
                AndroidString::unlocalizable("string_2", "string 2 value 1"),
            ],
        )
    }
}
