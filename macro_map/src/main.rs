use macro_map::hash_map;
use std::collections::HashMap;

fn main() {
    let empty: HashMap<u32, u32> = hash_map!();
    let new = hash_map!('a' => 22, 'b' => 1, 'c' => 10);
    let nested = hash_map!(
        "first" => hash_map!(
            "Rob" => 32.2,
            "Gen" => 44.1,
            "Chris" => 10.,
        ),
        "second" => hash_map!()
    );
    println!("{:?}", empty);
    println!("{:?}", new);
    println!("{:?}", nested);
}


// $ cargo run
// {}
// {'b': 1, 'a': 22, 'c': 10}
// {"first": {"Rob": 32.2, "Gen": 44.1, "Chris": 10.0}, "second": {}}
// $
