use report::ValueHistory;

mod report;

pub fn process_puzzle(input_str: &str, part_2: bool) -> i32 {
    let mut overall_sum = 0;
    for line in input_str.lines() {
        let values = line.split(" ").into_iter()
            .map(|value| i32::from_str_radix(value, 10).expect("unable to convert value to i32"))
            .collect::<Vec<i32>>();
        let value_history = ValueHistory::new(&values);
        overall_sum += match part_2 {
            false => value_history.calculate_right_extrapolation(),
            true => value_history.calculate_left_extrapolation(),
        }
    }

    overall_sum
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;

    #[test]
    fn test_example_input_part_1() {
        let input_str = read_to_string("example_input.txt").expect("unable to read example input");
        assert_eq!(114, process_puzzle(&input_str, false));
    }

    #[test]
    fn test_example_input_part_2() {
        let input_str = read_to_string("example_input.txt").expect("unable to read example input");
        assert_eq!(2, process_puzzle(&input_str, true));
    }
}
