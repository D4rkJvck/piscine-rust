use closures::*;

fn main() {
	println!("Hello, world!");
	let v1 = first_fifty_even_square();

	println!("All elements in {:?}, len = {}", v1, v1.len());
}


// $ cargo run
// All elements in [4, 16, 36, ..., 10000], len = 50
// $
