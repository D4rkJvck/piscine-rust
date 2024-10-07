pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    Box::new(
        s.split_whitespace()
            .map(|n| {
                let s = if n.ends_with('k') {
                    n[..n.len() - 1].parse::<f64>().unwrap() * 1_000.0
                } else {
                    n.parse::<f64>().unwrap_or_default()
                };

                s as u32
            })
            .collect::<Vec<u32>>(),
    )
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}
