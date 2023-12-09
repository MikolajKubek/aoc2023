use std::fs::read_to_string;

use day_06::process_puzzle;

fn main() {
    let input_str = read_to_string("./day-06/puzzle_input_2.txt").expect("unable to read puzzle input");
    println!("day-06 part 2 result: {}", process_puzzle(&input_str));
}
