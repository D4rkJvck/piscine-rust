use simple_hash::*;
#[allow(unused)]
use std::collections::HashMap;

fn main() {
    let sentence = "this is a very basic sentence with only few \
                repetitions. once again this is very basic and \
                but it should be enough for basic tests"
        .to_string();
    let words = sentence.split(" ").collect::<Vec<&str>>();

    let frequency_count = word_frequency_counter(words);
    println!("{:?}", frequency_count);
    println!("{}", nb_distinct_words(&frequency_count));
}

// $ cargo run
// {"tests": 1, "with": 1, "this": 2, "it": 1, "enough": 1, "is": 2, "but": 1, "sentence": 1, "only": 1, "basic": 3, "again": 1, "for": 1, "be": 1, "once": 1, "very": 2, "should": 1, "few": 1, "and": 1, "a": 1, "repetitions.": 1}
// 20
// $
