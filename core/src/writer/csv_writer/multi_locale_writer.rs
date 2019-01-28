use csv::Writer;
use localizable_strings::LocalizableStrings;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::Write;
use writer::csv_writer::error::Error;

pub fn write<W: Write>(
    strings_list: Vec<LocalizableStrings>,
    sink_provider: &mut SinkProvider<W>,
) -> Result<Vec<Box<W>>, Error> {
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
    let mut all_sinks = Vec::with_capacity(grouped_strings_list.len());
    for (_, some_strings_list) in grouped_strings_list.into_iter() {
        let mut sink = sink_provider.new_sink(
            some_strings_list
                .iter()
                .map(|s| String::from(s.to_locale()))
                .collect(),
        );

        write_to_sink(some_strings_list, &mut sink)?;
        all_sinks.push(sink);
    }

    Ok(all_sinks)
}

pub fn write_to_sink<W: Write>(
    strings_list: Vec<LocalizableStrings>,
    sink: &mut W,
) -> Result<(), Error> {
    // Sink is automatically buffered
    let mut writer = Writer::from_writer(sink);
    let locale_count = strings_list.len();
    let localizable_strings = strings_list.first().unwrap();
    let value_count = localizable_strings.default_locale_strings().len();

    // Write header record
    let mut header = Vec::with_capacity(locale_count + 2);
    header.push("string_name");
    header.push("default_locale");
    for i in 0..locale_count {
        header.push(strings_list[i].to_locale());
    }
    writer.write_record(header)?;

    // Write values
    let mut record = vec![""; locale_count + 2];
    for i in 0..value_count {
        let localizable_string = localizable_strings.default_locale_strings().get(i).unwrap();
        record[0] = localizable_string.name();
        record[1] = localizable_string.value();
        writer.write_record(&record)?;
    }

    writer.flush()?;
    Ok(())
}

fn find_grouping_hash_of(strings: &LocalizableStrings) -> u64 {
    let mut hasher = DefaultHasher::new();
    strings.default_locale_strings().hash(&mut hasher);
    hasher.finish()
}

pub trait SinkProvider<W: Write> {
    fn new_sink(&mut self, for_locales: Vec<String>) -> Box<W>;
}

#[cfg(test)]
mod tests {
    use super::SinkProvider;
    use android_string::AndroidString;
    use localizable_strings::LocalizableStrings;

    struct ByteSinkProvider {
        for_locales_list: Vec<Vec<String>>,
    }

    impl SinkProvider<Vec<u8>> for ByteSinkProvider {
        fn new_sink(&mut self, for_locales: Vec<String>) -> Box<Vec<u8>> {
            self.for_locales_list.push(for_locales);
            Box::new(vec![])
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
            for_locales_list: vec![],
        };

        let mut written_contents = super::write(strings_list, &mut sink_provider)
            .unwrap()
            .into_iter()
            .map(|s| String::from_utf8(s.to_vec()).unwrap())
            .collect::<Vec<String>>();

        // Since a map is used, sort the contents to be sure of the order
        written_contents.sort();
        sink_provider.for_locales_list.sort();

        // Time to assert
        assert_eq!(
            written_contents,
            vec![
                String::from("string_name,default_locale,french\nstring_1,english 1,\n"),
                String::from("string_name,default_locale,german,dutch\nstring_1,english 1,,\nstring_2,english 2,,\n"),
                String::from("string_name,default_locale,spanish\nstring_2,english 2,\n")
            ]
        );

        assert_eq!(
            sink_provider.for_locales_list,
            vec![
                vec![String::from("french")],
                vec![String::from("german"), String::from("dutch")],
                vec![String::from("spanish")]
            ]
        );
    }
}
