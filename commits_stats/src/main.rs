use commits_stats::{commits_per_week, commits_per_author};
use std::fs;

fn main() {
	let contents = fs::read_to_string("commits.json").unwrap();
	let serialized = json::parse(&contents).unwrap();
	println!("{:#?}", commits_per_week(&serialized));
	println!("{:#?}", commits_per_author(&serialized));
}

// $ cargo run
// {"2020-W44": 5, "2020-W36": 1, "2020-W31": 1, ... ,"2020-W45": 4, "2020-W46": 4}
// {"homembaixinho": 2, "mwenzkowski": 3, ... ,"tamirzb": 1, "paul-ri": 2, "RPigott": 1}
// $
