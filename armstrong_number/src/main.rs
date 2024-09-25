use armstrong_number::*;

fn main() {
    println!("{:?}", is_armstrong_number(0));
    println!("{:?}", is_armstrong_number(1));
    println!("{:?}", is_armstrong_number(153));
    println!("{:?}", is_armstrong_number(370));
    println!("{:?}", is_armstrong_number(371));
    println!("{:?}", is_armstrong_number(407));
    println!("{:?}", is_armstrong_number(400));
    println!("{:?}", is_armstrong_number(198));
}

// $ cargo run
// Some(0)
// Some(1)
// Some(153)
// Some(370)
// Some(371)
// Some(407)
// None
// None
// $
