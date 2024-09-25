#[allow(unused)]
pub fn arrange_phrase(phrase: &str) -> String {
    let mut idx = 1;
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    for i in 0..words.len() {
        while words[i].contains(idx.to_string().as_str()) {
            &mut words.swap(i, idx - 1);
            idx += 1;
        }
    }

    words
        .join(" ")
        .chars()
        .filter(|c| !c.is_digit(10))
        .collect::<String>()
}
