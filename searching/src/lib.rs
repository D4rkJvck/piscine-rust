pub fn search(array: &[i32], key: i32) -> Option<usize> {
    Some(array
        .iter()
        .enumerate()
        .find(|(_i, k)| **k == key)
        .unwrap().0
    )
}