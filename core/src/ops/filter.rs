use std::cmp::Ordering;

use crate::android_string::AndroidString;
use crate::ops::sort;

pub fn find_localizable_strings(strings: Vec<AndroidString>) -> Vec<AndroidString> {
    strings.into_iter().filter(|s| s.is_localizable()).collect()
}

/// It is assumed that neither lists have strings with the same names. If they
/// do, the result is undefined! This method doesn't check whether `all_strings`
/// contains everything that is contained in `lacking_strings`
pub fn find_missing_strings(
    lacking_strings: &mut [AndroidString],
    all_strings: &mut [AndroidString],
) -> Vec<AndroidString> {
    // Sort both the strings
    sort::sort_android_strings_by_name(lacking_strings);
    sort::sort_android_strings_by_name(all_strings);

    // Iterate through `all_strings` & find those missing in `lacking_strings`
    let mut result = vec![];
    let mut lacking_strings_index = 0;
    for string in all_strings {
        'lacking_strings_loop: loop {
            let lacking_string = lacking_strings.get(lacking_strings_index);
            match lacking_string {
                None => {
                    // `lacking_string` has run out of strings!
                    result.push(string.clone());
                    break 'lacking_strings_loop;
                }

                Some(ls) => match ls.name().cmp(string.name()) {
                    Ordering::Equal => break 'lacking_strings_loop,
                    Ordering::Less => {
                        // `lacking_strings` seems to have strings not in `android_strings`.
                        // There is a still a chance that `lacking_strings` has the required
                        // string
                        lacking_strings_index += 1;
                    }

                    Ordering::Greater => {
                        // `lacking_strings` doesn't have this string from `all_strings`
                        result.push(string.clone());
                        break 'lacking_strings_loop;
                    }
                },
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::android_string::AndroidString;

    #[test]
    fn finds_localizable_strings() {
        let mut localizable_strings = super::find_localizable_strings(vec![
            AndroidString::localizable("localizable_string_1", "string value"),
            AndroidString::unlocalizable("non_localizable_string", "string value"),
            AndroidString::localizable("localizable_string_2", "string value"),
        ])
        .into_iter();

        assert_eq!(
            localizable_strings.next().unwrap(),
            AndroidString::localizable("localizable_string_1", "string value")
        );

        assert_eq!(
            localizable_strings.next().unwrap(),
            AndroidString::localizable("localizable_string_2", "string value")
        );

        assert_eq!(localizable_strings.next(), None);
    }

    #[test]
    fn finds_missing_strings() {
        let mut lacking_strings = vec![
            AndroidString::localizable("common_string_3", "string value"),
            AndroidString::localizable("only_in_lacking_strings", "string value"),
            AndroidString::unlocalizable("common_string_1", "string value"),
        ];

        let mut all_strings = vec![
            AndroidString::unlocalizable("common_string_1", "string value"),
            AndroidString::unlocalizable("only_in_all_strings_1", "string value"),
            AndroidString::localizable("common_string_3", "string value"),
            AndroidString::unlocalizable("only_in_all_strings_3", "string value"),
            AndroidString::unlocalizable("only_in_all_strings_2", "string value"),
        ];

        let mut missing_strings =
            super::find_missing_strings(&mut lacking_strings, &mut all_strings).into_iter();

        assert_eq!(
            missing_strings.next().unwrap(),
            AndroidString::unlocalizable("only_in_all_strings_1", "string value")
        );
        assert_eq!(
            missing_strings.next().unwrap(),
            AndroidString::unlocalizable("only_in_all_strings_2", "string value")
        );
        assert_eq!(
            missing_strings.next().unwrap(),
            AndroidString::unlocalizable("only_in_all_strings_3", "string value")
        );
        assert_eq!(missing_strings.next(), None);
    }
}
