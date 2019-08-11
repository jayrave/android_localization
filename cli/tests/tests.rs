use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn one_locale_per_file_with_mapping() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_dir_path = temp_dir.path().to_str().unwrap();
    android_localization_cli::do_the_thing(vec![
        "does_not_matter",
        "localize",
        "--res",
        "./tests_data/localize/input",
        "--output",
        output_dir_path,
        "--mapping",
        "fr=french",
        "--mapping",
        "es=spanish",
    ])
    .unwrap();

    assert_equality_of_file_contents(
        "./tests_data/localize/output_one_locale_per_file_with_mapping/",
        "french.csv",
        temp_dir.path().to_str().unwrap(),
        "french.csv",
    );
    assert_equality_of_file_contents(
        "./tests_data/localize/output_one_locale_per_file_with_mapping/",
        "spanish.csv",
        temp_dir.path().to_str().unwrap(),
        "spanish.csv",
    );
}

#[test]
fn one_locale_per_file_without_mapping() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_dir_path = temp_dir.path().to_str().unwrap();
    android_localization_cli::do_the_thing(vec![
        "does_not_matter",
        "localize",
        "--res",
        "./tests_data/localize/input",
        "--output",
        output_dir_path,
    ])
    .unwrap();

    assert_equality_of_file_contents(
        "./tests_data/localize/output_one_locale_per_file_without_mapping/",
        "french.csv",
        temp_dir.path().to_str().unwrap(),
        "fr.csv",
    );
    assert_equality_of_file_contents(
        "./tests_data/localize/output_one_locale_per_file_without_mapping/",
        "spanish.csv",
        temp_dir.path().to_str().unwrap(),
        "es.csv",
    );
}

fn assert_equality_of_file_contents(
    file1_dir_path: &str,
    file1_filename: &str,
    file2_dir_path: &str,
    file2_filename: &str,
) {
    let file1_contents = read_file_contents(file1_dir_path, file1_filename);
    let file2_contents = read_file_contents(file2_dir_path, file2_filename);
    assert_eq!(file1_contents, file2_contents)
}

fn read_file_contents(dir_path: &str, filename: &str) -> String {
    let mut path = PathBuf::from(dir_path);
    path.push(filename);

    let mut file_contents = String::new();
    File::open(path.to_str().unwrap())
        .unwrap()
        .read_to_string(&mut file_contents)
        .unwrap();

    file_contents
}
