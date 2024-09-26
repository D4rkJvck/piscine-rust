use std::collections::HashMap;


pub fn is_permutation(s1: &str, s2: &str) -> bool {
    letter_freq_count(s1) == letter_freq_count(s2)
}

//__________________________________________________________
//

fn letter_freq_count(s: &str) -> HashMap<char, usize> {
    let mut l_map: HashMap<char, usize> = HashMap::new();

    for c in s.chars() {
        if !l_map.contains_key(&c) {
            l_map.insert(c, 1);
            continue;
        }

        *l_map.get_mut(&c).unwrap() += 1;
    }

    l_map
}