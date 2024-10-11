use chrono::{DateTime, Datelike };
pub use json::JsonValue;
pub use std::collections::HashMap;

pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    if data.is_array() {
        let _ = &data.members().into_iter().for_each(|elem| {
            let author_name = match &elem["commit"]["author"]["name"] {
                JsonValue::Short(name) => name.to_string(),
                JsonValue::String(name) => name.to_owned(),
                _ => "Not Found".to_string(),
            };

            *map.entry(author_name).or_insert(0) += 1;
        });
    }

    map
}

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    if data.is_array() {
        let _ = &data.members().into_iter().for_each(|elem| {
            let commit_date = match &elem["commit"]["committer"]["date"] {
                JsonValue::Short(date) => date.to_string(),
                JsonValue::String(date) => date.to_owned(),
                _ => "Not Found".to_string(),
            };

            let date = DateTime::parse_from_rfc3339(&commit_date).unwrap_or_default();
            let year = date.year();
            let week = date.iso_week().week();

            let week = format!("{}-W{:?}", year, week);

            *map.entry(week).or_insert(0) += 1;
        });
    }

    map
}
