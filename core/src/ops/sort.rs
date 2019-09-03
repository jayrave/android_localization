use std::cmp::Ordering;

use crate::android_string::AndroidString;
use crate::localized_string::LocalizedString;

/// In place, stable sorting
pub fn sort_android_strings_by_name(strings: &mut [AndroidString]) {
    // Not using `sort_by_key` as I can't figure out how to specify
    // lifetime for closure's return :(
    strings.sort_by(|s1, s2| s1.name().cmp(s2.name()));
}

/// In place, stable sorting
pub fn sort_localized_strings_by_name(strings: &mut [LocalizedString]) {
    // Not using `sort_by_key` as I can't figure out how to specify
    // lifetime for closure's return :(
    strings.sort_by(|s1, s2| s1.name().cmp(s2.name()));
}

pub fn compare_android_strings(s1: &AndroidString, s2: &AndroidString) -> Ordering {
    s1.name().cmp(s2.name())
}

#[cfg(test)]
mod tests {
    use test_utilities;

    use crate::android_string::AndroidString;
    use crate::localized_string::LocalizedString;

    #[test]
    fn sorts_android_strings_by_name() {
        let mut strings = vec![
            AndroidString::localizable("string_2", "string value"),
            AndroidString::localizable("string_3", "string 3 value 1"),
            AndroidString::localizable("string_3", "string 3 value 2"),
            AndroidString::localizable("string_1", "string value"),
        ];

        super::sort_android_strings_by_name(&mut strings);
        test_utilities::list::assert_strict_list_eq(
            strings,
            vec![
                AndroidString::localizable("string_1", "string value"),
                AndroidString::localizable("string_2", "string value"),
                AndroidString::localizable("string_3", "string 3 value 1"),
                AndroidString::localizable("string_3", "string 3 value 2"),
            ],
        )
    }

    #[test]
    fn sorts_localized_strings_by_name() {
        let mut strings = vec![
            LocalizedString::build("string_2", "string value", "string value"),
            LocalizedString::build("string_3", "string 3 value 1", "string 3 value 1"),
            LocalizedString::build("string_3", "string 3 value 2", "string 3 value 2"),
            LocalizedString::build("string_1", "string value", "string value"),
        ];

        super::sort_localized_strings_by_name(&mut strings);
        test_utilities::list::assert_strict_list_eq(
            strings,
            vec![
                LocalizedString::build("string_1", "string value", "string value"),
                LocalizedString::build("string_2", "string value", "string value"),
                LocalizedString::build("string_3", "string 3 value 1", "string 3 value 1"),
                LocalizedString::build("string_3", "string 3 value 2", "string 3 value 2"),
            ],
        )
    }
}
