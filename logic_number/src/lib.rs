use armstrong_number::is_armstrong_number;

pub fn number_logic(num: u32) -> bool {
    match is_armstrong_number(num) {
        Some(armstrong) => num == armstrong,
        None => false,
    }
}
