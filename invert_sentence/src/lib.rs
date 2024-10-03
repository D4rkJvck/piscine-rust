pub fn invert_sentence(string: &str) -> String {
    string
        .split(" ")
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}
