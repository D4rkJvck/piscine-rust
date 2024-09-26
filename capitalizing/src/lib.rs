pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return input.to_string()
    }
    
    input
        .chars()
        .next()
        .unwrap_or_default()
        .to_uppercase()
        .collect::<String>()
        + &input[1..]
}

//________________________________________________________
//

pub fn title_case(input: &str) -> String {
    input
    .split_whitespace()
    .map(|word| capitalize_first(word))
    .collect::<Vec<String>>()
    .join(" ")
}

//________________________________________________________
//

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .into_iter()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_uppercase().collect::<String>()
            }
        })
        .collect()
}
