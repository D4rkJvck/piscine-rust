pub fn talking(text: &str) -> &str {
    match text {
        s if is_yelling(s) && is_asking(s) => "Quiet, I am thinking!",
        s if is_yelling(s) => "There is no need to yell, calm down!",
        s if is_asking(s) => "Sure.",
        s if !is_saying(s) => "Just say something!",
        _ => "Interesting",
    }
}

//_________________________________________________________________
//

fn is_asking(s: &str) -> bool {
    s.ends_with('?')
}

fn is_saying(s: &str) -> bool {
    s.chars().any(|c| c.is_alphabetic())
}

fn is_yelling(s: &str) -> bool {
    is_saying(s) && !s.chars().any(|c| c.is_lowercase())
}
