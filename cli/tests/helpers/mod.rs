use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use test_helpers;

pub fn assert_eq_of_file_contents(
    actual_dir_path: &str,
    actual_filename: &str,
    expected_dir_path: &str,
    expected_filename: &str,
) {
    let actual_file_lines = read_file_contents_as_lines(actual_dir_path, actual_filename);
    let expected_file_lines = read_file_contents_as_lines(expected_dir_path, expected_filename);
    assert_eq!(actual_file_lines, expected_file_lines)
}

pub fn assert_eq_of_file_contents_to_either_or(
    actual_dir_path: &str,
    actual_filename: &str,
    expected_dir_path: &str,
    expected_filename1: &str,
    expected_filename2: &str,
) {
    test_helpers::assert_eq_to_either_or(
        read_file_contents_as_lines(actual_dir_path, actual_filename),
        read_file_contents_as_lines(expected_dir_path, expected_filename1),
        read_file_contents_as_lines(expected_dir_path, expected_filename2)
    );
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

fn read_file_contents_as_lines(dir_path: &str, filename: &str) -> Vec<String> {
    // By default, the writers we employ, use \n as line terminator which
    // wouldn't match when run on Windows! To work around this, we are using
    // lines instead (`String#lines` takes care of handling both \n & \r\n)
    read_file_contents(dir_path, filename).lines().map(String::from).collect()
}