use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use crate::android_string::AndroidString;
use crate::constants;
use crate::error::{Error, ResultExt};
use crate::localizable_strings::LocalizableStrings;
use crate::ops::filter;
use crate::util::foreign_locale_ids_finder;
use crate::util::xml_helper;
use crate::writer::csv_writer;

/// Returns the list of output files created by this call. These aren't guaranteed
/// to be valid paths to files. Sometimes, if a file's path can't be expressed by
/// `String` (in case it has non UTF-8 chars), it could just be the file's name
pub fn do_the_thing<S: ::std::hash::BuildHasher>(
    res_dir_path: &str,
    output_dir_path: &str,
    locale_id_to_name_map: HashMap<String, String, S>,
) -> Result<Vec<String>, Error> {
    let locale_id_to_name_map = foreign_locale_ids_finder::build_map_if_empty_or_return(
        locale_id_to_name_map,
        res_dir_path,
    )?;

    if locale_id_to_name_map.is_empty() {
        return Err::<_, Error>(From::from(String::from(
            "Res dir doesn't have any non-default values dir with strings file!",
        )))
        .with_context(String::from(res_dir_path));
    }

    create_output_dir_if_required(output_dir_path)?;

    // Read default strings
    let res_dir_path = Path::new(res_dir_path);
    let mut localizable_default_strings = filter::find_localizable_strings(
        xml_helper::read_default_strings(res_dir_path)?.into_strings(),
    );

    // For all languages, write out strings requiring localization
    write_out_strings_to_localize(
        res_dir_path,
        output_dir_path,
        locale_id_to_name_map,
        &mut localizable_default_strings,
    )
}

fn create_output_dir_if_required(output_dir_path: &str) -> Result<(), Error> {
    let output_path = PathBuf::from(output_dir_path);
    if output_path.is_file() {
        Err::<_, Error>(From::from(String::from(
            "Output directory path points to a file!",
        )))
        .with_context(String::from(output_dir_path))
    } else if output_path.exists() {
        Ok(())
    } else {
        fs::create_dir_all(PathBuf::from(output_dir_path))
            .with_context(String::from(output_dir_path))
    }
}

fn write_out_strings_to_localize<S: ::std::hash::BuildHasher>(
    res_dir_path: &Path,
    output_dir_path: &str,
    locale_id_to_name_map: HashMap<String, String, S>,
    localizable_default_strings: &mut Vec<AndroidString>,
) -> Result<Vec<String>, Error> {
    let mut localizable_strings_list = vec![];
    for (locale_id, locale_name) in locale_id_to_name_map {
        let mut foreign_strings =
            xml_helper::read_foreign_strings(res_dir_path, &locale_id)?.into_strings();

        let strings_to_localize =
            filter::find_missing_strings(&mut foreign_strings, localizable_default_strings);

        if !strings_to_localize.is_empty() {
            localizable_strings_list.push(LocalizableStrings::new(locale_name, strings_to_localize))
        }
    }

    if !localizable_strings_list.is_empty() {
        let mut sink_provider = FileProvider::new(String::from(output_dir_path));
        csv_writer::write(localizable_strings_list, &mut sink_provider)?;

        Ok(sink_provider.into_created_files())
    } else {
        Ok(vec![])
    }
}

struct FileProvider {
    count_of_files_created: usize,
    sink_dir: String,
    created_files: Vec<String>,
}

impl FileProvider {
    fn new(sink_dir: String) -> FileProvider {
        FileProvider {
            sink_dir,
            created_files: Vec::new(),
            count_of_files_created: 0,
        }
    }

    fn into_created_files(self) -> Vec<String> {
        self.created_files
    }

    /// Returns the created output file along with its path (if path computation
    /// is possible; if not, it passes out a fallback value)
    fn create_output_file(&mut self, output_file_name: &str) -> Result<File, Error> {
        let mut output_path = PathBuf::from(&self.sink_dir);
        output_path.push(output_file_name);
        output_path.set_extension(constants::extn::CSV);
        let output_path_or_fb = String::from(output_path.to_str().unwrap_or(output_file_name));

        if output_path.exists() {
            Err::<_, Error>(From::from(String::from("Output file already exists!")))
                .with_context(output_path_or_fb)
        } else {
            match File::create(output_path) {
                Ok(file) => {
                    self.created_files.push(output_path_or_fb);
                    Ok(file)
                }

                Err(error) => Err::<_, Error>(From::from(error)).with_context(output_path_or_fb),
            }
        }
    }
}

