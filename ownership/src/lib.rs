pub fn first_subword(s: String) -> String {
    for (i, c) in s.chars().enumerate() {
        if i != 0 && (c == '_' || c.is_uppercase()) {
            return s[0..i].to_string();
        }
    }

    s
}