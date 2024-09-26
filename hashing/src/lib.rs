use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    if list.len() == 0 {
        0.0
    } else {
        list.iter().sum::<i32>() as f64 / list.len() as f64
    }
}

//______________________________________________________________________
//

pub fn median(list: &Vec<i32>) -> i32 {
    let mut sorted: Vec<&i32> = list.as_slice().into_iter().collect();
    sorted.sort();

    println!("{:?}", sorted); // DEBUG: Check Sorting...

    match sorted.len() {
        0 => 0,
        len if len % 2 != 0 => *sorted[len / 2],
        len if len % 2 == 0 => mean(&vec![*sorted[len / 2 - 1], *sorted[len / 2]]) as i32,
        _ => unreachable!(),
    }
}

//______________________________________________________________________
//

pub fn mode(list: &Vec<i32>) -> i32 {
    if list.len() == 0 {
        return 0;
    }

    let mut n_map: HashMap<i32, i32> = HashMap::new();

    for n in list.iter() {
        if !n_map.contains_key(n) {
            n_map.insert(*n, 1);
            continue;
        }

        *n_map.get_mut(n).unwrap() += 1;
    }

    // *n_map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0
    *n_map
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap()
        .0
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(mean(&list), 5.5);
    }

    #[test]
    fn test_mean_empty_slice() {
        let list = Vec::new();
        assert_eq!(mean(&list), 0.0)
    }

    #[test]
    fn test_mean_negative_numbers() {
        let list = vec![1, 2, 3, 4, -5, 6, 7, 8, 9, -10];
        assert_eq!(mean(&list), 2.5)
    }

    #[test]
    fn test_median_empty_slice() {
        let list = Vec::new();
        assert_eq!(median(&list), 0);
    }

    #[test]
    fn test_median_odd_sorted_slice() {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(median(&list), 5)
    }

    #[test]
    fn test_median_even_sorted_slice() {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(median(&list), 5)
    }

    #[test]
    fn test_median_odd_unsorted_slice() {
        let list = vec![3, 8, 5, 4, 9, 2, 7, 6, 1];
        assert_eq!(median(&list), 5)
    }

    #[test]
    fn test_median_even_unsorted_slice() {
        let list = vec![1, 2, 2, 4, 5, 7];
        assert_eq!(median(&list), 3)
    }

    #[test]
    fn test_mode_empty_slice() {
        let list = Vec::new();
        assert_eq!(mode(&list), 0)
    }

    #[test]
    fn test_mode() {
        let list = vec![3, 8, 5, 4, 7, 2, 1, 9, 6, 1];
        assert_eq!(mode(&list), 1)
    }

    #[test]
    #[ignore = "not covered yet..."]
    fn test_no_mode() {
        let list = vec![3, 8, 5, 4, 7, 2, 10, 9, 6, 1];
        assert_eq!(mode(&list), 3)
    }
}
