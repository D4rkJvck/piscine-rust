pub fn get_products(vec: Vec<usize>) -> Vec<usize> {
    if vec.is_empty() {
        return vec![]
    };
    
    let mut result = vec![];

    for int in vec.iter() {
        let mut product = 1;

        for other in vec.iter() {
            if other != int {
                product *= other
            }
        }

        result.push(product);
    }

    result
}
