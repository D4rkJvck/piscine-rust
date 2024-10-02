use std::collections::HashMap;

pub fn bigger(hash_map: HashMap<&str, i32>) -> i32 {
    *hash_map
        .into_values()
        .collect::<Vec<i32>>()
        .iter()
        .max()
        .unwrap()
}
