use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::ops::Add;
use std::path::Path;
use std::path::PathBuf;

use crate::android_string::AndroidString;
use crate::constants;
use crate::error::{Error, ResultExt};
use crate::ops::dedup;
use crate::ops::extract;
use crate::ops::filter;
use crate::ops::merge;
use crate::reader::csv_reader;
use crate::util::foreign_locale_ids_finder;
use crate::util::xml_helper;
use crate::writer::xml_writer;

/// Returns the list of output files created by this call. These aren't guaranteed
/// to be valid paths to files. Sometimes, if a file's path can't be expressed by
/// `String` (in case it has non UTF-8 chars), it could just be the file's name
pub fn do_the_thing<S: ::std::hash::BuildHasher>(
    res_dir_path: &str,
    localized_text_file_path: &str,
    locale_name_to_id_map: HashMap<String, String, S>,
) -> Result<Vec<String>, Error> {
    let locale_name_to_id_map = foreign_locale_ids_finder::build_map_if_empty_or_return(
        locale_name_to_id_map,
        res_dir_path,
    )?;

    if locale_name_to_id_map.is_empty() {
        return Err::<_, Error>(From::from(String::from(
            "Res dir doesn't have any non-default values dir with strings file!",
        )))
        .with_context(String::from(res_dir_path));
    }

    // Read default strings
    let res_dir_path = Path::new(res_dir_path);
    let mut localizable_default_strings = filter::find_localizable_strings(
        xml_helper::read_default_strings(res_dir_path)?.into_strings(),
    );

    // For all languages, handle localized text
    handle_localized(
        res_dir_path,
        localized_text_file_path,
        locale_name_to_id_map,
        &mut localizable_default_strings,
    )
}

fn handle_localized<S: ::std::hash::BuildHasher>(
    res_dir_path: &Path,
    localized_text_file_path: &str,
    locale_name_to_id_map: HashMap<String, String, S>,
    localizable_default_strings: &mut Vec<AndroidString>,
) -> Result<Vec<String>, Error> {
    // Read all new localized strings
    let new_localized_foreign_strings_list = csv_reader::read(
        File::open(localized_text_file_path)
            .with_context(String::from(localized_text_file_path))?,
        locale_name_to_id_map
            .keys()
            .map(|s: &String| String::clone(s))
            .collect(),
    )?;

    let mut created_files_paths = vec![];
    for new_localized_foreign_strings in new_localized_foreign_strings_list {
        let locale_id = locale_name_to_id_map
            .get(new_localized_foreign_strings.locale())
            .expect("Read locale doesn't have a mapping! Please let the dev know about this issue");

        // Read already localized foreign strings for locale
        let mut already_localized_foreign_strings = filter::find_localizable_strings(
            xml_helper::read_foreign_strings(res_dir_path, locale_id)?.into_strings(),
        );

        // Extract android strings out of the newly localized strings
        let mut new_localized_foreign_strings = extract::extract_android_strings_from_localized(
            &mut new_localized_foreign_strings.into_strings(),
            localizable_default_strings,
        );

        // Merge & dedup foreign strings
        let to_be_written_foreign_strings =
            dedup::dedup_grouped_strings(merge::merge_and_group_strings(
                &mut new_localized_foreign_strings,
                &mut already_localized_foreign_strings,
            ));

        // Write out foreign strings back to file
        let (mut file, output_file_path) =
            writable_empty_foreign_strings_file(res_dir_path, locale_id)?;
        xml_writer::write(&mut file, to_be_written_foreign_strings)
            .with_context(output_file_path.clone())?;

        created_files_paths.push(output_file_path);
    }

    Ok(created_files_paths)
}

