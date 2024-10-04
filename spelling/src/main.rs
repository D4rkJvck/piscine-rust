use spelling::*;

fn main() {
    println!("{}", spell(0));
    println!("{}", spell(1));
    println!("{}", spell(14));
    println!("{}", spell(96));
    println!("{}", spell(100));
    println!("{}", spell(101));
    println!("{}", spell(348));
    println!("{}", spell(1_002));
    println!("{}", spell(9_996));
    println!("{}", spell(1_000_000));
}

//<- $ cargo run
//-> three hundred forty-eight
//-> nine thousand nine hundred ninety-six
//<- $
