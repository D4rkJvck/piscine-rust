use std::{collections::HashMap, hash::Hash};

pub fn slices_to_map<'a, K, V>(keys: &'a [K], values: &'a [V]) -> HashMap<&'a K, &'a V>
where K: Eq + Hash, V: Eq {
    keys.iter().zip(values).collect()
}
