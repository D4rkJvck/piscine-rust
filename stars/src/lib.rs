use std::iter::repeat;

pub fn stars(n: u32) -> String {
    String::from_iter(repeat("*").take(2_usize.pow(n))).to_string()
}
