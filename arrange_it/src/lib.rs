pub fn arrange_phrase(phrase: &str) -> String {
    let mut arranged_phrase = String::new();
    let words: Vec<&str> = phrase.split_whitespace().collect();

    for i in 1..words.len() + 1 {
        words.iter().map(|word| {
            if word.contains(stringify!(i) {
                words.swap(a, b);
            };
    }));
    }

    arranged_phrase
}