pub fn pig_latin(text: &str) -> String {
    let qu = text.to_string().find("qu");
    let mut count = 0;

    for c in text.chars() {
        if !is_vowel(c) {
            count += 1;
        } else {
            break;
        }
    };

    match (count, qu) {
        (c, _) if c == 0 => format!("{text}ay"),
        (c, q) => match q {
            Some(v) if v == 1 => format!("{}{}quay", &text[3..], &text.chars().next().unwrap()),
            Some(_) => format!("{}{}ay", &text[c..], &text[0..c]),
            None => format!("{}{}ay", &text[c..], &text[0..c])
        }
    }
}

//________________________________________________________________
//

fn is_vowel(c: char) -> bool {
    "aeiou".contains(c)
}
