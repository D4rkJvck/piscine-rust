pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.iter()
        .map(|name| {
            name.split_whitespace()
                .map(|inits|
                    inits.chars()
                    .next()
                    .unwrap_or_default()
                    .to_string() + ".")
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect()
}
