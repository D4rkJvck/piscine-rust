pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            name.split_whitespace()
                .map(|inits| format!("{}. ", inits.chars().next().unwrap_or_default()))
                .collect()
        })
        .collect()
}
