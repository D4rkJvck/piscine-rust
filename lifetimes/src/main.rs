use lifetimes::*;

fn main() {
    let person = Person::new("Leo");

    println!("Person = {:?}", person);
}

// $ cargo run
// Person = Person { name: "Leo", age: 0 }
// $
