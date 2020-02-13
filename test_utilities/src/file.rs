use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn read_content<P: AsRef<Path>>(file_path: P) -> String {
    let mut content = String::new();
    let mut file = File::open(file_path).unwrap();
    file.read_to_string(&mut content);
    content
}

pub fn write_content<P: AsRef<Path>, S: Into<String>>(file_path: P, content: S) {
    let mut file = File::create(file_path).unwrap();
    file.write(content.into().as_bytes()).unwrap();
}
