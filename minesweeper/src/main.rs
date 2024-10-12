use minesweeper::*;

fn main() {
    println!("{:?}", solve_board(&[]));
    println!("{:?}", solve_board(&[""]));
    println!("{:?}", solve_board(&["***"]));
    println!("{:#?}", solve_board(&["   ", " * ", "   ",]));
    println!("{:#?}", solve_board(&["*  ", "   ", "  *",]));
}

// $ cargo run
// []
// [""]
// ["***"]
// [
//     "111",
//     "1*1",
//     "111",
// ]
// [
//     "*1 ",
//     "121",
//     " 1*",
// ]
// $
