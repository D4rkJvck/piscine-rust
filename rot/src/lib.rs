pub fn rotate(input: &str, key: i8) -> String {
    if key < -25 || key > 25 {
        return input.to_string();
    };

    let mut ciphered = String::new();
    let negative = if key < 0 { true } else { false };

    for c in input.chars() {
        match c {
            'a'..='z' => {
                if negative {
                    ciphered.push((b'z' - (key.abs() as u8 + b'z' - c as u8) % 26) as char);
                } else {
                    ciphered.push(((c as u8 - b'a' + key as u8) % 26 + b'a') as char);
                }
            }
            'A'..='Z' => {
                if negative {
                    ciphered.push((b'Z' - (key.abs() as u8 + b'Z' - c as u8) % 26) as char);
                } else {
                    ciphered.push(((c as u8 - b'A' + key as u8) % 26 + b'A') as char);
                }
            }
            _ => ciphered.push(c),
        }
    }

    ciphered
}
