use crate::android_string::AndroidString;
use crate::localized_string::LocalizedString;
use crate::ops::sort;
use crate::util::two_pointer_comparison;

/// Localized strings will be converted into `AndroidString` only if both the name
/// & the default value from `LocalizedString` match up with whatever is in the
/// default string
pub fn extract_android_strings_from_localized(
    localized_strings: &mut [LocalizedString],
    default_strings: &mut [AndroidString],
) -> Vec<AndroidString> {
    // Sort both the incoming strings
    sort::sort_android_strings_by_name(default_strings);
    sort::sort_localized_strings_by_name(localized_strings);

    let mut result = Vec::with_capacity(localized_strings.len()); // Max number of expected strings
    two_pointer_comparison::compare(
        localized_strings,
        default_strings,
        |localized_string, default_string| localized_string.name().cmp(default_string.name()),
        |localized_string, default_string| {
            if localized_string.default() == default_string.value() {
                result.push(AndroidString::new(
                    String::from(localized_string.name()),
                    String::from(localized_string.localized()),
                    default_string.is_localizable(),
                ));
            }
        },
    );

    result
}

#[cfg(test)]
mod tests {
    use test_utilities;

    use crate::android_string::AndroidString;
    use crate::localized_string::LocalizedString;

    #[test]
    fn extracts() {
        let mut default_strings = vec![
            AndroidString::unlocalizable("string_2", "english 2 value"), // unlocalizable to make sure the flag is carried over from here
            AndroidString::localizable("string_3", "english 3 value"),
            AndroidString::localizable("string_4", "english 4 new value"), // new value to make sure match is against both name & value
            AndroidString::localizable("string_1", "english 1 value"),
        ];

        let mut localized_strings = vec![
            LocalizedString::build("string_3", "english 3 value", "french 3 value"),
            LocalizedString::build("string_4", "english 4 value", "french 4 value"),
            LocalizedString::build("string_2", "english 2 value", "french 2 value"),
        ];

        let strings = super::extract_android_strings_from_localized(
            &mut localized_strings,
            &mut default_strings,
        );

        test_utilities::assert_strict_list_eq(
            strings,
            vec![
                AndroidString::unlocalizable("string_2", "french 2 value"),
                AndroidString::localizable("string_3", "french 3 value"),
            ],
        )
    }
}
