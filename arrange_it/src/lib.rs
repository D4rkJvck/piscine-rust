pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    words.sort_by_key(|word| {
        let mut number = 0;

        for c in word.chars() {
            if c.is_digit(10) {
                number = c.to_digit(10).unwrap();
                break;
            }
        }

        number
    });

    words
        .join(" ")
        .chars()
        .filter(|c| !c.is_digit(10))
        .collect::<String>()
}
