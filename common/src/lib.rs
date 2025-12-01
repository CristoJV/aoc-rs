use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("File cannot be read at: {}", path))
}