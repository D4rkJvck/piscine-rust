pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, idx: usize) -> (&str, &str) {
    v.split_at(idx)
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).unwrap()
}
