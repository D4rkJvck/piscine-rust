pub fn first_fifty_even_square() -> Vec<i32> {
    (2..)
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .take(50)
        .collect()
}
