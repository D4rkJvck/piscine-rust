pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    fn eql(a: f64, b: f64) -> bool {
        (b - a).abs() < EPSILON
    }

    #[test]
    fn test_f_to_c() {
        let temp_f = 20.0;
        println!("{}°F = {}°C", temp_f, fahrenheit_to_celsius(temp_f));
        assert!(eql(fahrenheit_to_celsius(temp_f), -6.666666666666666));
        let temp_f = 83.0;
        println!("{}°F = {}°C", temp_f, fahrenheit_to_celsius(temp_f));
        assert!(eql(fahrenheit_to_celsius(temp_f), 28.333333333333332));
    }

    #[test]
    fn test_c_to_f() {
        let temp_c = 27.0;
        println!("{}°C = {}°F", temp_c, fahrenheit_to_celsius(temp_c));
        assert!(eql(celsius_to_fahrenheit(27.0), 80.6));
        let temp_c = 0.0;
        println!("{}°C = {}°F", temp_c, fahrenheit_to_celsius(temp_c));
        assert!(eql(celsius_to_fahrenheit(temp_c), 32.0))
    }
}
