pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut initials: Vec<String> = Vec::new();

    for name in names {
        let initial = name
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|inits| inits
                .chars()
                .next()
                .unwrap_or_default()
                .to_string() + ".")
            .collect::<Vec<String>>()
            .join(" ");

        initials.push(initial);
    }

    initials
}

// pub fn initials(names: Vec<&str>) -> Vec<String> {
//     names
//         .iter()
//         .map(|name| {
//             name.split_whitespace()
//                 .collect::<Vec<&str>>()
//                 .iter()
//                 .map(|inits| inits
//                     .chars()
//                     .next()
//                     .unwrap_or_default()
//                     .to_string() + ".")
//                 .collect::<Vec<String>>()
//                 .join(" ")
//         })
//         .collect()
// }
