use std::fs::File;
use std::fs;
use panic::*;

fn main() {
    let filename = "created.txt";
    File::create(filename).unwrap();

    let a = open_file(filename);
    println!("{:?}", a);
    
    fs::remove_file(filename).unwrap();

    //It must panic
    let _ = open_file(filename);
}

// $ cargo run
// File { fd: 3, path: "panic/a.txt", read: true, write: false }
// thread 'main' panicked at 'File not found'
// $
