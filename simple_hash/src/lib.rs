use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut w_map: HashMap<&str, usize> = HashMap::new();

    for w in words.iter() {
        if !w_map.contains_key(w) {
            w_map.insert(w, 1);
            continue;
        }

        *w_map.get_mut(w).unwrap() += 1;
    }

    w_map
}

//_____________________________________________________________________________
//

pub fn nb_distinct_words(w_map: &HashMap<&str, usize>) -> usize {
    w_map.len()
}
