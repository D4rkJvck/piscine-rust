pub use case::CaseExt;
pub use edit_distance::edit_distance;

pub fn expected_variable(to_cmp: &str, expected: &str) -> Option<String> {
    if !(is_camel(to_cmp) || is_snake(to_cmp) || to_cmp != "") {
        return None;
    }


    let diff = edit_distance(
        to_cmp.to_lowercase().as_str(),
        expected.to_ascii_lowercase().as_str(),
    );

    let similarity_percentage = (expected.len() - diff) * 100 / expected.len();

    if similarity_percentage > 50 {
        Some(format!("{}%", similarity_percentage))
    } else {
        None
    }
}

//_________________________________________________________________________________
//

fn is_camel(s: &str) -> bool {
    s.chars().any(|c| c.is_uppercase())
}

//-----------------------------------------------

fn is_snake(s: &str) -> bool {
    s.contains('_')
}