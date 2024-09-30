#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> Self {
        CipherError {
            validation,
            expected,
        }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original == "" || ciphered == "" {
        None
    } else if ciphered == to_cipher(original) {
        Some(Ok(true))
    } else {
        Some(Err(CipherError::new(false, to_cipher(original))))
    }
}

//------------------------------------------------------------------------------------------------

fn to_cipher(s: &str) -> String {
    let mut ciphered = String::new();

    for c in s.chars() {
        match c {
            'a'..='z' => ciphered.push(('z' as u8 - c as u8 + 'a' as u8) as char),
            'A'..='Z' => ciphered.push(('Z' as u8 - c as u8 + 'A' as u8) as char),
            _ => ciphered.push(c),
        }
    }

    ciphered
}
