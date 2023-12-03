use std::fs::read_to_string;

use day_02::process_part1;

fn main() {
    let input_str = read_to_string("./day-02/puzzle_input.txt").expect("unable to read puzzle input");
    println!("day-02 part 2 result: {}", process_part1(&input_str, true));
}
