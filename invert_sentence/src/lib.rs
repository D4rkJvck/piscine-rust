pub fn invert_sentence(string: &str) -> String {
    string
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}
