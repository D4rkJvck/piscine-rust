pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    match vec.get(index) {
        Some(s) => s.to_string(),
        None => String::from("Index out of bounds"),
    }
}
