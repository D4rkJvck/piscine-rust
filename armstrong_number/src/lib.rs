pub fn is_armstrong_number(nb: u32) -> Option<u32> {
    let digits = nb.to_string();
    let sum_pow: u32 = digits
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|digit| digit.pow(digits.len() as u32))
        .sum();

    if sum_pow == nb {
        Some(nb)
    } else {
        None
    }
}
