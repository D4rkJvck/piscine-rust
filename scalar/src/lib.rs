pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}

pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}

pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}

pub fn quo(a: i32, b: i32) -> i32 {
    a / b
}

pub fn rem(a: i32, b: i32) -> i32 {
    a % b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(sum(2, 2), 4);
        assert_eq!(diff(3, 9), -6);
        assert_eq!(pro(2, 2), 4);
        assert_eq!(quo(15, 5), 3);
        assert_eq!(rem(14, 2), 0);
    }
}
