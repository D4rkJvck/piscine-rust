pub fn num_to_ordinal(x: u32) -> String {
    match x {
        x if x % 10 == 1 && x != 11 => format!("{x}st"),
        x if x % 10 == 2 && x != 12 => format!("{x}nd"),
        x if x % 10 == 3 && x != 13 => format!("{x}rd"),
        _ => format!("{x}th"),
    }
}
