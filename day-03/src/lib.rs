use matrix::matrix::{Matrix, MatrixTrait};
use num_reader::num_reader::NumReader;

mod matrix;
mod num_reader;

pub fn process_part1(input_str: &str) -> u32 {
    read_input_matrix(input_str, false)
}

pub fn process_part2(input_str: &str) -> u32 {
    read_input_matrix(input_str, true)
}

fn read_input_matrix(input_str: &str, part2: bool) -> u32 {
    let matrix_height = input_str.lines().count();
    let matrix_width = input_str.lines().nth(0)
        .expect("unable to read first line of input")
        .len();

    let mut matrix = Matrix::new(matrix_height, matrix_width);
    let mut x: usize;
    let mut y: usize = 0;

    let mut num_reader = NumReader::new();
    let mut potential_gears: Vec<[usize; 2]> = vec![];

    for line in input_str.lines() {
        x = 0;
        for current_char in line.chars() {
            if current_char.is_digit(10) {
                num_reader.read_char(current_char, x, y);
            }
            else if current_char == '*' {
                potential_gears.push([x, y]);
                num_reader.finish_reading();
            }
            else {
                num_reader.finish_reading();                
            }
            matrix.set(x, y, current_char);
            x += 1;
        }
        y += 1;
    }

    if !part2 {
        let mut sum_numbers = 0;
        for number in num_reader.get_numbers() {
            for indices in &number.indices {
                let neighbour_chars = matrix.get_neighbour_set(indices[0], indices[1]);
                if !neighbour_chars.is_empty() {
                    sum_numbers += number.value;
                    break;
                }

            }
        }
        
        return sum_numbers
    }
    else {
        let mut sum_ratios = 0;
        for potential_gear in potential_gears {
            let mut gear_ratio = 1;
            let mut num_parts = 0;

            for number in num_reader.get_numbers() {
                for indices in &number.indices {
                    if are_neighbours(&potential_gear, indices) {
                        gear_ratio *= number.value;
                        num_parts += 1;
                        break;
                    }
                }
            }
            if num_parts == 2 {
                sum_ratios += gear_ratio;
            }
        }

        sum_ratios
    }
}

fn are_neighbours(point_a: &[usize; 2], point_b: &[usize; 2]) -> bool {
    let (x_a, y_a) = (point_a[0] as i32, point_a[1] as i32);
    let (x_b, y_b) = (point_b[0] as i32, point_b[1] as i32);

    (x_a - x_b).abs() <= 1 && (y_a - y_b).abs() <= 1
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example_input() {
        let input_str = read_to_string("example_input.txt").expect("unable to read example_input");
        assert_eq!(4361, process_part1(&input_str));
    }

    #[test]
    fn test_example_input_part2() {
        let input_str = read_to_string("example_input.txt").expect("unable to read example_input");
        assert_eq!(467835, process_part2(&input_str));
    }
}
