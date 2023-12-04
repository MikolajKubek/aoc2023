use std::fs::read_to_string;

use day_03::process_part2;

fn main() {
    let input_str = read_to_string("./day-03/puzzle_input.txt").expect("unable to read puzzle input");
    println!("day-03 part 2 result: {}", process_part2(&input_str));
}
