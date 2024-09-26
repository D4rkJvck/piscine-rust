#[allow(unreachable_patterns)]
pub fn edit_distance(src: &str, tgt: &str) -> usize {
    let s: Vec<char> = src.chars().collect();
    let t: Vec<char> = tgt.chars().collect();
    let long_enough = src.len() >= 2 && tgt.len() >= 2;
    let transposable = s[0] == t[1] && s[1] == t[0];

    match (s.as_slice(), t.as_slice()) {
        (&[], t) => t.len(),
        (s, &[]) => s.len(),
        (&[a, ..], &[b, ..]) => {
            if a == b {
                edit_distance(&src[1..], &tgt[1..]) // Match
            } else if long_enough && transposable {
                edit_distance(&src[2..], &tgt[2..]) // Transposition
            } else {
                1 + [
                    edit_distance(&src[1..], tgt),       // Deletion
                    edit_distance(src, &tgt[1..]),       // Insertion
                    edit_distance(&src[1..], &tgt[1..]), // Mismatch
                ]
                .iter()
                .min()
                .unwrap()
            }
        }
        _ => unreachable!(),
    }
}
