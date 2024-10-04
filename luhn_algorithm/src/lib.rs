pub fn is_luhn_formula(code: &str) -> bool {
    if code.len() < 2 {
        return false;
    }

    let mut is_second = false;
    let mut sum = 0;

    let code: String = code.chars().filter(|c| !c.is_whitespace()).collect();

    for c in code.chars().rev() {
        match c.to_string().parse::<i32>() {
            Err(_) => return false,
            Ok(mut num) => {
                if is_second {
                    num *= 2;

                    if num > 9 {
                        num -= 9
                    }
                }

                sum += num;
                is_second = !is_second;
            }
        }
    }

    sum % 10 == 0
}