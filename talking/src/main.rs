use talking::*;

fn main() {
    println!("{:?}", talking("JUST DO IT!"));
    println!("{:?}", talking("Hello how are you?"));
    println!("{:?}", talking("WHAT'S GOING ON?"));
    println!("{:?}", talking("something"));
    println!("{:?}", talking(""));
}

// $ cargo run
// "There is no need to yell, calm down!"
// "Sure."
// "Quiet, I am thinking!"
// "Interesting"
// "Just say something!"
// $
