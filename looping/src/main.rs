use std::io;

fn main() {
    let mut input = String::new();
    let answer = String::from("The letter e");
    let mut trials = 1;

    loop {
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");

        io::stdin()
            .read_line(&mut input)
            .expect("ERROR NO INPUT...");

        if input.trim() == &answer {
            println!("Number of trials: {trials}");
            break;
        };

        trials += 1;
        input.clear();
    }
}
