use std::collections::HashSet;

pub fn is_pangram(string: &str) -> bool {
    let mut set: HashSet<char> = HashSet::new();

    for c in string.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
        set.insert(c);
    }

    ('a'..='z').all(|c| set.contains(&c))
}
