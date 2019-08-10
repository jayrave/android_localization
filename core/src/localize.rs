use crate::android_string::AndroidString;
use crate::constants;
use crate::error::{Error, ResultExt};
use crate::localizable_strings::LocalizableStrings;
use crate::ops::filter;
use crate::reader::xml_reader;
use crate::util::foreign_lang_ids_finder;
use crate::util::xml_helper;
use crate::writer::csv_writer;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use std::path::PathBuf;

/// Returns the list of output files created by this call. These aren't guaranteed
/// to be valid paths to files. Sometimes, if a file's path can't be expressed by
/// `String` (in case it has non UTF-8 chars), it could just be the file's name
pub fn do_the_thing<S: ::std::hash::BuildHasher>(
    res_dir_path: &str,
    output_dir_path: &str,
    lang_id_to_human_friendly_name_mapping: HashMap<String, String, S>,
) -> Result<Vec<String>, Error> {
    let lang_id_to_human_friendly_name_mapping =
        foreign_lang_ids_finder::find_and_build_mapping_if_empty_or_return(
            lang_id_to_human_friendly_name_mapping,
            res_dir_path,
        )?;

    if lang_id_to_human_friendly_name_mapping.is_empty() {
        return Err::<_, Error>(From::from(String::from(
            "Res dir doesn't have any non-default values dir with strings file!",
        )))
        .with_context(String::from(res_dir_path));
    }

    let mut paths_of_created_file = vec![];
    create_output_dir_if_required(output_dir_path)?;

    // Read default strings
    let res_dir_path = Path::new(res_dir_path);
    let mut localizable_default_strings =
        filter::find_localizable_strings(xml_helper::read_default_strings(res_dir_path)?);

    // For all languages, write out strings requiring localization
    for (lang_id, human_friendly_name) in lang_id_to_human_friendly_name_mapping {
        let possible_output_file_path = write_out_strings_to_localize(
            res_dir_path,
            &lang_id,
            output_dir_path,
            &human_friendly_name,
            &mut localizable_default_strings,
        )?;

        if let Some(output_file_path) = possible_output_file_path {
            paths_of_created_file.push(output_file_path)
        }
    }

    Ok(paths_of_created_file)
}

fn create_output_dir_if_required(output_dir_path: &str) -> Result<(), Error> {
    let output_path = PathBuf::from(output_dir_path);
    if output_path.is_file() {
        return Err::<_, Error>(From::from(String::from(
            "Output directory path points to a file!",
        )))
        .with_context(String::from(output_dir_path));
    } else if output_path.exists() {
        Ok(())
    } else {
        fs::create_dir_all(PathBuf::from(output_dir_path))
            .with_context(String::from(output_dir_path))
    }
}

fn write_out_strings_to_localize(
    res_dir_path: &Path,
    lang_id: &str,
    output_dir_path: &str,
    file_name: &str,
    localizable_default_strings: &mut Vec<AndroidString>,
) -> Result<Option<String>, Error> {
    let mut foreign_strings =
        xml_helper::read_foreign_strings(res_dir_path, lang_id)?.into_strings();
    let strings_to_localize =
        filter::find_missing_strings(&mut foreign_strings, localizable_default_strings);

    if !strings_to_localize.is_empty() {
        let mut sink_provider = FileProvider::new(String::from(output_dir_path));
        let strings_to_localize = vec![LocalizableStrings::new(
            String::from(file_name),
            filter::find_missing_strings(&mut foreign_strings, localizable_default_strings),
        )];

        let result = csv_writer::write(strings_to_localize, &mut sink_provider);
        let created_file_name = String::from(
            sink_provider
                .created_files()
                .first()
                .unwrap_or(&String::from("no files created by sink")),
        );

        return result
            .map(|_| Some(created_file_name.clone()))
            .with_context(created_file_name);
    }

    Ok(None)
}

struct FileProvider {
    sink_dir: String,
    created_files: Vec<String>,
}

impl FileProvider {
    fn new(sink_dir: String) -> FileProvider {
        FileProvider {
            sink_dir,
            created_files: Vec::new(),
        }
    }

    fn created_files(self) -> Vec<String> {
        self.created_files
    }

    /// Returns the created output file along with its path (if path computation
    /// is possible; if not, it passes out a fallback value)
    fn create_output_file(&mut self, output_file_name: &str) -> Result<(File, String), Error> {
        let mut output_path = PathBuf::from(&self.sink_dir);
        output_path.push(output_file_name);
        output_path.set_extension(constants::extn::CSV);
        let output_path_or_fb = String::from(output_path.to_str().unwrap_or(output_file_name));

        if output_path.exists() {
            return Err::<_, Error>(From::from(String::from("Output file already exists!")))
                .with_context(output_path_or_fb);
        } else {
            match File::create(output_path) {
                Ok(file) => {
                    self.created_files.push(output_path_or_fb.clone());
                    Ok((file, output_path_or_fb))
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
        let filename = for_locales.join("_");
        let (mut sink, output_path_or_fb) = self.create_output_file(&filename).unwrap();
        writer.write(&mut sink)
    }
}

#[cfg(test)]
mod tests {
    use crate::android_string::AndroidString;
    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::path::Path;
    use tempfile::TempDir;

    #[test]
    fn do_the_thing_errors_for_empty_lang_id_to_human_friendly_name_mapping() {
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
        let (method_output, possible_output_file) =
            test_write_out_strings_to_localize(&temp_dir, &contents, default_strings);

        assert_eq!(method_output, None);
        assert!(!Path::new(&possible_output_file).exists())
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
        let (method_output, possible_output_file) =
            test_write_out_strings_to_localize(&temp_dir, &contents, default_strings);

        assert_eq!(method_output.unwrap(), possible_output_file);

        let mut output_file = File::open(possible_output_file).unwrap();
        let mut output = String::new();
        output_file.read_to_string(&mut output).unwrap();
        assert_eq!(
            output,
            "string_name,default_locale,french\nstring_1,string value,\nstring_2,string value,\n"
        );
    }

    /// Returns the output of the method call to `write_out_strings_to_localize`
    /// & the possible output file (built by the test)
    fn test_write_out_strings_to_localize(
        temp_dir: &TempDir,
        values_file_content: &str,
        mut default_strings: Vec<AndroidString>,
    ) -> (Option<String>, String) {
        // Build paths
        let mut values_dir_path = temp_dir.path().to_path_buf();
        values_dir_path.push("res");
        values_dir_path.push("values-fr");
        let mut strings_file_path = values_dir_path.clone();
        strings_file_path.push("strings.xml");
        let mut output_dir_path = temp_dir.path().to_path_buf();
        output_dir_path.push("output");
        let mut output_file_path = output_dir_path.clone();
        output_file_path.push("french.csv");

        // Create required dirs & files with content
        fs::create_dir_all(values_dir_path.clone()).unwrap();
        fs::create_dir_all(output_dir_path.clone()).unwrap();
        let mut strings_file = File::create(strings_file_path).unwrap();
        strings_file.write(values_file_content.as_bytes()).unwrap();
        strings_file.seek(SeekFrom::Start(0)).unwrap();

        // Perform action
        let result = super::write_out_strings_to_localize(
            values_dir_path.parent().unwrap(),
            "fr",
            output_dir_path.to_str().unwrap(),
            output_file_path.file_stem().unwrap().to_str().unwrap(),
            &mut default_strings,
        )
        .unwrap();

        (
            result,
            String::from(output_file_path.clone().to_str().unwrap()),
        )
    }
}
