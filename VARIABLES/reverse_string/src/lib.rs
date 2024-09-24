pub fn rev_str(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn test_reverse(input: &str, expected: &str) {
        assert_eq!(&rev_str(input), expected);
    }

    #[test]
    // testing just one word
    fn test_simple_word() {
        test_reverse("robot", "tobor");
        test_reverse("Ramen", "nemaR");
        test_reverse("I'm hungry!", "!yrgnuh m'I");
        test_reverse("racecar", "racecar");
        test_reverse("drawer", "reward");
        test_reverse("子猫", "猫子");
        test_reverse("", "");
    }

    #[test]
    // testing two or more words
    fn test_more_than_one() {
        test_reverse("Hello, world!", "!dlrow ,olleH");
        test_reverse("Hello, my name is Roman", "namoR si eman ym ,olleH");
        test_reverse("I have a nice car!", "!rac ecin a evah I");
        test_reverse("How old are You", "uoY era dlo woH");
        test_reverse("ex: this is an example água", "augá elpmaxe na si siht :xe");
    }
}
