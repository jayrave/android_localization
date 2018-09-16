use android_string::AndroidString;
use foreign_lang_ids_finder;
use std::error;
use std::fmt;
use std::io;
use std::path::Path;
use validate::apostrophe;
use validate::format_string;
use xml_read_helper;

/// Performs all validations for all foreign strings
fn do_the_thing(res_dir_path: &str) -> Result<(), Error> {
    let res_dir_path_string = res_dir_path;
    let res_dir_path = Path::new(res_dir_path);
    let default_strings = match xml_read_helper::read_default_strings(res_dir_path) {
        Err(error) => return Err(Error::XmlReadError(error)),
        Ok(strings) => strings,
    };

    let mut default_parsed_data = format_string::parse_and_build_data(&default_strings);
    let mut invalid_strings_files = vec![];

    let lang_ids = match foreign_lang_ids_finder::find(res_dir_path_string) {
        Err(error) => return Err(Error::IoError(error.error)),
        Ok(lang_ids) => lang_ids,
    };

    for lang_id in lang_ids {
        let mut foreign_strings =
            match xml_read_helper::read_foreign_strings(res_dir_path, &lang_id) {
                Err(error) => return Err(Error::XmlReadError(error)),
                Ok(strings) => strings,
            };

        let apos_result = apostrophe::validate(&foreign_strings);
        let fs_result = format_string::validate(&mut default_parsed_data, &mut foreign_strings);

        let invalid = if apos_result.is_err() && fs_result.is_err() {
            Some(InvalidStringsFile {
                foreign_lang_id: lang_id,
                apostrophe_error: Some(apos_result.unwrap_err()),
                format_string_error: Some(fs_result.unwrap_err()),
            })
        } else if apos_result.is_err() {
            Some(InvalidStringsFile {
                foreign_lang_id: lang_id,
                apostrophe_error: Some(apos_result.unwrap_err()),
                format_string_error: None,
            })
        } else if fs_result.is_err() {
            Some(InvalidStringsFile {
                foreign_lang_id: lang_id,
                apostrophe_error: None,
                format_string_error: Some(fs_result.unwrap_err()),
            })
        } else {
            None
        };

        if let Some(invalid_file) = invalid {
            invalid_strings_files.push(invalid_file)
        }
    }

    if invalid_strings_files.is_empty() {
        Ok(())
    } else {
        Err(Error::ValidationError(invalid_strings_files))
    }
}

#[derive(Debug)]
pub struct InvalidStringsFile {
    foreign_lang_id: String,
    apostrophe_error: Option<apostrophe::Error>,
    format_string_error: Option<format_string::Error>,
}

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ValidationError(Vec<InvalidStringsFile>),
    XmlReadError(xml_read_helper::Error),
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::IoError(error) => Some(error),
            Error::ValidationError(error) => None,
            Error::XmlReadError(error) => Some(error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IoError(error) => fmt::Display::fmt(error, f),
            Error::XmlReadError(error) => fmt::Display::fmt(error, f),
            Error::ValidationError(invalid_strings_files) => invalid_strings_files
                .iter()
                .map(|e| {
                    let apos_error_string = match e.apostrophe_error {
                        Some(ref e) => e.to_string(),
                        None => String::from(""),
                    };

                    let fs_error_string = match e.format_string_error {
                        Some(ref e) => e.to_string(),
                        None => String::from(""),
                    };

                    writeln!(
                        f,
                        "Lang ID: {}; apostrophe errors: [{}]; format string errors: [{}]",
                        e.foreign_lang_id, apos_error_string, fs_error_string
                    )
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use android_string::AndroidString;
    use std::fs;
    use std::fs::File;
    use std::path::Path;
    use std::path::PathBuf;
    use writer::xml_writer;

    #[test]
    fn validations_issues_are_raised() {
        let tempdir = tempfile::tempdir().unwrap();
        let mut res_dir_path = tempdir.path().to_path_buf();
        res_dir_path.push("res");

        let mut default_values_dir_path = res_dir_path.clone();
        default_values_dir_path.push("values");
        fs::create_dir_all(&default_values_dir_path).unwrap();
        let mut default_strings_file = create_strings_file_in(&default_values_dir_path);

        let mut french_values_dir_path = res_dir_path.clone();
        french_values_dir_path.push("values-fr");
        fs::create_dir_all(&french_values_dir_path).unwrap();
        let mut french_strings_file = create_strings_file_in(&french_values_dir_path);

        let mut spanish_values_dir_path = res_dir_path.clone();
        spanish_values_dir_path.push("values-es");
        fs::create_dir_all(&spanish_values_dir_path).unwrap();
        let mut spanish_strings_file = create_strings_file_in(&spanish_values_dir_path);

        xml_writer::to(
            &mut default_strings_file,
            vec![
                AndroidString::new(String::from("s1"), String::from("value"), true),
                AndroidString::new(String::from("s2"), String::from("v'alue"), true),
            ],
        );

        xml_writer::to(
            &mut french_strings_file,
            vec![
                AndroidString::new(String::from("s1"), String::from("val'ue %1$s"), true),
                AndroidString::new(String::from("s2"), String::from("v'alue %1$d"), true),
            ],
        );

        xml_writer::to(
            &mut spanish_strings_file,
            vec![AndroidString::new(
                String::from("s1"),
                String::from("v'alue"),
                true,
            )],
        );

        let error = super::do_the_thing(res_dir_path.to_str().unwrap()).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Lang ID: es; apostrophe errors: [(Translatable: true; Name: s1; Value: v'alue)]; format string errors: []\nLang ID: fr; apostrophe errors: [(Translatable: true; Name: s1; Value: val'ue %1$s)(Translatable: true; Name: s2; Value: v'alue %1$d)]; format string errors: [(Format string mismatch. Found format strings () in default (value) & found format strings (%1$s) in foreign (val'ue %1$s))(Format string mismatch. Found format strings () in default (v'alue) & found format strings (%1$d) in foreign (v'alue %1$d))]\n"
        );
    }

    fn create_strings_file_in(dir_path: &PathBuf) -> File {
        let mut strings_file_path = dir_path.clone();
        strings_file_path.push("strings.xml");
        File::create(strings_file_path).unwrap()
    }
}
