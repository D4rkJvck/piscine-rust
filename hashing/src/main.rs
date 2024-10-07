#[allow(unused)]
use hashing::*;

fn main() {
    println!("Hello, world!");
    let v = vec![4, 7, 5, 2, 5, 1, 3];
    println!("mean {}", hashing::mean(&v));
    println!("median {}", hashing::median(&v));
    println!("mode {}", hashing::mode(&v));
}

// $ cargo run
// mean 3.857142857142857
// median 4
// mode 5
// $
