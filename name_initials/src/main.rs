use name_initials::initials;

fn main() {
    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
    println!("{:?}", initials(names));
}

// $ cargo run
// ["H. P.", "S. E.", "J. L.", "B. O."]
// $
