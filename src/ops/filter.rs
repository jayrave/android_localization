use android_string::AndroidString;
use ops::sort;
use std::cmp::Ordering;

/// It is assumed that neither lists have strings with the same names. If they
/// do, the result is undefined! This method doesn't check whether `all_strings`
/// contains everything that is contained in `lacking_strings`
pub fn find_missing_from_unique_lists(
    lacking_strings: &mut Vec<AndroidString>,
    all_strings: &mut Vec<AndroidString>,
) -> Vec<String> {
    // Sort both the strings
    sort::sort_strings(lacking_strings);
    sort::sort_strings(all_strings);

    // Iterate through `all_strings` & find those missing in `lacking_strings`
    let mut result = vec![];
    let mut lacking_strings_index = 0;
    for string in all_strings {
        loop {
            let lacking_string = lacking_strings.get(lacking_strings_index);
            match lacking_string {
                None => {
                    // `lacking_string` has run out of strings!
                    result.push(String::from(string.name()));
                    break; // To go out of the infinite loop
                }

                Some(ls) => match ls.name().cmp(string.name()) {
                    Ordering::Equal => break, // To go out of the infinite loop
                    Ordering::Less => {
                        // `lacking_strings` seems to have strings not in `android_strings`.
                        // There is a still a chance that `lacking_strings` has the required
                        // string
                        lacking_strings_index += 1;
                    }

                    Ordering::Greater => {
                        // `lacking_strings` doesn't have this string from `all_strings`
                        result.push(String::from(string.name()));
                        break; // To go out of the infinite loop
                    }
                },
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use android_string::AndroidString;

    #[test]
    fn finds() {
        let mut lacking_strings = vec![
            AndroidString::new(
                String::from("common_string_3"),
                String::from("string value"),
                true,
            ),
            AndroidString::new(
                String::from("only_in_lacking_strings"),
                String::from("string value"),
                true,
            ),
            AndroidString::new(
                String::from("common_string_1"),
                String::from("string value"),
                false,
            ),
        ];

        let mut all_strings = vec![
            AndroidString::new(
                String::from("common_string_1"),
                String::from("string value"),
                false,
            ),
            AndroidString::new(
                String::from("only_in_all_strings_1"),
                String::from("string value"),
                false,
            ),
            AndroidString::new(
                String::from("common_string_3"),
                String::from("string value"),
                true,
            ),
            AndroidString::new(
                String::from("only_in_all_strings_3"),
                String::from("string value"),
                false,
            ),
            AndroidString::new(
                String::from("only_in_all_strings_2"),
                String::from("string value"),
                false,
            ),
        ];

        let mut missing_strings =
            super::find_missing_from_unique_lists(&mut lacking_strings, &mut all_strings)
                .into_iter();
        assert_eq!(missing_strings.next().unwrap(), "only_in_all_strings_1");
        assert_eq!(missing_strings.next().unwrap(), "only_in_all_strings_2");
        assert_eq!(missing_strings.next().unwrap(), "only_in_all_strings_3");
        assert_eq!(missing_strings.next(), None);
    }
}
