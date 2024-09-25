pub fn delete_and_backspace(s: &mut String) {
    signed_remove(s);
    signed_remove(s);
}

pub fn do_operations(s: &mut Vec<String>) {
    for op in s {
        if op.contains('+') {
            *op = op
                .split('+')
                .map(|num| num.parse::<i32>().unwrap())
                .sum::<i32>()
                .to_string()
        }

        if op.contains('-') {
            *op = op
                .split('-')
                .map(|num| num.parse::<i32>().unwrap())
                .reduce(|acc, num| acc - num)
                .unwrap()
                .to_string()
        }
    }
}

///////////////////////////////////////////////////////////////

fn signed_remove(s: &mut String) {
    let mut idx = 0;

    while idx < s.len() {
        while s.chars().nth(idx).unwrap_or_default() == '-' {
            s.remove(idx);

            if idx > 0 {
                s.remove(idx - 1);
                idx -= 1;
            }
        }

        idx += 1;
    }

    *s = s.chars().rev().collect::<String>().replace("+", "-");
}
