use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::ops::Add;
use std::path::Path;

use crate::android_string::AndroidString;
use crate::constants;
use crate::error::{Error, ResultExt};
use crate::ops::dedup;
use crate::ops::extract;
use crate::ops::filter;
use crate::ops::merge;
use crate::reader::csv_reader;
use crate::util::foreign_locale_ids_finder;
use crate::util::xml_utilities;
use crate::writer::xml_writer;

/// Returns the list of output files updated by this call. These aren't guaranteed
/// to be valid paths to files. Sometimes, if a file's path can't be expressed by
/// `String` (in case it has non UTF-8 chars), it could just be the file's name
pub fn localized<S: ::std::hash::BuildHasher>(
    res_dir_path: &str,
    localized_text_file_path: &str,
    locale_name_to_id_map: HashMap<String, String, S>,
) -> Result<Vec<String>, Error> {
    let locale_name_to_id_map = foreign_locale_ids_finder::build_map_if_empty_or_return(
        locale_name_to_id_map,
        res_dir_path,
    )?;

    if locale_name_to_id_map.is_empty() {
        return Err(Error::new(
            res_dir_path,
            "Res dir doesn't have any non-default values dir with strings file!",
        ));
    }

    // Read default strings
    let res_dir_path = Path::new(res_dir_path);
    let mut localizable_default_strings = filter::find_localizable_strings(
        xml_utilities::read_default_strings(res_dir_path)?.into_strings(),
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
    localizable_default_strings: &mut [AndroidString],
) -> Result<Vec<String>, Error> {
    // Read all new localized strings
    let new_localized_foreign_strings_list = csv_reader::read(
        File::open(localized_text_file_path)
            .with_context(String::from(localized_text_file_path))?,
        locale_name_to_id_map
            .keys()
            .map(|s: &String| String::clone(s))
            .collect(),
    )
    .with_context(localized_text_file_path)?;

    let mut updated_files_paths = vec![];
    for new_localized_foreign_strings in new_localized_foreign_strings_list {
        let locale_id = locale_name_to_id_map
            .get(new_localized_foreign_strings.locale())
            .expect("Read locale doesn't have a mapping! Please let the dev know about this issue");

        let existing_foreign_strings =
            xml_utilities::read_foreign_strings(res_dir_path, locale_id)?.into_strings();
        let existing_foreign_strings_hash = compute_hash_of(&existing_foreign_strings);

        // Read already localized foreign strings for locale
        let mut already_localized_foreign_strings =
            filter::find_localizable_strings(existing_foreign_strings);

        // Extract android strings out of the newly localized strings
        let mut new_localized_foreign_strings = extract::extract_android_strings_from_localized(
            &mut new_localized_foreign_strings.into_strings(),
            localizable_default_strings,
        );

        // Merge already existing & newly localized strings
        let mut to_be_written_foreign_strings = merge::merge_and_group_strings(
            &mut new_localized_foreign_strings,
            &mut already_localized_foreign_strings,
        );

        // There could be duplicates!
        dedup::dedup_grouped_strings(&mut to_be_written_foreign_strings);

        let new_foreign_strings_hash = compute_hash_of(&to_be_written_foreign_strings);

        // Write out foreign strings back to file
        let (mut file, output_file_path) =
            writable_empty_foreign_strings_file(res_dir_path, locale_id)?;
        xml_writer::write(&mut file, to_be_written_foreign_strings)
            .with_context(output_file_path.clone())?;

        // If the file's content isn't getting updated, needn't include it in the
        // updated files list
        if new_foreign_strings_hash != existing_foreign_strings_hash {
            updated_files_paths.push(output_file_path);
        }
    }

    Ok(updated_files_paths)
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

fn compute_hash_of(strings: &[AndroidString]) -> u64 {
    let mut hasher = DefaultHasher::new();
    strings.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;

    use test_utilities;

    use crate::android_string::AndroidString;
    use crate::util::xml_utilities;
    use crate::writer::xml_writer;

    #[test]
    fn errors_for_empty_locale_name_to_id_map() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut res_dir_path = temp_dir.path().to_path_buf();
        res_dir_path.push("res");
        fs::create_dir(res_dir_path.clone()).unwrap();

        let error =
            super::localized(res_dir_path.to_str().unwrap(), "", HashMap::new()).unwrap_err();
        assert_eq!(
            error.context(),
            &String::from(res_dir_path.to_str().unwrap())
        );
        assert!(error
            .to_string()
            .ends_with("Res dir doesn't have any non-default values dir with strings file!"))
    }

    #[test]
    fn updates_strings_files() {
        // Build paths
        let temp_dir = tempfile::tempdir().unwrap();
        let mut res_path = temp_dir.path().to_path_buf();
        res_path.push("res");

        let mut default_strings =
            test_utilities::res::setup_empty_strings_for_default_locale(res_path.clone());
        let mut fr_strings =
            test_utilities::res::setup_empty_strings_for_locale(res_path.clone(), "fr");
        let mut es_strings =
            test_utilities::res::setup_empty_strings_for_locale(res_path.clone(), "es");
        let mut de_strings =
            test_utilities::res::setup_empty_strings_for_locale(res_path.clone(), "de");
        let mut zh_strings =
            test_utilities::res::setup_empty_strings_for_locale(res_path.clone(), "zh");

        let mut localized_dir_path = temp_dir.path().to_path_buf();
        localized_dir_path.push("localized");
        let mut localized_file_path = localized_dir_path.clone();
        localized_file_path.push("localized.csv");

        // Write out required contents into files
        xml_writer::write(
            &mut default_strings.file,
            vec![
                AndroidString::localizable("s1", "english value 1"),
                AndroidString::localizable("s2", "english value 2"),
            ],
        )
        .unwrap();

        xml_writer::write(
            &mut fr_strings.file,
            vec![
                AndroidString::localizable("s1", "french old value 1"),
                AndroidString::localizable("s2", "french old value 2"),
            ],
        )
        .unwrap();

        xml_writer::write(
            &mut es_strings.file,
            vec![
                AndroidString::localizable("s1", "spanish old value 1"),
                AndroidString::localizable("s2", "spanish old value 2"),
            ],
        )
        .unwrap();

        let german_android_strings = vec![
            AndroidString::localizable("s1", "german old value 1"),
            AndroidString::localizable("s2", "german old value 2"),
        ];

        xml_writer::write(&mut de_strings.file, german_android_strings.clone()).unwrap();

        let chinese_android_strings = vec![
            AndroidString::localizable("s1", "chinese old value 1"),
            AndroidString::localizable("s2", "chinese old value 2"),
        ];

        xml_writer::write(&mut zh_strings.file, chinese_android_strings.clone()).unwrap();

        fs::create_dir_all(localized_dir_path.clone()).unwrap();
        let mut localized_file = File::create(localized_file_path.clone()).unwrap();
        localized_file
            .write(
                r#"string_name, default_locale, french, spanish, german, chinese
s1, english value 1, french new value 1,,german new value 1,chinese old value 1
s2, english value 2,,spanish new value 2,german new value 2,            "#
                    .as_bytes(),
            )
            .unwrap();

        // Not including german in this map to make sure that mappings also work as a filter
        let mut map = HashMap::new();
        map.insert(String::from("french"), String::from("fr"));
        map.insert(String::from("spanish"), String::from("es"));
        map.insert(String::from("chinese"), String::from("zh"));

        // Perform action
        let created_output_files_path = super::localized(
            res_path.clone().to_str().unwrap(),
            localized_file_path.to_str().unwrap(),
            map,
        )
        .unwrap();

        // Assert appropriate output
        test_utilities::list::assert_strict_list_eq(
            created_output_files_path,
            vec![fr_strings.path, es_strings.path],
        );

        test_utilities::list::assert_strict_list_eq(
            xml_utilities::read_foreign_strings(&res_path, "fr")
                .unwrap()
                .into_strings(),
            vec![
                AndroidString::localizable("s1", "french new value 1"),
                AndroidString::localizable("s2", "french old value 2"),
            ],
        );

        test_utilities::list::assert_strict_list_eq(
            xml_utilities::read_foreign_strings(&res_path, "es")
                .unwrap()
                .into_strings(),
            vec![
                AndroidString::localizable("s1", "spanish old value 1"),
                AndroidString::localizable("s2", "spanish new value 2"),
            ],
        );

        // German must not have changed since it wasn't included in the mapping
        test_utilities::list::assert_strict_list_eq(
            xml_utilities::read_foreign_strings(&res_path, "de")
                .unwrap()
                .into_strings(),
            german_android_strings,
        );

        // Chinese must not have changed since the localized text only container blank string
        // & already present localized value
        test_utilities::list::assert_strict_list_eq(
            xml_utilities::read_foreign_strings(&res_path, "zh")
                .unwrap()
                .into_strings(),
            chinese_android_strings,
        );
    }

    #[test]
    fn writable_empty_foreign_strings_file_creates_file() {
        let res_path = tempfile::tempdir().unwrap();

        let mut fr_strings = test_utilities::res::setup_empty_strings_for_locale(&res_path, "fr");
        fr_strings
            .file
            .write("example old content".as_bytes())
            .unwrap();

        let (mut file_with_new_content, file_path) =
            super::writable_empty_foreign_strings_file(res_path.path(), "fr").unwrap();
        file_with_new_content
            .write("example new content".as_bytes())
            .unwrap();

        let mut file_contents = String::new();
        File::open(&fr_strings.path)
            .unwrap()
            .read_to_string(&mut file_contents)
            .unwrap();

        assert_eq!(file_contents, "example new content");
        assert_eq!(file_path, fr_strings.path)
    }
}
