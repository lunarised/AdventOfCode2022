use std::fs;

pub fn read_file(file_name: String) -> String {
    return fs::read_to_string(file_name).expect("Should have been able to read the file");
}
