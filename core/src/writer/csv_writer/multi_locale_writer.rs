use csv;
use crate::localizable_strings::LocalizableStrings;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::Write;
use crate::writer::csv_writer::error::Error;

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
        let for_locales = some_strings_list
            .iter()
            .map(|s| String::from(s.to_locale()))
            .collect();

        let writer = Writer { strings_list: some_strings_list };
        sink_provider.execute_with_new_sink(for_locales, writer)?;
    }

    Ok(())
}

fn find_grouping_hash_of(strings: &LocalizableStrings) -> u64 {
    let mut hasher = DefaultHasher::new();
    strings.default_locale_strings().hash(&mut hasher);
    hasher.finish()
}

struct Writer {
    strings_list: Vec<LocalizableStrings>
}

impl Writer {
    fn write(self, sink: &mut Write) -> Result<(), Error> {
        // Sink is automatically buffered
        let mut csv_writer = csv::Writer::from_writer(sink);
        let locale_count = self.strings_list.len();
        let localizable_strings = self.strings_list.first().unwrap();
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
            let localizable_string = localizable_strings.default_locale_strings().get(i).unwrap();
            record[0] = localizable_string.name();
            record[1] = localizable_string.value();
            csv_writer.write_record(&record)?;
        }

        csv_writer.flush()?;
        Ok(())
    }
}

pub trait SinkProvider {
    fn execute_with_new_sink(
        &mut self,
        for_locales: Vec<String>,
        writer: Writer
    ) -> Result<(), Error>;
}

#[cfg(test)]
mod tests {
    use super::Error;
    use super::SinkProvider;
    use crate::android_string::AndroidString;
    use crate::localizable_strings::LocalizableStrings;
    
    use super::Writer;
    use std::io::Write;

    struct ByteSinkProvider {
        data: Vec<(Vec<String>, String)>
    }

    impl SinkProvider for ByteSinkProvider {
        fn execute_with_new_sink(&mut self, for_locales: Vec<String>, writer: Writer) -> Result<(), Error> {
            let mut contents = vec![];
            let result = writer.write(&mut contents);
            self.data.push((for_locales, String::from_utf8(contents).unwrap()));

            result
        }
    }

    #[test]
    fn strings_are_written_to_files() {
        let mut strings_list = vec![];
        strings_list.push(LocalizableStrings::new(
            String::from("french"),
            vec![AndroidString::new(
                String::from("string_1"),
                String::from("english 1"),
                true,
            )],
        ));

        strings_list.push(LocalizableStrings::new(
            String::from("german"),
            vec![
                AndroidString::new(String::from("string_1"), String::from("english 1"), true),
                AndroidString::new(String::from("string_2"), String::from("english 2"), true),
            ],
        ));

        strings_list.push(LocalizableStrings::new(
            String::from("spanish"),
            vec![AndroidString::new(
                String::from("string_2"),
                String::from("english 2"),
                true,
            )],
        ));

        strings_list.push(LocalizableStrings::new(
            String::from("dutch"),
            vec![
                AndroidString::new(String::from("string_1"), String::from("english 1"), true),
                AndroidString::new(String::from("string_2"), String::from("english 2"), true),
            ],
        ));

        // Convert all the written bytes into strings for the different sinks
        let mut sink_provider = ByteSinkProvider {
            data: vec![],
        };

        super::write(strings_list, &mut sink_provider);

        // Since a map is used, sort the contents to be sure of the order
        sink_provider.data.sort();

        // Time to assert
        assert_eq!(
            sink_provider.data,
            vec![
                (vec![String::from("french")], String::from("string_name,default_locale,french\nstring_1,english 1,\n")),
                (vec![String::from("german"), String::from("dutch")], String::from("string_name,default_locale,german,dutch\nstring_1,english 1,,\nstring_2,english 2,,\n")),
                (vec![String::from("spanish")], String::from("string_name,default_locale,spanish\nstring_2,english 2,\n"))
            ]
        );
    }
}
