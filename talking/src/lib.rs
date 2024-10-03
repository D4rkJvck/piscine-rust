pub fn talking(text: &str) -> &str {
    match text {
        s if is_question(s) && is_yelling(s) => "Quiet, I am thinking!",
        s if is_yelling(s) => "There is no need to yell, calm down!",
        s if is_question(s) => "Sure.",
        s if s.is_empty() => "Just say something!",
        _ => "Interesting",
    }
}

//________________________________________________________________
//

fn is_question(s: &str) -> bool {
    s.ends_with('?')
}

fn is_yelling(string: &str) -> bool {
    !string.is_empty()
        && string
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase())
}
