use to_url::*;

fn main() {
    let s = "Hello, world!";
    println!("{} to be use as an url is {}", s, to_url(s));
}

// $ cargo run
// Hello, world! to be use as an url is Hello,%20world!
// $
