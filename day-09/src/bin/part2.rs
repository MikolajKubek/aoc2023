use std::fs::read_to_string;

use day_09::process_puzzle;

fn main() {
    let input_str = read_to_string("./day-09/puzzle_input.txt").expect("unable to read puzzle input");
    println!("day-09 part 2 result: {}", process_puzzle(&input_str, true));
}
