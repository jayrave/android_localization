use android_string::AndroidString;
use constants;
use ops::filter;
use reader::xml_reader;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::ops::Add;
use std::path::Path;
use std::path::PathBuf;
use writer::csv_writer;

pub fn do_the_thing(
    res_dir_path: &str,
    output_dir_path: &str,
    lang_id_to_human_friendly_name_mapping: HashMap<String, String>,
) -> Result<(), Error> {
    if lang_id_to_human_friendly_name_mapping.is_empty() {
        return Err(Error::ArgError(String::from(
            "Language ID to human friendly name mapping can't be empty",
        )));
    }

    create_output_dir_if_required(output_dir_path)?;

    // Read default strings
    let res_dir_path = Path::new(res_dir_path);
    let mut translatable_default_strings =
        filter::find_translatable_strings(read_default_strings(res_dir_path)?);

    // For all languages, write out strings requiring translation
    for (lang_id, human_friendly_name) in lang_id_to_human_friendly_name_mapping {
        write_out_strings_to_translate(
            res_dir_path,
            &lang_id,
            output_dir_path,
            &human_friendly_name,
            &mut translatable_default_strings,
        )?;
    }

    Ok(())
}

fn create_output_dir_if_required(output_dir_path: &str) -> Result<(), Error> {
    let output_path = PathBuf::from(output_dir_path);
    match output_path.is_file() {
        true => Err(Error::ArgError(format!(
            "Output directory path ({}) points to a file!",
            output_dir_path
        ))),
        false => match output_path.exists() {
            true => Ok(()),
            false => match fs::create_dir_all(PathBuf::from(output_dir_path)) {
                Err(error) => Err(Error::IoError(error)),
                Ok(()) => Ok(()),
            },
        },
    }
}

fn create_output_file(output_dir_path: &str, output_file_name: &str) -> Result<File, Error> {
    let mut output_path = PathBuf::from(output_dir_path);
    output_path.push(output_file_name);
    match output_path.exists() {
        true => Err(Error::ArgError(format!(
            "File ({}) already exists in {}!",
            output_file_name, output_dir_path
        ))),
        false => match File::create(output_path) {
            Ok(file) => Ok(file),
            Err(error) => Err(Error::IoError(error)),
        },
    }
}

fn read_default_strings(res_dir_path: &Path) -> Result<Vec<AndroidString>, Error> {
    read_strings(res_dir_path, constants::fs::BASE_VALUES_DIR_NAME)
}

fn read_foreign_strings(res_dir_path: &Path, lang_id: &str) -> Result<Vec<AndroidString>, Error> {
    let mut values_dir_name = String::from(constants::fs::BASE_VALUES_DIR_NAME);
    let values_dir_name = values_dir_name.add(&format!("-{}", lang_id));
    read_strings(res_dir_path, &values_dir_name)
}

fn read_strings(res_dir_path: &Path, values_dir_name: &str) -> Result<Vec<AndroidString>, Error> {
    let mut default_values_file_path = res_dir_path.to_path_buf();
    default_values_file_path.push(values_dir_name);
    default_values_file_path.push(constants::fs::STRING_FILE_NAME);

    match File::open(default_values_file_path) {
        Err(error) => Err(Error::IoError(error)),
        Ok(file) => match xml_reader::reader::from(file) {
            Err(error) => Err(Error::XmlError(error)),
            Ok(strings) => Ok(strings),
        },
    }
}

