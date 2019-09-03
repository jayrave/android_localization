use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn setup_values_dir_for_default_locale<P: AsRef<Path>>(res_path: P) -> String {
    setup_values_dir::<P, String>(res_path, None)
}

pub fn setup_values_dir_for_locale<P: AsRef<Path>, L: AsRef<str>>(
    res_path: P,
    locale_id: L,
) -> String {
    setup_values_dir(res_path, Some(locale_id))
}

pub fn setup_empty_strings_for_default_locale<P: AsRef<Path>>(res_path: P) -> String {
    let values_dir_path = setup_values_dir_for_default_locale(res_path);
    setup_strings_file::<String, String>(values_dir_path, None)
}

pub fn setup_empty_strings_for_locale<P: AsRef<Path>, L: AsRef<str>>(
    res_path: P,
    locale_id: L,
) -> String {
    let values_dir_path = setup_values_dir_for_locale(res_path, locale_id.as_ref().clone());
    setup_strings_file::<String, L>(values_dir_path, Some(locale_id))
}

fn setup_values_dir<P: AsRef<Path>, L: AsRef<str>>(res_path: P, locale_id: Option<L>) -> String {
    let mut locale_values_dir_path = PathBuf::from(res_path.as_ref());
    match locale_id {
        None => locale_values_dir_path.push("values"),
        Some(id) => locale_values_dir_path.push(format!("values-{}", id.as_ref())),
    };

    fs::create_dir_all(&locale_values_dir_path).unwrap();
    String::from(locale_values_dir_path.to_str().unwrap())
}

fn setup_strings_file<P: AsRef<Path>, L: AsRef<str>>(
    locale_values_dir_path: P,
    locale_id: Option<L>,
) -> String {
    let mut strings_file_path = PathBuf::from(locale_values_dir_path.as_ref());
    strings_file_path.push("strings.xml");
    File::create(&strings_file_path).unwrap();

    String::from(strings_file_path.to_str().unwrap())
}
