use std::fs::read_to_string;

use day_07::process_puzzle;

fn main() {
    let input_str = read_to_string("./day-07/puzzle_input.txt").expect("unable to read puzzle input");
    println!("day-07 part 1 result: {}", process_puzzle(&input_str, false));
}
