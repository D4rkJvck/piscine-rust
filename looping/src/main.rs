use std::io;

fn main() {
    let mut input = String::new();
    let answer = String::from("The letter e");
    let mut trials = 1;

    let result = loop {
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read...");

        if input.trim() == &answer {
            break trials
        };
        
        trials += 1;
        input.clear();
    };

    println!("Number of trials: {result}");
}
