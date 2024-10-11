use roman_numbers::RomanNumber;

fn main() {
	println!("{:?}", RomanNumber::from(32));
	println!("{:?}", RomanNumber::from(900));
	println!("{:?}", RomanNumber::from(45));
	println!("{:?}", RomanNumber::from(19));
	println!("{:?}", RomanNumber::from(0));
	println!("{:?}", RomanNumber::from(400));
	println!("{:?}", RomanNumber::from(40));
	println!("{:?}", RomanNumber::from(4000));
	println!("{:?}", RomanNumber::from(200));
}

// $ cargo run
// RomanNumber([X, X, X, I, I])
// RomanNumber([I, X])
// RomanNumber([X, L, V])
// RomanNumber([Nulla])
// $