use std::collections::HashMap;

pub fn is_pangram(string: &str) -> bool {
    let mut map: HashMap<char, bool> = HashMap::new();

    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        map.insert(c, false);
    }

    for c in string
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>()
    {
        if !map.get(&c).unwrap() {
            *map.get_mut(&c).unwrap() = true
        }
    }

    println!("{:#?}", map);

    map.iter().all(|(_, b)| *b == true)
}
