use cipher::*;

fn main() {
    println!("{:?}", cipher("1Hello 2world!", "1Svool 2dliow!"));
    println!("{:?}", cipher("1Hello 2world!", "svool"));
    println!("{:?}", cipher("", "svool"));
}

// $ cargo run
// Some(Ok(true))
// Some(Err(CipherError { validation: false, expected: "1Svool 2dliow!" }))
// None
// $
