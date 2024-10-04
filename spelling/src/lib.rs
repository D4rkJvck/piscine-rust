pub fn spell(n: u64) -> String {
    if n == 0 {
        String::from("zero")
    } else {
        spelling(n).trim().to_string()
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
        21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 => "twenty-".to_owned() + &spelling(n % 10),
        30 => String::from("thirty"),
        31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 => "thirty-".to_owned() + &spelling(n % 10),
        50 => String::from("fifty"),
        51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 => "fifty-".to_owned() + &spelling(n % 10),
        n if n >= 1_000_000 => spelling(n / 1_000_000) + " million " + &spelling(n % 1_000_000),
        n if n >= 1_000 => spelling(n / 1_000) + " thousand " + &spelling(n % 1_000),
        n if n >= 100 => spelling(n / 100) + " hundred " + &spelling(n % 100),
        n if n > 10 => spelling(n / 10) + "ty-" + &spelling(n % 10),
        _ => String::from(""),
    }
}
