pub fn binary_search(sorted_list: &[i32], target: i32) -> Option<usize> {
    if sorted_list.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = sorted_list.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if sorted_list[mid] == target {
            return Some(mid);
        }

        if sorted_list[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}
