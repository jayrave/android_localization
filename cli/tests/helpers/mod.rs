use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn assert_equality_of_file_contents(
    file1_dir_path: &str,
    file1_filename: &str,
    file2_dir_path: &str,
    file2_filename: &str,
) {
    let file1_contents = read_file_contents(file1_dir_path, file1_filename);
    let file2_contents = read_file_contents(file2_dir_path, file2_filename);
    assert_eq!(file1_contents, file2_contents)
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
