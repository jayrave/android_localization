use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn assert_eq_of_file_contents(
    actual_dir_path: &str,
    actual_filename: &str,
    expected_dir_path: &str,
    expected_filename: &str,
) {
    let actual_file_contents = read_file_contents(actual_dir_path, actual_filename);
    let expected_file_contents = read_file_contents(expected_dir_path, expected_filename);
    assert_eq!(actual_file_contents, expected_file_contents)
}

pub fn assert_eq_of_file_contents_to_either_or(
    actual_dir_path: &str,
    actual_filename: &str,
    expected_dir_path: &str,
    expected_filename1: &str,
    expected_filename2: &str,
) {
    let actual_file_contents = read_file_contents(actual_dir_path, actual_filename);
    let expected_file_contents1 = read_file_contents(expected_dir_path, expected_filename1);
    let expected_file_contents2 = read_file_contents(expected_dir_path, expected_filename2);

    let result1 = actual_file_contents == expected_file_contents1;
    let result2 = actual_file_contents == expected_file_contents2;
    assert!(
        result1 || result2,
        r#"Actual: {};
        Expected either
        {}
        or
        {}"#,
        actual_file_contents,
        expected_file_contents1,
        expected_file_contents2
    )
}

pub fn read_file_contents(dir_path: &str, filename: &str) -> String {
    let mut path = PathBuf::from(dir_path);
    path.push(filename);

    let mut file_contents = String::new();
    File::open(path.to_str().unwrap())
        .unwrap()
        .read_to_string(&mut file_contents)
        .unwrap();

    file_contents
}
