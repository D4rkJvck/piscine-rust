use borrow::*;

fn main() {
    let s = "hello";
    let s1 = "camelCase".to_string();

    println!("\tstr_len(\"{}\") = {}", s, str_len(s));
    println!("\tstr_len(\"{}\") = {}", s1, str_len(&s1));
}

// $ cargo run
// str_len("hello") = 5
// str_len("camelCase") = 9
// $
