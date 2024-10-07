use string_permutation::is_permutation;

pub fn is_anagram(s1: &str, s2: &str) -> bool {
    let str_a = s1.to_lowercase();
    let str_b = s2.to_lowercase();
    is_permutation(str_a.as_str(), str_b.as_str())
}
