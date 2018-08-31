use android_string::AndroidString;

/// In place, stable sorting based on the string's name
pub fn sort_strings(strings: &mut Vec<AndroidString>) {
    // Sort both the incoming strings. Not using `sort_by_key` as I can't figure out
    // how to specify lifetime for closure's return :(
    strings.sort_by(|s1, s2| s1.name().cmp(s2.name()));
}

#[cfg(test)]
mod tests {
    use android_string::AndroidString;

    #[test]
    fn sorted() {
        let mut strings = vec![
            AndroidString::new(String::from("string_2"), String::from("string value"), true),
            AndroidString::new(
                String::from("string_3"),
                String::from("string 3 value 1"),
                true,
            ),
            AndroidString::new(
                String::from("string_3"),
                String::from("string 3 value 2"),
                true,
            ),
            AndroidString::new(String::from("string_1"), String::from("string value"), true),
        ];

        super::sort_strings(&mut strings);
        let mut strings = strings.into_iter();

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(String::from("string_1"), String::from("string value"), true)
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(String::from("string_2"), String::from("string value"), true)
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(
                String::from("string_3"),
                String::from("string 3 value 1"),
                true
            )
        );

        assert_eq!(
            strings.next().unwrap(),
            AndroidString::new(
                String::from("string_3"),
                String::from("string 3 value 2"),
                true
            )
        );

        assert_eq!(strings.next(), None);
    }
}
