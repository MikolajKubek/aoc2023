use std::fs::read_to_string;

use day_05::process_puzzle;

fn main() {
    let input_str = read_to_string("./day-05/puzzle_input.txt").expect("unable to read puzzle input");
    println!("day-05 part 1 result: {}", process_puzzle(&input_str, false));
}