fn write_out_strings_to_translate(
    res_dir_path: &Path,
    lang_id: &str,
    output_dir_path: &str,
    file_name: &str,
    translatable_default_strings: &mut Vec<AndroidString>,
) -> Result<(), Error> {
    let mut foreign_strings = read_foreign_strings(res_dir_path, lang_id)?;
    let strings_to_translate =
        filter::find_missing_strings(&mut foreign_strings, translatable_default_strings);
    if !strings_to_translate.is_empty() {
        let mut sink = create_output_file(output_dir_path, file_name)?;
        if let Err(error) = csv_writer::to(&mut sink, strings_to_translate) {
            return Err(Error::CsvError(error));
        }
    }

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    ArgError(String),
    CsvError(csv_writer::Error),
    IoError(io::Error),
    XmlError(xml_reader::Error),
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::ArgError(_message) => None,
            Error::CsvError(error) => Some(error),
            Error::IoError(error) => Some(error),
            Error::XmlError(error) => Some(error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ArgError(message) => fmt::Display::fmt(message, f),
            Error::CsvError(error) => fmt::Display::fmt(error, f),
            Error::IoError(error) => fmt::Display::fmt(error, f),
            Error::XmlError(error) => fmt::Display::fmt(error, f),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use android_string::AndroidString;
    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::path::Path;

    #[test]
    fn do_the_thing_errors_for_empty_lang_id_to_human_friendly_name_mapping() {
        let error = super::do_the_thing("", "", HashMap::new());
        assert_eq!(
            error.unwrap_err().to_string(),
            "Language ID to human friendly name mapping can't be empty"
        )
    }

    #[test]
    fn create_output_dir_if_required_errors_if_output_dir_is_a_file_instead() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut output_dir_path = temp_dir.path().to_path_buf();
        output_dir_path.push("example file");

        // Output directory should instead be made a path
        File::create(&output_dir_path).unwrap();
        let output_dir_path = output_dir_path.to_str().unwrap();

        let error = super::create_output_dir_if_required(output_dir_path);
        assert_eq!(
            error.unwrap_err().to_string(),
            format!(
                "Output directory path ({}) points to a file!",
                output_dir_path
            )
        )
    }

    #[test]
    fn create_output_file_errors_if_output_file_exists() {
        let temp_dir = tempfile::tempdir().unwrap();
        let output_dir_path = temp_dir.path();
        let mut output_file_path = output_dir_path.to_path_buf();
        output_file_path.push("op_file");

        File::create(&output_file_path).unwrap();
        let output_dir_path = output_dir_path.to_str().unwrap();

        let error = super::create_output_file(output_dir_path, "op_file");
        assert_eq!(
            error.unwrap_err().to_string(),
            format!("File (op_file) already exists in {}!", output_dir_path)
        )
    }

    #[test]
    fn read_strings_errors_if_values_dir_is_missing() {
        let res_dir = tempfile::tempdir().unwrap();
        let error = super::read_strings(res_dir.path(), "values");
        assert!(
            error
                .unwrap_err()
                .to_string()
                .starts_with("No such file or directory")
        )
    }

    #[test]
    fn read_strings_errors_if_strings_file_is_missing() {
        let res_dir = tempfile::tempdir().unwrap();

        let mut values_dir_path = res_dir.path().to_path_buf();
        values_dir_path.push("values");
        fs::create_dir(values_dir_path).unwrap();

        let error = super::read_strings(res_dir.path(), "values");
        assert!(
            error
                .unwrap_err()
                .to_string()
                .starts_with("No such file or directory")
        )
    }

    #[test]
    fn write_out_strings_to_translate_does_not_write_out_if_there_is_no_strings_to_translate() {
        let contents = r##"
			<?xml version="1.0" encoding="utf-8"?>
			<resources>
			    <string name="string">string value</string>
			</resources>
		"##;

        let mut default_strings = vec![AndroidString::new(
            String::from("string"),
            String::from("string value"),
            true,
        )];

        test_write_out_strings_to_translate(&contents, default_strings, |output_file_path| {
            assert!(!Path::new(output_file_path).exists())
        })
    }

    #[test]
    fn write_out_strings_to_translate_writes_out_if_there_are_strings_to_translate() {
        let contents = r##"
			<?xml version="1.0" encoding="utf-8"?>
			<resources>
			</resources>
		"##;

        let mut default_strings = vec![
            AndroidString::new(String::from("string_1"), String::from("string value"), true),
            AndroidString::new(String::from("string_2"), String::from("string value"), true),
        ];

        test_write_out_strings_to_translate(&contents, default_strings, |output_file_path| {
            let mut output_file = File::open(output_file_path).unwrap();
            let mut output = String::new();
            output_file.read_to_string(&mut output);
            assert_eq!(output, "string_1,string value\nstring_2,string value\n");
        })
    }

    fn test_write_out_strings_to_translate<A: Fn(&str)>(
        values_file_content: &str,
        mut default_strings: Vec<AndroidString>,
        asserter: A,
    ) {
        let temp_dir = tempfile::tempdir().unwrap();

        // Build paths
        let mut values_dir_path = temp_dir.path().to_path_buf();
        values_dir_path.push("res");
        values_dir_path.push("values-fr");
        let mut strings_file_path = values_dir_path.clone();
        strings_file_path.push("strings.xml");
        let mut output_dir_path = temp_dir.path().to_path_buf();
        output_dir_path.push("output");
        let mut output_file_path = output_dir_path.clone();
        output_file_path.push("french");

        // Create required dirs & files with content
        fs::create_dir_all(values_dir_path.clone());
        fs::create_dir_all(output_dir_path.clone());
        let mut strings_file = File::create(strings_file_path).unwrap();
        strings_file.write(values_file_content.as_bytes());
        strings_file.seek(SeekFrom::Start(0)).unwrap();

        // Perform action
        super::write_out_strings_to_translate(
            values_dir_path.parent().unwrap(),
            "fr",
            output_dir_path.to_str().unwrap(),
            output_file_path.file_name().unwrap().to_str().unwrap(),
            &mut default_strings,
        ).unwrap();

        // Assert appropriate output
        asserter(output_file_path.clone().to_str().unwrap());
    }
}
