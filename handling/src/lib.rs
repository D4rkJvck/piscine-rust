use std::fs::{self, File};

pub fn open_or_create(file: &str, content: &str) {
    let _ = File::open(file).unwrap_or(File::create(file).unwrap());
    fs::write(file, content).unwrap();
}