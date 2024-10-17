use iterators::*;

fn main() {
    println!("{:?}", collatz(0));
    println!("{:?}", collatz(1));
    println!("{:?}", collatz(4));
    println!("{:?}", collatz(5));
    println!("{:?}", collatz(6));
    println!("{:?}", collatz(7));
    println!("{:?}", collatz(12));
}

// $ cargo run
// 0
// 0
// 2
// 5
// 8
// 16
// 9
// $
