pub fn talking(text: &str) -> &str {
    match text {
        s if !s.chars().any(|c| c.is_alphabetic()) => "Just say something!",
        s if !s.chars().any(|c| c.is_lowercase()) && s.ends_with('?') => "Quiet, I am thinking!",
        s if s.ends_with('?') => "Sure.",
        s if !s.chars().any(|c| c.is_lowercase()) => "There is no need to yell, calm down!",
        _ => "Interesting",
    }
}
