use string_permutation::is_permutation;

pub fn is_anagram(s1: &str, s2: &str) -> bool {
    is_permutation(s1, s2)
}