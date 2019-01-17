use android_string::AndroidString;
use constants;
use helper::xml_read_helper;
use ops::filter;
use reader::xml_reader;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use util::foreign_lang_ids_finder;
use writer::csv_writer;

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
        return Err(Error {
            path: Some(String::from(res_dir_path)),
            kind: ErrorKind::ArgError(String::from(
                "Res dir doesn't have any non-default values dir with strings file!",
            )),
        });
    }

    let mut paths_of_created_file = vec![];
    create_output_dir_if_required(output_dir_path)?;

    // Read default strings
    let res_dir_path = Path::new(res_dir_path);
    let mut translatable_default_strings =
        filter::find_translatable_strings(xml_read_helper::read_default_strings(res_dir_path)?);

    // For all languages, write out strings requiring translation
    for (lang_id, human_friendly_name) in lang_id_to_human_friendly_name_mapping {
        let possible_output_file_path = write_out_strings_to_translate(
            res_dir_path,
            &lang_id,
            output_dir_path,
            &human_friendly_name,
            &mut translatable_default_strings,
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
        Err(Error {
            path: Some(String::from(output_dir_path)),
            kind: ErrorKind::ArgError(String::from("Output directory path points to a file!")),
        })
    } else if output_path.exists() {
        Ok(())
    } else {
        match fs::create_dir_all(PathBuf::from(output_dir_path)) {
            Ok(()) => Ok(()),
            Err(error) => Err(Error {
                path: Some(String::from(output_dir_path)),
                kind: ErrorKind::IoError(error),
            }),
        }
    }
}

/// Returns the created output file along with its path (if path computation
/// is possible; if not, it passes out a fallback value)
fn create_output_file(
    output_dir_path: &str,
    output_file_name: &str,
) -> Result<(File, String), Error> {
    let mut output_path = PathBuf::from(output_dir_path);
    output_path.push(output_file_name);
    output_path.set_extension(constants::extn::CSV);
    let output_path_or_fb = String::from(output_path.to_str().unwrap_or(output_file_name));

    if output_path.exists() {
        Err(Error {
            path: Some(output_path_or_fb),
            kind: ErrorKind::ArgError(String::from("Output file already exists!")),
        })
    } else {
        match File::create(output_path) {
            Ok(file) => Ok((file, output_path_or_fb)),
            Err(error) => Err(Error {
                path: Some(output_path_or_fb),
                kind: ErrorKind::IoError(error),
            }),
        }
    }
}

fn write_out_strings_to_translate(
    res_dir_path: &Path,
    lang_id: &str,
    output_dir_path: &str,
    file_name: &str,
    translatable_default_strings: &mut Vec<AndroidString>,
) -> Result<Option<String>, Error> {
    let mut foreign_strings =
        xml_read_helper::read_foreign_strings(res_dir_path, lang_id)?.into_strings();
    let strings_to_translate =
        filter::find_missing_strings(&mut foreign_strings, translatable_default_strings);

    if !strings_to_translate.is_empty() {
        let (mut sink, output_path_or_fb) = create_output_file(output_dir_path, file_name)?;
        return match csv_writer::single_locale_write(&mut sink, strings_to_translate) {
            Ok(_) => Ok(Some(output_path_or_fb)),
            Err(error) => Err(Error {
                path: Some(output_path_or_fb),
                kind: ErrorKind::CsvError(error),
            }),
        };
    }

    Ok(None)
}

#[derive(Debug)]
pub struct Error {
    path: Option<String>,
    kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    ArgError(String),
    CsvError(csv_writer::Error),
    ForeignLangIdsFinder(foreign_lang_ids_finder::Error),
    IoError(io::Error),
    XmlError(xml_reader::Error),
    XmlReadHelperError(xml_read_helper::Error),
}

impl From<foreign_lang_ids_finder::Error> for Error {
    fn from(error: foreign_lang_ids_finder::Error) -> Self {
        Error {
            path: None,
            kind: ErrorKind::ForeignLangIdsFinder(error),
        }
    }
}

impl From<xml_read_helper::Error> for Error {
    fn from(error: xml_read_helper::Error) -> Self {
        Error {
            path: None,
            kind: ErrorKind::XmlReadHelperError(error),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match &self.kind {
            ErrorKind::ArgError(_message) => None,
            ErrorKind::CsvError(error) => Some(error),
            ErrorKind::ForeignLangIdsFinder(error) => Some(error),
            ErrorKind::IoError(error) => Some(error),
            ErrorKind::XmlError(error) => Some(error),
            ErrorKind::XmlReadHelperError(error) => Some(error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(path) = &self.path {
            write!(f, "Path: {}; Error: ", path)?;
        }

        match &self.kind {
            ErrorKind::ArgError(message) => fmt::Display::fmt(message, f),
            ErrorKind::CsvError(error) => fmt::Display::fmt(error, f),
            ErrorKind::ForeignLangIdsFinder(error) => fmt::Display::fmt(error, f),
            ErrorKind::IoError(error) => fmt::Display::fmt(error, f),
            ErrorKind::XmlError(error) => fmt::Display::fmt(error, f),
            ErrorKind::XmlReadHelperError(error) => fmt::Display::fmt(error, f),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use self::tempfile::TempDir;
    use android_string::AndroidString;
    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::path::Path;

    #[test]
    fn do_the_thing_errors_for_empty_lang_id_to_human_friendly_name_mapping() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut res_dir_path = temp_dir.path().to_path_buf();
        res_dir_path.push("res");
        fs::create_dir(res_dir_path.clone()).unwrap();

        let error =
            super::do_the_thing(res_dir_path.to_str().unwrap(), "", HashMap::new()).unwrap_err();
        assert_eq!(
            error.path,
            Some(String::from(res_dir_path.to_str().unwrap()))
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
        assert_eq!(error.path.unwrap(), output_dir_path);
    }

    #[test]
    fn create_output_file_errors_if_output_file_exists() {
        let temp_dir = tempfile::tempdir().unwrap();
        let output_dir_path = temp_dir.path();
        let mut output_file_path = output_dir_path.to_path_buf();
        output_file_path.push("op_file.csv");

        File::create(&output_file_path.clone()).unwrap();
        let output_dir_path = output_dir_path.to_str().unwrap();

        let error = super::create_output_file(output_dir_path, "op_file").unwrap_err();
        assert!(error.to_string().ends_with("Output file already exists!"));
        assert_eq!(
            error.path.unwrap(),
            String::from(output_file_path.to_str().unwrap())
        );
    }

    #[test]
    fn write_out_strings_to_translate_does_not_write_out_if_there_is_no_strings_to_translate() {
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
            test_write_out_strings_to_translate(&temp_dir, &contents, default_strings);

        assert_eq!(method_output, None);
        assert!(!Path::new(&possible_output_file).exists())
    }

    #[test]
    fn write_out_strings_to_translate_writes_out_if_there_are_strings_to_translate() {
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
            test_write_out_strings_to_translate(&temp_dir, &contents, default_strings);

        assert_eq!(method_output.unwrap(), possible_output_file);

        let mut output_file = File::open(possible_output_file).unwrap();
        let mut output = String::new();
        output_file.read_to_string(&mut output).unwrap();
        assert_eq!(output, "string_1,string value\nstring_2,string value\n");
    }

    /// Returns the output of the method call to `write_out_strings_to_translate`
    /// & the possible output file (built by the test)
    fn test_write_out_strings_to_translate(
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
        let result = super::write_out_strings_to_translate(
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
