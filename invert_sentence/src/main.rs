use invert_sentence::invert_sentence;

fn main() {
    println!("{}", invert_sentence("Rust is Awesome"));
    println!("{}", invert_sentence("   word1     word2  "));
    println!("{}", invert_sentence("Hello, World!"));
}

// $ cargo run | cat -e
// Awesome is Rust$
//   word2     word1   $
// World! Hello,$
// $
