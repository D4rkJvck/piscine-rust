pub fn talking(text: &str) -> &str {
    match text {
        s if said_nothing(s) => "Just say something!",
        s if is_yelling(s) && ask_question(s) => "Quiet, I am thinking!",
        s if ask_question(s) => "Sure.",
        s if is_yelling(s) => "There is no need to yell, calm down!",
        _ => "Interesting",
    }
}

//_________________________________________________________________
//

fn is_yelling(s: &str) -> bool {
    !s.chars().any(|c| c.is_lowercase())
}

fn ask_question(s: &str) -> bool {
    s.ends_with('?')
}

fn said_nothing(s: &str) -> bool {
    !s.chars().any(|c| c.is_alphabetic()) && !ask_question(s)
}
