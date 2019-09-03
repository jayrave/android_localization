use test_utilities;

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
    test_utilities::eq::assert_eq_to_either_or(
        read_file_contents_as_lines(actual_file_path),
        read_file_contents_as_lines(expected_file_path1),
        read_file_contents_as_lines(expected_file_path2),
    );
}

fn read_file_contents_as_lines(file_path: &str) -> Vec<String> {
    // By default, the writers we employ, use \n as line terminator which
    // wouldn't match when run on Windows! To work around this, we are using
    // lines instead (`String#lines` takes care of handling both \n & \r\n)
    test_utilities::file::read_content(file_path)
        .lines()
        .map(String::from)
        .collect()
}