impl csv_writer::SinkProvider for FileProvider {
    fn execute_with_new_sink(
        &mut self,
        for_locales: Vec<String>,
        writer: csv_writer::Writer,
    ) -> Result<(), Error> {
        self.count_of_files_created += 1;
        let filename = format!("to_localize_{}", self.count_of_files_created);
        let mut sink = self.create_output_file(&filename).unwrap();
        writer.write(&mut sink)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fmt::Display;
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::{Path, PathBuf};

    use tempfile::TempDir;

    use crate::android_string::AndroidString;

    #[test]
    fn do_the_thing_errors_for_empty_locale_id_to_name_map() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut res_dir_path = temp_dir.path().to_path_buf();
        res_dir_path.push("res");
        fs::create_dir(res_dir_path.clone()).unwrap();

        let error =
            super::do_the_thing(res_dir_path.to_str().unwrap(), "", HashMap::new()).unwrap_err();
        assert_eq!(
            error.context(),
            &Some(String::from(res_dir_path.to_str().unwrap()))
        );
        assert!(error
            .to_string()
            .ends_with("Res dir doesn't have any non-default values dir with strings file!"))
    }

    #[test]
    fn create_output_dir_if_required_errors_if_output_dir_is_a_file_instead() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut output_dir_path = temp_dir.path().to_path_buf();
        output_dir_path.push("example file");

        // Output directory should instead be made a path
        File::create(&output_dir_path).unwrap();
        let output_dir_path = output_dir_path.to_str().unwrap();

