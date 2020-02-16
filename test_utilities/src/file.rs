use crate::eq;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn read_content<P: AsRef<Path>>(file_path: P) -> String {
    let mut content = String::new();
    let mut file = File::open(file_path).unwrap();
    file.read_to_string(&mut content).unwrap();
    content
}

pub fn write_content<P: AsRef<Path>, S: Into<String>>(file_path: P, content: S) {
    let mut file = File::create(file_path).unwrap();
    file.write_all(content.into().as_bytes()).unwrap();
}

pub fn assert_eq_of_file_contents(actual_file_path: &str, expected_file_path: &str) {
    let actual_file_lines = read_file_contents_as_lines(actual_file_path);
    let expected_file_lines = read_file_contents_as_lines(expected_file_path);
    assert_eq!(actual_file_lines, expected_file_lines)
}

pub fn assert_eq_of_file_contents_to_either_or(
    actual_file_path: &str,
    expected_file_path1: &str,
    expected_file_path2: &str,
) {
    eq::assert_eq_to_either_or(
        read_file_contents_as_lines(actual_file_path),
        read_file_contents_as_lines(expected_file_path1),
        read_file_contents_as_lines(expected_file_path2),
    );
}

fn read_file_contents_as_lines(file_path: &str) -> Vec<String> {
    // By default, the writers we employ, use \n as line terminator which
    // wouldn't match when run on Windows! To work around this, we are using
    // lines instead (`String#lines` takes care of handling both \n & \r\n)
    read_content(file_path).lines().map(String::from).collect()
}
