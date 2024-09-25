pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            name.split_whitespace()
                .map(|initial| format!("{}. ", initial.chars().next().unwrap_or_default()))
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect()
}