/// Returns the created output file along with its path (if path computation
/// is possible; if not, it passes out a fallback value)
fn writable_empty_foreign_strings_file(
    res_dir_path: &Path,
    locale_id: &str,
) -> Result<(File, String), Error> {
    let values_dir_name = String::from(constants::fs::BASE_VALUES_DIR_NAME);
    let values_dir_name = values_dir_name.add(&format!("-{}", locale_id));

    let mut strings_file_path = res_dir_path.to_path_buf();
    strings_file_path.push(values_dir_name);
    strings_file_path.push(constants::fs::STRING_FILE_NAME);
    let output_path_or_fb = String::from(strings_file_path.to_str().unwrap_or(locale_id));

    // empties out the file if it has any content
    Ok((
        File::create(strings_file_path).with_context(output_path_or_fb.clone())?,
        output_path_or_fb,
    ))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;

    use crate::android_string::AndroidString;
    use crate::util::xml_helper;
    use crate::writer::xml_writer;

    #[test]
    fn do_the_thing_errors_for_empty_locale_name_to_id_map() {
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
    fn do_the_thing() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Build paths
        let mut res_dir_path = temp_dir.path().to_path_buf();
        res_dir_path.push("res");
        let mut default_values_dir_path = res_dir_path.clone();
        default_values_dir_path.push("values");
        let mut default_strings_file_path = default_values_dir_path.clone();
        default_strings_file_path.push("strings.xml");

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

        let mut localized_dir_path = temp_dir.path().to_path_buf();
        localized_dir_path.push("localized");
        let mut localized_file_path = localized_dir_path.clone();
        localized_file_path.push("localized.csv");

        // Create required dirs & files with content
        fs::create_dir_all(default_values_dir_path.clone()).unwrap();
        fs::create_dir_all(fr_values_dir_path.clone()).unwrap();
        fs::create_dir_all(es_values_dir_path.clone()).unwrap();
        fs::create_dir_all(de_values_dir_path.clone()).unwrap();
        fs::create_dir_all(localized_dir_path.clone()).unwrap();
        let mut default_strings_file = File::create(default_strings_file_path).unwrap();
        let mut fr_strings_file = File::create(fr_strings_file_path.clone()).unwrap();
        let mut es_strings_file = File::create(es_strings_file_path.clone()).unwrap();
        let mut de_strings_file = File::create(de_strings_file_path.clone()).unwrap();
        let mut localized_file = File::create(localized_file_path.clone()).unwrap();

        // Write out required contents into files
        xml_writer::write(
            &mut default_strings_file,
            vec![
                AndroidString::new(String::from("s1"), String::from("english value 1"), true),
                AndroidString::new(String::from("s2"), String::from("english value 2"), true),
            ],
        )
        .unwrap();

        xml_writer::write(
            &mut fr_strings_file,
            vec![
                AndroidString::new(String::from("s1"), String::from("french old value 1"), true),
                AndroidString::new(String::from("s2"), String::from("french old value 2"), true),
            ],
        )
        .unwrap();

        xml_writer::write(
            &mut es_strings_file,
            vec![
                AndroidString::new(
                    String::from("s1"),
                    String::from("spanish old value 1"),
                    true,
                ),
                AndroidString::new(
                    String::from("s2"),
                    String::from("spanish old value 2"),
                    true,
                ),
            ],
        )
        .unwrap();

        let german_android_strings = vec![
            AndroidString::new(String::from("s1"), String::from("german old value 1"), true),
            AndroidString::new(String::from("s2"), String::from("german old value 2"), true),
        ];

        xml_writer::write(
            &mut de_strings_file,
            german_android_strings.clone(),
        )
            .unwrap();

        localized_file
            .write(
                r#"string_name, default_locale, french, spanish, german
s1, english value 1, french new value 1,,german new value 1
s2, english value 2,,spanish new value 2,german new value 2"#
                    .as_bytes(),
            )
            .unwrap();

        // Not including german in this map to make sure that mappings also work as a filter
        let mut map = HashMap::new();
        map.insert(String::from("french"), String::from("fr"));
        map.insert(String::from("spanish"), String::from("es"));

        // Perform action
        let created_output_files_path = super::do_the_thing(
            res_dir_path.clone().to_str().unwrap(),
            localized_file_path.to_str().unwrap(),
            map,
        )
        .unwrap();

        // Assert appropriate output
        assert_eq!(
            created_output_files_path,
            vec![
                fr_strings_file_path.to_str().unwrap(),
                es_strings_file_path.to_str().unwrap()
            ]
        );

        assert_eq!(
            xml_helper::read_foreign_strings(&res_dir_path, "fr")
                .unwrap()
                .into_strings(),
            vec![
                AndroidString::new(String::from("s1"), String::from("french new value 1"), true),
                AndroidString::new(String::from("s2"), String::from("french old value 2"), true),
            ]
        );

        assert_eq!(
            xml_helper::read_foreign_strings(&res_dir_path, "es")
                .unwrap()
                .into_strings(),
            vec![
                AndroidString::new(
                    String::from("s1"),
                    String::from("spanish old value 1"),
                    true
                ),
                AndroidString::new(
                    String::from("s2"),
                    String::from("spanish new value 2"),
                    true
                ),
            ]
        );

        assert_eq!(
            xml_helper::read_foreign_strings(&res_dir_path, "de")
                .unwrap()
                .into_strings(),
            german_android_strings
        );
    }

    #[test]
    fn writable_empty_foreign_strings_file() {
        let res_dir = tempfile::tempdir().unwrap();

        let mut values_dir_path = res_dir.path().to_path_buf();
        values_dir_path.push("values-fr");

        let mut strings_file_path = values_dir_path.clone();
        strings_file_path.push("strings.xml");

        fs::create_dir(values_dir_path).unwrap();
        let mut file_with_old_content: File = File::create(strings_file_path.clone()).unwrap();
        file_with_old_content
            .write("example old content".as_bytes())
            .unwrap();

        let (mut file_with_new_content, file_path) =
            super::writable_empty_foreign_strings_file(res_dir.path(), "fr").unwrap();
        file_with_new_content
            .write("example new content".as_bytes())
            .unwrap();

        let mut file_contents = String::new();
        File::open(strings_file_path.clone())
            .unwrap()
            .read_to_string(&mut file_contents)
            .unwrap();

        assert_eq!(file_contents, "example new content");
        assert_eq!(file_path, strings_file_path.to_str().unwrap())
    }
}
