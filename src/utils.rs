use std::{fs, io};

pub fn read_file(path: &str) -> io::Result<String> {
    Ok(fs::read_to_string(path)?.to_string())
}

pub fn must_read_file(path: &str) -> String {
    read_file(path).unwrap()
}
