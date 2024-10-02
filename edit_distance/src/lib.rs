pub fn edit_distance(src: &str, tgt: &str) -> usize {
    let m = src.len();
    let n = tgt.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i; // Deleting all characters from src
    }

    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            if src.chars().nth(i - 1) == tgt.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1]; // Match
            } else {
                dp[i][j] = *[
                    dp[i - 1][j] + 1,     // Deletion
                    dp[i][j - 1] + 1,     // Insertion
                    dp[i - 1][j - 1] + 1, // Substitution
                ]
                .iter()
                .min()
                .unwrap();
            }

            if i > 1 && j > 1 && src.chars().nth(i - 1) == tgt.chars().nth(j - 2) && src.chars().nth(i - 2) == tgt.chars().nth(j - 1) {
                dp[i][j] = dp[i][j].min(dp[i - 2][j - 2] + 1); // Transposition
            }
        }
    }

    dp[m][n]
}

//----------------------------------------------------------------------------------------------------------------------------------------

// #[allow(unreachable_patterns)]
// pub fn edit_distance(src: &str, tgt: &str) -> usize {
//     let s: Vec<char> = src.chars().collect();
//     let t: Vec<char> = tgt.chars().collect();
//     let transposable = src.len() >= 2 && tgt.len() >= 2 && s[0] == t[1] && s[1] == t[0];

//     match (s.as_slice(), t.as_slice()) {
//         (&[], t) => t.len(),
//         (s, &[]) => s.len(),
//         (&[a, ..], &[b, ..]) => {
//             if a == b {
//                 edit_distance(&src[1..], &tgt[1..]) // Match
//             } else if transposable {
//                 1 + edit_distance(&src[2..], &tgt[2..]) // Transposition
//             } else {
//                 1 + [
//                     edit_distance(&src[1..], tgt),       // Deletion
//                     edit_distance(src, &tgt[1..]),       // Insertion
//                     edit_distance(&src[1..], &tgt[1..]), // Mismatch
//                 ]
//                 .iter()
//                 .min()
//                 .unwrap()
//             }
//         }
//         _ => unreachable!(),
//     }
// }
