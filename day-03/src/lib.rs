use matrix::matrix::{Matrix, MatrixTrait};
use num_reader::num_reader::NumReader;

mod matrix;
mod num_reader;

pub fn process_part1(input_str: &str) -> u32 {
    read_input_matrix(input_str)
}

fn read_input_matrix(input_str: &str) -> u32 {
    let matrix_height = input_str.lines().count();
    let matrix_width = input_str.lines().nth(0)
        .expect("unable to read first line of input")
        .len();

    let mut matrix = Matrix::new(matrix_height, matrix_width);
    let mut x: usize;
    let mut y: usize = 0;

    let mut num_reader = NumReader::new();

    for line in input_str.lines() {
        x = 0;
        for current_char in line.chars() {
            if current_char.is_digit(10) {
                num_reader.read_char(current_char, x, y);
            }
            else {
                num_reader.finish_reading();                
            }
            matrix.set(x, y, current_char);
            x += 1;
        }
        y += 1;
    }

    for y in 0..matrix_height {
        for x in 0..matrix_width {
            print!("{}", matrix.get(x, y));
        }
        println!("");
    }

    let mut sum_numbers = 0;
    for number in num_reader.get_numbers() {
        println!("{:?}", number);
        for indices in &number.indices {
            let neighbour_chars = matrix.get_neighbour_set(indices[0], indices[1]);
            println!("{:?}", neighbour_chars);
            if !neighbour_chars.is_empty() {
                sum_numbers += number.value;
                break;
            }

        }
    }
    
    sum_numbers
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
}
