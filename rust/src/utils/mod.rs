use std::fs;

pub fn read_file(path: &str) -> String {
    let file = fs::read_to_string(path).expect("Error reading file");
    return file;
}
