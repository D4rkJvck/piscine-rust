pub fn spell(n: u64) -> String {
    if n == 0 {
        String::from("zero")
    } else {
        spelling(n)
    }
}

fn spelling(n: u64) -> String {
    match n {
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        15 => String::from("fifteen"),
        14 | 16 | 17 | 18 | 19 => spelling(n % 10) + "teen",
        20 => String::from("twenty"),
        30 => String::from("thirty-"),
        50 => String::from("fifty-"),
        n if n >= 1_000_000 => spelling(n / 1_000_000) + " million " + &spelling(n % 1_000_000),
        n if n >= 1_000 => spelling(n / 1_000) + " thousand " + &spelling(n % 1_000),
        n if n >= 100 => spelling(n / 100) + " hundred " + &spelling(n % 100),
        n if n > 10 => spelling(n / 10) + "ty-" + &spelling(n % 10),
        _ => String::from(""),
    }
}