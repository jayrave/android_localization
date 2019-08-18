use std::cmp::Ordering;

use regex::Regex;

use crate::android_string::AndroidString;
use crate::ops::sort;

lazy_static::lazy_static! {
    static ref FORMAT_STRING: Regex = Regex::new(r"(%\d+\$[ds])").unwrap();
}

pub fn validate(
    default_parsed_data: &mut Vec<ParsedData>,
    foreign_strings: &mut Vec<AndroidString>,
) -> Result<(), Mismatches> {
    // Sort both the lists
    sort::sort_android_strings_by_name(foreign_strings);
    default_parsed_data.sort_by(|pd1, pd2| {
        sort::compare_android_strings(&pd1.android_string, &pd2.android_string)
    });

    let total_strings_count = default_parsed_data.len() + foreign_strings.len();
    let mut mismatches = vec![];
    let mut default_parsed_data_index = 0;
    let mut foreign_strings_index = 0;

    for _ in 0..total_strings_count {
        let default_parsed_datum = default_parsed_data.get(default_parsed_data_index);
        let foreign_string = foreign_strings.get(foreign_strings_index);

        // Can't compare if either of the lists have run out! This check is imperative as the
        // code flow in the else block increments both lists' pointers if there is a match
        if default_parsed_datum.is_none() || foreign_string.is_none() {
            break;
        } else {
            let default_parsed_datum = default_parsed_datum.unwrap();
            let foreign_string = foreign_string.unwrap();
            match default_parsed_datum
                .android_string
                .name()
                .cmp(foreign_string.name())
            {
                Ordering::Less => default_parsed_data_index += 1,
                Ordering::Greater => foreign_strings_index += 1,
                Ordering::Equal => {
                    let format_strings = parse_and_sort_format_strings(foreign_string);
                    if default_parsed_datum.sorted_format_strings != format_strings {
                        mismatches.push(Mismatch {
                            default_parsed_data: default_parsed_datum.clone(),
                            foreign_parsed_data: ParsedData {
                                android_string: foreign_string.clone(),
                                sorted_format_strings: format_strings,
                            },
                        });
                    }

                    // Feel free to increment both the indices as we have a `is_none` check
                    // for both the strings
                    default_parsed_data_index += 1;
                    foreign_strings_index += 1;
                }
            }
        }
    }

    if mismatches.is_empty() {
        Ok(())
    } else {
        Err(Mismatches { mismatches })
    }
}

pub fn parse_and_build_data(strings: &[AndroidString]) -> Vec<ParsedData> {
    strings
        .iter()
        .map(|s| ParsedData {
            android_string: s.clone(),
            sorted_format_strings: parse_and_sort_format_strings(s),
        })
        .collect()
}

fn parse_and_sort_format_strings(string: &AndroidString) -> Vec<String> {
    let mut format_strings = FORMAT_STRING
        .find_iter(string.value())
        .map(|m| String::from(m.as_str()))
        .collect::<Vec<String>>();

    format_strings.sort();
    format_strings
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParsedData {
    pub android_string: AndroidString,
    pub sorted_format_strings: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct Mismatch {
    pub default_parsed_data: ParsedData,
    pub foreign_parsed_data: ParsedData,
}

#[derive(Debug, PartialEq)]
pub struct Mismatches {
    pub mismatches: Vec<Mismatch>,
}

#[cfg(test)]
mod tests {
    use crate::android_string::AndroidString;

    use super::Mismatch;
    use super::ParsedData;

    #[test]
    fn passes_in_absence_of_mismatches() {
        let mut default_parsed_data = vec![
            ParsedData {
                android_string: AndroidString::new(String::from("s1"), String::from("value"), true),
                sorted_format_strings: vec![],
            },
            ParsedData {
                android_string: AndroidString::new(String::from("s2"), String::from("value"), true),
                sorted_format_strings: vec![String::from("%1$s")],
            },
        ];

        let mut foreign_strings = vec![
            AndroidString::new(String::from("s2"), String::from("value %1$s"), true),
            AndroidString::new(String::from("s3"), String::from("value"), true),
        ];

        assert!(super::validate(&mut default_parsed_data, &mut foreign_strings).is_ok())
    }

    #[test]
    fn errors_in_presence_of_mismatches() {
        let mut default_parsed_data = vec![
            ParsedData {
                android_string: AndroidString::new(String::from("s3"), String::from("value"), true),
                sorted_format_strings: vec![],
            },
            ParsedData {
                android_string: AndroidString::new(String::from("s1"), String::from("value"), true),
                sorted_format_strings: vec![],
            },
            ParsedData {
                android_string: AndroidString::new(String::from("s2"), String::from("value"), true),
                sorted_format_strings: vec![String::from("%1$s")],
            },
        ];

        let mut foreign_strings = vec![
            AndroidString::new(String::from("s3"), String::from("value %1$s"), true),
            AndroidString::new(String::from("s2"), String::from("value %1$d"), true),
            AndroidString::new(String::from("s4"), String::from("value %2$d"), true),
        ];

        assert_eq!(
            super::validate(&mut default_parsed_data, &mut foreign_strings)
                .unwrap_err()
                .mismatches,
            vec![
                Mismatch {
                    default_parsed_data: ParsedData {
                        android_string: AndroidString::new(
                            String::from("s2"),
                            String::from("value"),
                            true,
                        ),
                        sorted_format_strings: vec![String::from("%1$s")],
                    },
                    foreign_parsed_data: ParsedData {
                        android_string: AndroidString::new(
                            String::from("s2"),
                            String::from("value %1$d"),
                            true,
                        ),
                        sorted_format_strings: vec![String::from("%1$d")],
                    },
                },
                Mismatch {
                    default_parsed_data: ParsedData {
                        android_string: AndroidString::new(
                            String::from("s3"),
                            String::from("value"),
                            true,
                        ),
                        sorted_format_strings: vec![],
                    },
                    foreign_parsed_data: ParsedData {
                        android_string: AndroidString::new(
                            String::from("s3"),
                            String::from("value %1$s"),
                            true,
                        ),
                        sorted_format_strings: vec![String::from("%1$s")],
                    },
                },
            ]
        )
    }

    #[test]
    fn parse_builds_returns_appropriate_parsed_data() {
        let strings = vec![
            AndroidString::new(String::from("s1"), String::from("value"), true),
            AndroidString::new(
                String::from("s1"),
                String::from(r"%2$s a %1$d %2$d b %2$z c %1$s"),
                true,
            ),
        ];

        let expected_output = vec![
            ParsedData {
                android_string: strings[0].clone(),
                sorted_format_strings: vec![],
            },
            ParsedData {
                android_string: strings[1].clone(),
                sorted_format_strings: vec![
                    String::from("%1$d"),
                    String::from("%1$s"),
                    String::from("%2$d"),
                    String::from("%2$s"),
                ],
            },
        ];

        assert_eq!(super::parse_and_build_data(&strings), expected_output)
    }

    #[test]
    fn parse_returns_empty_list_in_case_of_no_format_strings() {
        assert!(super::parse_and_sort_format_strings(&AndroidString::new(
            String::from("s1"),
            String::from("value"),
            true
        ))
        .is_empty())
    }

    #[test]
    fn parse_returns_only_valid_format_strings() {
        assert_eq!(
            super::parse_and_sort_format_strings(&AndroidString::new(
                String::from("s1"),
                String::from(r"%2$s a %1$d %2$d b %2$z c %1$s"),
                true
            )),
            vec![
                String::from("%1$d"),
                String::from("%1$s"),
                String::from("%2$d"),
                String::from("%2$s"),
            ]
        )
    }
}
