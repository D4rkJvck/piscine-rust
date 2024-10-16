pub fn get_products(vec: Vec<usize>) -> Vec<usize> {
    if vec.len() < 2 {
        return vec![]
    };
    
    let product: usize = vec
        .iter()
        .product();

    vec
        .iter()
        .map(|&x| product / x)
        .collect()
}