        let error = super::create_output_dir_if_required(output_dir_path).unwrap_err();
        assert!(error
            .to_string()
            .ends_with("Output directory path points to a file!"));
        assert_eq!(error.context(), &Some(String::from(output_dir_path)));
    }

    #[test]
    fn create_output_file_errors_if_output_file_exists() {
        let temp_dir = tempfile::tempdir().unwrap();
        let output_dir_path = temp_dir.path();
        let mut output_file_path = output_dir_path.to_path_buf();
        output_file_path.push("op_file.csv");

        File::create(&output_file_path.clone()).unwrap();
        let output_dir_path = output_dir_path.to_str().unwrap();

        let mut file_provider = super::FileProvider::new(String::from(output_dir_path));
        let error = file_provider.create_output_file("op_file").unwrap_err();

        assert!(error.to_string().ends_with("Output file already exists!"));
        assert_eq!(
            error.context(),
            &Some(String::from(output_file_path.to_str().unwrap()))
        );
    }

    #[test]
    fn write_out_strings_to_localize_does_not_write_out_if_there_is_no_strings_to_localize() {
        let contents = r##"
			<?xml version="1.0" encoding="utf-8"?>
			<resources>
			    <string name="string">string value</string>
			</resources>
		"##;

        let default_strings = vec![AndroidString::new(
            String::from("string"),
            String::from("string value"),
            true,
        )];

        let temp_dir = tempfile::tempdir().unwrap();
        let (file_paths, output_dir) = test_write_out_strings_to_localize(
            &temp_dir,
            &contents.clone(),
            &contents.clone(),
            &contents,
            default_strings,
        );

        assert!(file_paths.is_empty());
        assert!(fs::read_dir(output_dir)
            .unwrap()
            .into_iter()
            .next()
            .is_none())
    }

    #[test]
    fn write_out_strings_to_localize_writes_out_if_there_are_strings_to_localize() {
        let contents = r##"
			<?xml version="1.0" encoding="utf-8"?>
			<resources>
			</resources>
		"##;

        let default_strings = vec![
            AndroidString::new(String::from("string_1"), String::from("string value"), true),
            AndroidString::new(String::from("string_2"), String::from("string value"), true),
        ];

        let temp_dir = tempfile::tempdir().unwrap();
        let (file_paths, output_dir) = test_write_out_strings_to_localize(
            &temp_dir,
            &contents.clone(),
            &contents.clone(),
            &contents,
            default_strings,
        );

        assert_eq!(file_paths.len(), 1);
        assert_eq!(
            file_paths,
            fs::read_dir(output_dir)
                .unwrap()
                .map(|f| String::from(f.unwrap().path().to_str().unwrap()))
                .collect::<Vec<String>>()
        );

        assert_eq!(
            Path::new(file_paths.first().unwrap())
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
            "to_localize_1.csv"
        );

        let mut output_file =
            File::open(&Path::new(&file_paths.into_iter().next().unwrap())).unwrap();
        let mut output = String::new();
        output_file.read_to_string(&mut output).unwrap();
        assert_eq_to_either_or(
            output,
            String::from("string_name,default_locale,spanish,french\nstring_1,string value,,\nstring_2,string value,,\n"),
            String::from("string_name,default_locale,french,spanish\nstring_1,string value,,\nstring_2,string value,,\n")
        );
    }

    /// Returns the output of the method call to `write_out_strings_to_localize`
    /// & the output dir path (built by the test)
    fn test_write_out_strings_to_localize(
        temp_dir: &TempDir,
        french_values_file_content: &str,
        spanish_values_file_content: &str,
        unmapped_german_values_file_content: &str,
        mut default_strings: Vec<AndroidString>,
    ) -> (Vec<String>, PathBuf) {
        // Build paths
        let mut res_dir_path = temp_dir.path().to_path_buf();
        res_dir_path.push("res");
        let mut fr_values_dir_path = res_dir_path.clone();
        fr_values_dir_path.push("values-fr");
        let mut fr_strings_file_path = fr_values_dir_path.clone();
        fr_strings_file_path.push("strings.xml");
        let mut es_values_dir_path = res_dir_path.clone();
        es_values_dir_path.push("values-es");
        let mut es_strings_file_path = es_values_dir_path.clone();
        es_strings_file_path.push("strings.xml");
        let mut de_values_dir_path = res_dir_path.clone();
        de_values_dir_path.push("values-de");
        let mut de_strings_file_path = de_values_dir_path.clone();
        de_strings_file_path.push("strings.xml");
        let mut output_dir_path = temp_dir.path().to_path_buf();
        output_dir_path.push("output");

        // Create required dirs & files with content
        fs::create_dir_all(fr_values_dir_path.clone()).unwrap();
        fs::create_dir_all(es_values_dir_path.clone()).unwrap();
        fs::create_dir_all(de_values_dir_path.clone()).unwrap();
        fs::create_dir_all(output_dir_path.clone()).unwrap();
        let mut fr_strings_file = File::create(fr_strings_file_path).unwrap();
        fr_strings_file
            .write(french_values_file_content.as_bytes())
            .unwrap();
        let mut es_strings_file = File::create(es_strings_file_path).unwrap();
        es_strings_file
            .write(spanish_values_file_content.as_bytes())
            .unwrap();
        let mut de_strings_file = File::create(de_strings_file_path).unwrap();
        de_strings_file
            .write(unmapped_german_values_file_content.as_bytes())
            .unwrap();

        // Not including german in this map to make sure that mappings also work as a filter
        let mut locale_id_to_name_map = HashMap::new();
        locale_id_to_name_map.insert(String::from("fr"), String::from("french"));
        locale_id_to_name_map.insert(String::from("es"), String::from("spanish"));

        // Perform action
        let result = super::write_out_strings_to_localize(
            &res_dir_path,
            output_dir_path.to_str().unwrap(),
            locale_id_to_name_map,
            &mut default_strings,
        )
        .unwrap();

        (result, output_dir_path)
    }

    fn assert_eq_to_either_or<T>(actual: T, expected1: T, expected2: T)
    where
        T: PartialEq,
        T: Display,
    {
        let result1 = actual == expected1;
        let result2 = actual == expected2;
        assert!(
            result1 || result2,
            r#"Actual: {};
        Expected either
        {}
        or
        {}"#,
            actual,
            expected1,
            expected2
        )
    }
}
