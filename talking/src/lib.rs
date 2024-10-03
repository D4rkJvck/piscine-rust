pub fn talking(text: &str) -> &str {
    match text {
        s if s.is_empty() => "Just say something!",
        s if (is_question(s) && is_yelling(s)) => "Quiet, I am thinking!",
        s if is_yelling(s) => "There is no need to yell, calm down!",
        s if is_question(s) => "Sure.",
        _ => "Interesting",
    }
}

//________________________________________________________________
//

fn is_question(s: &str) -> bool {
    s.ends_with('?')
}

fn is_yelling(string: &str) -> bool {
    !string.chars().any(|c| c.is_ascii_lowercase())
}
