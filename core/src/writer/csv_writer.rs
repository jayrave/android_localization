use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::Write;

use csv;

use android_localization_utilities::DevExpt;

use crate::error::{Error, InnerError};
use crate::localizable_strings::LocalizableStrings;

pub fn write(
    strings_list: Vec<LocalizableStrings>,
    sink_provider: &mut SinkProvider,
) -> Result<(), Error> {
    // Split strings into groups requiring localization for the same strings
    let mut grouped_strings_list: HashMap<u64, Vec<LocalizableStrings>> = HashMap::new();
    for strings in strings_list {
        grouped_strings_list
            .entry(find_grouping_hash_of(&strings))
            .or_default()
            .push(strings)
    }

    // We may need multiple sinks to write locale requiring
    // different strings to be localized
    for (_, some_strings_list) in grouped_strings_list.into_iter() {
        sink_provider.execute_with_new_sink(Writer {
            strings_list: some_strings_list,
        })?;
    }

    Ok(())
}

fn find_grouping_hash_of(strings: &LocalizableStrings) -> u64 {
    let mut hasher = DefaultHasher::new();
    strings.default_locale_strings().hash(&mut hasher);
    hasher.finish()
}

pub struct Writer {
    strings_list: Vec<LocalizableStrings>,
}

impl Writer {
    pub fn write(self, sink: &mut Write) -> Result<(), InnerError> {
        // Sink is automatically buffered
        let mut csv_writer = csv::Writer::from_writer(sink);
        let locale_count = self.strings_list.len();
        let localizable_strings = self.strings_list.first().expt("Empty strings list!");
        let value_count = localizable_strings.default_locale_strings().len();

        // Write header record
        let mut header = Vec::with_capacity(locale_count + 2);
        header.push("string_name");
        header.push("default_locale");
        for i in 0..locale_count {
            header.push(self.strings_list[i].to_locale());
        }
        csv_writer.write_record(header)?;

        // Write values
        let mut record = vec![""; locale_count + 2];
        for i in 0..value_count {
            let localizable_string = localizable_strings
                .default_locale_strings()
                .get(i)
                .expt("Already checked the size but it still fails!");
            record[0] = localizable_string.name();
            record[1] = localizable_string.value();
            csv_writer.write_record(&record)?;
        }

        csv_writer.flush()?;
        Ok(())
    }
}

pub trait SinkProvider {
    fn execute_with_new_sink(&mut self, writer: Writer) -> Result<(), Error>;
}

#[cfg(test)]
mod tests {
    use test_utilities;

    use crate::android_string::AndroidString;
    use crate::error::ResultExt;
    use crate::localizable_strings::LocalizableStrings;

    use super::Error;
    use super::SinkProvider;
    use super::Writer;

    struct ByteSinkProvider {
        data: Vec<String>,
    }

    impl SinkProvider for ByteSinkProvider {
        fn execute_with_new_sink(&mut self, writer: Writer) -> Result<(), Error> {
            let mut contents = vec![];
            let result = writer.write(&mut contents);
            self.data.push(String::from_utf8(contents).unwrap());
            result.with_context("added context for tests")
        }
    }

    #[test]
    fn writes_strings_to_files() {
        let strings_list = vec![
            LocalizableStrings::new(
                String::from("french"),
                vec![AndroidString::localizable("string_1", "english 1")],
            ),
            LocalizableStrings::new(
                String::from("german"),
                vec![
                    AndroidString::localizable("string_1", "english 1"),
                    AndroidString::localizable("string_2", "english 2"),
                ],
            ),
            LocalizableStrings::new(
                String::from("spanish"),
                vec![AndroidString::localizable("string_2", "english 2")],
            ),
            LocalizableStrings::new(
                String::from("dutch"),
                vec![
                    AndroidString::localizable("string_1", "english 1"),
                    AndroidString::localizable("string_2", "english 2"),
                ],
            ),
        ];

        // Convert all the written bytes into strings
        let mut sink_provider = ByteSinkProvider { data: vec![] };

        super::write(strings_list, &mut sink_provider).unwrap();

        // Since a map is used, sort the contents to be sure of the order
        sink_provider.data.sort();

        // Time to assert
        test_utilities::list::assert_strict_list_eq(
            sink_provider.data,
            vec![
                String::from("string_name,default_locale,french\nstring_1,english 1,\n"),
                String::from("string_name,default_locale,german,dutch\nstring_1,english 1,,\nstring_2,english 2,,\n"),
                String::from("string_name,default_locale,spanish\nstring_2,english 2,\n")
            ]
        );
    }
}
