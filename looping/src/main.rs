use std::io;

fn main() {
    let riddle = String::from("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
    let mut tries = 0;
    let answer = String::from("The letter e");
    
    loop {
        let mut input = String::new();
        
        println!("{:?}", &riddle);

        io::stdin().read_line(&mut input).unwrap();
        println!("{:?}", &input);

        if input.trim() == &answer {
            println!("{:?}", tries);
            break;
        };

        tries += 1;
    }
}
